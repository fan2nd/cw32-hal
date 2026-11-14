use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Crc",
        extends: None,
        description: Some("desc CRC."),
        items: &[
            BlockItem {
                name: "cr",
                description: Some("desc CR."),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cr"),
                }),
            },
            BlockItem {
                name: "dr",
                description: Some("desc DR."),
                array: None,
                byte_offset: 0x8,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Dr"),
                }),
            },
            BlockItem {
                name: "result",
                description: Some("desc RESULT."),
                array: None,
                byte_offset: 0xc,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Result"),
                }),
            },
            BlockItem {
                name: "init",
                description: Some(
                    "SDK-declared initial value register; load semantics and write ordering are undocumented in the L052 V1.5 manual.",
                ),
                array: None,
                byte_offset: 0x10,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Init"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some("desc CR."),
            bit_size: 32,
            fields: &[Field {
                name: "mode",
                description: Some("desc MODE."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 4,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Dr",
            extends: None,
            description: Some("desc DR."),
            bit_size: 32,
            fields: &[Field {
                name: "dr",
                description: Some("desc DR."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 8,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Init",
            extends: None,
            description: Some(
                "SDK-declared initial value field; custom seed protocol is not qualified.",
            ),
            bit_size: 32,
            fields: &[Field {
                name: "init",
                description: Some(
                    "SDK-declared initial value register; load semantics and write ordering are undocumented in the L052 V1.5 manual.",
                ),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 16,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Result",
            extends: None,
            description: Some("desc RESULT."),
            bit_size: 32,
            fields: &[Field {
                name: "result",
                description: Some("desc RESULT."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 16,
                array: None,
                enumm: None,
            }],
        },
    ],
    enums: &[],
};
