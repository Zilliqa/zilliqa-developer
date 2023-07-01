use crate::constants::{TraversalResult, TreeTraversalMode};
use crate::highlevel_ir::*;

pub trait HighlevelIrPass {
    fn visit_symbol_kind(
        &mut self,
        mode: TreeTraversalMode,
        symbol_kind: &mut IrIndentifierKind,
    ) -> Result<TraversalResult, String>;
    fn visit_symbol_name(
        &mut self,
        mode: TreeTraversalMode,
        symbol_name: &mut IrIdentifier,
    ) -> Result<TraversalResult, String>;
    fn visit_enum_value(
        &mut self,
        mode: TreeTraversalMode,
        enum_value: &mut EnumValue,
    ) -> Result<TraversalResult, String>;
    fn visit_tuple(
        &mut self,
        mode: TreeTraversalMode,
        tuple: &mut Tuple,
    ) -> Result<TraversalResult, String>;
    fn visit_variant(
        &mut self,
        mode: TreeTraversalMode,
        variant: &mut Variant,
    ) -> Result<TraversalResult, String>;
    fn visit_identifier(
        &mut self,
        mode: TreeTraversalMode,
        identifier: &mut Identifier,
    ) -> Result<TraversalResult, String>;
    fn visit_variable_declaration(
        &mut self,
        mode: TreeTraversalMode,
        var_dec: &mut VariableDeclaration,
    ) -> Result<TraversalResult, String>;
    fn visit_operation(
        &mut self,
        mode: TreeTraversalMode,
        operation: &mut Operation,
    ) -> Result<TraversalResult, String>;
    fn visit_instruction(
        &mut self,
        mode: TreeTraversalMode,
        instruction: &mut  Instruction,
    ) -> Result<TraversalResult, String>;
    fn visit_function_block(
        &mut self,
        mode: TreeTraversalMode,
        function_block: &mut FunctionBlock,
    ) -> Result<TraversalResult, String>;
    fn visit_function_body(
        &mut self,
        mode: TreeTraversalMode,
        function_body: &mut FunctionBody,
    ) -> Result<TraversalResult, String>;
    fn visit_concrete_type(
        &mut self,
        mode: TreeTraversalMode,
        con_type: &mut ConcreteType,
    ) -> Result<TraversalResult, String>;
    fn visit_function_kind(
        &mut self,
        mode: TreeTraversalMode,
        function_kind: &mut FunctionKind,
    ) -> Result<TraversalResult, String>;
    fn visit_concrete_function(
        &mut self,
        mode: TreeTraversalMode,
        con_function: &mut ConcreteFunction,
    ) -> Result<TraversalResult, String>;
    fn visit_highlevel_ir(
        &mut self,
        mode: TreeTraversalMode,
        highlevel_ir:&mut  HighlevelIr,
    ) -> Result<TraversalResult, String>;
}
