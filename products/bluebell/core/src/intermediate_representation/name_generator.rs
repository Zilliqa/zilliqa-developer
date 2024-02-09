use scilla_parser::parser::lexer::SourcePosition;

use crate::intermediate_representation::primitives::{
    FunctionBlock, IrIdentifier, IrIndentifierKind,
};

#[derive(Debug, Clone)]
pub struct NameGenerator {
    anonymous_type_number: u64,
    intermediate_counter: u64,
    block_counter: u64,
}

impl NameGenerator {
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
            source_location: (
                SourcePosition::invalid_position(),
                SourcePosition::invalid_position(),
            ),
        }
    }

    pub fn hex_type(&self) -> IrIdentifier {
        IrIdentifier {
            unresolved: "String".to_string(), // TODO: Correct structure would be Dynamic Byte String, see https://scilla-cookbook.org/recipes/scilla-recipes/addresses
            resolved: None,
            type_reference: None,
            kind: IrIndentifierKind::TypeName,
            is_definition: false,
            source_location: (
                SourcePosition::invalid_position(),
                SourcePosition::invalid_position(),
            ),
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
            source_location: (
                SourcePosition::invalid_position(),
                SourcePosition::invalid_position(),
            ),
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
            source_location: (
                SourcePosition::invalid_position(),
                SourcePosition::invalid_position(),
            ),
        }
    }
}
