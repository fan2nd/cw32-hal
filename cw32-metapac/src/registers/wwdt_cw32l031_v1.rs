use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Wwdt",
        extends: None,
        description: Some("desc WWDT."),
        items: &[
            BlockItem {
                name: "cr0",
                description: Some("desc CR0."),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cr0"),
                }),
            },
            BlockItem {
                name: "cr1",
                description: Some("desc CR1."),
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cr1"),
                }),
            },
            BlockItem {
                name: "sr",
                description: Some("desc SR."),
                array: None,
                byte_offset: 0x8,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Sr"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Cr0",
            extends: None,
            description: Some("desc CR0."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wcnt",
                    description: Some("desc WCNT."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "en",
                    description: Some("desc EN."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr1",
            extends: None,
            description: Some("desc CR1."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "winr",
                    description: Some("desc WINR."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "prs",
                    description: Some("desc PRS."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ie",
                    description: Some("desc IE."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 10 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sr",
            extends: None,
            description: Some("desc SR."),
            bit_size: 32,
            fields: &[Field {
                name: "pov",
                description: Some("desc POV."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 1,
                array: None,
                enumm: None,
            }],
        },
    ],
    enums: &[],
};
