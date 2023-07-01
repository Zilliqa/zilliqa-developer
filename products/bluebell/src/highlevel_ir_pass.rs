use crate::constants::{TraversalResult, TreeTraversalMode};
use crate::highlevel_ir::*;

pub trait HighlevelIrPass {
    fn visit_symbol_kind(
        &mut self,
        mode: TreeTraversalMode,
        symbol_kind: IrIndentifierKind,
    ) -> Result<TraversalResult, String>;
    fn visit_symbol_name(
        &mut self,
        mode: TreeTraversalMode,
        symbol_name: IrIdentifier,
    ) -> Result<TraversalResult, String>;
    fn visit_enum_value(
        &mut self,
        mode: TreeTraversalMode,
        enum_value: EnumValue,
    ) -> Result<TraversalResult, String>;
    fn visit_tuple(
        &mut self,
        mode: TreeTraversalMode,
        tuple: Tuple,
    ) -> Result<TraversalResult, String>;
    fn visit_variant(
        &mut self,
        mode: TreeTraversalMode,
        variant: Variant,
    ) -> Result<TraversalResult, String>;
    fn visit_identifier(
        &mut self,
        mode: TreeTraversalMode,
        identifier: Identifier,
    ) -> Result<TraversalResult, String>;
    fn visit_variable_declaration(
        &mut self,
        mode: TreeTraversalMode,
        var_dec: VariableDeclaration,
    ) -> Result<TraversalResult, String>;
    fn visit_operation(
        &mut self,
        mode: TreeTraversalMode,
        operation: Operation,
    ) -> Result<TraversalResult, String>;
    fn visit_instruction(
        &mut self,
        mode: TreeTraversalMode,
        instruction: Instruction,
    ) -> Result<TraversalResult, String>;
    fn visit_function_block(
        &mut self,
        mode: TreeTraversalMode,
        function_block: FunctionBlock,
    ) -> Result<TraversalResult, String>;
    fn visit_function_body(
        &mut self,
        mode: TreeTraversalMode,
        function_body: FunctionBody,
    ) -> Result<TraversalResult, String>;
    fn visit_concrete_type(
        &mut self,
        mode: TreeTraversalMode,
        con_type: ConcreteType,
    ) -> Result<TraversalResult, String>;
    fn visit_function_kind(
        &mut self,
        mode: TreeTraversalMode,
        function_kind: FunctionKind,
    ) -> Result<TraversalResult, String>;
    fn visit_concrete_function(
        &mut self,
        mode: TreeTraversalMode,
        con_function: ConcreteFunction,
    ) -> Result<TraversalResult, String>;
    fn visit_highlevel_ir(
        &mut self,
        mode: TreeTraversalMode,
        highlevel_ir: HighlevelIr,
    ) -> Result<TraversalResult, String>;
}
