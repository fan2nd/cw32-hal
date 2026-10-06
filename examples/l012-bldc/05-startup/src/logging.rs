//! 仅处理复制出的观测值。在普通线程模式下调用 `report`，不得在电机中断中调用。
use crate::control::{MotorController, MotorState, SensorlessState, StartupState};
use crate::protection::Fault;

/// 保存最新值的诊断样本，不是电机事件队列，也不是 DMA 序列结束（EOS）快照。
#[derive(Clone, Copy)]
pub(crate) struct Snapshot {
    pub ms: u32,
    pub state: MotorState,
    pub startup: StartupState,
    pub fault: Option<Fault>,
    sensorless: SensorlessState,
    powered_off: bool,
    outputs_armed: bool,
    speed: u8,
    duty: u32,
    sector: u8,
    iterations: u16,
    crossings: u32,
    step_ticks: u16,
    adc1: [u16; 4],
    adc2: [u16; 5],
    adc1_sequences: u32,
    calibration_mv: u16,
    bus_decivolts: u32,
    current_ma: u32,
}

impl Snapshot {
    pub fn capture(
        controller: &MotorController,
        ms: u32,
        adc1: [u16; 4],
        adc2: [u16; 5],
        adc1_sequences: u32,
        outputs_armed: bool,
        calibration_mv: u16,
    ) -> Self {
        let measurements = controller.measurements();
        Self {
            ms,
            state: controller.state(),
            startup: controller.startup_state(),
            fault: controller.fault(),
            sensorless: controller.sensorless_state(),
            powered_off: controller.powered_off(),
            outputs_armed,
            speed: controller.speed_percent(),
            duty: controller.duty(),
            sector: controller.sector(),
            iterations: controller.startup_iterations(),
            crossings: controller.good_crossings(),
            step_ticks: controller.step_time(),
            adc1,
            adc2,
            adc1_sequences,
            calibration_mv,
            bus_decivolts: measurements.bus_decivolts,
            current_ma: measurements.current_ma,
        }
    }

    pub fn report(&self, previous_fault: Option<Fault>) {
        defmt::info!(
            "t={}ms state={} startup={} fault={} off={} armed={} speed={}% duty={}/4800 sector={} observer={} steps={}/200 crossings={}/15 step_ticks={}",
            self.ms, self.state, self.startup, self.fault, self.powered_off,
            self.outputs_armed, self.speed, self.duty, self.sector, self.sensorless,
            self.iterations, self.crossings, self.step_ticks,
        );
        defmt::info!(
            "ADC1[current,A,B,C]={} seq={} ADC2[current,bus,pot,temp,vref]={} calibration={}mV last_protection_bus={}dV current={}mA",
            self.adc1, self.adc1_sequences, self.adc2, self.calibration_mv,
            self.bus_decivolts, self.current_ma,
        );
        if self.fault != previous_fault {
            if let Some(fault) = self.fault {
                defmt::error!("fault={} code={}", fault, fault as u8);
                if fault == Fault::StartupFailed {
                    defmt::error!("StartupFailed: forced start ended without 15 consecutive accepted BEMF crossings; inspect armed, ADC phases/bus, steps and crossings above");
                }
            } else {
                defmt::info!("fault cleared");
            }
        }
    }
}
