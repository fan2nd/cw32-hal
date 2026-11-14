use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Irmod",
        extends: None,
        description: Some("desc IRMOD."),
        items: &[BlockItem {
            name: "cr",
            description: Some("Input selection register."),
            array: None,
            byte_offset: 0x0,
            inner: BlockItemInner::Register(Register {
                access: Access::ReadWrite,
                bit_size: 32,
                fieldset: Some("Cr"),
            }),
        }],
    }],
    fieldsets: &[FieldSet {
        name: "Cr",
        extends: None,
        description: Some("Input selection register."),
        bit_size: 32,
        fields: &[
            Field {
                name: "mod_",
                description: Some("desc MOD."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 4,
                array: None,
                enumm: Some("IrMode"),
            },
            Field {
                name: "irsw",
                description: Some("desc IRSW."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "inv",
                description: Some("desc INV."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 5 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
        ],
    }],
    enums: &[Enum {
        name: "IrMode",
        description: Some(
            "Exact own-family infrared boolean combinations; sources are configured separately.",
        ),
        bit_size: 4,
        variants: &[
            EnumVariant {
                name: "Gtim1Ch1AndGtim2Ch1AndIrsw",
                description: Some("GTIM1_CH1 & GTIM2_CH1 & IRSW"),
                value: 0,
            },
            EnumVariant {
                name: "Gtim1Ch1AndGtim2Ch2AndIrsw",
                description: Some("GTIM1_CH1 & GTIM2_CH2 & IRSW"),
                value: 1,
            },
            EnumVariant {
                name: "Gtim1Ch1OrGtim2Ch1OrIrsw",
                description: Some("GTIM1_CH1 | GTIM2_CH1 | IRSW"),
                value: 4,
            },
            EnumVariant {
                name: "Gtim1Ch1OrGtim2Ch2OrIrsw",
                description: Some("GTIM1_CH1 | GTIM2_CH2 | IRSW"),
                value: 5,
            },
            EnumVariant {
                name: "Gtim1Ch2AndGtim2Ch1AndIrsw",
                description: Some("GTIM1_CH2 & GTIM2_CH1 & IRSW"),
                value: 2,
            },
            EnumVariant {
                name: "Gtim1Ch2AndGtim2Ch2AndIrsw",
                description: Some("GTIM1_CH2 & GTIM2_CH2 & IRSW"),
                value: 3,
            },
            EnumVariant {
                name: "Gtim1Ch2OrGtim2Ch1OrIrsw",
                description: Some("GTIM1_CH2 | GTIM2_CH1 | IRSW"),
                value: 6,
            },
            EnumVariant {
                name: "Gtim1Ch2OrGtim2Ch2OrIrsw",
                description: Some("GTIM1_CH2 | GTIM2_CH2 | IRSW"),
                value: 7,
            },
            EnumVariant {
                name: "Uart1TxdAndGtim1Ch1AndIrsw",
                description: Some("UART1_TXD & GTIM1_CH1 & IRSW"),
                value: 8,
            },
            EnumVariant {
                name: "Uart1TxdAndGtim1Ch2AndIrsw",
                description: Some("UART1_TXD & GTIM1_CH2 & IRSW"),
                value: 10,
            },
            EnumVariant {
                name: "Uart1TxdOrGtim1Ch1OrIrsw",
                description: Some("UART1_TXD | GTIM1_CH1 | IRSW"),
                value: 9,
            },
            EnumVariant {
                name: "Uart1TxdOrGtim1Ch2OrIrsw",
                description: Some("UART1_TXD | GTIM1_CH2 | IRSW"),
                value: 11,
            },
            EnumVariant {
                name: "Uart2TxdAndGtim2Ch1AndIrsw",
                description: Some("UART2_TXD & GTIM2_CH1 & IRSW"),
                value: 12,
            },
            EnumVariant {
                name: "Uart2TxdAndGtim2Ch2AndIrsw",
                description: Some("UART2_TXD & GTIM2_CH2 & IRSW"),
                value: 14,
            },
            EnumVariant {
                name: "Uart2TxdOrGtim2Ch1OrIrsw",
                description: Some("UART2_TXD | GTIM2_CH1 | IRSW"),
                value: 13,
            },
            EnumVariant {
                name: "Uart2TxdOrGtim2Ch2OrIrsw",
                description: Some("UART2_TXD | GTIM2_CH2 | IRSW"),
                value: 15,
            },
        ],
    }],
};
