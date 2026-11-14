use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Dmachannel",
        extends: None,
        description: Some("DMA Channel 1."),
        items: &[
            BlockItem {
                name: "csr",
                description: Some("Channel.y control and status register."),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Csr"),
                }),
            },
            BlockItem {
                name: "cnt",
                description: Some("Channel.y counter register."),
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cnt"),
                }),
            },
            BlockItem {
                name: "srcaddr",
                description: Some("Channel.y source address register."),
                array: None,
                byte_offset: 0x8,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Srcaddr"),
                }),
            },
            BlockItem {
                name: "dstaddr",
                description: Some("Channel.y destination address register."),
                array: None,
                byte_offset: 0xc,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Dstaddr"),
                }),
            },
            BlockItem {
                name: "trig",
                description: Some("Channel.y trigger register."),
                array: None,
                byte_offset: 0x10,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Trig"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Cnt",
            extends: None,
            description: Some("Channel.y counter register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cnt",
                    description: Some("desc CNT."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "repeat",
                    description: Some("desc REPEAT."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Csr",
            extends: None,
            description: Some("Channel.y control and status register."),
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
                    name: "tcie",
                    description: Some("desc TCIE."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "teie",
                    description: Some("desc TEIE."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trans",
                    description: Some("desc TRANS."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("TransferMode"),
                },
                Field {
                    name: "srcinc",
                    description: Some("desc SRCINC."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dstinc",
                    description: Some("desc DSTINC."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 5 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "size",
                    description: Some("desc SIZE."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 6 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Width"),
                },
                Field {
                    name: "status",
                    description: Some("desc STATUS."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 3,
                    array: None,
                    enumm: Some("Status"),
                },
            ],
        },
        FieldSet {
            name: "Dstaddr",
            extends: None,
            description: Some("Channel.y destination address register."),
            bit_size: 32,
            fields: &[Field {
                name: "dstaddr",
                description: Some("desc DSTADDR."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Srcaddr",
            extends: None,
            description: Some("Channel.y source address register."),
            bit_size: 32,
            fields: &[Field {
                name: "srcaddr",
                description: Some("desc SRCADDR."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Trig",
            extends: None,
            description: Some("Channel.y trigger register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "type_",
                    description: Some("desc TYPE."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Trigger"),
                },
                Field {
                    name: "softsrc",
                    description: Some("desc SOFTSRC."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hardsrc",
                    description: Some("desc HARDSRC."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Status",
            description: Some(
                "Own-family manuals sections 8.8.3-8.8.4 agree across all canonical users; see docs/dma-owned-copy-evidence.md.",
            ),
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "AddressOutOfRange",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "Complete",
                    description: None,
                    value: 5,
                },
                EnumVariant {
                    name: "DestinationError",
                    description: None,
                    value: 4,
                },
                EnumVariant {
                    name: "Initial",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "SourceError",
                    description: None,
                    value: 3,
                },
                EnumVariant {
                    name: "Stopped",
                    description: None,
                    value: 2,
                },
            ],
        },
        Enum {
            name: "TransferMode",
            description: Some(
                "Own-family manuals sections 8.8.3-8.8.4 agree across all canonical users; see docs/dma-owned-copy-evidence.md.",
            ),
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Block",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "Bulk",
                    description: None,
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Trigger",
            description: Some(
                "Own-family manuals sections 8.8.3-8.8.4 agree across all canonical users; see docs/dma-owned-copy-evidence.md.",
            ),
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Hardware",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "Software",
                    description: None,
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Width",
            description: Some(
                "Own-family manuals sections 8.8.3-8.8.4 agree across all canonical users; see docs/dma-owned-copy-evidence.md.",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Byte",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "HalfWord",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "Word",
                    description: None,
                    value: 2,
                },
            ],
        },
    ],
};
