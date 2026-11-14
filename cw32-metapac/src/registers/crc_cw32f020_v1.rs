use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Crc",
        extends: None,
        description: Some(
            "F020 CRC16 engine. Only MODE values 0 through 7 are documented; CRC32 algorithms are unsupported.",
        ),
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
                name: "dr16",
                description: Some("Data register."),
                array: None,
                byte_offset: 0x8,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 16,
                    fieldset: Some("Dr16"),
                }),
            },
            BlockItem {
                name: "dr32",
                description: Some("Data register."),
                array: None,
                byte_offset: 0x8,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Dr32"),
                }),
            },
            BlockItem {
                name: "dr8",
                description: Some("Data register."),
                array: None,
                byte_offset: 0x8,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 8,
                    fieldset: Some("Dr8"),
                }),
            },
            BlockItem {
                name: "result16",
                description: Some("Result register."),
                array: None,
                byte_offset: 0xc,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 16,
                    fieldset: Some("Result16"),
                }),
            },
            BlockItem {
                name: "result32",
                description: Some(
                    "32-bit bus view of the 16-bit CRC result. Bits 31:16 are reserved, not a CRC32 result.",
                ),
                array: None,
                byte_offset: 0xc,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Result32"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some("Control register."),
            bit_size: 32,
            fields: &[Field {
                name: "mode",
                description: Some(
                    "CRC16 preset: 0 IBM, 1 MAXIM, 2 USB, 3 MODBUS, 4 CCITT, 5 CCITT-FALSE, 6 X25, 7 XMODEM. Encodings 8 through 15 are undocumented and must not be used.",
                ),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 4,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Dr16",
            extends: None,
            description: Some("Data register."),
            bit_size: 16,
            fields: &[Field {
                name: "dr16",
                description: Some("desc DR16."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 16,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Dr32",
            extends: None,
            description: Some("Data register."),
            bit_size: 32,
            fields: &[Field {
                name: "dr32",
                description: Some("desc DR32."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Dr8",
            extends: None,
            description: Some("Data register."),
            bit_size: 8,
            fields: &[Field {
                name: "dr8",
                description: Some("desc DR8."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 8,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Result16",
            extends: None,
            description: Some("Result register."),
            bit_size: 16,
            fields: &[Field {
                name: "result16",
                description: Some("desc RESULT16."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 16,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Result32",
            extends: None,
            description: Some("32-bit bus view; only bits 15:0 hold a CRC16 result."),
            bit_size: 32,
            fields: &[Field {
                name: "result32",
                description: Some(
                    "CRC16 result in bits 15:0; bits 31:16 are reserved (F020 manual section10.6.3).",
                ),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 16,
                array: None,
                enumm: None,
            }],
        },
    ],
    enums: &[],
};
