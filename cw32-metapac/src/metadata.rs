pub mod ir {
    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct IR {
        pub blocks: &'static [Block],
        pub fieldsets: &'static [FieldSet],
        pub enums: &'static [Enum],
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct Block {
        pub name: &'static str,
        pub extends: Option<&'static str>,

        pub description: Option<&'static str>,
        pub items: &'static [BlockItem],
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct BlockItem {
        pub name: &'static str,
        pub description: Option<&'static str>,

        pub array: Option<Array>,
        pub byte_offset: u32,

        pub inner: BlockItemInner,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum BlockItemInner {
        Block(BlockItemBlock),
        Register(Register),
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct Register {
        pub access: Access,
        pub bit_size: u32,
        pub fieldset: Option<&'static str>,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct BlockItemBlock {
        pub block: &'static str,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum Access {
        ReadWrite,
        Read,
        Write,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct FieldSet {
        pub name: &'static str,
        pub extends: Option<&'static str>,

        pub description: Option<&'static str>,
        pub bit_size: u32,
        pub fields: &'static [Field],
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct Field {
        pub name: &'static str,
        pub description: Option<&'static str>,

        pub bit_offset: BitOffset,
        pub bit_size: u32,
        pub array: Option<Array>,
        pub enumm: Option<&'static str>,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum Array {
        Regular(RegularArray),
        Cursed(CursedArray),
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct RegularArray {
        pub len: u32,
        pub stride: u32,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct CursedArray {
        pub offsets: &'static [u32],
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum BitOffset {
        Regular(RegularBitOffset),
        Cursed(CursedBitOffset),
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct RegularBitOffset {
        pub offset: u32,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct CursedBitOffset {
        pub ranges: &'static [core::ops::RangeInclusive<u32>],
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct Enum {
        pub name: &'static str,
        pub description: Option<&'static str>,
        pub bit_size: u32,
        pub variants: &'static [EnumVariant],
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct EnumVariant {
        pub name: &'static str,
        pub description: Option<&'static str>,
        pub value: u64,
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Metadata {
    pub name: &'static str,
    pub family: &'static str,
    pub line: &'static str,
    pub memory: &'static [&'static [MemoryRegion]],
    pub peripherals: &'static [Peripheral],
    pub nvic_priority_bits: Option<u8>,
    pub interrupts: &'static [Interrupt],
    pub dma_channels: &'static [DmaChannel],
    pub pins: &'static [Pin],
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MemoryRegion {
    pub name: &'static str,
    pub kind: MemoryRegionKind,
    pub address: u32,
    pub size: u32,
    pub settings: Option<FlashSettings>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FlashSettings {
    pub erase_size: u32,
    pub write_size: u32,
    pub erase_value: u8,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum MemoryRegionKind {
    Flash,
    Ram,
    Eeprom,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Interrupt {
    pub name: &'static str,
    pub number: u32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Package {
    pub name: &'static str,
    pub package: &'static str,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Peripheral {
    pub name: &'static str,
    pub address: u64,
    pub registers: Option<PeripheralRegisters>,
    pub rcc: Option<PeripheralRcc>,
    pub rcc_control: Option<PeripheralRccControl>,
    pub spi: Option<PeripheralSpi>,
    pub classic_timer_input: Option<PeripheralClassicTimerInput>,
    pub classic_atim_input: Option<PeripheralClassicAtimInput>,
    pub atim_complementary: Option<PeripheralAtimComplementary>,
    pub lvd: Option<PeripheralLvd>,
    pub lcd: Option<PeripheralLcd>,
    pub rtc_calendar: Option<PeripheralRtcCalendar>,
    pub rtc_alarms: Option<PeripheralRtcAlarms>,
    pub ir: Option<PeripheralIr>,
    pub gpio_interrupt: Option<PeripheralGpioInterrupt>,
    pub gpio: Option<PeripheralGpio>,
    pub cordic: Option<PeripheralCordic>,
    pub aes: Option<PeripheralAes>,
    pub trng: Option<PeripheralTrng>,
    pub ram_parity: Option<PeripheralRamParity>,
    pub clock_limits: Option<PeripheralClockLimits>,
    pub adc_limits: Option<PeripheralAdcLimits>,
    pub comparator_limits: Option<PeripheralComparatorLimits>,
    pub reference_divider: Option<PeripheralReferenceDivider>,
    pub dac_limits: Option<PeripheralDacLimits>,
    pub opa_limits: Option<PeripheralOpaLimits>,
    pub iwdt_clock: Option<PeripheralIwdtClock>,
    pub i2c_limits: Option<PeripheralI2cLimits>,
    pub flash_limits: Option<PeripheralFlashLimits>,
    pub pins: &'static [PeripheralPin],
    pub dma_channels: &'static [PeripheralDmaChannel],
    pub triggers: &'static [PeripheralTrigger],
    pub interrupts: &'static [PeripheralInterrupt],
    pub afio: Option<PeripheralAfio>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralRegisters {
    pub kind: &'static str,
    pub version: &'static str,
    pub block: &'static str,
    pub ir: &'static ir::IR,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralInterrupt {
    pub signal: &'static str,
    pub interrupt: &'static str,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralAfio {
    pub register: &'static str,
    pub field: &'static str,
    pub values: &'static [PeripheralAfioValue],
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralAfioValue {
    pub value: u8,
    pub pins: &'static [&'static str],
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralGpioInterrupt {
    pub serviced_mask: u16,
    pub clear_noop_mask: u16,
    pub level_trigger: bool,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralRccControl {
    pub controller: &'static str,
    pub bus_clock: &'static str,
    pub enable: PeripheralRccRegister,
    pub enable_active_value: bool,
    pub enable_write_key: Option<PeripheralRccWriteKey>,
    pub reset: Option<PeripheralRccRegister>,
    pub reset_asserted_value: Option<bool>,
    pub shared_enable_group: Option<&'static str>,
    pub shared_reset_group: Option<&'static str>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralRccWriteKey {
    pub field: PeripheralRccRegister,
    pub value: u32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralClassicTimerInput {
    pub capture_mux: &'static str,
    pub encoder_fixed_reload: Option<u16>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum PeripheralClassicAtimInput {
    ClassicPairedV1,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralSpi {
    pub maximum_frequency: u32,
    pub minimum_divisor: u16,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralRcc {
    pub bus_clock: &'static str,
    pub kernel_clock: PeripheralRccKernelClock,
    pub enable: Option<PeripheralRccRegister>,
    pub reset: Option<PeripheralRccRegister>,
    pub stop_mode: StopMode,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralRccRegister {
    pub register: &'static str,
    pub field: &'static str,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum PeripheralRccKernelClock {
    Clock(&'static str),
    Mux(PeripheralRccRegister),
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub enum StopMode {
    #[default]
    Stop1, // Peripheral prevents chip from entering Stop1
    Stop2,   // Peripheral prevents chip from entering Stop2
    Standby, // Peripheral does not prevent chip from entering Stop
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralPin {
    pub pin: &'static str,
    pub signal: &'static str,
    pub af: Option<u8>,
    /// Hardware ADC mux encoding; independent of the source signal label.
    pub adc_mux: Option<u8>,
    /// Source-qualified hardware input mux, independent of signal numbering.
    pub comparator_mux: Option<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Pin {
    pub name: &'static str,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct DmaChannel {
    pub name: &'static str,
    pub dma: &'static str,
    pub channel: u32,
    pub dmamux: Option<&'static str>,
    pub dmamux_channel: Option<u32>,
    pub supports_2d: Option<bool>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralDmaChannel {
    pub signal: &'static str,
    pub channel: Option<&'static str>,
    pub dmamux: Option<&'static str>,
    pub dma: Option<&'static str>,
    pub remap: &'static [RemapInfo],
    pub request: Option<u32>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralTrigger {
    pub signal: &'static str,
    pub source: &'static str,
    pub registers: &'static [TriggerRegister],
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TriggerRegister {
    pub peripheral: &'static str,
    pub register: &'static str,
    pub field: &'static str,
    pub role: &'static str,
    pub bit_offset: u32,
    pub bit_size: u32,
    pub value: u32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct RemapInfo {
    pub register: &'static str,
    pub field: &'static str,
    pub value: u8,
}

include!(env!("CW32_METAPAC_METADATA_PATH"));
include!("all_chips.rs");
include!("all_peripheral_versions.rs");

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralExternalClockLimits {
    pub minimum_hz: u32,
    pub maximum_hz: u32,
    pub supply_mv: (u16, u16),
    pub temperature_c: (i16, i16),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralHseLimits {
    pub crystal: PeripheralExternalClockLimits,
    pub bypass: PeripheralExternalClockLimits,
    pub filter_maximum_hz: u32,
    pub startup_cycles: &'static [u32; 4],
    /// Hardware frequency-range selector bins; absent when the selector does not exist.
    pub frequency_ranges_hz: Option<[(u32, u32); 4]>,
    /// Fixed HSIOSC divisor imposed by CCS, only when explicitly source-qualified.
    pub fixed_ccs_hsi_divisor: Option<u8>,
    pub ccs_cycle_count: u32,
    pub ccs_numerator_hz: u64,
    pub ccs_maximum_count: u16,
    pub ccs_lsi_maximum_hz: u32,
    pub ccs_lsi_supply_mv: (u16, u16),
    pub ccs_lsi_temperature_c: (i16, i16),
    pub ccs_requires_lsi: bool,
    pub bypass_duty_percent: (u8, u8),
    pub bypass_minimum_high_low_ns: u16,
    pub bypass_maximum_rise_fall_ns: u16,
}

#[derive(Debug, Eq, PartialEq, Clone)]
/// Direct HEX input limits; no crystal, detector or automatic fallback is implied.
pub struct PeripheralHexLimits {
    pub input: PeripheralExternalClockLimits,
    pub duty_percent: (u8, u8),
    pub minimum_high_low_ns: u16,
    pub maximum_rise_fall_ns: u16,
    /// Percent of VDDIOx relative to VSS.
    pub high_level_vddio_percent: (u8, u8),
    pub low_level_vddio_percent: (u8, u8),
}

#[derive(Debug, Eq, PartialEq, Clone)]
/// Inherited LSE pad ownership; does not qualify LSE configuration.
pub struct PeripheralLse {
    /// Whether the oscillator provides a separate pad lock.
    pub pin_lock: bool,
    /// Whether the pad lock requires the oscillator enable lock.
    pub pin_lock_requires_enable_lock: bool,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralLseRtcReset {
    pub register: &'static str,
    pub byte_offset: u32,
    pub value: u32,
    pub mask: u32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralLseOutputRoute {
    pub pin: &'static str,
    pub af: u8,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralLseWorkGatedConsumer {
    pub source: u8,
    pub gate_controls_work: bool,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralLseStartupConsumers {
    pub startup_analog: bool,
    pub autotrim_source: u8,
    pub lptim: PeripheralLseWorkGatedConsumer,
    pub lcd: PeripheralLseWorkGatedConsumer,
    pub uarts: &'static [&'static str],
    pub lsi_output_routes: &'static [PeripheralLseOutputRoute],
}

#[derive(Debug, Eq, PartialEq, Clone)]
/// Exact-family native low-power drive, detector and RTC facts. No amplitude field exists.
pub struct PeripheralLseNativeLowPower {
    pub drive_bits: u8,
    pub startup_drive_bits: u8,
    /// Qualified unchanged LSI reference upper bound; STABLE alone does not prove it.
    pub monitored_lsi_maximum_hz: u32,
    pub detector_lse_edges: u16,
    pub detector_lsi_cycles: u16,
    /// Engineering phase margin, separate from the hardware edge threshold.
    pub detector_margin_lse_edges: u16,
    /// Exact qualification: inherited_legal or factory_trim, checked per family.
    pub monitor_reference: &'static str,
    /// Own-source LSI halfword address; required for factory_trim, absent otherwise.
    pub lsi_factory_trim_address: Option<u32>,
    /// Actual divisors; PSC stores each divisor minus one.
    pub rtc_first_divisor: u16,
    pub rtc_second_divisor: u16,
    pub rtc_calendar_divisor: u32,
    pub rtc_output_routes: &'static [PeripheralLseOutputRoute],
}

#[derive(Debug, Eq, PartialEq, Clone)]
/// Optional init-only SYSCLK detector qualification. Not a second oscillator configuration.
pub struct PeripheralLseSysclkDetector {
    /// Own-manual hardware edge count in one detector window.
    pub lse_edges: u16,
    /// Own-manual LSI cycles in one detector window.
    pub lsi_cycles: u16,
    /// Software phase margin, additional to the hardware edge count.
    pub margin_lse_edges: u16,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralLseConfiguration {
    pub nominal_hz: u32,
    pub maximum_hz: u32,
    /// True when CLKCCS/HSECCS/LSECCS are configurable hardware controls.
    pub configurable_ccs: bool,
    pub supply_mv: (u16, u16),
    pub temperature_c: (i16, i16),
    pub startup_cycles: &'static [u32; 4],
    pub rtc_source: u8,
    pub uart_source: u8,
    pub awt_source: Option<u8>,
    pub startup_consumers: Option<PeripheralLseStartupConsumers>,
    pub native_low_power: Option<PeripheralLseNativeLowPower>,
    pub sysclk_detector: Option<PeripheralLseSysclkDetector>,
    pub mco_source: u8,
    pub gpio_dir_offset: u32,
    pub gpio_speed_offset: Option<u32>,
    pub rtc_reset: &'static [PeripheralLseRtcReset],
    pub output_routes: &'static [PeripheralLseOutputRoute],
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralPllLimits {
    pub input_range_hz: (u32, u32),
    pub output_range_hz: (u32, u32),
    pub input_ranges_hz: &'static [(u32, u32); 4],
    pub output_ranges_hz: &'static [(u32, u32); 5],
    pub multiplier_range: (u8, u8),
    pub supply_mv: (u16, u16),
    pub temperature_c: (i16, i16),
    pub hsi_supported: bool,
    /// Vendor-documented functional HSE oscillator and bypass references; rate bounds only.
    pub hse_supported: bool,
    pub startup_cycles: u32,
    pub startup_encoding: u8,
    pub reserved_debug_default: u8,
    pub cycle_to_cycle_jitter_ps: u32,
}

/// Own-source init-only factory LSI; rate bounds only, with the complete direct-consumer roster.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralLsiSysclk {
    pub nominal_hz: u32,
    pub minimum_hz: u32,
    pub maximum_hz: u32,
    pub supply_mv: (u16, u16),
    pub temperature_c: (i16, i16),
    pub factory_trim_address: u32,
    pub rtc_allowed_sources: &'static [u8],
    pub awt_allowed_sources: &'static [u8],
    pub uart_allowed_sources: &'static [u8],
    pub uarts: &'static [&'static str],
    pub gpio_banks: &'static [&'static str],
    pub gpio_filter_allowed_sources: &'static [u8],
    pub mco_allowed_sources: &'static [u8],
    pub lsi_output_pin: Option<&'static str>,
    pub lsi_output_allowed_af: &'static [u8],
    pub rcc_irq: u16,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralMco {
    pub disabled_source: u8,
    pub hclk_source: u8,
    pub pclk_source: u8,
    /// Full hardware code-to-divisor pairs; the static borrowed fixed array matches the existing PLL array projection convention.
    pub divisors: &'static [(u8, u16); 8],
    /// Conservative HAL MCO qualification ceiling under the documented board/source/waveform conditions; not an unconditional silicon maximum.
    pub maximum_output_hz: u32,
    pub supply_mv: (u16, u16),
    pub temperature_c: (i16, i16),
    pub maximum_load_pf: u16,
    pub gpio_speed: u8,
    /// Condition in the loaded-I/O vendor fmax definition, not an output guarantee.
    pub fmax_condition_duty_percent: (u8, u8),
    /// Maximum (tr+tf)/T in the loaded-I/O vendor fmax definition, not proven by rate bounds.
    pub fmax_condition_edge_sum_period_ratio: (u8, u8),
    /// False describes this exact vendor datasheet table characterization. It does not assert whether an application board has been measured.
    pub electrical_values_measured: bool,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralClockLimits {
    pub hsi_frequency_hz: u32,
    pub hsi_error_percent: u32,
    pub hsi_supply_mv: (u16, u16),
    pub hsi_temperature_c: (i16, i16),
    pub low_voltage_threshold_mv: u16,
    pub low_voltage_bus_max_hz: u32,
    pub high_voltage_bus_max_hz: u32,
    pub factory_hsi_trim_address: u32,
    /// Required legal incoming HSIOSC range; not an arbitrary-TRIM frequency guarantee.
    pub hsi_operating_range_hz: Option<(u32, u32)>,
    pub default_hsi_divisor: u8,
    pub initial_flash_wait: u32,
    pub flash_wait_step_hz: u32,
    pub hse: Option<PeripheralHseLimits>,
    pub hex: Option<PeripheralHexLimits>,
    pub lse: Option<PeripheralLse>,
    pub lse_configuration: Option<PeripheralLseConfiguration>,
    pub pll: Option<PeripheralPllLimits>,
    pub lsi_sysclk: Option<PeripheralLsiSysclk>,
    pub mco: Option<PeripheralMco>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralAdcBand {
    pub supply_min_mv: u16,
    pub maximum_clock_hz: u32,
    pub maximum_sample_rate_hz: u32,
    pub minimum_acquisition_ps: u32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralAdcSequence {
    pub maximum_length: u8,
    pub programmable_order: bool,
    pub per_slot_sample_time: bool,
    pub per_slot_result: bool,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralClassicAdcScan {
    pub slots_per_sequence_register: u8,
    pub buffered_requires_single_channel: bool,
    pub internal_requires_single_channel: bool,
    pub first_internal_channel: u8,
    pub supply_channel: u8,
    pub temperature_channel: Option<u8>,
    pub bandgap_channel: Option<u8>,
    /// Conservative software guard, not a characterized maximum.
    pub temperature_startup_us: u32,
    /// Conservative software guard for the approximate BGR startup.
    pub bandgap_startup_us: u32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralAdcLimits {
    pub supply_mv: (u16, u16),
    pub minimum_clock_hz: u32,
    pub sample_cycles: &'static [u16],
    pub comparison_cycles: u16,
    pub supply_bands: &'static [PeripheralAdcBand],
    pub internal_1v5_bands: &'static [PeripheralAdcBand],
    pub internal_2v5_bands: &'static [PeripheralAdcBand],
    pub input_follower_maximum_rate_hz: u32,
    pub temperature_acquisition_maximum_rate_hz: u32,
    pub internal_acquisition_ps: u64,
    pub sequence: Option<PeripheralAdcSequence>,
    pub classic_scan: Option<PeripheralClassicAdcScan>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralIwdtClock {
    pub typical_hz: u32,
    pub minimum_hz: u32,
    pub maximum_hz: u32,
    pub uses_lsi: bool,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralI2cLimits {
    pub maximum_frequency_hz: u32,
    pub waveform: Option<PeripheralI2cWaveform>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralFlashLimits {
    pub supply_mv: (u16, u16),
    pub lock_group_bytes: u32,
    pub lock_mask: u64,
    pub has_cache_control: bool,
    pub maximum_hclk_hz: u32,
    pub low_voltage_threshold_mv: u16,
    pub low_voltage_maximum_hclk_hz: u32,
    pub wait_step_hz: u32,
    pub maximum_wait_states: u32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralI2cWaveform {
    pub maximum_frequency_hz: &'static [u32],
    pub low: &'static [u32],
    pub high: &'static [u32],
    pub hold: &'static [u32],
    pub setup: &'static [u32],
    pub data_setup: &'static [u32],
    pub data_valid: &'static [u32],
    pub rise: &'static [u32],
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralComparatorLimits {
    pub supply_mv: (u16, u16),
    pub input_uses_vdda: bool,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralLvdInput {
    pub pin: &'static str,
    pub selector: u8,
}
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralLvd {
    pub thresholds_mv: &'static [u16],
    pub supply_name: &'static str,
    pub inputs: &'static [PeripheralLvdInput],
}
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralIr {
    pub owner: &'static str,
    pub register: &'static str,
    pub mode_configurable: bool,
    pub software_control: bool,
    pub invert: bool,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralReferenceDivider {
    pub clock_owner: &'static str,
    pub consumers: &'static [&'static str],
    pub negative_mux: u8,
    pub supply_mv: (u16, u16),
    pub input_uses_vdda: bool,
}
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralDacLimits {
    pub supply_mv: (u16, u16),
    pub resolution_bits: u8,
}
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralOpaLimits {
    pub supply_mv: (u16, u16),
    pub output_headroom_mv: u16,
}
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralLcd {
    pub segments: &'static [u8],
    pub ram_registers: &'static [u8],
    pub lsi_typical_hz: u32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralRtcCalendar {
    pub source: &'static str,
    pub source_encoding: u8,
    pub nominal_hz: u32,
    pub minimum_hz: u32,
    pub maximum_hz: u32,
    pub temperature_c: (i16, i16),
    pub supply_mv: (u16, u16),
    pub factory_trim_address: u32,
    pub calendar_divisor: u32,
    pub prescaler_first: u16,
    pub prescaler_second: u32,
}

/// Own-source alarm capabilities, separate from calendar clock facts.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralRtcAlarms {
    pub direct_register_access: bool,
    pub async_wait: bool,
    pub alarm_a_ignore_bit: u8,
    pub alarm_b_configuration_supported: bool,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralAtimComplementary {
    pub channels: u8,
    pub dead_time_max_ticks: u16,
    pub break_inputs: u8,
}

#[derive(Debug, Eq, PartialEq, Clone)]
/// Family output-capable pads and supported pull-downs, independent of package bonding and interrupt/command masks.
pub struct PeripheralGpio {
    pub output_mask: u16,
    pub pull_down_mask: u16,
}

#[derive(Debug, Eq, PartialEq, Clone)]
/// Inclusive signed Q1.31 bounds rounded inward from the own-manual rational domain.
pub struct PeripheralCordicDomain {
    pub name: &'static str,
    pub minimum: i32,
    pub maximum: i32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
/// Own-source qualified numeric domains; operation and format selectors remain register IR.
pub struct PeripheralCordic {
    pub domains: &'static [PeripheralCordicDomain],
}

#[derive(Debug, Eq, PartialEq, Clone)]
/// Own-source block and supported key geometry, in 32-bit register words.
pub struct PeripheralAes {
    pub block_words: u8,
    pub key_words_128: u8,
    pub key_words_192: u8,
    pub key_words_256: u8,
}

#[derive(Debug, Eq, PartialEq, Clone)]
/// Own-source output geometry, in 32-bit register words; no entropy guarantee.
pub struct PeripheralTrng {
    pub output_words: u8,
}

#[derive(Debug, Eq, PartialEq, Clone)]
/// Availability of the read-only checking-enabled status; no software parity control.
pub struct PeripheralRamParity {
    pub enable_status: bool,
}
