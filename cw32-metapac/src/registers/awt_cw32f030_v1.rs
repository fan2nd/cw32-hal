use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Awt",
        extends: None,
        description: Some("desc AWT."),
        items: &[
            BlockItem {
                name: "cr",
                description: Some("Control register."),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cr"),
                }),
            },
            BlockItem {
                name: "arr",
                description: Some("Auto reload register."),
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Arr"),
                }),
            },
            BlockItem {
                name: "cnt",
                description: Some("counter."),
                array: None,
                byte_offset: 0x8,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Cnt"),
                }),
            },
            BlockItem {
                name: "ier",
                description: Some("Interrupt enable register."),
                array: None,
                byte_offset: 0x10,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ier"),
                }),
            },
            BlockItem {
                name: "isr",
                description: Some("Interrupt status register."),
                array: None,
                byte_offset: 0x14,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Isr"),
                }),
            },
            BlockItem {
                name: "icr",
                description: Some("Interrupt flag clear register."),
                array: None,
                byte_offset: 0x1c,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Icr"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Arr",
            extends: None,
            description: Some("Auto reload register."),
            bit_size: 32,
            fields: &[Field {
                name: "arr",
                description: Some("desc ARR."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 16,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Cnt",
            extends: None,
            description: Some("counter."),
            bit_size: 32,
            fields: &[Field {
                name: "cnt",
                description: Some("desc CNT."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 16,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some("Control register."),
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
                    name: "md",
                    description: Some("desc MD."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Mode"),
                },
                Field {
                    name: "prs",
                    description: Some("desc PRS."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 4,
                    array: None,
                    enumm: Some("Prescaler"),
                },
                Field {
                    name: "src",
                    description: Some("desc SRC."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 3,
                    array: None,
                    enumm: Some("Source"),
                },
            ],
        },
        FieldSet {
            name: "Icr",
            extends: None,
            description: Some("Interrupt flag clear register."),
            bit_size: 32,
            fields: &[Field {
                name: "ud",
                description: Some("desc UD."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                bit_size: 1,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ier",
            extends: None,
            description: Some("Interrupt enable register."),
            bit_size: 32,
            fields: &[Field {
                name: "ud",
                description: Some("desc UD."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                bit_size: 1,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Isr",
            extends: None,
            description: Some("Interrupt status register."),
            bit_size: 32,
            fields: &[Field {
                name: "ud",
                description: Some("desc UD."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                bit_size: 1,
                array: None,
                enumm: None,
            }],
        },
    ],
    enums: &[
        Enum {
            name: "Mode",
            description: Some("Reviewed timer encodings; see docs/low-power-timers-evidence.json."),
            bit_size: 2,
            variants: &[EnumVariant {
                name: "Timer",
                description: Some("Required timer mode."),
                value: 3,
            }],
        },
        Enum {
            name: "Prescaler",
            description: Some("Reviewed timer encodings; see docs/low-power-timers-evidence.json."),
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "Div1024",
                    description: Some("Divide by 1024."),
                    value: 10,
                },
                EnumVariant {
                    name: "Div128",
                    description: Some("Divide by 128."),
                    value: 7,
                },
                EnumVariant {
                    name: "Div16",
                    description: Some("Divide by 16."),
                    value: 4,
                },
                EnumVariant {
                    name: "Div16384",
                    description: Some("Divide by 16384."),
                    value: 14,
                },
                EnumVariant {
                    name: "Div2",
                    description: Some("Divide by 2."),
                    value: 1,
                },
                EnumVariant {
                    name: "Div2048",
                    description: Some("Divide by 2048."),
                    value: 11,
                },
                EnumVariant {
                    name: "Div256",
                    description: Some("Divide by 256."),
                    value: 8,
                },
                EnumVariant {
                    name: "Div32",
                    description: Some("Divide by 32."),
                    value: 5,
                },
                EnumVariant {
                    name: "Div32768",
                    description: Some("Divide by 32768."),
                    value: 15,
                },
                EnumVariant {
                    name: "Div4",
                    description: Some("Divide by 4."),
                    value: 2,
                },
                EnumVariant {
                    name: "Div4096",
                    description: Some("Divide by 4096."),
                    value: 12,
                },
                EnumVariant {
                    name: "Div512",
                    description: Some("Divide by 512."),
                    value: 9,
                },
                EnumVariant {
                    name: "Div64",
                    description: Some("Divide by 64."),
                    value: 6,
                },
                EnumVariant {
                    name: "Div8",
                    description: Some("Divide by 8."),
                    value: 3,
                },
                EnumVariant {
                    name: "Div8192",
                    description: Some("Divide by 8192."),
                    value: 13,
                },
            ],
        },
        Enum {
            name: "Source",
            description: Some("Reviewed timer encodings; see docs/low-power-timers-evidence.json."),
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "Hse",
                    description: Some(
                        "External high-speed oscillator, own F020/x030 AWT clock selector.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "Hsiosc",
                    description: Some("Undivided HSI oscillator."),
                    value: 0,
                },
                EnumVariant {
                    name: "Lse",
                    description: Some(
                        "External low-speed oscillator, own x030 RM Rev2.5 PDF page173.",
                    ),
                    value: 3,
                },
            ],
        },
    ],
};
