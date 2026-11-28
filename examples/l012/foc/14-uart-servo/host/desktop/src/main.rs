#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]
#[allow(dead_code)]
#[path = "../../../src/protocol.rs"]
mod protocol;
mod transport;
use dioxus::desktop::{Config, LogicalSize, WindowBuilder};
use dioxus::prelude::*;
use std::time::Duration;
use transport::{Action, Connection};

const CSS: &str = include_str!("../../style.css");
const WAVE_JS: &str = include_str!("../../wave.js");
const MODES: [&str; 4] = ["位置", "速度", "力矩", "MIT"];
const COMMAND_FIELDS: [&str; 7] = [
    "多圈位置 °",
    "目标速度 rpm",
    "Iq 计数",
    "MIT Kp · Iq/°",
    "MIT Kd · Iq/rpm",
    "速度上限 rpm",
    "Iq 上限",
];
const GAIN_FIELDS: [&str; 7] = [
    "Id Kp",
    "Id Ki",
    "Iq Kp",
    "Iq Ki",
    "速度 Kp",
    "速度 Ki",
    "位置 Kp",
];
fn main() {
    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            Config::new().with_window(
                WindowBuilder::new()
                    .with_title("L012 伺服调试")
                    .with_inner_size(LogicalSize::new(840., 600.))
                    .with_min_inner_size(LogicalSize::new(640., 480.)),
            ),
        )
        .launch(App);
}
fn ports() -> Vec<String> {
    let mut names: Vec<_> = serialport::available_ports()
        .unwrap_or_default()
        .into_iter()
        .map(|p| p.port_name)
        .collect();
    names.sort_by_key(|n| {
        (
            !n.contains("usbmodem") && !n.contains("usbserial"),
            n.starts_with("/dev/tty."),
            n.clone(),
        )
    });
    names
}
fn drive(mode: u8, fields: &[String; 7], gains: &[String; 7]) -> Result<[u8; 23], String> {
    let mut c = protocol::Command::DEFAULT;
    c.mode = mode;
    let number = |i: usize, scale: f64| -> Result<i32, String> {
        let n = fields[i]
            .parse::<f64>()
            .map_err(|_| format!("请检查{}", COMMAND_FIELDS[i]))?
            * scale;
        if !n.is_finite()
            || (n - n.round()).abs() > 1e-6
            || n.round() < i32::MIN as f64
            || n.round() > i32::MAX as f64
        {
            return Err(format!("请检查{}的精度和范围", COMMAND_FIELDS[i]));
        }
        Ok(n.round() as i32)
    };
    let word = |i, scale| {
        i16::try_from(number(i, scale)?).map_err(|_| format!("{}超出协议范围", COMMAND_FIELDS[i]))
    };
    if mode == 1 || mode == 4 {
        c.angle = number(0, 10.)?;
    }
    if mode == 2 || mode == 4 {
        c.rpm = word(1, 1.)?;
    }
    if mode >= 3 {
        c.iq = word(2, 1.)?;
    }
    if mode == 4 {
        c.kp = word(3, 100.)?;
        c.kd = word(4, 100.)?;
    }
    c.max_rpm = word(5, 1.)?;
    c.max_iq = word(6, 1.)?;
    c.speed_kp = gains[4].parse().map_err(|_| "请读取速度 Kp")?;
    c.speed_ki = gains[5].parse().map_err(|_| "请读取速度 Ki")?;
    c.pos_gain = gains[6].parse().map_err(|_| "请读取位置 Kp")?;
    let mut b = [0; 23];
    c.encode(&mut b);
    protocol::Command::decode(&b).ok_or("指令超出设备参数范围")?;
    Ok(b)
}
#[component]
fn App() -> Element {
    let mut choices = use_signal(ports);
    let mut port = use_signal(|| choices.read().first().cloned().unwrap_or_default());
    let mut connection = use_signal(|| None::<Connection>);
    let mut ready = use_signal(|| false);
    let mut tab = use_signal(|| 0usize);
    let mut mode = use_signal(|| 2u8);
    let mut device_mode = use_signal(|| 0u8);
    let mut fields = use_signal(|| ["0", "0", "0", "0", "0", "600", "200"].map(str::to_string));
    let mut gains =
        use_signal(|| ["2500", "64", "2500", "64", "125", "6", "4"].map(str::to_string));
    let mut applied = use_signal(|| gains.read().clone());
    let mut flags = use_signal(|| 0u8);
    let mut fb = use_signal(|| (0i32, 0i16, 0i16));
    let mut notice = use_signal(String::new);
    let mut chart = use_signal(|| None::<document::Eval>);
    use_future(move || async move {
        loop {
            tokio::time::sleep(Duration::from_millis(32)).await;
            let mut failed = false;
            if let Some(c) = connection.read().as_ref() {
                let mut s = c.state.lock().unwrap();
                if *ready.peek() != s.ready {
                    ready.set(s.ready);
                }
                failed = s.failed;
                if let Some(b) = s.command.take() {
                    if let Some(cmd) = protocol::Command::decode(&b) {
                        mode.set(cmd.mode);
                        fields.set(
                            [
                                cmd.angle as f64 / 10.,
                                cmd.rpm.into(),
                                cmd.iq.into(),
                                cmd.kp as f64 / 100.,
                                cmd.kd as f64 / 100.,
                                cmd.max_rpm.into(),
                                cmd.max_iq.into(),
                            ]
                            .map(|v| v.to_string()),
                        );
                    }
                }
                if let Some(v) = s.gains.take() {
                    let v = v.map(|g| g.to_string());
                    applied.set(v.clone());
                    gains.set(v);
                }
                if !s.notice.is_empty() {
                    notice.set(std::mem::take(&mut s.notice));
                }
                if *flags.peek() != s.flags {
                    flags.set(s.flags);
                }
                if *device_mode.peek() != s.mode {
                    device_mode.set(s.mode);
                }
                let feedback = (s.angle, s.rpm, s.iq);
                if *fb.peek() != feedback {
                    fb.set(feedback);
                }
                if !s.waves.is_empty() {
                    if let Some(js) = chart.read().as_ref() {
                        let _ = js.send(std::mem::take(&mut s.waves));
                    }
                }
            }
            if failed {
                connection.set(None);
            }
        }
    });
    let send = move |a| {
        if let Some(c) = connection.read().as_ref() {
            let _ = c.actions.send(a);
        }
    };
    let state = *flags.read();
    let (angle, rpm, iq) = *fb.read();
    let running = state & 2 != 0;
    rsx! {
      style{"{CSS}"}
      main{
       header{h1{"L012 伺服调试"}
        if connection.read().is_none(){
         select{aria_label:"串口",value:"{port}",onchange:move|e|port.set(e.value()),for name in choices.read().iter(){option{value:"{name}","{name}"}}}
         button{onclick:move|_|{let v=ports();if !v.contains(&port.read()){port.set(v.first().cloned().unwrap_or_default());}choices.set(v);},"刷新"}
         button{disabled:port.read().is_empty(),onclick:move|_|{tab.set(0);notice.set("连接中".into());connection.set(Some(Connection::open(port.read().clone())));},"连接"}
        }else{span{if ready(){"已连接 · 1 Mbps"}else{"连接中"}}button{onclick:move|_|{connection.set(None);ready.set(false);flags.set(0);device_mode.set(0);notice.set("已断开".into());},"断开"}}
       }
       section{class:"status",span{"设备模式  "{MODES.get(device_mode().wrapping_sub(1) as usize).unwrap_or(&"—").to_string()}} span{"功率桥  " if !ready(){"未知"}else if running{"MOE 已开启"}else{"已释放"}}
        strong{if state&4!=0{"驱动故障"}else if state&8!=0{"心跳超时"}}
       }
       nav{for (i,label) in ["控制","调参","电流"].into_iter().enumerate(){button{aria_pressed:tab()==i,onclick:move|_|{
          if tab()!=i && (tab()==2||i==2){if let Some(js)=chart.read().as_ref(){let _=js.send("reset");}send(Action::Stream(i==2));}
          tab.set(i);
       },"{label}"}}button{id:"waveRelease",disabled:!ready(),onclick:move|_|send(Action::Release),"释放"}}
       section{hidden:tab()==2,style:"padding:8px 20px",
        if tab()==0{
          div{id:"modeRow",label{"指令模式"}select{value:"{mode}",onchange:move|e|{if let Ok(n)=e.value().parse(){mode.set(n);}},for (i,label) in MODES.into_iter().enumerate(){option{value:"{i+1}","{label}"}}}}
          div{id:"fields",for i in 0..7 {
            if i>=5 || mode()==4 || (mode()==1&&i==0) || (mode()==2&&i==1) || (mode()==3&&i==2){
               label{"{COMMAND_FIELDS[i]}" input{r#type:"number",step:"any",value:"{fields.read()[i]}",oninput:move|e|fields.write()[i]=e.value()}}
            }
          }}
          footer{output{"{notice}"}button{id:"apply",disabled:!ready(),onclick:move|_|match drive(mode(),&fields.read(),&applied.read()){
           Ok(b)=>send(Action::Drive(b)),Err(e)=>notice.set(e)},"应用并使能"}}
        }else{
          div{id:"fields",for (i,label) in GAIN_FIELDS.iter().enumerate(){label{"{label}" input{r#type:"number",value:"{gains.read()[i]}",oninput:move|e|gains.write()[i]=e.value()}}}}
          footer{output{"{notice}"}button{disabled:!ready(),onclick:move|_|{
             let mut values=[0u16;7]; for (i,s) in gains.read().iter().enumerate(){match s.parse(){Ok(v)=>values[i]=v,Err(_)=>{notice.set(format!("请检查 {}",GAIN_FIELDS[i]));return}}}send(Action::Tune(values));
          },"应用参数"}}
        }
       }
       section{class:"feedback",hidden:tab()==2,div{"多圈位置 °" b{"{angle as f64/10.0}"}}div{"实测速度 rpm" b{"{rpm}"}}div{"实测 Iq 计数" b{if running{"{iq}"}else{"—"}}}}
       section{id:"waves",hidden:tab()!=2,onmounted:move|_|{
         let js=format!("{WAVE_JS}\nfor(;;){{const data=await dioxus.recv();if(data==='reset')scope.reset();else for(const frame of data)scope.ingest(Uint8Array.from(frame));}}");
         chart.set(Some(document::eval(&js)));
       },
       div{class:"waveTools",strong{id:"waveState","等待波形"}span{id:"loss"}select{id:"span",aria_label:"显示时长",hidden:true,option{value:"2000","1 s"}option{value:"400","200 ms"}option{value:"100","50 ms"}}input{id:"pan",r#type:"range",min:0,max:0,hidden:true,aria_label:"冻结窗口位置"}button{id:"freeze","冻结"}}
       for (name,unit,id,legend) in [("三相电流","ADC 计数","phases","phaseLegend"),("PWM 指令占空比","%","pwm","pwmLegend"),("dq 电流","ADC 计数","dq","dqLegend")]{
          div{class:"plot",div{"{name} " small{"{unit}"}span{id:"{legend}"}}canvas{id:"{id}",aria_label:"{name}"}}
       }
       div{id:"timeAxis"}
       }
      }
    }
}
