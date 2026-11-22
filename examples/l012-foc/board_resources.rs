//! Board resource assignments used by both motor-control firmware images.
use embassy_cw32::{Peri, Peripherals, peripherals};

macro_rules! assign_resources {
    ($($group:ident: $ty:ident { $($signal:ident: $token:ident),+ $(,)? }),+ $(,)?) => {
        $(pub struct $ty {
            $(pub $signal: Peri<'static, peripherals::$token>,)+
        })+

        pub struct BoardResources {
            $(pub $group: $ty,)+
        }

        impl From<Peripherals> for BoardResources {
            fn from(p: Peripherals) -> Self {
                Self {
                    $($group: $ty { $($signal: p.$token,)+ },)+
                }
            }
        }
    };
}

assign_resources! {
    current_adc: CurrentAdc {
        adc: ADC1,
        phase_a: PB0,
        phase_b: PB1,
    },
    aux_adc: AuxAdc {
        adc: ADC2,
        bus_voltage: PA8,
        potentiometer: PA10,
    },
    current_sense: CurrentSense {
        opa_a: OPA1,
        positive_a: PA6,
        negative_a: PA7,
        opa_b: OPA2,
        positive_b: PA4,
        negative_b: PA5,
    },
    bridge: Bridge {
        timer: ATIM,
        ch1: PB5,
        ch1n: PA15,
        ch2: PB6,
        ch2n: PB3,
        ch3: PB7,
        ch3n: PB4,
    },
    encoder: EncoderSpi {
        spi: SPI3,
        sck: PB8,
        mosi: PA11,
        miso: PC14,
        cs: PF0,
    },
    status: StatusLed {
        led: PC13,
    },
}
