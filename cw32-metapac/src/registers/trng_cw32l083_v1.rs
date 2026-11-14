use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Trng",
        extends: None,
        description: Some(
            "CW32L083 TRNG, own User Manual CN Rev2.0; see docs/l083-aes-trng-evidence.md.",
        ),
        items: &[
            BlockItem {
                name: "cr1",
                description: Some("desc CR1."),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cr1"),
                }),
            },
            BlockItem {
                name: "cr2",
                description: Some("desc CR2."),
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cr2"),
                }),
            },
            BlockItem {
                name: "data0",
                description: Some("desc DATA0."),
                array: None,
                byte_offset: 0xc,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Data0"),
                }),
            },
            BlockItem {
                name: "data1",
                description: Some("desc DATA1."),
                array: None,
                byte_offset: 0x10,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Data1"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Cr1",
            extends: None,
            description: Some("desc CR1."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some("desc EN."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "start",
                    description: Some(
                        "Write 1 to start; read 0 after completion, read 1 while active. W0 abort is not documented.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Operation"),
                },
            ],
        },
        FieldSet {
            name: "Cr2",
            extends: None,
            description: Some("desc CR2."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "source",
                    description: Some("desc SOURCE."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Source"),
                },
                Field {
                    name: "shift",
                    description: Some("desc SHIFT."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 3,
                    array: None,
                    enumm: Some("Shift"),
                },
            ],
        },
        FieldSet {
            name: "Data0",
            extends: None,
            description: Some("desc DATA0."),
            bit_size: 32,
            fields: &[Field {
                name: "data0",
                description: Some(
                    "Output bits31:0; valid only after a submitted operation completes. No health or error status is documented.",
                ),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Data1",
            extends: None,
            description: Some("desc DATA1."),
            bit_size: 32,
            fields: &[Field {
                name: "data1",
                description: Some(
                    "Output bits63:32; valid only after a submitted operation completes. No health or error status is documented.",
                ),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
    ],
    enums: &[
        Enum {
            name: "Operation",
            description: Some(
                "START command/status, CW32L083 UM Rev2.0 section 26.6.1. Writing zero is not an abort.",
            ),
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Active",
                    description: Some("Read: operation busy. Write: start an operation."),
                    value: 1,
                },
                EnumVariant {
                    name: "Idle",
                    description: Some(
                        "Read: operation complete. Write-zero behavior is unspecified.",
                    ),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Shift",
            description: Some(
                "LFSR shift count, UM section26.6.2. Other encodings are undocumented.",
            ),
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "Cycles128",
                    description: None,
                    value: 5,
                },
                EnumVariant {
                    name: "Cycles16",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "Cycles256",
                    description: None,
                    value: 6,
                },
                EnumVariant {
                    name: "Cycles32",
                    description: None,
                    value: 3,
                },
                EnumVariant {
                    name: "Cycles64",
                    description: None,
                    value: 4,
                },
                EnumVariant {
                    name: "Cycles8",
                    description: None,
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Source",
            description: Some(
                "LFSR input, UM section26.6.2. Feedback-only is deterministic and excluded by HAL.",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Analog",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "AnalogXorFeedback",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "Feedback",
                    description: None,
                    value: 0,
                },
            ],
        },
    ],
};
