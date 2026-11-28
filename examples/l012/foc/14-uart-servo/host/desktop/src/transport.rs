use crate::protocol::crc;
use std::{
    io::{self, Read, Write},
    sync::{Arc, Mutex, mpsc},
    time::{Duration, Instant},
};

pub enum Action {
    Drive([u8; 23]),
    Tune([u16; 7]),
    Release,
    Stream(bool),
    Close,
}
#[derive(Default)]
pub struct Snapshot {
    pub ready: bool,
    pub flags: u8,
    pub rpm: i16,
    pub angle: i32,
    pub iq: i16,
    pub command: Option<[u8; 23]>,
    pub gains: Option<[u16; 7]>,
    pub waves: Vec<Vec<u8>>,
    pub notice: String,
    pub failed: bool,
    pub mode: u8,
}
pub struct Connection {
    pub actions: mpsc::Sender<Action>,
    pub state: Arc<Mutex<Snapshot>>,
}
impl Drop for Connection {
    fn drop(&mut self) {
        let _ = self.actions.send(Action::Close);
    }
}
impl Connection {
    pub fn open(name: String) -> Self {
        let (actions, requests) = mpsc::channel();
        let state = Arc::new(Mutex::new(Snapshot::default()));
        let shared = state.clone();
        std::thread::spawn(move || {
            let result = run(&name, requests, &shared);
            let mut s = shared.lock().unwrap();
            s.ready = false;
            s.flags = 0;
            if let Err(e) = result {
                s.notice = e.to_string();
                s.failed = true;
            }
        });
        Self { actions, state }
    }
}
struct Wire {
    port: Box<dyn serialport::SerialPort>,
    seq: u8,
    bytes: Vec<u8>,
}
impl Wire {
    fn send(&mut self, op: u8, data: &[u8]) -> io::Result<u8> {
        self.seq = self.seq.wrapping_add(1);
        let mut b = [0u8; 64];
        b[..4].copy_from_slice(&[83, 67, op, self.seq]);
        b[4..4 + data.len()].copy_from_slice(data);
        let check = crc(&b[..30]);
        b[30..32].copy_from_slice(&check.to_le_bytes());
        self.port.write_all(&b)?;
        Ok(self.seq)
    }
    fn read(&mut self) -> io::Result<()> {
        let mut data = [0; 4096];
        match self.port.read(&mut data) {
            Ok(n) => self.bytes.extend_from_slice(&data[..n]),
            Err(e) if e.kind() == io::ErrorKind::TimedOut => {}
            Err(e) => return Err(e),
        }
        Ok(())
    }
    fn frame(&mut self) -> Option<Vec<u8>> {
        let mut i = 0;
        while i + 1 < self.bytes.len() {
            let len = match &self.bytes[i..i + 2] {
                b"SR" => 48,
                b"SP" => 236,
                _ => {
                    i += 1;
                    continue;
                }
            };
            if self.bytes.len() - i < len {
                break;
            }
            let b = &self.bytes[i..i + len];
            if crc(&b[..len - 2]) == u16::from_le_bytes([b[len - 2], b[len - 1]]) {
                let frame = b.to_vec();
                self.bytes.drain(..i + len);
                return Some(frame);
            }
            i += 1;
        }
        self.bytes.drain(..i);
        None
    }
    fn receive(&mut self, state: &Mutex<Snapshot>, pending: Option<(u8, u8)>) -> io::Result<bool> {
        self.read()?;
        let mut matched = false;
        while let Some(b) = self.frame() {
            let mut s = state.lock().unwrap();
            if b[1] == b'P' {
                // Retain the most recent display window if rendering is suspended.
                if s.waves.len() >= 125 {
                    s.waves.remove(0);
                }
                s.waves.push(b);
                continue;
            }
            let Some((seq, op)) = pending else { continue };
            if b[3] != seq {
                continue;
            }
            matched = true;
            s.flags = b[27];
            s.angle = i32::from_le_bytes(b[28..32].try_into().unwrap());
            s.rpm = i16::from_le_bytes([b[32], b[33]]);
            s.iq = i16::from_le_bytes([b[34], b[35]]);
            if b[2] != 0 {
                s.notice = format!("设备拒绝操作 {op}");
                continue;
            }
            match op {
                9 | 10 => {
                    s.gains = Some(std::array::from_fn(|i| {
                        u16::from_le_bytes([b[4 + 2 * i], b[5 + 2 * i]])
                    }));
                    if op == 10 {
                        s.notice = "PI 参数已回读".into();
                    }
                }
                _ => {
                    s.mode = b[4];
                    if matches!(op, 4 | 2) {
                        s.command = Some(b[4..27].try_into().unwrap());
                    }
                    if matches!(op, 2 | 3) {
                        s.notice = if op == 2 {
                            "设备已接受指令"
                        } else {
                            "已释放"
                        }
                        .into();
                    }
                }
            }
        }
        Ok(matched)
    }
    fn exchange(&mut self, state: &Mutex<Snapshot>, op: u8, data: &[u8]) -> io::Result<()> {
        let seq = self.send(op, data)?;
        let start = Instant::now();
        while start.elapsed() < Duration::from_millis(65) {
            if self.receive(state, Some((seq, op)))? {
                return Ok(());
            }
        }
        Err(io::Error::new(
            io::ErrorKind::TimedOut,
            format!("操作 {op} 应答超时"),
        ))
    }
}
fn run(
    name: &str,
    requests: mpsc::Receiver<Action>,
    state: &Mutex<Snapshot>,
) -> Result<(), Box<dyn std::error::Error>> {
    let port = serialport::new(name, 1_000_000)
        .timeout(Duration::from_millis(2))
        .open()?;
    let mut w = Wire {
        port,
        seq: 0,
        bytes: vec![],
    };
    w.exchange(state, 4, &[])?;
    w.exchange(state, 1, &[])?;
    w.exchange(state, 9, &[])?;
    state.lock().unwrap().ready = true;
    let mut heartbeat = Instant::now();
    loop {
        if heartbeat.elapsed() >= Duration::from_millis(50) {
            heartbeat = Instant::now();
            w.exchange(state, 1, &[])?;
            continue;
        }
        match requests.try_recv() {
            Ok(Action::Drive(c)) => w.exchange(state, 2, &c)?,
            Ok(Action::Tune(values)) => {
                let mut data = [0; 14];
                for (dst, g) in data.chunks_exact_mut(2).zip(values) {
                    dst.copy_from_slice(&g.to_le_bytes());
                }
                w.exchange(state, 10, &data)?;
            }
            Ok(Action::Release) => w.exchange(state, 3, &[])?,
            Ok(Action::Stream(on)) => w.exchange(state, if on { 7 } else { 6 }, &[])?,
            Ok(Action::Close) | Err(mpsc::TryRecvError::Disconnected) => {
                w.exchange(state, 3, &[])?;
                w.exchange(state, 6, &[])?;
                return Ok(());
            }
            Err(mpsc::TryRecvError::Empty) => {
                w.receive(state, None)?;
            }
        }
    }
}
