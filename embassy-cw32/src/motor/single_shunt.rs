//! L012 六路互补 PWM 与单电阻双触发的窄接口。
//! 不改变原有六步 PwmBridge。ADC 和 GPIO 仍由调用者独占配置。
use crate::{
    interrupt::typelevel::{self, Binding, Handler},
    pac,
};
use core::marker::PhantomData;

pub struct SingleShuntPwm {
    _domain: PhantomData<*mut ()>,
}
impl SingleShuntPwm {
    /// # Safety
    /// 调用者独占 ATIM，并串行化全部租约与中断。外设单例只能保留、不能另作他用。
    pub unsafe fn acquire() -> Self {
        Self {
            _domain: PhantomData,
        }
    }
    /// 配置上数 PWM1 CH1–3 和 PWM2 CH4/5，TRGO2=两次上升沿。
    /// # Safety
    /// 仅限停止状态初始化；六个栅极均已由 GPIO 保持低，ADC 触发已断开。
    /// dead_ticks 只接受 CKD=0 下的线性编码 0..127；物理死区必须上板验证。
    pub unsafe fn configure(
        &mut self,
        period: u16,
        dead_ticks: u8,
        duty: [u16; 3],
        sample: [u16; 2],
    ) {
        assert!(period > 0 && dead_ticks <= 127);
        assert!(duty.iter().chain(sample.iter()).all(|x| *x < period));
        let mut clock =
            <crate::peripherals::ATIM as crate::rcc::PeripheralClock>::acquire_no_reset();
        clock.pin();
        let r = pac::ATIM;
        r.cr1().write(|v| {
            v.set_arpe(true);
            v.set_urs(true);
        });
        r.bdtr().write(|v| {
            v.set_ossi(true);
            v.set_ossr(true);
            v.set_dtg(dead_ticks);
        });
        r.dier().write(|_| {});
        r.ccer().write(|_| {});
        r.smcr().write(|_| {});
        r.psc().write(|_| {});
        r.arr().write(|v| v.set_arr(period - 1));
        r.rcr().write(|_| {});
        r.cnt().write(|_| {});
        for index in 0..3 {
            r.ccmr_cmp(index).write(|v| {
                for half in 0..2 {
                    let ch = index * 2 + half;
                    if ch < 5 {
                        v.set_ocm(
                            half,
                            if ch < 3 {
                                pac::atim::vals::CcmrCmpOcm::PWM1
                            } else {
                                pac::atim::vals::CcmrCmpOcm::PWM2
                            },
                        );
                        v.set_ocpe(half, true);
                    }
                }
            });
        }
        r.dtr2().write(|_| {}); // DTAE=0：上升、下降使用同一个死区。
        r.af1().write(|v| v.set_bkine(false));
        r.af2().write(|v| v.set_bk2ine(false)); // 本板未提供已验证的硬件 BK 过流连线。
        r.ccr_group(1).write_value(pac::atim::regs::CcrGroup(0));
        self.stage(duty, sample); // 全字写 CCR5，同时清除 GC5C1..6 组合位。
                                  // UG 时屏蔽触发；只在 CEN=0 的此初始化步骤装载一次。
        r.cr2().write(|v| {
            v.set_mms(1);
            v.set_mms2(1);
        });
        r.egr().write(|v| v.set_ug(true));
        r.icr().write_value(pac::atim::regs::Icr(0));
        // RM1.4 §17.10.2，印刷页 372–373，11010=OC4REFC 上升沿或 OC5REFC 上升沿。
        r.cr2().write(|v| {
            v.set_mms(1);
            v.set_mms2(26);
        });
        r.ccer().write(|v| {
            for ch in 0..3 {
                v.set_cce(ch, true);
                v.set_ccne(ch, true);
            }
        });
    }
    /// 写下一次真实 reload 才装载的五个预载寄存器。
    /// 调用者必须保证整批写入不会跨越 reload，并维护相同延迟的软件帧身份。
    pub fn stage(&mut self, duty: [u16; 3], sample: [u16; 2]) {
        for (ch, value) in duty.into_iter().enumerate() {
            pac::ATIM
                .ccr(ch)
                .write_value(pac::atim::regs::Ccr(u32::from(value)));
        }
        pac::ATIM
            .ccr(3)
            .write_value(pac::atim::regs::Ccr(u32::from(sample[0])));
        pac::ATIM
            .ccr_group(0)
            .write_value(pac::atim::regs::CcrGroup(u32::from(sample[1])));
    }
    pub fn enable_update_interrupt<H: Handler<typelevel::ATIM>>(
        &mut self,
        _irq: impl Binding<typelevel::ATIM, H>,
    ) {
        pac::ATIM.dier().write(|v| v.set_uie(true));
    }
    pub fn take_update(&mut self) -> bool {
        let r = pac::ATIM;
        if !(r.isr().read().uif() && r.dier().read().uie()) {
            return false;
        }
        // ICR W0C：只确认更新，不吞掉硬件制动旗标。
        r.icr().write(|v| v.set_uif(false));
        true
    }
    pub fn update_pending(&self) -> bool {
        pac::ATIM.isr().read().uif()
    }
    pub fn counter(&self) -> u16 {
        pac::ATIM.cnt().read().cnt()
    }
    pub fn start(&mut self) {
        pac::ATIM.cr1().modify(|v| v.set_cen(true));
    }
    /// 关闭桥臂并停止计数/更新中断，终止 CH4/5 后续 ADC 触发；保留故障旗标。
    /// 仅关 NVIC 不能停止 ADC 硬件触发，故障后输出日志前必须停止计数器。
    pub fn stop(&mut self) {
        self.disarm();
        pac::ATIM.dier().write(|_| {});
        pac::ATIM.cr1().modify(|v| v.set_cen(false));
    }
    pub fn fault_pending(&self) -> bool {
        let f = pac::ATIM.isr().read();
        f.bif() || f.b2if() || f.sbif()
    }
    pub fn outputs_enabled(&self) -> bool {
        pac::ATIM.bdtr().read().moe()
    }
    /// 停机保留 OSSI/OSSR、CCxE/NE 和低 OIS，使六路强制低电平。
    /// 不清除故障，不停止用于偏置校准的内部采样触发。
    pub fn disarm(&mut self) {
        let mut v = pac::ATIM.bdtr().read();
        v.set_aoe(false);
        v.set_moe(false);
        pac::ATIM.bdtr().write_value(v);
    }
    /// # Safety
    /// 仅由明确的启动命令授权；调用者已完成偏置/母线/参数检查且无软件故障。
    /// 不能用于自动重启，MOE 失效后必须由调用者锁存故障。
    pub unsafe fn arm(&mut self) -> Result<(), super::FaultActive> {
        if self.fault_pending() {
            self.disarm();
            return Err(super::FaultActive);
        }
        let mut v = pac::ATIM.bdtr().read();
        v.set_aoe(false);
        v.set_moe(true);
        pac::ATIM.bdtr().write_value(v);
        if self.fault_pending() || !self.outputs_enabled() {
            self.disarm();
            Err(super::FaultActive)
        } else {
            Ok(())
        }
    }
}
