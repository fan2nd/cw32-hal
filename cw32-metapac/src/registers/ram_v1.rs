use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Ram",
        extends: None,
        description: Some("desc RAM."),
        items: &[
            BlockItem {
                name: "ier",
                description: Some("Interrupt enable register."),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ier"),
                }),
            },
            BlockItem {
                name: "addr",
                description: Some("Parity check error addr register."),
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Addr"),
                }),
            },
            BlockItem {
                name: "isr",
                description: Some("Interrupt flag register."),
                array: None,
                byte_offset: 0x8,
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
                byte_offset: 0xc,
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
            name: "Addr",
            extends: None,
            description: Some("Parity check error addr register."),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("desc ADDR."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Icr",
            extends: None,
            description: Some("Interrupt flag clear register."),
            bit_size: 32,
            fields: &[Field {
                name: "parity",
                description: Some("desc PARITY."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
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
                    name: "parity",
                    description: Some("desc PARITY."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Isr",
            extends: None,
            description: Some("Interrupt flag register."),
            bit_size: 32,
            fields: &[Field {
                name: "parity",
                description: Some("desc PARITY."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 1,
                array: None,
                enumm: None,
            }],
        },
    ],
    enums: &[],
};
