use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Aes",
        extends: None,
        description: Some(
            "CW32L083 AES, own User Manual CN Rev2.0; see docs/l083-aes-trng-evidence.md.",
        ),
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
                name: "data0",
                description: Some("desc DATA0."),
                array: None,
                byte_offset: 0x10,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Data0"),
                }),
            },
            BlockItem {
                name: "data1",
                description: Some("desc DATA1."),
                array: None,
                byte_offset: 0x14,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Data1"),
                }),
            },
            BlockItem {
                name: "data2",
                description: Some("desc DATA2."),
                array: None,
                byte_offset: 0x18,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Data2"),
                }),
            },
            BlockItem {
                name: "data3",
                description: Some("desc DATA3."),
                array: None,
                byte_offset: 0x1c,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Data3"),
                }),
            },
            BlockItem {
                name: "key0",
                description: Some("desc KEY0."),
                array: None,
                byte_offset: 0x20,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Key0"),
                }),
            },
            BlockItem {
                name: "key1",
                description: Some("desc KEY1."),
                array: None,
                byte_offset: 0x24,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Key1"),
                }),
            },
            BlockItem {
                name: "key2",
                description: Some("desc KEY2."),
                array: None,
                byte_offset: 0x28,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Key2"),
                }),
            },
            BlockItem {
                name: "key3",
                description: Some("desc KEY3."),
                array: None,
                byte_offset: 0x2c,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Key3"),
                }),
            },
            BlockItem {
                name: "key4",
                description: Some("desc KEY4."),
                array: None,
                byte_offset: 0x30,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Key4"),
                }),
            },
            BlockItem {
                name: "key5",
                description: Some("desc KEY5."),
                array: None,
                byte_offset: 0x34,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Key5"),
                }),
            },
            BlockItem {
                name: "key6",
                description: Some("desc KEY6."),
                array: None,
                byte_offset: 0x38,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Key6"),
                }),
            },
            BlockItem {
                name: "key7",
                description: Some("desc KEY7."),
                array: None,
                byte_offset: 0x3c,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Key7"),
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
            fields: &[
                Field {
                    name: "start",
                    description: Some(
                        "Write 1 to start; read 0 after completion, read 1 while active. W0 abort is not documented.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Operation"),
                },
                Field {
                    name: "mode",
                    description: Some("desc MODE."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Mode"),
                },
                Field {
                    name: "keysize",
                    description: Some("desc KEYSIZE."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("KeySize"),
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
                    "DATA bits31:0; index zero is the least-significant register word. No standard byte-array mapping is asserted.",
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
                    "DATA bits63:32; index zero is the least-significant register word. No standard byte-array mapping is asserted.",
                ),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Data2",
            extends: None,
            description: Some("desc DATA2."),
            bit_size: 32,
            fields: &[Field {
                name: "data2",
                description: Some(
                    "DATA bits95:64; index zero is the least-significant register word. No standard byte-array mapping is asserted.",
                ),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Data3",
            extends: None,
            description: Some("desc DATA3."),
            bit_size: 32,
            fields: &[Field {
                name: "data3",
                description: Some(
                    "DATA bits127:96; index zero is the least-significant register word. No standard byte-array mapping is asserted.",
                ),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Key0",
            extends: None,
            description: Some("desc KEY0."),
            bit_size: 32,
            fields: &[Field {
                name: "key0",
                description: Some(
                    "KEY bits31:0; index zero is the least-significant register word. No standard byte-array mapping is asserted.",
                ),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Key1",
            extends: None,
            description: Some("desc KEY1."),
            bit_size: 32,
            fields: &[Field {
                name: "key1",
                description: Some(
                    "KEY bits63:32; index zero is the least-significant register word. No standard byte-array mapping is asserted.",
                ),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Key2",
            extends: None,
            description: Some("desc KEY2."),
            bit_size: 32,
            fields: &[Field {
                name: "key2",
                description: Some(
                    "KEY bits95:64; index zero is the least-significant register word. No standard byte-array mapping is asserted.",
                ),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Key3",
            extends: None,
            description: Some("desc KEY3."),
            bit_size: 32,
            fields: &[Field {
                name: "key3",
                description: Some(
                    "KEY bits127:96; index zero is the least-significant register word. No standard byte-array mapping is asserted.",
                ),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Key4",
            extends: None,
            description: Some("desc KEY4."),
            bit_size: 32,
            fields: &[Field {
                name: "key4",
                description: Some(
                    "KEY bits159:128; index zero is the least-significant register word. No standard byte-array mapping is asserted.",
                ),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Key5",
            extends: None,
            description: Some("desc KEY5."),
            bit_size: 32,
            fields: &[Field {
                name: "key5",
                description: Some(
                    "KEY bits191:160; index zero is the least-significant register word. No standard byte-array mapping is asserted.",
                ),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Key6",
            extends: None,
            description: Some("desc KEY6."),
            bit_size: 32,
            fields: &[Field {
                name: "key6",
                description: Some(
                    "KEY bits223:192; index zero is the least-significant register word. No standard byte-array mapping is asserted.",
                ),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Key7",
            extends: None,
            description: Some("desc KEY7."),
            bit_size: 32,
            fields: &[Field {
                name: "key7",
                description: Some(
                    "KEY bits255:224; index zero is the least-significant register word. No standard byte-array mapping is asserted.",
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
            name: "KeySize",
            description: Some(
                "AES key length, UM section28.5.1. Encoding3 aliases128; HAL uses canonical0.",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Bits128",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "Bits128Alternate",
                    description: None,
                    value: 3,
                },
                EnumVariant {
                    name: "Bits192",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "Bits256",
                    description: None,
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Mode",
            description: Some("AES direction, UM section28.5.1."),
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Decrypt",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "Encrypt",
                    description: None,
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Operation",
            description: Some(
                "START command/status, CW32L083 UM Rev2.0 section 28.5.1. Writing zero is not an abort.",
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
    ],
};
