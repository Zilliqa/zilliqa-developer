use crate::highlevel_ir::{FunctionBlock, IrIdentifier, IrIndentifierKind};

pub struct IntermediateNameGenerator {
    anonymous_type_number: u64,
    intermediate_counter: u64,
    block_counter: u64,
}

impl IntermediateNameGenerator {
    // TODO: Rename to NameManager or the like
    pub fn new() -> Self {
        Self {
            anonymous_type_number: 0,
            intermediate_counter: 0,
            block_counter: 0,
        }
    }

    pub fn string_type(&self) -> IrIdentifier {
        IrIdentifier {
            unresolved: "String".to_string(),
            resolved: None,
            type_reference: None,
            kind: IrIndentifierKind::TypeName,
            is_definition: false,
        }
    }

    pub fn generate_anonymous_type_id(&mut self, prefix: String) -> IrIdentifier {
        let n = self.anonymous_type_number;
        self.anonymous_type_number += 1;

        IrIdentifier {
            unresolved: format!("{}{}", prefix, n).to_string(),
            resolved: None,
            type_reference: None,
            kind: IrIndentifierKind::TypeName,
            is_definition: true,
        }
    }

    pub fn new_block_label(&mut self, prefix: &str) -> IrIdentifier {
        let n = self.block_counter;
        self.block_counter += 1;
        let label = format!("{}_{}", prefix, n);
        FunctionBlock::new_label(label)
    }

    pub fn new_intermediate(&mut self) -> IrIdentifier {
        let n = self.intermediate_counter;
        self.intermediate_counter += 1;
        IrIdentifier {
            unresolved: format!("__imm_{}", n),
            resolved: None,
            type_reference: None,
            kind: IrIndentifierKind::VirtualRegisterIntermediate,
            is_definition: true,
        }
    }
}
