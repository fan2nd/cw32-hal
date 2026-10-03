//! Generate the HAL identity layer from the separately generated PAC metadata.
//! No register addresses, chip pin lists or interrupt numbers live in this file.
use cw32_metapac::metadata::{Metadata, METADATA};
use std::{
    collections::{BTreeMap, BTreeSet},
    env,
    fmt::Write,
    fs,
    path::PathBuf,
};

fn ident(value: &str) -> &str {
    assert!(
        !value.is_empty()
            && value
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || c == b'_')
            && !value.as_bytes()[0].is_ascii_digit(),
        "invalid metadata Rust identifier: {value}"
    );
    value
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let md = &METADATA;
    let chips: Vec<_> = env::vars()
        .filter_map(|(name, _)| {
            name.strip_prefix("CARGO_FEATURE_CW32")
                .map(|suffix| format!("cw32{}", suffix.to_ascii_lowercase()))
        })
        .collect();
    assert_eq!(
        chips.len(),
        1,
        "select exactly one supported CW32 chip feature"
    );
    assert_eq!(
        chips[0],
        md.name.to_ascii_lowercase(),
        "HAL/PAC chip feature mismatch"
    );
    // Chip features choose metadata only. Drivers use peripheral presence,
    // independently selected IP versions and capabilities of those registers.
    emit_driver_cfgs(md);
    fs::write(out.join("_generated_associations.rs"), associations(md)).unwrap();
    fs::write(out.join("_generated_dma.rs"), dma_bindings(md)).unwrap();
    let target = env::var("TARGET").unwrap();
    if target.starts_with("thumb") {
        assert_eq!(
            target, md.target,
            "selected chip does not support this Rust target"
        );
    }

    let gpios: Vec<_> = md
        .peripherals
        .iter()
        .filter(|p| p.block.eq_ignore_ascii_case("gpio"))
        .collect();
    assert!(!gpios.is_empty(), "chip metadata has no GPIO blocks");
    let mut ports = String::from("#[derive(Clone, Copy, Debug, PartialEq, Eq)]\npub enum Port {\n");
    for p in &gpios {
        writeln!(
            ports,
            "    {},",
            ident(p.name.strip_prefix("GPIO").expect("GPIO peripheral naming"))
        )
        .unwrap();
    }
    ports.push_str("}\nimpl Port {\n");
    for (method, ty) in [
        ("base", "usize"),
        ("implemented_mask", "u16"),
        ("pulldown_mask", "u16"),
    ] {
        writeln!(
            ports,
            "    pub(crate) const fn {method}(self) -> {ty} {{ match self {{"
        )
        .unwrap();
        for p in &gpios {
            let variant = ident(p.name.strip_prefix("GPIO").unwrap());
            let value = match method {
                "base" => p.address as u64,
                "implemented_mask" => u64::from(p.implemented_mask),
                _ => {
                    assert_eq!(p.pulldown_mask & !p.implemented_mask, 0);
                    u64::from(p.pulldown_mask)
                }
            };
            writeln!(ports, "        Self::{variant} => {value},").unwrap();
        }
        ports.push_str("    }}\n");
    }
    ports.push_str("pub(crate) const fn number(self)->u8 {match self {\n");
    for p in &gpios {
        let letter = p.name.strip_prefix("GPIO").unwrap();
        assert!(letter.len() == 1 && letter.as_bytes()[0].is_ascii_uppercase());
        writeln!(
            ports,
            "Self::{}=>{},",
            ident(letter),
            letter.as_bytes()[0] - b'A'
        )
        .unwrap();
    }
    ports
        .push_str("}}\npub(crate) const fn from_number(number:u8)->Option<Self> {match number {\n");
    for p in &gpios {
        let letter = p.name.strip_prefix("GPIO").unwrap();
        writeln!(
            ports,
            "{}=>Some(Self::{}),",
            letter.as_bytes()[0] - b'A',
            ident(letter)
        )
        .unwrap();
    }
    ports.push_str("_=>None}}\n");
    ports.push_str("pub(crate) fn enable_clock(self) {match self {\n");
    for p in &gpios {
        let variant = ident(p.name.strip_prefix("GPIO").unwrap());
        let gate = p.clock_gate.expect("GPIO clock gate");
        assert_eq!((gate.peripheral, gate.register), ("SYSCTRL", "AHBEN"));
        assert!(gate.bit < 16, "GPIO clock bit overlaps key");
        if let Some(bit) = p.clock_bit {
            assert_eq!(bit, gate.bit, "legacy GPIO clock bit differs");
        }
        let resource = clock_resource_name(&gate);
        writeln!(
            ports,
            "Self::{variant}=>crate::rcc::{resource}.enable_pinned(),"
        )
        .unwrap();
    }
    ports.push_str("}}\n}\n");
    fs::write(out.join("_generated_gpio.rs"), ports).unwrap();

    // GPIO port registers and SYSCTRL are HAL-managed shared resources. Only
    // pin tokens and standalone peripherals are exposed as exclusive tokens.
    let peripheral_names: Vec<_> = md
        .peripherals
        .iter()
        .filter(|p| {
            !p.block.eq_ignore_ascii_case("gpio")
                && !p.block.eq_ignore_ascii_case("sysctrl")
                && p.ownership_parent.is_none()
        })
        .map(|p| ident(p.name))
        .collect();
    let mut names = BTreeSet::new();
    let dma_channel_names: BTreeSet<_> = md
        .peripherals
        .iter()
        .filter_map(|p| p.dma)
        .flat_map(|d| d.channels.iter().map(|ch| ch.peripheral))
        .collect();
    for name in peripheral_names
        .iter()
        .copied()
        .chain(md.pins.iter().map(|p| ident(p.name)))
        .chain(dma_channel_names.iter().copied().map(ident))
    {
        assert!(names.insert(name), "duplicate singleton name: {name}");
    }
    for pin in md.pins {
        let gpio = gpios
            .iter()
            .find(|p| p.name == pin.port)
            .expect("pin references unknown GPIO peripheral");
        assert!(
            pin.number < 16 && gpio.implemented_mask & (1u16 << pin.number) != 0,
            "pin is not implemented on this die"
        );
        // Pin identity is a die capability. Board wiring/bonding is deliberately
        // outside this package-independent HAL model.
    }
    let mut generated = String::from("embassy_hal_internal::peripherals_definition!(\n");
    for name in &names {
        writeln!(generated, "    {name},").unwrap();
    }
    let reserved_timer = env::var_os("CARGO_FEATURE_TIME_DRIVER_GTIM1").is_some();
    if reserved_timer {
        let timer = md
            .peripherals
            .iter()
            .find(|p| p.name == "GTIM1")
            .expect("GTIM1 time driver unavailable on selected chip");
        assert_eq!(timer.block, "gtim");
        assert!(matches!(timer.version, "l012" | "f030"));
        assert!(
            md.interrupt_bindings
                .iter()
                .any(|b| b.peripheral == "GTIM1" && b.interrupt == "GTIM1"),
            "GTIM1 needs its audited independent IRQ"
        );
        assert_eq!(
            md.interrupt_bindings
                .iter()
                .filter(|b| b.interrupt == "GTIM1")
                .count(),
            1,
            "time driver must not steal a shared IRQ"
        );
    }
    let owned_names: Vec<_> = names
        .iter()
        .copied()
        .filter(|name| !dma_channel_names.contains(name) && !(reserved_timer && *name == "GTIM1"))
        .collect();
    let hse_pins = hse_pin_roles(md);
    generated.push_str(");\n#[allow(non_snake_case)]\npub struct Peripherals {\n");
    for name in &owned_names {
        if let Some(role) = hse_pins.get(name) {
            writeln!(generated,"/// None when initialization reserves this HSE {role} pad.\npub {name}: Option<crate::Peri<'static,peripherals::{name}>>,").unwrap();
        } else {
            writeln!(
                generated,
                "pub {name}:crate::Peri<'static,peripherals::{name}>,"
            )
            .unwrap();
        }
    }
    generated.push_str("}\n");
    for pin in md.pins {
        let variant = ident(pin.port.strip_prefix("GPIO").unwrap());
        writeln!(generated,
            "impl From<peripherals::{name}> for crate::gpio::AnyPin {{ fn from(_: peripherals::{name}) -> Self {{ crate::gpio::AnyPin::new(crate::gpio::Port::{variant}, {number}) }} }}\nimpl crate::gpio::sealed::Pin for peripherals::{name} {{fn pin_port(&self)->u8 {{crate::gpio::Port::{variant}.number()*16+{number} }} }}\nimpl crate::gpio::Pin for peripherals::{name} {{}}",
            name=pin.name, number=pin.number).unwrap();
        let irq_bindings: Vec<_> = md
            .interrupt_bindings
            .iter()
            .filter(|binding| binding.peripheral == pin.port && binding.signal == "GLOBAL")
            .collect();
        assert_eq!(
            irq_bindings.len(),
            1,
            "GPIO async input requires one audited port IRQ"
        );
        let irq = ident(irq_bindings[0].interrupt);
        writeln!(generated,
            "impl crate::gpio::interrupt_input::sealed::InterruptPin for peripherals::{name} {{ const PORT:crate::gpio::Port=crate::gpio::Port::{variant}; const NUMBER:u8={number}; fn state()->&'static crate::interrupt::EventState {{ static STATE:crate::interrupt::EventState=crate::interrupt::EventState::new(); &STATE }} }}\nimpl crate::gpio::InterruptPin for peripherals::{name} {{ type Interrupt=crate::interrupt::typelevel::{irq}; }}",
            name=pin.name,number=pin.number).unwrap();
    }
    generated.push_str("unsafe fn take_generated(hse_input:bool,hse_output:bool) -> Peripherals {\n    Peripherals {\n");
    for name in &owned_names {
        if let Some(role) = hse_pins.get(name) {
            writeln!(generated,"{name}:if hse_{role} {{None}} else {{Some(unsafe{{peripherals::{name}::steal()}})}},").unwrap();
        } else {
            writeln!(generated, "{name}:unsafe{{peripherals::{name}::steal()}},").unwrap();
        }
    }
    generated.push_str("    }\n}\n");
    fs::write(out.join("_generated.rs"), generated).unwrap();

    fs::write(
        out.join("_generated_peripheral_clocks.rs"),
        clock_bindings(md, &peripheral_names),
    )
    .unwrap();
    for kind in ["eau", "cordic", "iwdt", "wwdt"] {
        let mut instances = String::new();
        for p in md
            .peripherals
            .iter()
            .filter(|p| p.block == kind && p.ownership_parent.is_none())
        {
            assert_eq!(p.version, "l012", "unsupported math/watchdog IP version");
            let name = ident(p.name);
            let ty = match kind {
                "eau" => "Eau",
                "cordic" => "Cordic",
                "iwdt" => "Iwdt",
                "wwdt" => "Wwdt",
                _ => unreachable!(),
            };
            let public_trait = if kind == "wwdt" {
                "WindowInstance"
            } else {
                "Instance"
            };
            writeln!(instances, "impl sealed::Instance for crate::peripherals::{name} {{ fn regs()->crate::pac::{kind}::{ty} {{crate::pac::{name}}}").unwrap();
            if kind == "cordic" {
                writeln!(
                    instances,
                    "fn state()->&'static State {{static STATE:State=State::new(); &STATE}}"
                )
                .unwrap();
            }
            writeln!(
                instances,
                "}} impl {public_trait} for crate::peripherals::{name} {{"
            )
            .unwrap();
            if kind == "cordic" {
                let irqs: Vec<_> = md
                    .interrupt_bindings
                    .iter()
                    .filter(|i| i.peripheral == p.name && i.signal == "GLOBAL")
                    .collect();
                assert_eq!(irqs.len(), 1, "CORDIC requires one audited completion IRQ");
                writeln!(
                    instances,
                    "type Interrupt=crate::interrupt::typelevel::{};",
                    ident(irqs[0].interrupt)
                )
                .unwrap();
            }
            instances.push_str("}\n");
        }
        fs::write(out.join(format!("_generated_{kind}.rs")), instances).unwrap();
    }
    let mut atim_pins = String::new();
    let mut sealed_pins = BTreeSet::new();
    for route in md
        .pin_routes
        .iter()
        .filter(|r| r.peripheral == "ATIM" && r.remap.is_none())
    {
        // Do not silently repurpose the SWD pins as motor-control signals.
        if matches!(route.pin, "PA13" | "PA14") {
            continue;
        }
        if !md.pins.iter().any(|p| p.name == route.pin) {
            continue;
        }
        let Some(af) = route.af else {
            continue;
        };
        let name = ident(route.pin);
        let implementation = match route.signal {
            "CH1" | "CH1A" => "OutputPin<1,false>",
            "CH1N" | "CH1B" => "OutputPin<1,true>",
            "CH2" | "CH2A" => "OutputPin<2,false>",
            "CH2N" | "CH2B" => "OutputPin<2,true>",
            "CH3" | "CH3A" => "OutputPin<3,false>",
            "CH3N" | "CH3B" => "OutputPin<3,true>",
            "BK" => "BrakePin",
            _ => continue,
        };
        if sealed_pins.insert(name) {
            writeln!(
                atim_pins,
                "impl sealed::Sealed for peripherals::{name} {{}}"
            )
            .unwrap();
        }
        writeln!(
            atim_pins,
            "impl {implementation} for peripherals::{name} {{ const AF:u8={af}; }}"
        )
        .unwrap();
    }
    fs::write(out.join("_generated_atim_pins.rs"), atim_pins).unwrap();
    let mut timer = String::new();
    let timers: Vec<_> = md
        .peripherals
        .iter()
        .filter(|p| matches!(p.block, "atim" | "gtim") && p.ownership_parent.is_none())
        .collect();
    for p in &timers {
        assert!(matches!(p.version, "l012" | "f030"), "unaudited timer IP");
        let name = ident(p.name);
        let variant = if p.block == "atim" { "Atim" } else { "Gtim" };
        writeln!(timer,"impl sealed::Instance for peripherals::{name} {{fn regs()->Registers {{Registers::{variant}(pac::{name})}} }} impl CoreInstance for peripherals::{name} {{}} impl sealed::PwmInstance for peripherals::{name} {{fn state()->&'static critical_section::Mutex<core::cell::RefCell<simple_pwm::State>> {{static STATE:critical_section::Mutex<core::cell::RefCell<simple_pwm::State>>=critical_section::Mutex::new(core::cell::RefCell::new(simple_pwm::State::new())); &STATE}} }} impl PwmInstance for peripherals::{name} {{}}").unwrap();
    }
    let mut timer_routes = BTreeMap::new();
    for route in md.pin_routes {
        let Some(p) = timers.iter().find(|p| p.name == route.peripheral) else {
            continue;
        };
        if route.remap.is_some() || matches!(route.pin, "PA13" | "PA14") {
            continue;
        }
        let Some(af) = route.af else { continue };
        let channel = match route.signal {
            "CH1" | "CH1A" => 1,
            "CH2" | "CH2A" => 2,
            "CH3" | "CH3A" => 3,
            "CH4" => 4,
            _ => continue,
        };
        if p.block == "atim" && p.version == "f030" && channel == 4 {
            panic!("F030 ATIM CH4 has no external output route");
        }
        assert!(
            md.pins.iter().any(|pin| pin.name == route.pin),
            "timer route references missing pin"
        );
        let key = (route.pin, route.peripheral, channel);
        if let Some(previous) = timer_routes.insert(key, af) {
            assert_eq!(previous, af, "ambiguous timer output AF");
            continue;
        }
        let pin = ident(route.pin);
        let peripheral = ident(route.peripheral);
        writeln!(timer,"impl sealed::Pin<peripherals::{peripheral},Ch{channel}> for peripherals::{pin} {{}} impl TimerPin<peripherals::{peripheral},Ch{channel}> for peripherals::{pin} {{const AF:u8={af};}}").unwrap();
    }
    timer.push_str(&capture_bindings(md));
    fs::write(out.join("_generated_timer.rs"), timer).unwrap();
    fs::write(
        out.join("_generated_complementary_pwm.rs"),
        complementary_bindings(md),
    )
    .unwrap();
    let mut adc = String::new();
    for p in md
        .peripherals
        .iter()
        .filter(|p| p.block == "adc" && p.ownership_parent.is_none())
    {
        let name = ident(p.name);
        let irq = md
            .interrupt_bindings
            .iter()
            .find(|b| b.peripheral == p.name && b.signal == "GLOBAL")
            .expect("ADC global IRQ")
            .interrupt;
        writeln!(adc,"impl sealed::Sealed for peripherals::{name} {{fn regs()->pac::adc::Adc {{pac::{name}}} fn state()->&'static crate::interrupt::EventState {{static STATE:crate::interrupt::EventState=crate::interrupt::EventState::new(); &STATE}}").unwrap();
        if md.peripherals.iter().any(|p| p.block == "dma") {
            let signal = match p.version {
                "l012" => "SEQUENCE",
                "f030" => "CONVERSION",
                _ => panic!("unaudited ADC DMA request semantics"),
            };
            let requests: Vec<_> = md
                .peripherals
                .iter()
                .filter_map(|p| p.dma)
                .flat_map(|dma| dma.requests)
                .filter(|r| r.peripheral == p.name && r.signal == signal)
                .collect();
            assert_eq!(
                requests.len(),
                1,
                "ADC DMA requires one audited request route"
            );
            let request = format!(
                "{}_{}",
                ident(requests[0].peripheral),
                ident(requests[0].signal)
            );
            writeln!(adc,"#[cfg(any(dma_l012,dma_f030))] fn dma_state()->&'static crate::adc::common::DmaState {{static STATE:crate::adc::common::DmaState=crate::adc::common::DmaState::new(); &STATE}} #[cfg(any(dma_l012,dma_f030))] fn dma_request()->crate::dma::Request {{crate::dma::Request::{request}}}").unwrap();
        }
        writeln!(adc,"}} impl Instance for peripherals::{name} {{type Interrupt=crate::interrupt::typelevel::{irq};}} ").unwrap();
        for route in md
            .pin_routes
            .iter()
            .filter(|r| r.peripheral == name && r.af.is_none() && r.remap.is_none())
        {
            let Some(channel) = route
                .signal
                .strip_prefix("IN")
                .and_then(|n| n.parse::<u8>().ok())
            else {
                continue;
            };
            if !md.pins.iter().any(|p| p.name == route.pin) {
                continue;
            }
            let max_external = match p.version {
                "l012" => 11, // RM1.4 table 25-4: 12/13 are internal DAC outputs.
                "f030" => 12, // RM2.5 table 22-5: 13 is the internal VDDA divider.
                _ => panic!("unaudited external ADC channel range"),
            };
            assert!(
                channel <= max_external,
                "GPIO route aliases an internal ADC source"
            );
            let pin = ident(route.pin);
            writeln!(adc,"impl sealed::PinSealed<peripherals::{name}> for peripherals::{pin} {{}} impl ChannelPin<peripherals::{name}> for peripherals::{pin} {{ const CHANNEL:u8={channel}; }}").unwrap();
        }
    }
    fs::write(out.join("_generated_adc.rs"), adc).unwrap();
    for kind in ["uart", "spi", "i2c"] {
        fs::write(
            out.join(format!("_generated_{kind}.rs")),
            bus_bindings(md, kind),
        )
        .unwrap();
    }
    // The ownership-bypass motor API still retains the physical peripheral
    // identity so enabling a source requires its actual type-level IRQ binding.
    // ADC reuses the existing Instance mapping above; only BTIM is new here.
    let mut motor_timer = String::new();
    for p in md
        .peripherals
        .iter()
        .filter(|p| p.block == "btim" && p.version == "l012" && p.ownership_parent.is_none())
    {
        let name = ident(p.name);
        let irqs: Vec<_> = md
            .interrupt_bindings
            .iter()
            .filter(|binding| binding.peripheral == p.name && binding.signal == "GLOBAL")
            .collect();
        assert_eq!(
            irqs.len(),
            1,
            "motor timer needs one audited GLOBAL interrupt"
        );
        let irq = ident(irqs[0].interrupt);
        writeln!(motor_timer, "impl sealed::Instance for crate::peripherals::{name} {{fn regs()->crate::pac::btim::Btim {{crate::pac::{name}}}}} impl BasicTimerInstance for crate::peripherals::{name} {{type Interrupt=crate::interrupt::typelevel::{irq};}}").unwrap();
    }
    fs::write(out.join("_generated_motor_timer.rs"), motor_timer).unwrap();
    let mut analog = String::new();
    if md
        .peripherals
        .iter()
        .any(|p| p.block == "adc" && p.version == "l012")
    {
        let dacs = md.peripherals.iter().filter(|p| p.block == "dac").count();
        assert!(dacs <= 1, "L012 internal ADC channels IN12/IN13 are audited for one shared DAC domain; multiple DAC connections require explicit data");
    }
    for p in md
        .peripherals
        .iter()
        .filter(|p| matches!(p.block, "opa" | "vc" | "vcref" | "dac"))
    {
        let name = ident(p.name);
        let (instance, block) = match p.block {
            "opa" => ("OpaInstance", "opa"),
            "vc" => ("VcInstance", "vc"),
            "dac" => ("DacInstance", "dac"),
            _ => ("RefInstance", "vcref"),
        };
        let block_type = format!("{}{}", block[..1].to_ascii_uppercase(), &block[1..]);
        writeln!(analog,"impl sealed::{instance} for peripherals::{name} {{ fn regs()->pac::{block}::{block_type} {{pac::{name}}}").unwrap();
        if p.block == "vc" {
            writeln!(analog,"fn state()->&'static crate::interrupt::EventState {{ static STATE:crate::interrupt::EventState=crate::interrupt::EventState::new(); &STATE }}").unwrap();
        }
        writeln!(analog, "}} impl {instance} for peripherals::{name} {{").unwrap();
        if p.block == "vc" {
            let irq = md
                .interrupt_bindings
                .iter()
                .find(|b| b.peripheral == p.name && b.signal == "GLOBAL")
                .expect("VC global IRQ")
                .interrupt;
            writeln!(analog, "type Interrupt=crate::interrupt::typelevel::{irq};").unwrap();
            let number = name.strip_prefix("VC").unwrap().parse::<u8>().unwrap();
            writeln!(analog, "const NUMBER:u8={number};").unwrap();
            if p.version == "l012"
                && md
                    .peripherals
                    .iter()
                    .any(|r| r.block == "vcref" && r.version == "l012")
            {
                // The reference pairing is an instance connection, unrelated
                // to the comparator's IRQ grouping or its diagnostic number.
                let reference = p
                    .comparator
                    .and_then(|c| c.reference)
                    .expect("comparator reference capability requires audited connection metadata");
                let target = md
                    .peripherals
                    .iter()
                    .find(|r| r.name == reference)
                    .expect("comparator reference target must exist");
                assert_eq!((target.block, target.version), ("vcref", "l012"));
                writeln!(
                    analog,
                    "#[cfg(vcref_l012)] type Reference=peripherals::{};",
                    ident(reference)
                )
                .unwrap();
            }
        }
        writeln!(analog, "}}").unwrap();
        if let Some(connection) = p
            .opa
            .map(|opa| opa.dac)
            .or_else(|| p.comparator.and_then(|vc| vc.dac))
        {
            let dac = md
                .peripherals
                .iter()
                .find(|dac| dac.name == connection.peripheral)
                .expect("analog DAC target");
            assert_eq!(
                (dac.block, dac.version),
                ("dac", "l012"),
                "DAC source owner needs audited hardware"
            );
            assert!(
                matches!(connection.channel, 1 | 2),
                "unmodeled DAC source channel"
            );
            let dac_name = ident(dac.name);
            writeln!(analog,"#[cfg(dac_l012)] impl sealed::DacSourceInstance<peripherals::{dac_name},{}> for peripherals::{name} {{}} #[cfg(dac_l012)] impl DacSourceInstance<peripherals::{dac_name},{}> for peripherals::{name} {{}}",connection.channel,connection.channel).unwrap();
        }
        if p.block == "opa" && p.version == "l012" {
            let outputs: Vec<_> = md
                .pin_routes
                .iter()
                .filter(|r| {
                    r.peripheral == p.name
                        && r.signal == "OUT"
                        && r.af.is_none()
                        && r.remap.is_none()
                        && md.pins.iter().any(|pin| pin.name == r.pin)
                })
                .collect();
            // The OPA owner erases its output pin type. An instance-only ADC
            // capability is valid only when that instance has one output pad.
            assert_eq!(
                outputs.len(),
                1,
                "OPA ADC bridge needs one unambiguous output pad"
            );
            for adc in md
                .peripherals
                .iter()
                .filter(|adc| adc.block == "adc" && adc.version == "l012")
            {
                let channels: BTreeSet<_> = md
                    .pin_routes
                    .iter()
                    .filter(|r| {
                        r.pin == outputs[0].pin
                            && r.peripheral == adc.name
                            && r.af.is_none()
                            && r.remap.is_none()
                    })
                    .filter_map(|r| {
                        r.signal
                            .strip_prefix("IN")
                            .and_then(|n| n.parse::<u8>().ok())
                    })
                    .collect();
                if channels.is_empty() {
                    continue;
                }
                assert_eq!(channels.len(), 1, "ambiguous OPA output ADC route");
                let channel = *channels.first().unwrap();
                assert!(
                    channel <= 11,
                    "OPA output must use a verified external ADC channel"
                );
                let adc = ident(adc.name);
                writeln!(analog,"#[cfg(all(opa_l012,bgr_l012,adc_l012))] impl sealed::OpaOutputChannel<peripherals::{adc}> for peripherals::{name} {{const CHANNEL:u8={channel};}} #[cfg(all(opa_l012,bgr_l012,adc_l012))] impl OpaOutputChannel<peripherals::{adc}> for peripherals::{name} {{}}").unwrap();
            }
        }
    }
    for r in md
        .pin_routes
        .iter()
        .filter(|r| r.af.is_none() && r.remap.is_none())
    {
        if !md.pins.iter().any(|p| p.name == r.pin) {
            continue;
        }
        let peripheral = md
            .peripherals
            .iter()
            .find(|p| p.name == r.peripheral)
            .expect("analog route peripheral");
        let signal = if peripheral.block == "opa" {
            match r.signal {
                "OUT" => 0,
                "INP1" => 1,
                "INP2" => 2,
                "INP3" => 3,
                "INN1" => 11,
                "INN2" => 12,
                _ => continue,
            }
        } else if peripheral.block == "vc" {
            match r
                .signal
                .strip_prefix("CH")
                .and_then(|s| s.parse::<u8>().ok())
            {
                Some(s) => {
                    // The explicitly audited route determines external-input validity.
                    // Register metadata checks representability, not a chip-family guess.
                    for selector in ["INP", "INN"] {
                        let field = field(peripheral, "CR0", selector);
                        assert!(
                            u32::from(s) < (1u32 << field.bit_size),
                            "VC route exceeds input selector"
                        );
                    }
                    s
                }
                _ => continue,
            }
        } else if peripheral.block == "dac" {
            match r.signal {
                "OUT1" => 1,
                "OUT2" => 2,
                _ => continue,
            }
        } else {
            continue;
        };
        let name = ident(r.peripheral);
        let pin = ident(r.pin);
        writeln!(analog,"impl sealed::Pin<peripherals::{name},{signal}> for peripherals::{pin} {{}} impl SignalPin<peripherals::{name},{signal}> for peripherals::{pin} {{}}").unwrap();
    }
    fs::write(out.join("_generated_analog.rs"), analog).unwrap();

    let mut irqs = String::from("embassy_hal_internal::interrupt_mod!(\n");
    let mut irq_names = BTreeSet::new();
    for irq in md.interrupts {
        assert!(irq_names.insert(irq.name), "duplicate IRQ name");
        writeln!(irqs, "{},", ident(irq.name)).unwrap();
    }
    irqs.push_str(");\n");
    fs::write(out.join("_generated_interrupts.rs"), irqs).unwrap();

    let mut memory = String::from(
        "/* Generated from cw32-metapac metadata; direct-reset layout. */\nMEMORY {\n",
    );
    let mut regions = BTreeSet::new();
    for region in md.memory {
        assert!(region.size > 0);
        assert!(regions.insert(region.name), "duplicate memory region");
        writeln!(
            memory,
            "  {} : ORIGIN = {:#x}, LENGTH = {:#x}",
            ident(region.name),
            region.address,
            region.size
        )
        .unwrap();
    }
    assert!(regions.contains("FLASH") && regions.contains("RAM"));
    if let Some(controller) = md.peripherals.iter().find(|p| p.block == "flash") {
        assert!(matches!(controller.version, "l012" | "f030"));
        let flash = md.memory.iter().find(|r| r.name == "FLASH").unwrap();
        assert_eq!(
            (flash.address, flash.size),
            (0, 65_536),
            "Flash protection geometry has only been audited for the current 64 KiB main array"
        );
        fs::write(out.join("_generated_flash.rs"), format!(
            "/// Main-array base from selected chip metadata.\npub const FLASH_BASE:usize={};\n/// Main-array size from selected chip metadata.\npub const FLASH_SIZE:usize={};\n", flash.address, flash.size)).unwrap();
    }
    memory.push_str("}\n");
    if env::var_os("CARGO_FEATURE_MEMORY_X").is_some() {
        fs::write(out.join("memory.x"), memory).unwrap();
    }
    if target.starts_with("thumb") {
        println!("cargo:rustc-link-search={}", out.display());
    }
}

fn clock_resource_name(gate: &cw32_metapac::metadata::RegisterBit) -> String {
    format!(
        "CLOCK_{}_{}_{}",
        ident(gate.peripheral),
        ident(gate.register),
        ident(gate.field)
    )
}

fn ownership_root<'a>(md: &'a Metadata, mut name: &'a str) -> &'a str {
    let mut seen = BTreeSet::new();
    loop {
        assert!(seen.insert(name), "cyclic clock ownership domain");
        let peripheral = md
            .peripherals
            .iter()
            .find(|p| p.name == name)
            .expect("clock owner identity");
        match peripheral.ownership_parent {
            Some(parent) => name = parent,
            None => return name,
        }
    }
}

/// One state object per physical gate. A shared reset is usable only when all
/// affected views belong to one ownership domain, such as a DMA controller and
/// its child channel aliases. Independent siblings and cross-effects suppress it.
fn clock_bindings(md: &Metadata, names: &[&str]) -> String {
    use cw32_metapac::metadata::{ClockSource, RegisterBit};
    let key = |r: RegisterBit| (r.peripheral, r.register, r.field);
    let mut groups = BTreeMap::<_, Vec<_>>::new();
    for p in md.peripherals {
        if let Some(gate) = p.clock_gate {
            groups.entry(key(gate)).or_default().push(p);
        }
    }
    let frequency = |source: ClockSource| match source {
        ClockSource::Hclk => "clocks().hclk_hz()",
        ClockSource::Pclk => "clocks().pclk_hz()",
    };
    let mut out = String::new();
    for members in groups.values() {
        let gate = members[0].clock_gate.unwrap();
        let resource = clock_resource_name(&gate);
        let owner = md
            .peripherals
            .iter()
            .find(|p| p.name == gate.peripheral)
            .expect("clock controller");
        assert_eq!(owner.name, "SYSCTRL", "unaudited clock key controller");
        assert!(matches!(gate.register, "AHBEN" | "APBEN1" | "APBEN2"));
        let register = ident(gate.register).to_ascii_lowercase();
        let field = ident(gate.field).to_ascii_lowercase();
        let domains: BTreeSet<_> = members.iter().map(|p| ownership_root(md, p.name)).collect();
        let reset = members[0].reset.filter(|reset| {
            domains.len() == 1
                && members
                    .iter()
                    .all(|p| p.reset.map(&key) == Some(key(*reset)) && p.reset_effects.is_empty())
                && md
                    .peripherals
                    .iter()
                    .filter(|p| p.reset.map(&key) == Some(key(*reset)))
                    .all(|p| {
                        domains.contains(ownership_root(md, p.name)) && p.reset_effects.is_empty()
                    })
        });
        writeln!(
            out,
            "pub(crate) static {resource}:ClockResource=ClockResource::new(|enabled|{{"
        )
        .unwrap();
        match owner.version {
            "l012" => writeln!(out,"let mut value=pac::SYSCTRL.{register}().read();value.set_key((pac::SYSCTRL_KEY>>16) as u16);value.set_{field}(enabled);pac::SYSCTRL.{register}().write_value(value);").unwrap(),
            "f030" => writeln!(out,"pac::SYSCTRL.{register}().modify(|w|w.set_{field}(enabled));").unwrap(),
            _ => panic!("unaudited clock gate semantics"),
        }
        if let Some(reset) = reset {
            assert_eq!(reset.peripheral, "SYSCTRL");
            assert!(matches!(reset.register, "AHBRST" | "APBRST1" | "APBRST2"));
            let register = ident(reset.register).to_ascii_lowercase();
            let field = ident(reset.field).to_ascii_lowercase();
            writeln!(out,"}},Some(||{{pac::SYSCTRL.{register}().modify(|w|w.set_{field}(false));pac::SYSCTRL.{register}().modify(|w|w.set_{field}(true));}}));").unwrap();
        } else {
            out.push_str("},None);\n");
        }
        for p in members {
            let tree = p
                .clock_tree
                .expect("clock gate requires sourced bus metadata");
            let expected_bus = if gate.register == "AHBEN" {
                ClockSource::Hclk
            } else {
                ClockSource::Pclk
            };
            assert_eq!(
                core::mem::discriminant(&tree.bus_clock),
                core::mem::discriminant(&expected_bus),
                "clock bus metadata disagrees with audited gate controller"
            );
            if !names.contains(&p.name) {
                continue;
            }
            let name = ident(p.name);
            let bus = frequency(tree.bus_clock);
            writeln!(out,"impl PeripheralClock for crate::peripherals::{name} {{fn clock_resource()->&'static ClockResource {{&{resource}}} fn bus_frequency()->u32 {{{bus}}}}}").unwrap();
            if let Some(kernel) = tree.kernel_clock {
                let kernel = frequency(kernel);
                writeln!(out,"impl KernelClock for crate::peripherals::{name} {{fn frequency()->u32 {{{kernel}}}}}").unwrap();
            }
        }
    }
    // Oscillator pads are physical routes, independent of the package name.
    for route in md
        .pin_routes
        .iter()
        .filter(|r| matches!(r.signal, "LSE_IN" | "LSE_OUT"))
    {
        let peripheral = md
            .peripherals
            .iter()
            .find(|p| p.name == route.peripheral)
            .expect("oscillator controller");
        assert_eq!(peripheral.block, "sysctrl");
        assert!(matches!(peripheral.version, "l012" | "f030"));
        assert!(
            route.af.is_none() && route.remap.is_none(),
            "LSE is a dedicated analog route"
        );
        assert!(
            md.pins.iter().any(|p| p.name == route.pin),
            "LSE pad is not implemented"
        );
        let pin = ident(route.pin);
        let direction = if route.signal == "LSE_IN" {
            "Input"
        } else {
            "Output"
        };
        writeln!(out, "#[cfg(gpio)] impl crate::rcc::low_speed::sealed::Lse{direction}Pin for crate::peripherals::{pin} {{}} #[cfg(gpio)] impl crate::rcc::Lse{direction}Pin for crate::peripherals::{pin} {{}}").unwrap();
    }
    out.push_str(&hse_pin_configuration(md));
    out.push_str(&lsi_startup_audit(md));
    out
}

/// Dedicated crystal/input pads are physical metadata, not a SYSCTRL-version
/// naming assumption. Only those identities become optional after startup.
fn hse_pin_roles(md: &Metadata) -> BTreeMap<&'static str, &'static str> {
    let mut pins = BTreeMap::new();
    let mut signals = BTreeSet::new();
    for route in md
        .pin_routes
        .iter()
        .filter(|r| matches!(r.signal, "HSE_IN" | "HSE_OUT"))
    {
        let controller = md
            .peripherals
            .iter()
            .find(|p| p.name == route.peripheral)
            .expect("HSE controller");
        assert_eq!(controller.block, "sysctrl");
        assert!(matches!(controller.version, "l012" | "f030"));
        assert!(
            route.af.is_none() && route.remap.is_none(),
            "HSE uses dedicated pads"
        );
        assert!(
            md.pins.iter().any(|p| p.name == route.pin),
            "HSE pad unavailable"
        );
        assert!(signals.insert(route.signal), "ambiguous HSE signal route");
        let role = if route.signal == "HSE_IN" {
            "input"
        } else {
            "output"
        };
        assert!(
            pins.insert(route.pin, role).is_none(),
            "HSE input/output must be distinct"
        );
    }
    assert_eq!(pins.len(), 2, "startup HSE requires two evidenced pads");
    pins
}

fn hse_pin_configuration(md: &Metadata) -> String {
    let mut out=String::from("/// Exclusive early startup, after retained-oscillator rejection.\npub(crate) fn configure_hse_pins(mode:HseMode)->Result<(),ClockError>{\n");
    for (name, role) in hse_pin_roles(md) {
        let pin = md.pins.iter().find(|p| p.name == name).unwrap();
        let port = md
            .peripherals
            .iter()
            .find(|p| p.name == pin.port)
            .expect("HSE GPIO port");
        assert_eq!(port.block, "gpio");
        let gpio = ident(port.name);
        let variant = ident(port.name.strip_prefix("GPIO").expect("GPIO port identity"));
        let n = pin.number;
        if role == "output" {
            out.push_str("if mode==HseMode::Crystal {\n");
        }
        writeln!(out,"let pin=crate::gpio::AnyPin::new(crate::gpio::Port::{variant},{n});\nlet analog=mode==HseMode::Crystal;\nif analog {{pin.configure_analog();}} else {{pin.disconnect();}}\nlet r=crate::pac::{gpio};\nif !r.dir().read().pin({n}) || r.analog().read().pin({n})!=analog || r.pur().read().pin({n}) || r.afr({}).read().afr({})!=0 {{return Err(ClockError::HsePinReadbackMismatch);}}",n/8,n%8).unwrap();
        if port.pulldown_mask & (1u16 << n) != 0 {
            if port.version == "f030" {
                writeln!(out,"if r.pdr().read().pin({n}) {{return Err(ClockError::HsePinReadbackMismatch);}}").unwrap();
            } else {
                assert_eq!(n, 3, "unaudited scalar pull-down");
                out.push_str(
                    "if r.pdr().read().pin3() {return Err(ClockError::HsePinReadbackMismatch);}\n",
                );
            }
        }
        if role == "output" {
            out.push_str("}\n");
        }
    }
    out.push_str("Ok(())\n}\n");
    out
}

/// Startup-only off-state proof for the LSI calibration write. LSIEN does not
/// reflect automatic clock demands (L012 RM4.7.2); STABLE=0 may mean starting.
/// Inspect every documented requester without reset, then restore config gates.
fn lsi_startup_audit(md: &Metadata) -> String {
    let controller = md
        .peripherals
        .iter()
        .find(|p| p.block == "sysctrl")
        .expect("SYSCTRL");
    assert!(matches!(controller.version, "l012" | "f030"));
    let requesters: Vec<_> = md
        .peripherals
        .iter()
        .filter(|p| {
            p.block == "gpio"
                || (controller.version == "l012" && matches!(p.block, "vc" | "lvd" | "iwdt"))
        })
        .collect();
    let mut gates = BTreeMap::<&str, BTreeSet<&str>>::new();
    for p in &requesters {
        assert!(
            matches!(p.version, "l012" | "f030"),
            "unaudited LSI requester IP"
        );
        if let Some(gate) = p.clock_gate {
            assert_eq!(gate.peripheral, controller.name);
            assert!(matches!(gate.register, "AHBEN" | "APBEN1" | "APBEN2"));
            gates.entry(gate.register).or_default().insert(gate.field);
        }
    }
    let mut out = String::from("/// Exclusive early startup only; no peripheral reset or policy writes.\npub(crate) fn lsi_trim_is_safe()->bool {\nlet cr1=pac::SYSCTRL.cr1().read();if cr1.lsien()||cr1.hseccs()||cr1.lseccs()||pac::SYSCTRL.lsi().read().stable(){return false;}\n");
    for (register, fields) in &gates {
        let register = register.to_ascii_lowercase();
        writeln!(out, "let saved_{register}=pac::SYSCTRL.{register}().read();let mut enabled_{register}=saved_{register};").unwrap();
        // AHBEN/APBEN are ordinary RW on F030 and keyed on L012.
        if controller.version == "l012" {
            writeln!(
                out,
                "enabled_{register}.set_key((pac::SYSCTRL_KEY>>16) as u16);"
            )
            .unwrap();
        }
        for field in fields {
            writeln!(
                out,
                "enabled_{register}.set_{}(true);",
                field.to_ascii_lowercase()
            )
            .unwrap();
        }
        writeln!(
            out,
            "pac::SYSCTRL.{register}().write_value(enabled_{register});"
        )
        .unwrap();
    }
    out.push_str("let mut active=false;\n");
    for p in requesters {
        let name = ident(p.name);
        let condition = match p.block {
            "gpio" => format!("pac::{name}.filter().read().fltclk()==5"),
            "vc" => format!("pac::{name}.cr0().read().en()&&!pac::{name}.cr1().read().fltclk()"),
            "lvd" => {
                let reg = if p.version == "l012" { "cr0" } else { "cr1" };
                format!("pac::{name}.cr0().read().en()&&!pac::{name}.{reg}().read().fltclk()")
            }
            "iwdt" => format!("pac::{name}.sr().read().run()"),
            _ => unreachable!(),
        };
        writeln!(out, "active|={condition};").unwrap();
    }
    for register in gates.keys().rev() {
        let register = register.to_ascii_lowercase();
        if controller.version == "l012" {
            writeln!(out, "let mut saved_{register}=saved_{register};saved_{register}.set_key((pac::SYSCTRL_KEY>>16) as u16);").unwrap();
        }
        writeln!(
            out,
            "pac::SYSCTRL.{register}().write_value(saved_{register});"
        )
        .unwrap();
    }
    out.push_str("let cr1=pac::SYSCTRL.cr1().read();!active&&!cr1.lsien()&&!cr1.hseccs()&&!cr1.lseccs()&&!pac::SYSCTRL.lsi().read().stable()\n}\n");
    out
}

/// Split identities and request encodings are controller topology, not register-name guesses.
fn dma_bindings(md: &Metadata) -> String {
    let controllers: Vec<_> = md.peripherals.iter().filter(|p| p.block == "dma").collect();
    if controllers.is_empty() {
        return String::new();
    }
    assert_eq!(
        controllers.len(),
        1,
        "multiple DMA controllers need independent driver domains"
    );
    let controller = controllers[0];
    let name = ident(controller.name);
    let dma = controller
        .dma
        .expect("DMA driver requires audited controller topology");
    assert!(!dma.channels.is_empty(), "DMA has no audited channels");
    let mut out = format!(
        "/// The sole DMA controller selected by device metadata.\npub type Controller=crate::peripherals::{name};\n\
         /// Channel tokens obtained only by consuming the complete controller.\npub struct Channels<'d> {{\n"
    );
    let mut numbers = BTreeSet::new();
    let mut indices = BTreeSet::new();
    let mut aliases = BTreeSet::new();
    for channel in dma.channels {
        assert!(numbers.insert(channel.number) && indices.insert(channel.index));
        assert!(
            aliases.insert(channel.peripheral),
            "duplicate DMA channel alias"
        );
        let alias = ident(channel.peripheral);
        let p = md
            .peripherals
            .iter()
            .find(|p| p.name == alias)
            .expect("DMA channel instance");
        assert_eq!(p.block, "dmachannel");
        assert_eq!(p.ownership_parent, Some(controller.name));
        assert!(md.interrupt_bindings.iter().any(|b| b.peripheral == alias
            && b.signal == "GLOBAL"
            && b.interrupt == channel.interrupt));
        writeln!(
            out,
            "pub ch{}:crate::Peri<'d,crate::peripherals::{alias}>,",
            channel.number
        )
        .unwrap();
    }
    out.push_str("}\npub(crate) unsafe fn split_tokens<'d>()->Channels<'d>{Channels{\n");
    for channel in dma.channels {
        writeln!(
            out,
            "ch{}:unsafe{{crate::peripherals::{}::steal()}},",
            channel.number,
            ident(channel.peripheral)
        )
        .unwrap();
    }
    out.push_str("}}\n");
    for channel in dma.channels {
        let alias = ident(channel.peripheral);
        let irq = ident(channel.interrupt);
        writeln!(out, "impl sealed::Instance for crate::peripherals::{alias} {{const INDEX:usize={}; fn state()->&'static crate::dma::ChannelState {{static STATE:crate::dma::ChannelState=crate::dma::ChannelState::new(); &STATE}}}}\nimpl Instance for crate::peripherals::{alias}{{type Interrupt=crate::interrupt::typelevel::{irq};}}", channel.index).unwrap();
    }
    out.push_str("/// Audited hardware trigger selectors. The peripheral remains the caller's responsibility.\n#[allow(non_camel_case_types)]\n#[derive(Clone,Copy,Debug,PartialEq,Eq)]\n#[repr(u8)]\npub enum Request {\n");
    let mut requests = BTreeSet::new();
    let mut selectors = BTreeSet::new();
    for request in dma.requests {
        let variant = format!("{}_{}", ident(request.peripheral), ident(request.signal));
        assert!(
            requests.insert(variant.clone()),
            "duplicate generated DMA request name"
        );
        assert!(
            selectors.insert(request.selector),
            "duplicate DMA selector discriminant"
        );
        writeln!(out, "{variant}={},", request.selector).unwrap();
    }
    out.push_str("}\n");
    // The normal clock implementation deliberately omits a shared reset. Only
    // this whole-controller acquisition may reset its descendant channel views.
    let reset = controller.reset.expect("audited DMA controller reset");
    assert!(
        controller.reset_effects.is_empty(),
        "DMA reset affects another domain"
    );
    for p in md.peripherals {
        if p.reset.is_some_and(|r| {
            r.peripheral == reset.peripheral
                && r.register == reset.register
                && r.field == reset.field
        }) {
            assert!(
                p.name == controller.name || aliases.contains(p.name),
                "DMA reset affects a separately owned peripheral"
            );
            assert!(
                p.reset_effects.is_empty(),
                "DMA descendant reset affects another domain"
            );
        }
    }
    out.push_str("pub(crate) fn initialize_controller(){\n");
    for channel in dma.channels {
        writeln!(out, "crate::dma::quarantine_for_controller_reset(<crate::peripherals::{} as sealed::Instance>::state());", ident(channel.peripheral)).unwrap();
    }
    out.push_str("let _clock=<Controller as crate::rcc::PeripheralClock>::acquire();}\n");
    out
}

/// Digital bus identities come from the selected peripheral and physical AF
/// routes. Sharing a vector generates the same Interrupt type for each source;
/// the application's bind_interrupts! invocation dispatches all source handlers.
fn capture_bindings(md: &Metadata) -> String {
    let mut out = String::new();
    for p in md
        .peripherals
        .iter()
        .filter(|p| matches!(p.block, "atim" | "gtim") && p.ownership_parent.is_none())
    {
        assert!(matches!(p.version, "l012" | "f030"), "unaudited capture IP");
        let name = ident(p.name);
        let irqs: Vec<_> = md
            .interrupt_bindings
            .iter()
            .filter(|i| i.peripheral == p.name && i.signal == "GLOBAL")
            .collect();
        assert_eq!(
            irqs.len(),
            1,
            "timer capture requires one physical global IRQ"
        );
        let irq = ident(irqs[0].interrupt);
        let selector = if p.block == "gtim" && p.version == "f030" {
            let mux = p
                .timer_capture_mux
                .expect("F030 GTIM requires its external capture selector");
            let controller = md
                .peripherals
                .iter()
                .find(|c| c.name == mux.peripheral)
                .expect("capture mux controller");
            assert_eq!(
                (controller.block, controller.version),
                ("sysctrl", "f030"),
                "unaudited capture mux controller"
            );
            let register = controller
                .registers
                .iter()
                .find(|r| r.name == mux.register)
                .expect("capture mux register");
            assert_eq!(
                register.elements.len(),
                4,
                "audited four-timer mux register array"
            );
            let field = register
                .fields
                .iter()
                .find(|f| f.name == mux.field)
                .expect("capture mux channel field");
            assert_eq!(
                (field.elements.len(), field.bit_size),
                (4, 3),
                "audited four-channel capture mux"
            );
            assert_eq!(mux.external_value, 0, "audited external pin selection");
            format!(
                "crate::pac::{}.{}({}).modify(|r|r.set_{}(channel,{}));",
                ident(mux.peripheral),
                ident(mux.register).to_ascii_lowercase(),
                mux.index,
                ident(mux.field).to_ascii_lowercase(),
                mux.external_value
            )
        } else {
            assert!(
                p.timer_capture_mux.is_none(),
                "unhandled external capture mux"
            );
            "let _=channel;".into()
        };
        writeln!(out,"impl input_capture::sealed::Instance for peripherals::{name} {{fn state()->&'static crate::interrupt::EventState {{static STATE:crate::interrupt::EventState=crate::interrupt::EventState::new(); &STATE}} fn select_external_input(channel:usize) {{{selector}}}}} impl input_capture::Instance for peripherals::{name} {{type Interrupt=crate::interrupt::typelevel::{irq};}}").unwrap();
        let (first, second) = if p.block == "atim" && p.version == "f030" {
            ("Ch1A", "Ch1B")
        } else {
            ("Ch1", "Ch2")
        };
        writeln!(out,"impl qei::sealed::Instance for peripherals::{name} {{}} impl qei::Instance for peripherals::{name} {{type First=input_capture::{first};type Second=input_capture::{second};}}").unwrap();
        let mut pins = BTreeMap::new();
        for route in md
            .pin_routes
            .iter()
            .filter(|r| r.peripheral == p.name && r.remap.is_none())
        {
            if matches!(route.pin, "PA13" | "PA14") {
                continue;
            }
            let marker = match (p.block, p.version, route.signal) {
                ("atim", "f030", "CH1A") => "Ch1A",
                ("atim", "f030", "CH1B") => "Ch1B",
                ("atim", "f030", "CH2A") => "Ch2A",
                ("atim", "f030", "CH2B") => "Ch2B",
                ("atim", "f030", "CH3A") => "Ch3A",
                ("atim", "f030", "CH3B") => "Ch3B",
                ("atim", "f030", _) => continue,
                (_, _, "CH1") => "Ch1",
                (_, _, "CH2") => "Ch2",
                (_, _, "CH3") => "Ch3",
                (_, _, "CH4") => "Ch4",
                _ => continue,
            };
            let af = route.af.expect("capture pin requires an AF route");
            assert!(af < 16 && md.pins.iter().any(|pin| pin.name == route.pin));
            if let Some(old) = pins.insert((route.pin, marker), af) {
                assert_eq!(old, af, "ambiguous capture pin AF");
                continue;
            }
            let pin = ident(route.pin);
            writeln!(out,"impl input_capture::sealed::Pin<peripherals::{name},input_capture::{marker}> for peripherals::{pin} {{}} impl input_capture::CapturePin<peripherals::{name},input_capture::{marker}> for peripherals::{pin} {{const AF:u8={af};}}").unwrap();
        }
    }
    out
}

fn complementary_bindings(md: &Metadata) -> String {
    let mut out = String::new();
    for p in md
        .peripherals
        .iter()
        .filter(|p| p.block == "atim" && p.ownership_parent.is_none())
    {
        assert!(
            matches!(p.version, "l012" | "f030"),
            "unaudited complementary PWM IP"
        );
        let name = ident(p.name);
        writeln!(out,"impl sealed::Instance for peripherals::{name} {{fn registers()->pac::atim::Atim {{pac::{name}}}}} impl Instance for peripherals::{name} {{}}").unwrap();
        let mut pins = BTreeMap::new();
        for route in md
            .pin_routes
            .iter()
            .filter(|r| r.peripheral == p.name && r.remap.is_none())
        {
            if matches!(route.pin, "PA13" | "PA14") {
                continue;
            }
            let marker = match (p.version, route.signal) {
                ("l012", "CH1N") | ("f030", "CH1B") => "Ch1",
                ("l012", "CH2N") | ("f030", "CH2B") => "Ch2",
                ("l012", "CH3N") | ("f030", "CH3B") => "Ch3",
                (_, "BK") => "Brake",
                _ => continue,
            };
            let af = route.af.expect("complementary pin requires AF");
            assert!(af < 16 && md.pins.iter().any(|pin| pin.name == route.pin));
            if let Some(old) = pins.insert((route.pin, marker), af) {
                assert_eq!(old, af, "ambiguous complementary AF");
                continue;
            }
            let pin = ident(route.pin);
            if marker == "Brake" {
                writeln!(out,"impl sealed::BrakePin<peripherals::{name}> for peripherals::{pin} {{}} impl TimerBrakePin<peripherals::{name}> for peripherals::{pin} {{const AF:u8={af};}}").unwrap();
            } else {
                writeln!(out,"impl sealed::Pin<peripherals::{name},{marker}> for peripherals::{pin} {{}} impl ComplementaryPin<peripherals::{name},{marker}> for peripherals::{pin} {{const AF:u8={af};}}").unwrap();
            }
        }
    }
    out
}

fn bus_bindings(md: &Metadata, kind: &str) -> String {
    let (register_type, sealed, signals): (&str, &str, &[(&str, &str)]) = match kind {
        "uart" => ("Uart", "Instance", &[("TX", "TxPin"), ("RX", "RxPin")]),
        "spi" => (
            "Spi",
            "Instance",
            &[("SCK", "SckPin"), ("MOSI", "MosiPin"), ("MISO", "MisoPin")],
        ),
        "i2c" => ("I2c", "Sealed", &[("SCL", "SclPin"), ("SDA", "SdaPin")]),
        _ => unreachable!(),
    };
    let mut out = String::new();
    for p in md
        .peripherals
        .iter()
        .filter(|p| p.block == kind && p.ownership_parent.is_none())
    {
        let name = ident(p.name);
        let interrupts: Vec<_> = md
            .interrupt_bindings
            .iter()
            .filter(|b| b.peripheral == name && b.signal == "GLOBAL")
            .collect();
        assert_eq!(interrupts.len(), 1, "bus needs one audited GLOBAL vector");
        let irq = ident(interrupts[0].interrupt);
        if kind == "i2c" && p.version == "f030" {
            // This IP has no source interrupt-enable register. Its driver
            // masks NVIC while consuming SI; that is valid only for an
            // independently owned physical vector, never a shared alias.
            assert_eq!(
                md.interrupt_bindings
                    .iter()
                    .filter(|b| b.interrupt == irq)
                    .count(),
                1,
                "F030 I2C requires its documented dedicated interrupt vector"
            );
        }
        let state_type = if kind == "uart" {
            "crate::uart::State".to_owned()
        } else {
            "crate::interrupt::EventState".to_owned()
        };
        writeln!(out, "impl sealed::{sealed} for crate::peripherals::{name} {{fn regs()->crate::pac::{kind}::{register_type} {{crate::pac::{name}}} fn state()->&'static {state_type} {{static STATE:{state_type}={state_type}::new(); &STATE}}}} impl Instance for crate::peripherals::{name} {{type Interrupt=crate::interrupt::typelevel::{irq};}}").unwrap();
        if matches!(kind, "uart" | "spi") {
            assert!(matches!(p.version, "l012" | "f030"), "unaudited bus DMA IP");
            for controller in md.peripherals.iter().filter(|p| p.block == "dma") {
                let dma = controller
                    .dma
                    .expect("bus DMA requires controller topology");
                for (signal, direction) in [("TX", "Tx"), ("RX", "Rx")] {
                    let requests: Vec<_> = dma
                        .requests
                        .iter()
                        .filter(|r| r.peripheral == name && r.signal == signal)
                        .collect();
                    assert_eq!(
                        requests.len(),
                        1,
                        "bus direction needs exactly one audited DMA request"
                    );
                    let request = format!(
                        "{}_{}",
                        ident(requests[0].peripheral),
                        ident(requests[0].signal)
                    );
                    for channel in dma.channels {
                        let channel = ident(channel.peripheral);
                        writeln!(out, "#[cfg(dma)] impl crate::{kind}::dma::sealed::{direction}Dma<crate::peripherals::{name}> for crate::peripherals::{channel} {{}} #[cfg(dma)] impl crate::{kind}::dma::{direction}Dma<crate::peripherals::{name}> for crate::peripherals::{channel} {{const REQUEST:crate::dma::Request=crate::dma::Request::{request};}}").unwrap();
                    }
                }
            }
        }
        let mut routes = BTreeMap::new();
        for route in md.pin_routes.iter().filter(|r| r.peripheral == name) {
            if route.remap.is_some() || matches!(route.pin, "PA13" | "PA14") {
                // Debug pins need an explicit debug-port release API first.
                continue;
            }
            let Some((_, signal_trait)) = signals.iter().find(|(s, _)| *s == route.signal) else {
                continue;
            };
            let Some(af) = route.af else { continue };
            assert!(af < 16, "bus AF exceeds GPIO selector");
            assert!(
                md.pins.iter().any(|p| p.name == route.pin),
                "bus route pin is unavailable"
            );
            if let Some(previous) = routes.insert((route.pin, route.signal), af) {
                assert_eq!(previous, af, "ambiguous bus pin alternate function");
                continue;
            }
            let pin = ident(route.pin);
            writeln!(out,"impl sealed::{signal_trait}<crate::peripherals::{name}> for crate::peripherals::{pin} {{}} impl {signal_trait}<crate::peripherals::{name}> for crate::peripherals::{pin} {{const AF:u8={af};}}").unwrap();
        }
    }
    out
}

fn associations(md: &Metadata) -> String {
    let mut out = String::from(
        "// Generated from cw32-metapac metadata. No driver or ISR registration is implied.\n\
         #[derive(Clone, Copy, Debug, PartialEq, Eq)]\n\
         pub struct InterruptBinding { pub peripheral: &'static str, pub signal: &'static str, pub interrupt: &'static str, pub number: u16 }\n\
         #[derive(Clone, Copy, Debug, PartialEq, Eq)]\n\
         pub struct PinRoute { pub pin: &'static str, pub peripheral: &'static str, pub signal: &'static str, pub af: Option<u8>, pub remap: Option<&'static str> }\n\
         #[derive(Clone, Copy, Debug, PartialEq, Eq)]\n\
         pub struct Quirk { pub peripheral: Option<&'static str>, pub name: &'static str, pub description: &'static str, pub source: &'static str }\n\
         #[derive(Clone, Copy, Debug, PartialEq, Eq)]\n\
         pub struct Remap { pub name: &'static str, pub peripheral: &'static str, pub description: &'static str }\n\
         pub const INTERRUPT_BINDINGS: &[InterruptBinding] = &[\n",
    );
    let mut seen = BTreeSet::new();
    for binding in md.interrupt_bindings {
        assert!(
            seen.insert((binding.peripheral, binding.signal, binding.interrupt)),
            "duplicate peripheral/signal/vector association"
        );
        // Only the complete triple is unique: many signals/peripherals may
        // share one vector, and one signal may refer to multiple vectors.
        let irq = md
            .interrupts
            .iter()
            .find(|i| i.name == binding.interrupt)
            .expect("interrupt association references an unknown vector");
        writeln!(out, "    InterruptBinding {{ peripheral: {:?}, signal: {:?}, interrupt: {:?}, number: {} }},",
                 binding.peripheral, binding.signal, binding.interrupt, irq.number).unwrap();
    }
    out.push_str("];\npub const PIN_ROUTES: &[PinRoute] = &[\n");
    for route in md.pin_routes {
        writeln!(
            out,
            "    PinRoute {{ pin: {:?}, peripheral: {:?}, signal: {:?}, af: {:?}, remap: {:?} }},",
            route.pin, route.peripheral, route.signal, route.af, route.remap
        )
        .unwrap();
    }
    out.push_str("];\npub const QUIRKS: &[Quirk] = &[\n");
    for quirk in md.quirks {
        writeln!(
            out,
            "    Quirk {{ peripheral: {:?}, name: {:?}, description: {:?}, source: {:?} }},",
            quirk.peripheral, quirk.name, quirk.description, quirk.source
        )
        .unwrap();
    }
    out.push_str("];\npub const REMAPS: &[Remap] = &[\n");
    for remap in md.remaps {
        writeln!(
            out,
            "    Remap {{ name: {:?}, peripheral: {:?}, description: {:?} }},",
            remap.name, remap.peripheral, remap.description
        )
        .unwrap();
    }
    out.push_str("];\n");
    out
}

/// Driver contracts supported by this HAL, not a map from chip names to families.
/// Every entry is selected independently using the peripheral's register metadata.
const DRIVER_IPS: &[(&str, &[&str])] = &[
    ("gpio", &["l012", "f030"]),
    ("sysctrl", &["l012", "f030"]),
    ("gtim", &["l012", "f030"]),
    ("adc", &["l012", "f030"]),
    ("atim", &["l012", "f030"]),
    ("cordic", &["l012"]),
    ("eau", &["l012"]),
    ("opa", &["l012"]),
    ("vc", &["l012", "f030"]),
    ("vcref", &["l012"]),
    ("dac", &["l012"]),
    ("bgr", &["l012"]),
    ("dma", &["l012", "f030"]),
    ("dmachannel", &["l012", "f030"]),
    ("btim", &["l012", "f030"]),
    ("uart", &["l012", "f030"]),
    ("spi", &["l012", "f030"]),
    ("i2c", &["l012", "f030"]),
    ("crc", &["l012", "f030"]),
    ("iwdt", &["l012"]),
    ("wwdt", &["l012"]),
    ("rtc", &["l012", "f030"]),
    ("flash", &["l012", "f030"]),
];
const GPIO_CAPABILITIES: &[&str] = &[
    "gpio_has_speed",
    "gpio_has_drive_strength",
    "gpio_has_level_interrupts",
    "gpio_pulldown_indexed",
];
const CRC_CAPABILITIES: &[&str] = &["crc_has_crc32", "crc_has_wide_data"];

fn field<'a>(
    p: &'a cw32_metapac::metadata::Peripheral,
    register: &str,
    name: &str,
) -> &'a cw32_metapac::metadata::RegisterField {
    p.registers
        .iter()
        .find(|r| r.name == register)
        .and_then(|r| r.fields.iter().find(|f| f.name == name))
        .expect("driver requires the audited register field")
}

fn gpio_capabilities(p: &cw32_metapac::metadata::Peripheral) -> BTreeSet<&'static str> {
    use cw32_metapac::metadata::{Access, Array, FieldKind, ReadBehavior, WriteBehavior};
    let mut result = BTreeSet::new();
    let indexed = |name: &str| {
        let Some(r) = p.registers.iter().find(|r| r.name == name) else {
            return false;
        };
        assert!(
            matches!(r.access, Access::ReadWrite)
                && matches!(r.write_behavior, WriteBehavior::Ordinary)
                && matches!(r.read_behavior, ReadBehavior::Ordinary)
                && r.bit_size == 32
                && r.array.is_none(),
            "GPIO capability requires an ordinary scalar RW register"
        );
        let f = field(p, name, "PIN");
        assert!(
            f.bit_offset == 0
                && f.bit_size == 1
                && matches!(f.kind, FieldKind::Bool)
                && matches!(f.access, Access::ReadWrite)
                && f.array.is_some_and(|a| match a {
                    Array::Regular { len, stride } => len == 16 && stride == 1,
                    Array::Explicit(offsets) =>
                        offsets.len() == 16
                            && offsets.iter().enumerate().all(|(n, offset)| n == *offset),
                }),
            "GPIO capability requires the audited indexed PIN layout"
        );
        true
    };
    if indexed("SPEED") {
        result.insert("gpio_has_speed");
    }
    if indexed("DRIVER") {
        result.insert("gpio_has_drive_strength");
    }
    let high = indexed("HIGHIE");
    let low = indexed("LOWIE");
    assert_eq!(
        high, low,
        "GPIO level interrupt capability requires both polarities"
    );
    if high {
        result.insert("gpio_has_level_interrupts");
    }
    // Per-pin masks still decide which actual pins can use this register.
    // Do not treat a register layout as evidence of package bonding.
    if p.registers
        .iter()
        .find(|r| r.name == "PDR")
        .is_some_and(|r| r.fields.iter().any(|f| f.name == "PIN"))
    {
        assert!(indexed("PDR"));
        result.insert("gpio_pulldown_indexed");
    } else {
        let r = p.registers.iter().find(|r| r.name == "PDR").unwrap();
        let f = field(p, "PDR", "PIN3");
        assert!(
            matches!(r.access, Access::ReadWrite)
                && matches!(r.write_behavior, WriteBehavior::Ordinary)
                && matches!(r.read_behavior, ReadBehavior::Ordinary)
                && r.bit_size == 32
                && r.array.is_none()
                && matches!(f.access, Access::ReadWrite)
                && f.bit_offset == 3
                && matches!(f.kind, FieldKind::Bool)
                && f.bit_size == 1
                && f.array.is_none(),
            "unaudited scalar pull-down layout"
        );
    }
    result
}

/// Select each IP and capability independently from the selected metadata.
fn driver_cfgs(md: &Metadata) -> BTreeSet<String> {
    let mut enabled = BTreeSet::new();
    let mut gpio = None;
    for p in md.peripherals {
        if let Some((_, versions)) = DRIVER_IPS.iter().find(|(kind, _)| *kind == p.block) {
            assert!(
                versions.contains(&p.version),
                "unsupported HAL IP contract: {}_{}",
                p.block,
                p.version
            );
        }
        enabled.insert(p.block.to_owned());
        enabled.insert(format!("{}_{}", p.block, p.version));
        enabled.insert(format!("peri_{}", p.name.to_ascii_lowercase()));
        if p.block == "gpio" {
            let caps = gpio_capabilities(p);
            if let Some(previous) = &gpio {
                assert_eq!(
                    previous, &caps,
                    "heterogeneous GPIO capabilities need per-instance drivers"
                );
            }
            gpio = Some(caps);
        }
        if p.block == "crc" {
            let mode = field(p, "CR", "MODE");
            assert!(matches!(mode.kind, cw32_metapac::metadata::FieldKind::Enum));
            if mode
                .values
                .iter()
                .any(|v| v.name == "CRC32" && v.value == 8)
            {
                assert!(mode
                    .values
                    .iter()
                    .any(|v| v.name == "CRC32_MPEG2" && v.value == 9));
                enabled.insert("crc_has_crc32".into());
            }
            if p.registers
                .iter()
                .any(|r| r.name == "DR16" && r.bit_size == 16)
            {
                assert!(p
                    .registers
                    .iter()
                    .any(|r| r.name == "DR8" && r.bit_size == 8));
                assert!(p
                    .registers
                    .iter()
                    .any(|r| r.name == "DR32" && r.bit_size == 32));
                enabled.insert("crc_has_wide_data".into());
            }
        }
    }
    for cap in gpio.into_iter().flatten() {
        enabled.insert(cap.to_owned());
    }
    enabled
}

fn emit_driver_cfgs(md: &Metadata) {
    let mut declared = BTreeSet::new();
    for (kind, versions) in DRIVER_IPS {
        declared.insert((*kind).to_owned());
        for version in *versions {
            declared.insert(format!("{kind}_{version}"));
        }
    }
    declared.extend(GPIO_CAPABILITIES.iter().map(|c| (*c).to_owned()));
    declared.extend(CRC_CAPABILITIES.iter().map(|c| (*c).to_owned()));
    // Only instance predicates explicitly consumed by hand-written drivers need
    // declaration when absent. All selected metadata instances are declared below.
    declared.extend(["peri_adc1", "peri_adc2", "peri_gtim1"].map(str::to_owned));
    let enabled = driver_cfgs(md);
    declared.extend(enabled.iter().cloned());
    for cfg in declared {
        println!("cargo:rustc-check-cfg=cfg({cfg})");
    }
    for cfg in enabled {
        println!("cargo:rustc-cfg={cfg}");
    }
}
