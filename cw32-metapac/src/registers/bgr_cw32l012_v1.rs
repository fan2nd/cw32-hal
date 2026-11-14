use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Bgr",
        extends: None,
        description: Some("BGR."),
        items: &[BlockItem {
            name: "cr",
            description: Some("Control register1."),
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
        description: Some("Control register1."),
        bit_size: 32,
        fields: &[
            Field {
                name: "bgren",
                description: Some("desc BGREN."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "tsen",
                description: Some("desc TSEN."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
        ],
    }],
    enums: &[],
};
