//! PA3消抖与故障确认：故障出现后必须重新释放，再按下只确认故障。
use crate::config::OUTPUT_LEVELS_PERCENT;
#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    None,
    Output(i32),
    AcknowledgeFault,
}
pub struct Button {
    level: usize,
    held: u16,
    released_ms: u16,
    released: bool,
    fault: u8,
}
impl Button {
    pub const fn new() -> Self {
        Self {
            level: 0,
            held: 0,
            released_ms: 0,
            released: false,
            fault: 0,
        }
    }
    /// 每毫秒调用；任务延迟超过2ms则丢弃未完成的按键，不能将卡顿算作持续按住。
    pub fn update(&mut self, low: bool, fault: u8, elapsed_ms: u32) -> Action {
        if fault != self.fault || elapsed_ms > 2 {
            self.held = 0;
            self.released_ms = 0;
            self.released = false;
        }
        self.fault = fault;
        if fault != 0 {
            self.level = 0;
        }
        if !low {
            self.held = 0;
            self.released_ms = self.released_ms.saturating_add(1);
            if self.released_ms >= 60 {
                self.released = true;
            }
            return Action::None;
        }
        self.released_ms = 0;
        self.held = self.held.saturating_add(1);
        if self.held != 60 || !self.released {
            return Action::None;
        }
        self.released = false;
        if fault != 0 {
            return Action::AcknowledgeFault;
        }
        self.level = (self.level + 1) % OUTPUT_LEVELS_PERCENT.len();
        Action::Output(OUTPUT_LEVELS_PERCENT[self.level])
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fault_requires_new_release_and_press_never_requests_speed() {
        let mut b = Button::new();
        for _ in 0..60 {
            assert_eq!(b.update(false, 0, 1), Action::None);
        }
        // 故障恰在按键前到达，旧的释放资格不能作为确认。
        for _ in 0..100 {
            assert_eq!(b.update(true, 13, 1), Action::None);
        }
        for _ in 0..60 {
            assert_eq!(b.update(false, 13, 1), Action::None);
        }
        for _ in 0..59 {
            assert_eq!(b.update(true, 13, 1), Action::None);
        }
        assert_eq!(b.update(true, 13, 1), Action::AcknowledgeFault);
        for _ in 0..100 {
            assert_eq!(b.update(true, 13, 1), Action::None);
        }
        // 系统复位后的新实例即使按键仍低也不能启动。
        let mut boot = Button::new();
        for _ in 0..100 {
            assert_eq!(boot.update(true, 0, 1), Action::None);
        }
        for _ in 0..60 {
            boot.update(false, 0, 1);
        }
        for _ in 0..59 {
            boot.update(true, 0, 1);
        }
        assert_eq!(boot.update(true, 0, 1), Action::Output(20));
    }
    #[test]
    fn normal_button_cycles_six_levels_and_rejects_incomplete_presses() {
        let mut b = Button::new();
        for target in [20, 40, 60, 80, 100, 0] {
            for _ in 0..60 {
                b.update(false, 0, 1);
            }
            for _ in 0..59 {
                assert_eq!(b.update(true, 0, 1), Action::None);
            }
            assert_eq!(b.update(true, 0, 1), Action::Output(target));
            for _ in 0..100 {
                assert_eq!(b.update(true, 0, 1), Action::None);
            }
        }
        for _ in 0..60 {
            b.update(false, 0, 1);
        }
        for _ in 0..59 {
            b.update(true, 0, 1);
        }
        assert_eq!(b.update(true, 0, 10), Action::None);
        for _ in 0..100 {
            assert_eq!(b.update(true, 0, 1), Action::None);
        }
    }
}
