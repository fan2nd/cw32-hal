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
                enumm: None,
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
    enums: &[],
};
