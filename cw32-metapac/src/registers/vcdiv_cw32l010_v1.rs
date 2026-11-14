use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Vcdiv",
        extends: None,
        description: Some("Shared comparator reference divider."),
        items: &[BlockItem {
            name: "div",
            description: Some("VCREF Control register."),
            array: None,
            byte_offset: 0x0,
            inner: BlockItemInner::Register(Register {
                access: Access::ReadWrite,
                bit_size: 32,
                fieldset: Some("Div"),
            }),
        }],
    }],
    fieldsets: &[FieldSet {
        name: "Div",
        extends: None,
        description: Some(
            "Shared comparator reference control; configure only while no consumer selects it.",
        ),
        bit_size: 32,
        fields: &[
            Field {
                name: "div",
                description: Some(
                    "Ratio code 0..7 selects (1 + DIV) / 8. Values 8..15 are unqualified on L012.",
                ),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 3,
                array: None,
                enumm: Some("DividerRatio"),
            },
            Field {
                name: "en",
                description: Some("Enable the shared resistor divider."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "vin",
                description: Some(
                    "Analog supply (VDD on L010, VDDA on L011/L012) or nominal 1.6 V core rail.",
                ),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 5 }),
                bit_size: 1,
                array: None,
                enumm: Some("InputSource"),
            },
        ],
    }],
    enums: &[
        Enum {
            name: "DividerRatio",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "EightEighths",
                    description: Some("8/8 of the selected input; subject to analog limits."),
                    value: 7,
                },
                EnumVariant {
                    name: "FiveEighths",
                    description: Some("5/8 of the selected input; subject to analog limits."),
                    value: 4,
                },
                EnumVariant {
                    name: "FourEighths",
                    description: Some("4/8 of the selected input; subject to analog limits."),
                    value: 3,
                },
                EnumVariant {
                    name: "OneEighth",
                    description: Some("1/8 of the selected input; subject to analog limits."),
                    value: 0,
                },
                EnumVariant {
                    name: "SevenEighths",
                    description: Some("7/8 of the selected input; subject to analog limits."),
                    value: 6,
                },
                EnumVariant {
                    name: "SixEighths",
                    description: Some("6/8 of the selected input; subject to analog limits."),
                    value: 5,
                },
                EnumVariant {
                    name: "ThreeEighths",
                    description: Some("3/8 of the selected input; subject to analog limits."),
                    value: 2,
                },
                EnumVariant {
                    name: "TwoEighths",
                    description: Some("2/8 of the selected input; subject to analog limits."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "InputSource",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Core",
                    description: Some(
                        "Internal Vcore, nominally 1.6 V; not a calibrated reference.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "Supply",
                    description: Some("VDD on L010; VDDA on L011/L012."),
                    value: 0,
                },
            ],
        },
    ],
};
