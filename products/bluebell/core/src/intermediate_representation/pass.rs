use crate::constants::{TraversalResult, TreeTraversalMode};
use crate::intermediate_representation::primitives::*;
use crate::symbol_table::SymbolTable;

pub trait HighlevelIrPass {
    fn visit_symbol_kind(
        &mut self,
        mode: TreeTraversalMode,
        symbol_kind: &mut IrIndentifierKind,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String>;
    fn visit_symbol_name(
        &mut self,
        mode: TreeTraversalMode,
        symbol_name: &mut IrIdentifier,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String>;
    fn visit_enum_value(
        &mut self,
        mode: TreeTraversalMode,
        enum_value: &mut EnumValue,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String>;
    fn visit_tuple(
        &mut self,
        mode: TreeTraversalMode,
        tuple: &mut Tuple,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String>;
    fn visit_variant(
        &mut self,
        mode: TreeTraversalMode,
        variant: &mut Variant,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String>;

    fn visit_variable_declaration(
        &mut self,
        mode: TreeTraversalMode,
        var_dec: &mut VariableDeclaration,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String>;
    fn visit_operation(
        &mut self,
        mode: TreeTraversalMode,
        operation: &mut Operation,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String>;
    fn visit_instruction(
        &mut self,
        mode: TreeTraversalMode,
        instruction: &mut Instruction,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String>;
    fn visit_function_block(
        &mut self,
        mode: TreeTraversalMode,
        function_block: &mut FunctionBlock,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String>;
    fn visit_function_body(
        &mut self,
        mode: TreeTraversalMode,
        function_body: &mut FunctionBody,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String>;
    fn visit_concrete_type(
        &mut self,
        mode: TreeTraversalMode,
        con_type: &mut ConcreteType,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String>;
    fn visit_function_kind(
        &mut self,
        mode: TreeTraversalMode,
        function_kind: &mut FunctionKind,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String>;
    fn visit_concrete_function(
        &mut self,
        mode: TreeTraversalMode,
        con_function: &mut ConcreteFunction,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String>;
    fn visit_primitives(
        &mut self,
        mode: TreeTraversalMode,
        primitives: &mut HighlevelIr,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String>;
}
