use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Ir",
        extends: None,
        description: Some("desc IR."),
        items: &[BlockItem {
            name: "irmod",
            description: Some("desc IRMOD."),
            array: None,
            byte_offset: 0x0,
            inner: BlockItemInner::Register(Register {
                access: Access::ReadWrite,
                bit_size: 32,
                fieldset: Some("Irmod"),
            }),
        }],
    }],
    fieldsets: &[FieldSet {
        name: "Irmod",
        extends: None,
        description: Some("desc IRMOD."),
        bit_size: 32,
        fields: &[Field {
            name: "mod_",
            description: Some("desc MOD."),
            bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
            bit_size: 4,
            array: None,
            enumm: None,
        }],
    }],
    enums: &[],
};
