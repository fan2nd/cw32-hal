use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Cordic",
        extends: None,
        description: Some("desc CORDIC."),
        items: &[
            BlockItem {
                name: "csr",
                description: Some("Control and State register."),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Csr"),
                }),
            },
            BlockItem {
                name: "x",
                description: Some("Data register X."),
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("X"),
                }),
            },
            BlockItem {
                name: "y",
                description: Some("Data register Y."),
                array: None,
                byte_offset: 0x8,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Y"),
                }),
            },
            BlockItem {
                name: "z",
                description: Some("desc Z."),
                array: None,
                byte_offset: 0xc,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Z"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Csr",
            extends: None,
            description: Some("Control and State register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "func",
                    description: Some("desc FUNC."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 4,
                    array: None,
                    enumm: Some("Func"),
                },
                Field {
                    name: "scale",
                    description: Some("desc SCALE."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 3,
                    array: None,
                    enumm: Some("Scale"),
                },
                Field {
                    name: "format",
                    description: Some("desc FORMAT."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Format"),
                },
                Field {
                    name: "iter",
                    description: Some("desc ITER."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 4,
                    array: None,
                    enumm: Some("Iter"),
                },
                Field {
                    name: "comp",
                    description: Some("desc COMP."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 12 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Comp"),
                },
                Field {
                    name: "ie",
                    description: Some("desc IE."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 13 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dmaeoc",
                    description: Some("desc DMAEOC."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 14 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dmaidle",
                    description: Some("desc DMAIDLE."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 15 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eoc",
                    description: Some("desc EOC."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "busy",
                    description: Some("desc BUSY."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 17 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "X",
            extends: None,
            description: Some("Data register X."),
            bit_size: 32,
            fields: &[Field {
                name: "x",
                description: Some("desc X."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Y",
            extends: None,
            description: Some("Data register Y."),
            bit_size: 32,
            fields: &[Field {
                name: "y",
                description: Some("desc Y."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Z",
            extends: None,
            description: Some("desc Z."),
            bit_size: 32,
            fields: &[Field {
                name: "z",
                description: Some("desc Z."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
    ],
    enums: &[
        Enum {
            name: "Comp",
            description: Some(
                "L012 CORDIC Comp encodings, own CN manual Rev1.4; see docs/l012-math-evidence.md.",
            ),
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Compensated",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "Uncompensated",
                    description: None,
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Format",
            description: Some(
                "L012 CORDIC Format encodings, own CN manual Rev1.4; see docs/l012-math-evidence.md.",
            ),
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Q115",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "Q131",
                    description: None,
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Func",
            description: Some(
                "L012 CORDIC Func encodings, own CN manual Rev1.4; see docs/l012-math-evidence.md.",
            ),
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "Atan",
                    description: None,
                    value: 4,
                },
                EnumVariant {
                    name: "Atan2",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "Atanh",
                    description: None,
                    value: 7,
                },
                EnumVariant {
                    name: "Cos",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "Cosh",
                    description: None,
                    value: 5,
                },
                EnumVariant {
                    name: "Hypot",
                    description: None,
                    value: 3,
                },
                EnumVariant {
                    name: "Ln",
                    description: None,
                    value: 8,
                },
                EnumVariant {
                    name: "Sin",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "Sinh",
                    description: None,
                    value: 6,
                },
                EnumVariant {
                    name: "Sqrt",
                    description: None,
                    value: 9,
                },
            ],
        },
        Enum {
            name: "Iter",
            description: Some(
                "L012 CORDIC Iter encodings, own CN manual Rev1.4; see docs/l012-math-evidence.md.",
            ),
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "Iter10",
                    description: None,
                    value: 4,
                },
                EnumVariant {
                    name: "Iter12",
                    description: None,
                    value: 5,
                },
                EnumVariant {
                    name: "Iter14",
                    description: None,
                    value: 6,
                },
                EnumVariant {
                    name: "Iter16",
                    description: None,
                    value: 7,
                },
                EnumVariant {
                    name: "Iter18",
                    description: None,
                    value: 8,
                },
                EnumVariant {
                    name: "Iter2",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "Iter20",
                    description: None,
                    value: 9,
                },
                EnumVariant {
                    name: "Iter22",
                    description: None,
                    value: 10,
                },
                EnumVariant {
                    name: "Iter24",
                    description: None,
                    value: 11,
                },
                EnumVariant {
                    name: "Iter26",
                    description: None,
                    value: 12,
                },
                EnumVariant {
                    name: "Iter28",
                    description: None,
                    value: 13,
                },
                EnumVariant {
                    name: "Iter30",
                    description: None,
                    value: 14,
                },
                EnumVariant {
                    name: "Iter32",
                    description: None,
                    value: 15,
                },
                EnumVariant {
                    name: "Iter4",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "Iter6",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "Iter8",
                    description: None,
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Scale",
            description: Some(
                "L012 CORDIC Scale encodings, own CN manual Rev1.4; see docs/l012-math-evidence.md.",
            ),
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "Scale0",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "Scale1",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "Scale2",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "Scale3",
                    description: None,
                    value: 3,
                },
                EnumVariant {
                    name: "Scale4",
                    description: None,
                    value: 4,
                },
                EnumVariant {
                    name: "Scale5",
                    description: None,
                    value: 5,
                },
                EnumVariant {
                    name: "Scale6",
                    description: None,
                    value: 6,
                },
                EnumVariant {
                    name: "Scale7",
                    description: None,
                    value: 7,
                },
            ],
        },
    ],
};
