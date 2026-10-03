//! Generate the HAL identity layer from the separately generated PAC metadata.
//! No register addresses, chip pin lists or interrupt numbers live in this file.
mod build_support;
use cw32_metapac::metadata::METADATA;
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
    println!("cargo:rerun-if-changed=build_support.rs");
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
    build_support::emit_driver_cfgs(md);
    fs::write(
        out.join("_generated_associations.rs"),
        build_support::associations(md),
    )
    .unwrap();
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
        let owner = md
            .peripherals
            .iter()
            .find(|p| p.name == gate.peripheral)
            .unwrap();
        assert!(
            matches!(owner.version, "l012" | "f030"),
            "unaudited GPIO clock controller"
        );
        let register = gate.register.to_ascii_lowercase();
        let field = gate.field.to_ascii_lowercase();
        if owner.version == "l012" {
            writeln!(ports,"Self::{variant}=>{{let mut value=pac::SYSCTRL.{register}().read();value.set_key((pac::SYSCTRL_KEY>>16) as u16);value.set_{field}(true);pac::SYSCTRL.{register}().write_value(value);}},").unwrap();
        } else {
            writeln!(
                ports,
                "Self::{variant}=>pac::SYSCTRL.{register}().modify(|w|w.set_{field}(true)),"
            )
            .unwrap();
        }
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
    for name in peripheral_names
        .iter()
        .copied()
        .chain(md.pins.iter().map(|p| ident(p.name)))
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
        .filter(|name| !(reserved_timer && *name == "GTIM1"))
        .collect();
    generated.push_str(");\n#[allow(non_snake_case)]\npub struct Peripherals {\n");
    for name in &owned_names {
        writeln!(
            generated,
            "    pub {name}: crate::Peri<'static, peripherals::{name}>,"
        )
        .unwrap();
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
            "impl crate::gpio::interrupt_input::sealed::InterruptPin for peripherals::{name} {{ const PORT:crate::gpio::Port=crate::gpio::Port::{variant}; const NUMBER:u8={number}; fn state()->&'static crate::async_support::EventState {{ static STATE:crate::async_support::EventState=crate::async_support::EventState::new(); &STATE }} }}\nimpl crate::gpio::InterruptPin for peripherals::{name} {{ type Interrupt=crate::interrupt::typelevel::{irq}; }}",
            name=pin.name,number=pin.number).unwrap();
    }
    generated.push_str("unsafe fn take_generated() -> Peripherals {\n    Peripherals {\n");
    for name in &owned_names {
        writeln!(
            generated,
            "        {name}: unsafe {{ peripherals::{name}::steal() }},"
        )
        .unwrap();
    }
    generated.push_str("    }\n}\n");
    fs::write(out.join("_generated.rs"), generated).unwrap();

    let mut clocks = String::new();
    for p in md
        .peripherals
        .iter()
        .filter(|p| peripheral_names.contains(&p.name))
    {
        if let Some(gate) = &p.clock_gate {
            let owner = md
                .peripherals
                .iter()
                .find(|p| p.name == gate.peripheral)
                .expect("clock owner");
            assert_eq!(
                owner.name, "SYSCTRL",
                "clock key semantics need audit for new controller"
            );
            assert!(matches!(gate.register, "AHBEN" | "APBEN1" | "APBEN2"));
            writeln!(clocks, "impl PeripheralClock for crate::peripherals::{} {{ fn enable_and_reset() {{ critical_section::with(|_| {{", ident(p.name)).unwrap();
            let register = gate.register.to_ascii_lowercase();
            let field = gate.field.to_ascii_lowercase();
            if owner.version == "l012" {
                writeln!(clocks,"let mut value=pac::SYSCTRL.{register}().read();value.set_key((pac::SYSCTRL_KEY>>16) as u16);value.set_{field}(true);pac::SYSCTRL.{register}().write_value(value);").unwrap();
            } else if owner.version == "f030" {
                writeln!(
                    clocks,
                    "pac::SYSCTRL.{register}().modify(|w|w.set_{field}(true));"
                )
                .unwrap();
            } else {
                panic!("unaudited clock gate semantics");
            }
            // A private reset bit may still affect another peripheral resource.
            // Both shared-bit ownership and explicit cross-effects forbid a local reset.
            if let Some(reset) = p
                .reset
                .as_ref()
                .filter(|r| !r.shared && p.reset_effects.is_empty())
            {
                assert_eq!(reset.peripheral, "SYSCTRL");
                assert!(matches!(reset.register, "AHBRST" | "APBRST1" | "APBRST2"));
                let register = reset.register.to_ascii_lowercase();
                let field = reset.field.to_ascii_lowercase();
                writeln!(clocks,"pac::SYSCTRL.{register}().modify(|w|w.set_{field}(false));pac::SYSCTRL.{register}().modify(|w|w.set_{field}(true));").unwrap();
            }
            clocks.push_str("}); } }\n");
        }
    }
    fs::write(out.join("_generated_peripheral_clocks.rs"), clocks).unwrap();
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
        writeln!(timer,"impl sealed::Instance for peripherals::{name} {{fn regs()->Registers {{Registers::{variant}(pac::{name})}} }} impl CoreInstance for peripherals::{name} {{}} impl sealed::PwmInstance for peripherals::{name} {{}} impl PwmInstance for peripherals::{name} {{}}").unwrap();
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
    fs::write(out.join("_generated_timer.rs"), timer).unwrap();
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
        writeln!(adc,"impl sealed::Sealed for peripherals::{name} {{fn regs()->pac::adc::Adc {{pac::{name}}} fn state()->&'static crate::async_support::EventState {{static STATE:crate::async_support::EventState=crate::async_support::EventState::new(); &STATE}} }} impl Instance for peripherals::{name} {{type Interrupt=crate::interrupt::typelevel::{irq};}} ").unwrap();
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
            if channel > 13 || !md.pins.iter().any(|p| p.name == route.pin) {
                continue;
            }
            let pin = ident(route.pin);
            writeln!(adc,"impl sealed::PinSealed<peripherals::{name}> for peripherals::{pin} {{}} impl ChannelPin<peripherals::{name}> for peripherals::{pin} {{ const CHANNEL:u8={channel}; }}").unwrap();
        }
    }
    fs::write(out.join("_generated_adc.rs"), adc).unwrap();
    let mut analog = String::new();
    for p in md
        .peripherals
        .iter()
        .filter(|p| matches!(p.block, "opa" | "vc" | "vcref"))
    {
        let name = ident(p.name);
        let (instance, block) = match p.block {
            "opa" => ("OpaInstance", "opa"),
            "vc" => ("VcInstance", "vc"),
            _ => ("RefInstance", "vcref"),
        };
        let block_type = format!("{}{}", block[..1].to_ascii_uppercase(), &block[1..]);
        writeln!(analog,"impl sealed::{instance} for peripherals::{name} {{ fn regs()->pac::{block}::{block_type} {{pac::{name}}}").unwrap();
        if p.block == "vc" {
            writeln!(analog,"fn state()->&'static crate::async_support::EventState {{ static STATE:crate::async_support::EventState=crate::async_support::EventState::new(); &STATE }}").unwrap();
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
            if p.version == "f030" {
                writeln!(analog, "const NUMBER:u8={number};").unwrap();
            } else {
                let reference = match name {
                    "VC1" | "VC2" => "VC12REF",
                    "VC3" | "VC4" => "VC34REF",
                    _ => panic!("unaudited comparator reference pair"),
                };
                writeln!(
                    analog,
                    "#[cfg(vcref_l012)] type Reference=peripherals::{reference}; const NUMBER:u8={number};"
                )
                .unwrap();
            }
        }
        writeln!(analog, "}}").unwrap();
    }
    for r in md
        .pin_routes
        .iter()
        .filter(|r| r.af.is_none() && r.remap.is_none())
    {
        if !md.pins.iter().any(|p| p.name == r.pin) {
            continue;
        }
        let signal = if r.peripheral.starts_with("OPA") {
            match r.signal {
                "OUT" => 0,
                "INP1" => 1,
                "INP2" => 2,
                "INP3" => 3,
                "INN1" => 11,
                "INN2" => 12,
                _ => continue,
            }
        } else if r.peripheral.starts_with("VC") {
            match r
                .signal
                .strip_prefix("CH")
                .and_then(|s| s.parse::<u8>().ok())
            {
                Some(s) => {
                    let p = md
                        .peripherals
                        .iter()
                        .find(|p| p.name == r.peripheral)
                        .unwrap();
                    // The explicitly audited route determines external-input validity.
                    // Register metadata checks representability, not a chip-family guess.
                    for selector in ["INP", "INN"] {
                        let field = build_support::field(p, "CR0", selector);
                        assert!(
                            u32::from(s) < (1u32 << field.bit_size),
                            "VC route exceeds input selector"
                        );
                    }
                    s
                }
                _ => continue,
            }
        } else if r.peripheral == "DAC" {
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
    memory.push_str("}\n");
    if env::var_os("CARGO_FEATURE_MEMORY_X").is_some() {
        fs::write(out.join("memory.x"), memory).unwrap();
    }
    if target.starts_with("thumb") {
        println!("cargo:rustc-link-search={}", out.display());
    }
}
