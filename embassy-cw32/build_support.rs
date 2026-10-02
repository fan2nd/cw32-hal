//! Pure generation of associations; the tables never install hardware handlers.
use cw32_metapac::metadata::Metadata;
use std::{collections::BTreeSet, fmt::Write, string::String};

pub fn associations(md: &Metadata) -> String {
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
