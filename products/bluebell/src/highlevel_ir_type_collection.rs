use crate::constants::{TraversalResult, TreeTraversalMode};
use crate::highlevel_ir::Instruction;
use crate::highlevel_ir::{
    ConcreteFunction, ConcreteType, EnumValue, FunctionBlock, FunctionBody, FunctionKind,
    HighlevelIr, IrIdentifier, IrIndentifierKind, IrLowering, Operation, Tuple,
    VariableDeclaration, Variant,
};
use crate::highlevel_ir_pass::HighlevelIrPass;
use crate::highlevel_ir_pass_executor::HighlevelIrPassExecutor;

pub struct HighlevelIrTypeCollection {
    
}

impl HighlevelIrPass for HighlevelIrTypeCollection {
    fn visit_symbol_kind(
        &mut self,
        _mode: TreeTraversalMode,
        kind: &mut IrIndentifierKind,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::SkipChildren)
    }
    fn visit_symbol_name(
        &mut self,
        mode: TreeTraversalMode,
        symbol: &mut IrIdentifier,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
    }

    fn visit_enum_value(
        &mut self,
        mode: TreeTraversalMode,
        enum_value: &mut EnumValue,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
    }

    fn visit_tuple(
        &mut self,
        mode: TreeTraversalMode,
        tuple: &mut Tuple,
    ) -> Result<TraversalResult, String> {
        println!("Found Tuple");
        Ok(TraversalResult::Continue)
    }

    fn visit_variant(
        &mut self,
        mode: TreeTraversalMode,
        variant: &mut Variant,
    ) -> Result<TraversalResult, String> {
        // Pass through deliberate
        Ok(TraversalResult::Continue)
    }

    fn visit_variable_declaration(
        &mut self,
        _mode: TreeTraversalMode,
        var_dec: &mut VariableDeclaration,
    ) -> Result<TraversalResult, String> {
        // TODO: 
        Ok(TraversalResult::SkipChildren)
    }

    fn visit_concrete_type(
        &mut self,
        mode: TreeTraversalMode,
        con_type: &mut ConcreteType,
    ) -> Result<TraversalResult, String> {
        match con_type {
            ConcreteType::Tuple { name, data_layout } => {
                name.visit(self)?;
                data_layout.visit(self);
            }
            ConcreteType::Variant { name, data_layout } => {
                name.visit(self)?;
                data_layout.visit(self);
            }
        }
        Ok(TraversalResult::SkipChildren)
    }

    fn visit_highlevel_ir(
        &mut self,
        mode: TreeTraversalMode,
        highlevel_ir: &mut HighlevelIr,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
    }

    fn visit_function_body(
        &mut self,
        _mode: TreeTraversalMode,
        _function_body: &mut FunctionBody,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::SkipChildren)
    }


    fn visit_function_kind(
        &mut self,
        _mode: TreeTraversalMode,
        _function_kind: &mut FunctionKind,
    ) -> Result<TraversalResult, String> {

        Ok(TraversalResult::SkipChildren)
    }

    fn visit_concrete_function(
        &mut self,
        _mode: TreeTraversalMode,
        _fnc: &mut ConcreteFunction,
    ) -> Result<TraversalResult, String> {

        Ok(TraversalResult::SkipChildren)
    }

    fn visit_operation(
        &mut self,
        _mode: TreeTraversalMode,
        _operation: &mut Operation,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::SkipChildren)
    }

    fn visit_instruction(
        &mut self,
        _mode: TreeTraversalMode,
        _instr: &mut Instruction,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::SkipChildren)
    }    

    fn visit_function_block(
        &mut self,
        _mode: TreeTraversalMode,
        _block: &mut FunctionBlock,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::SkipChildren)
    }

}

impl HighlevelIrTypeCollection {
    pub fn new() -> Self {
        HighlevelIrTypeCollection {
        }
    }
}


