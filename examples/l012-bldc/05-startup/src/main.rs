#![no_std]
#![no_main]

pub mod control;
pub mod protection;
use crate::control::{Actions, Adc1Sample, Bridge, MotorController, TimerCommand};
use crate::protection::Adc2Sample;
use core::sync::atomic::{AtomicBool, Ordering};
use embassy_cw32::{
    interrupt::{self, InterruptExt},
    pac, rcc,
};

// Original clock profile, confirmed VDDA=5 V: HCLK/PCLK=96 MHz.
// ADC1=48 MHz, 70-cycle sampling; ADC2=12 MHz, 518-cycle sampling.
const CPU_HZ: u32 = 96_000_000;
const PWM_PERIOD: u16 = 4800;
const BTIM_ICR_MASK: u32 = 0x41;
// DMA is the only writer after setup. CPU uses raw volatile reads, never Rust references.
static mut ADC2_DMA: [u32; 5] = [0; 5];
static INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Motor-domain observations. Logical bridge requests are not physical outputs.
/// Read only while halted with power disconnected; a live debugger can tear.
#[derive(Clone, Copy, Debug)]
pub struct Diagnostics {
    pub adc1: [u16; 4],
    pub adc2: [u16; 5],
    pub milliseconds: u32,
    pub adc1_sequences: u32,
    pub sector: u8,
    pub logical_bridge: Bridge,
    pub outputs_armed: bool,
    pub state: u8,
    pub fault: u8,
}

// Main initializes before unmask. Motor IRQs are all P1 and cannot nest.
// The tight foreground loop accesses these only inside a finite critical section;
// no reference escapes. UI uses only its mailbox. Fatal exceptions never return.
static mut CONTROLLER: Option<MotorController> = None;
static mut OUTPUTS_ARMED: bool = false;
static mut BOOTSTRAP_MS: u8 = 0;
static mut DIAGNOSTICS: Diagnostics = Diagnostics {
    adc1: [0; 4],
    adc2: [0; 5],
    milliseconds: 0,
    adc1_sequences: 0,
    sector: 0,
    logical_bridge: Bridge::off(),
    outputs_armed: false,
    state: 0,
    fault: 0,
};

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut config = embassy_cw32::Config::default();
    config.rcc.hsi_frequency = rcc::HsiFrequency::Mhz96;
    config.rcc.pclk_divider = rcc::PclkDivider::Div1;
    let p = embassy_cw32::init(config);
    for irq in [interrupt::ADC1, interrupt::BTIM1, interrupt::BTIM3_HALLTIM] {
        irq.disable();
    }
    // SAFETY: global HAL initialization transferred all singletons here;
    // interrupts remain masked until the controller has been installed below.
    unsafe {
        let mut ahben = pac::SYSCTRL.ahben().read();
        ahben.set_key(0x5a5a);
        ahben.set_dma(true);
        ahben.set_gpioa(true);
        ahben.set_gpiob(true);
        ahben.set_gpioc(true);
        pac::SYSCTRL.ahben().write_value(ahben);
        let mut apben1 = pac::SYSCTRL.apben1().read();
        apben1.set_key(0x5a5a);
        apben1.set_adc(true);
        apben1.set_atim(true);
        pac::SYSCTRL.apben1().write_value(apben1);
        let mut apben2 = pac::SYSCTRL.apben2().read();
        apben2.set_key(0x5a5a);
        apben2.set_btim123(true);
        apben2.set_opa(true);
        pac::SYSCTRL.apben2().write_value(apben2);
        // All six gates get low latches before their output directions.
        for (port, pin) in [
            (pac::GPIOA.as_ptr(), 15),
            (pac::GPIOB.as_ptr(), 3),
            (pac::GPIOB.as_ptr(), 4),
            (pac::GPIOB.as_ptr(), 5),
            (pac::GPIOB.as_ptr(), 6),
            (pac::GPIOB.as_ptr(), 7),
        ] {
            configure_pin(port, pin, PinMode::OutputLow, 0);
        }
        configure_pin(pac::GPIOC.as_ptr(), 13, PinMode::OutputHigh, 0);
        configure_pin(pac::GPIOA.as_ptr(), 3, PinMode::InputPullUp, 0);
        for (port, pin) in [
            (pac::GPIOA.as_ptr(), 0),
            (pac::GPIOA.as_ptr(), 1),
            (pac::GPIOA.as_ptr(), 2),
            (pac::GPIOA.as_ptr(), 6),
            (pac::GPIOA.as_ptr(), 7),
            (pac::GPIOB.as_ptr(), 0),
            (pac::GPIOB.as_ptr(), 2),
            (pac::GPIOA.as_ptr(), 8),
            (pac::GPIOA.as_ptr(), 10),
            (pac::GPIOA.as_ptr(), 11),
        ] {
            configure_pin(port, pin, PinMode::Analog, 0);
        }
        pac::ATIM.cr1().write(|r| r.set_arpe(true));
        pac::ATIM.bdtr().write(|_| {});
        pac::ATIM.dier().write(|_| {});
        pac::ATIM.ccer().write(|_| {});
        pac::ATIM.cr2().write(|_| {});
        pac::ATIM.smcr().write(|_| {});
        pac::ATIM.psc().write(|_| {});
        pac::ATIM.arr().write(|r| r.set_arr(PWM_PERIOD - 1));
        pac::ATIM.rcr().write(|_| {});
        pac::ATIM.cnt().write(|_| {});
        // Original PWM1 and preload on all four channels.
        pac::ATIM.ccmr_cmp(0).write(|r| {
            r.set_ocm(0, 6);
            r.set_ocpe(0, true);
            r.set_ocm(1, 6);
            r.set_ocpe(1, true);
        });
        pac::ATIM.ccmr_cmp(1).write(|r| {
            r.set_ocm(0, 6);
            r.set_ocpe(0, true);
            r.set_ocm(1, 6);
            r.set_ocpe(1, true);
        });
        pac::ATIM.ccr(0).write(|_| {});
        pac::ATIM.ccr(1).write(|_| {});
        pac::ATIM.ccr(2).write(|_| {});
        pac::ATIM.ccr(3).write(|r| r.set_ccr(2400)); // Original PWM_PERIOD / 2.
        pac::ATIM.dtr2().write(|_| {});
        pac::ATIM.af1().write(|r| r.set_bkine(false));
        pac::ATIM.af2().write(|r| r.set_bk2ine(false));
        pac::ATIM.bdtr().write(|_| {});
        pac::ATIM.ccer().write(|r| {
            r.set_cc1e(true);
            r.set_cc2e(true);
            r.set_cc3e(true);
            r.set_cc4e(true);
        });
        pac::ATIM.icr().write_value(pac::atim::regs::Icr(0));
        // OPA1: external feedback, PA6 INP2, PA7 INN2, PB0 output.
        // PB0 is read by ADC1 CH8 without creating a second pin owner.
        pac::BGR.cr().modify(|r| r.set_bgren(true));
        pac::OPA1.cr().write(|r| {
            r.set_inn2en(true);
            r.set_inp2en(true);
            r.set_en(false);
        });
        pac::OPA1.cal().write(|_| {});
        pac::OPA1.cr().write(|r| {
            r.set_inn2en(true);
            r.set_inp2en(true);
            r.set_en(true);
        });
        for (r, length, channels, sample, divider) in [
            (pac::ADC1, 4, [8, 0, 1, 2, 0], [9, 9, 9, 9, 0], 1),
            (pac::ADC2, 5, [11, 5, 7, 8, 15], [15; 5], 3),
        ] {
            r.trigger().write(|_| {});
            r.start().write(|r| r.set_start(false));
            r.ier().write(|_| {});
            // Read once, clear only the documented low byte, and retain all
            // reserved bits from that same snapshot through both CR writes.
            let mut control = r.cr().read();
            control.set_slave(false);
            control.set_ens(0);
            control.set_clk(0);
            control.set_cont(false);
            control.set_en(false);
            r.cr().write_value(control);
            r.awdcr().write(|_| {});
            r.sqrcfr().write(|r| {
                r.set_sqrch(0, channels[0]);
                r.set_sqrch(1, channels[1]);
                r.set_sqrch(2, channels[2]);
                r.set_sqrch(3, channels[3]);
                r.set_sqrch(4, channels[4]);
            });
            r.sample().write(|r| {
                r.set_sqrch(0, sample[0]);
                r.set_sqrch(1, sample[1]);
                r.set_sqrch(2, sample[2]);
                r.set_sqrch(3, sample[3]);
                r.set_sqrch(4, sample[4]);
            });
            r.icr().write_value(pac::adc::regs::Icr(0));
            control.set_clk(divider);
            control.set_ens(length - 1);
            control.set_en(true);
            r.cr().write_value(control);
        }
        for (r, prescaler, reload, oneshot) in [
            (pac::BTIM1, 95, 999, false),
            (pac::BTIM2, 11, 65530, false),
            (pac::BTIM3, 11, 65530, false),
        ] {
            r.cr1().write(|r| r.set_oneshot(oneshot));
            r.dier().write(|_| {});
            r.cr2().write(|_| {});
            r.smcr().write(|_| {});
            r.psc().write(|r| r.set_psc(prescaler));
            r.arr().write(|r| r.set_arr(reload));
            r.cnt().write(|r| r.set_cnt(0));
            r.icr().write_value(pac::btim::regs::Icr(0));
        }
    }
    // >=1 ms nominal instruction delay: exceeds BGR (~30 us), OPA and ADC
    // startup requirements. It is not used as the motor timebase.
    cortex_m::asm::delay(CPU_HZ / 1_000);
    // SAFETY: documented factory calibration halfword, RM25.10/SDK
    // ADC_BGR_VOL_ADDRESS. Keep the source calibration value; protection
    // handles undefined arithmetic explicitly rather than fabricating a voltage.
    let calibration_mv = unsafe { core::ptr::read_volatile(0x0010_07d2 as *const u16) };
    INITIALIZED.store(true, Ordering::Release);

    // Keep the real HAL singleton tokens for the lifetime of this program.
    // Foreground motor MMIO is serialized with P1 handlers using critical sections.
    let motor_peripherals = (
        p.DMA, p.ATIM, p.ADC1, p.ADC2, p.OPA1, p.BGR, p.BTIM1, p.BTIM2, p.BTIM3, p.PA15, p.PB3,
        p.PB4, p.PB5, p.PB6, p.PB7, p.PA0, p.PA1, p.PA2, p.PA6, p.PA7, p.PB0, p.PB2, p.PA8, p.PA10,
        p.PA11, p.PC13, p.PA3,
    );
    let mut controller = MotorController::new(calibration_mv);
    controller.begin_bootstrap();
    let output_opt_in = cfg!(feature = "motor-output-enable");
    let armed = output_opt_in;
    // Always preserve the original six-tick startup bookkeeping. Physical
    // low-side bootstrap charge is enabled only by the output opt-in feature.
    unsafe {
        if armed {
            // Configure the physical PWM connection once. Commutation below
            // changes CCR/low sides only, as in the C source.
            for pin in [5, 6, 7] {
                set_af(pac::GPIOB.as_ptr(), pin, 7);
            }
            let mut bdtr = pac::ATIM.bdtr().read();
            bdtr.set_moe(true);
            pac::ATIM.bdtr().write_value(bdtr);
            // Source bootstrap toggles only the three low-side GPIOs.
            pac::GPIOA.bsrr().write(|r| r.set_bss(15, true));
            pac::GPIOB.bsrr().write(|r| r.set_bss(3, true));
            pac::GPIOB.bsrr().write(|r| r.set_bss(4, true));
        }
        core::ptr::addr_of_mut!(CONTROLLER).write(Some(controller));
        core::ptr::addr_of_mut!(OUTPUTS_ARMED).write(armed);
        core::ptr::addr_of_mut!(BOOTSTRAP_MS).write(6);
        (*core::ptr::addr_of_mut!(DIAGNOSTICS)).outputs_armed = output_opt_in;
        pac::ADC1.icr().write_value(pac::adc::regs::Icr(0));
        pac::ADC2.icr().write_value(pac::adc::regs::Icr(0));
        pac::ADC1.ier().write(|r| r.set_eos(true));
        // Original EOC + BLOCK intent: one 32-bit result per conversion.
        // Defined correction: ADC2_SINGLE (15), not source's mismatched SEQ (14).
        // EOS DMA is explicitly disabled; CNT=5, REPEAT=1, both addresses increment.
        pac::DMA.ch(1).csr().write(|_| {});
        pac::DMA.ch(1).cnt().write(|r| {
            r.set_repeat(1);
            r.set_cnt(5);
        });
        pac::DMA
            .ch(1)
            .srcaddr()
            .write(|r| r.set_srcaddr(pac::ADC2.result(0).as_ptr() as u32));
        pac::DMA
            .ch(1)
            .dstaddr()
            .write(|r| r.set_dstaddr(core::ptr::addr_of_mut!(ADC2_DMA).cast::<u32>() as u32));
        pac::DMA.ch(1).trig().write(|r| {
            r.set_type(true);
            r.set_hardsrc(15);
        });
        pac::DMA.ch(1).csr().write(|r| {
            r.set_restart(true);
            r.set_size(2);
            r.set_dstinc(true);
            r.set_srcinc(true);
            r.set_trans(true);
            r.set_en(true);
        });
        pac::ADC2.ier().write(|r| r.set_dmaeoc(true));
        pac::ADC2.trigger().write(|r| r.set_atimoc4refc(true));

        pac::BTIM1
            .icr()
            .write_value(pac::btim::regs::Icr(BTIM_ICR_MASK & !1));
        pac::BTIM3
            .icr()
            .write_value(pac::btim::regs::Icr(BTIM_ICR_MASK & !1));
        pac::BTIM1.dier().write(|r| r.set_uie(true));
        pac::BTIM3.dier().write(|r| r.set_uie(true));
        for irq in [interrupt::ADC1, interrupt::BTIM1, interrupt::BTIM3_HALLTIM] {
            irq.unpend();
            irq.set_priority(interrupt::Priority::P1);
        }
        pac::ADC1.trigger().write(|r| r.set_atimoc4refc(true));
        pac::BTIM1.cr1().write(|r| r.set_en(true)); // Original timer enable.
        pac::BTIM2.cr1().write(|r| r.set_en(true));
        pac::ATIM.cr1().write(|r| {
            r.set_arpe(true);
            r.set_cen(true);
        });
        pac::ADC2.start().write(|r| r.set_start(true));
        // All shared values are ready and no reference survives unmask.
        // Later foreground access is serialized with IRQs by critical_section.
        core::sync::atomic::compiler_fence(Ordering::Release);
        for irq in [interrupt::ADC1, interrupt::BTIM1, interrupt::BTIM3_HALLTIM] {
            irq.enable();
        }
    }

    loop {
        critical_section::with(|_| unsafe {
            if BOOTSTRAP_MS != 0 {
                return;
            }
            let controller = (&mut *core::ptr::addr_of_mut!(CONTROLLER))
                .as_mut()
                .unwrap();
            let diagnostics = &mut *core::ptr::addr_of_mut!(DIAGNOSTICS);
            let armed = &mut *core::ptr::addr_of_mut!(OUTPUTS_ARMED);
            let dma = core::ptr::addr_of!(ADC2_DMA).cast::<u32>();
            let raw = [
                core::ptr::read_volatile(dma) as u16,
                core::ptr::read_volatile(dma.add(1)) as u16,
                core::ptr::read_volatile(dma.add(2)) as u16,
                core::ptr::read_volatile(dma.add(3)) as u16,
                core::ptr::read_volatile(dma.add(4)) as u16,
            ];
            diagnostics.adc2 = raw;
            controller.update_adc2(Adc2Sample {
                current: raw[0],
                bus_voltage: raw[1],
                potentiometer: raw[2],
                temperature: raw[3],
                reference: raw[4],
            });
            let actions = controller.foreground_step(pac::BTIM2.cnt().read().cnt());
            apply_actions(controller, diagnostics, armed, actions);
            core::hint::black_box(&*diagnostics);
        });
        core::hint::black_box(&motor_peripherals);
    }
}

// Motor IRQs call directly; foreground calls inside its finite critical section.
unsafe fn apply_actions(
    controller: &MotorController,
    diagnostics: &mut Diagnostics,
    armed: &mut bool,
    actions: Actions,
) {
    if let Some((sector, duty)) = actions.pre_alignment_pwm {
        unsafe { apply_bridge(Bridge::commutation(sector, duty), armed, true) };
    }
    let alignment = actions
        .bridge
        .is_some_and(|b| b.low_sides == [false, true, true]);
    if let Some(mut bridge) = actions.bridge {
        if alignment {
            bridge.low_sides[2] = false;
        }
        unsafe { apply_bridge(bridge, armed, actions.pwm_only) };
    }

    if let Some(ticks) = actions.step_timer_preset {
        pac::BTIM2.cnt().write(|r| r.set_cnt(ticks));
    }
    match actions.sensorless_timer {
        TimerCommand::Unchanged => {}
        TimerCommand::Stop => pac::BTIM3.cr1().write(|_| {}),
        TimerCommand::Arm(reload) => {
            pac::BTIM3.arr().write(|r| r.set_arr(reload));
            pac::BTIM3.cnt().write(|_| {});
            pac::BTIM3.cr1().write(|r| r.set_en(true)); // Original repetitive timer enable.
        }
    }

    // Source alignment turns C- on after Commutation(0) and its timer writes.
    if alignment && *armed {
        pac::GPIOB.bsrr().write(|r| r.set_bss(4, true));
    }
    if actions.start_adc2 {
        pac::ADC2.start().write(|r| r.set_start(true));
    }
    if let Some(on) = actions.led_on {
        if on {
            pac::GPIOC.brr().write(|r| r.set_brr(13, true));
        } else {
            pac::GPIOC.bsrr().write(|r| r.set_bss(13, true));
        }
    }
    diagnostics.outputs_armed = *armed;
    diagnostics.logical_bridge = controller.bridge();
    diagnostics.sector = controller.sector();
    diagnostics.state = controller.state() as u8;
    diagnostics.fault = controller.fault().map_or(0, |fault| fault as u8);
}

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "C" fn ADC1() {
    unsafe {
        let r = pac::ADC1;
        if !r.isr().read().eos() {
            return;
        }
        r.icr().write_value(pac::adc::regs::Icr(0));
        let controller = (&mut *core::ptr::addr_of_mut!(CONTROLLER))
            .as_mut()
            .unwrap();
        let diagnostics = &mut *core::ptr::addr_of_mut!(DIAGNOSTICS);
        let armed = &mut *core::ptr::addr_of_mut!(OUTPUTS_ARMED);
        let raw = [
            r.result(0).read().result(),
            r.result(1).read().result(),
            r.result(2).read().result(),
            r.result(3).read().result(),
        ];
        diagnostics.adc1 = raw;
        diagnostics.adc1_sequences = diagnostics.adc1_sequences.wrapping_add(1);
        let dma = core::ptr::addr_of!(ADC2_DMA).cast::<u32>();
        controller.update_adc2(Adc2Sample {
            current: core::ptr::read_volatile(dma) as u16,
            bus_voltage: core::ptr::read_volatile(dma.add(1)) as u16,
            potentiometer: core::ptr::read_volatile(dma.add(2)) as u16,
            temperature: core::ptr::read_volatile(dma.add(3)) as u16,
            reference: core::ptr::read_volatile(dma.add(4)) as u16,
        });
        let actions = controller.on_adc1(Adc1Sample::from(raw), pac::BTIM2.cnt().read().cnt());
        apply_actions(controller, diagnostics, armed, actions);
        core::hint::black_box(&*diagnostics);
    }
}

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "C" fn BTIM1() {
    // SAFETY: P1 IRQs cannot nest; foreground borrows only with interrupts masked.
    unsafe {
        if !(pac::BTIM1.isr().read().uif() & pac::BTIM1.dier().read().uie()) {
            return;
        }
        pac::BTIM1
            .icr()
            .write_value(pac::btim::regs::Icr(BTIM_ICR_MASK & !1));
        let controller = (&mut *core::ptr::addr_of_mut!(CONTROLLER))
            .as_mut()
            .unwrap();
        let diagnostics = &mut *core::ptr::addr_of_mut!(DIAGNOSTICS);
        let armed = &mut *core::ptr::addr_of_mut!(OUTPUTS_ARMED);
        diagnostics.milliseconds = diagnostics.milliseconds.wrapping_add(1);
        let key_pressed = !pac::GPIOA.idr().read().pin(3);
        let actions = controller.tick_1ms(key_pressed, pac::BTIM2.cnt().read().cnt());
        apply_actions(controller, diagnostics, armed, actions);
        if BOOTSTRAP_MS > 0 {
            BOOTSTRAP_MS -= 1;
            if BOOTSTRAP_MS == 0 {
                if *armed {
                    pac::GPIOA.brr().write(|r| r.set_brr(15, true));
                    pac::GPIOB.brr().write(|r| r.set_brr(3, true));
                    pac::GPIOB.brr().write(|r| r.set_brr(4, true));
                }
                controller.finish_bootstrap();
            }
        }
        core::hint::black_box(&*diagnostics);
    }
}

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "C" fn BTIM3_HALLTIM() {
    // SAFETY: P1 IRQs cannot nest; foreground borrows only with interrupts masked.
    unsafe {
        if !(pac::BTIM3.isr().read().uif() & pac::BTIM3.dier().read().uie()) {
            return;
        }
        pac::BTIM3
            .icr()
            .write_value(pac::btim::regs::Icr(BTIM_ICR_MASK & !1));
        let controller = (&mut *core::ptr::addr_of_mut!(CONTROLLER))
            .as_mut()
            .unwrap();
        let diagnostics = &mut *core::ptr::addr_of_mut!(DIAGNOSTICS);
        let armed = &mut *core::ptr::addr_of_mut!(OUTPUTS_ARMED);
        let actions = controller.on_sensorless_timer(pac::BTIM2.cnt().read().cnt());
        apply_actions(controller, diagnostics, armed, actions);
        core::hint::black_box(&*diagnostics);
    }
}

// Preserve source Commutation/UPPWM write order without remuxing or masking MOE.
// Fatal exceptions use a separate shutdown path and never return.
unsafe fn apply_bridge(bridge: Bridge, armed: &mut bool, pwm_only: bool) {
    if !*armed {
        return;
    }
    let lows = bridge.low_sides;
    if !pwm_only && lows == [false; 3] {
        pac::ATIM.ccr(0).write(|_| {});
        pac::ATIM.ccr(1).write(|_| {});
        pac::ATIM.ccr(2).write(|_| {});
        pac::GPIOA.brr().write(|r| r.set_brr(15, true));
        pac::GPIOB.brr().write(|r| r.set_brr(3, true));
        pac::GPIOB.brr().write(|r| r.set_brr(4, true));
        pac::ATIM.ccr(3).write(|r| r.set_ccr(bridge.sample_compare));
        return;
    }

    if !pwm_only {
        // Source Commutation: first switch off only the unselected low sides.
        if !lows[0] {
            pac::GPIOA.brr().write(|r| r.set_brr(15, true));
        }
        if !lows[1] {
            pac::GPIOB.brr().write(|r| r.set_brr(3, true));
        }
        if !lows[2] {
            pac::GPIOB.brr().write(|r| r.set_brr(4, true));
        }
    }
    // Preserve the source order: zero inactive CCRs before writing active CCR.
    for i in 0..3 {
        if bridge.pwm_counts[i] == 0 {
            pac::ATIM.ccr(i).write(|r| r.set_ccr(0));
        }
    }
    for i in 0..3 {
        if bridge.pwm_counts[i] != 0 {
            // Preserve the complete source compare word, including its u32
            // representation; this is an intentional whole-register write.
            pac::ATIM
                .ccr(i)
                .write_value(pac::atim::regs::Ccr(bridge.pwm_counts[i]));
        }
    }
    if !pwm_only {
        if lows[0] {
            pac::GPIOA.bsrr().write(|r| r.set_bss(15, true));
        }
        if lows[1] {
            pac::GPIOB.bsrr().write(|r| r.set_bss(3, true));
        }
        if lows[2] {
            pac::GPIOB.bsrr().write(|r| r.set_bss(4, true));
        }
    }
    pac::ATIM.ccr(3).write(|r| r.set_ccr(bridge.sample_compare));
}

unsafe fn drive_off() {
    unsafe {
        let mut bdtr = pac::ATIM.bdtr().read();
        bdtr.set_moe(false);
        pac::ATIM.bdtr().write_value(bdtr);
        pac::ATIM.ccer().write(|r| r.set_cc4e(true));
        pac::GPIOA.brr().write(|r| r.set_brr(15, true));
        pac::GPIOB.brr().write(|r| {
            for pin in [3, 4, 5, 6, 7] {
                r.set_brr(pin, true);
            }
        });
        for pin in [5, 6, 7] {
            set_af(pac::GPIOB.as_ptr(), pin, 0);
        }
        pac::ATIM.ccr(0).write(|_| {});
        pac::ATIM.ccr(1).write(|_| {});
        pac::ATIM.ccr(2).write(|_| {});
    }
}

enum PinMode {
    OutputLow,
    OutputHigh,
    InputPullUp,
    Analog,
}
unsafe fn set_af(port: *mut (), pin: usize, af: u8) {
    let port: pac::gpio::Gpio = unsafe { pac::gpio::Gpio::from_ptr(port) };
    if pin < 8 {
        port.afr(0).modify(|r| r.set_afr(pin, af));
    } else {
        port.afr(1).modify(|r| r.set_afr(pin - 8, af));
    }
}
unsafe fn configure_pin(port: *mut (), pin: usize, mode: PinMode, af: u8) {
    let port: pac::gpio::Gpio = unsafe { pac::gpio::Gpio::from_ptr(port) };
    port.dir().modify(|r| r.set_pin(pin, true));
    unsafe { set_af(port.as_ptr(), pin, af) };
    port.riseie().modify(|r| r.set_pin(pin, false));
    port.fallie().modify(|r| r.set_pin(pin, false));
    port.opendrain().modify(|r| r.set_pin(pin, false));
    port.pur()
        .modify(|r| r.set_pin(pin, matches!(mode, PinMode::InputPullUp)));
    port.analog()
        .modify(|r| r.set_pin(pin, matches!(mode, PinMode::Analog)));
    if matches!(mode, PinMode::OutputLow | PinMode::OutputHigh) {
        if matches!(mode, PinMode::OutputHigh) {
            port.bsrr().write(|r| r.set_bss(pin, true));
        } else {
            port.brr().write(|r| r.set_brr(pin, true));
        }
        port.dir().modify(|r| r.set_pin(pin, false));
    }
}

fn fatal() -> ! {
    cortex_m::interrupt::disable();
    if INITIALIZED.load(Ordering::Acquire) {
        unsafe { drive_off() };
    }
    loop {
        cortex_m::asm::wfi();
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    fatal()
}

#[cortex_m_rt::exception]
unsafe fn HardFault(_: &cortex_m_rt::ExceptionFrame) -> ! {
    fatal()
}

#[cortex_m_rt::exception]
unsafe fn NonMaskableInt() -> ! {
    fatal()
}

#[cortex_m_rt::exception]
unsafe fn DefaultHandler(_: i16) {
    fatal()
}
