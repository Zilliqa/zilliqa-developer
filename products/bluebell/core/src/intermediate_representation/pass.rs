use scilla_parser::ast::{TraversalResult, TreeTraversalMode};

use crate::intermediate_representation::{primitives::*, symbol_table::SymbolTable};

/// `IrPass` is an abstract pass that is used by the `PassManager` to manipulate the Intermediate Representation (IR).
/// It provides methods to visit and potentially alter different parts of the IR during the traversal.
pub trait IrPass {
    /// Visit and potentially alter a symbol kind in the IR.
    fn visit_symbol_kind(
        &mut self,
        mode: TreeTraversalMode,
        symbol_kind: &mut IrIndentifierKind,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String>;

    /// Visit and potentially alter a symbol name in the IR.
    fn visit_symbol_name(
        &mut self,
        mode: TreeTraversalMode,
        symbol_name: &mut IrIdentifier,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String>;

    /// Visit and potentially alter an enum value in the IR.
    fn visit_enum_value(
        &mut self,
        mode: TreeTraversalMode,
        enum_value: &mut EnumValue,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String>;

    /// Visit and potentially alter a tuple in the IR.
    fn visit_tuple(
        &mut self,
        mode: TreeTraversalMode,
        tuple: &mut Tuple,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String>;

    /// Visit and potentially alter a variant in the IR.
    fn visit_variant(
        &mut self,
        mode: TreeTraversalMode,
        variant: &mut Variant,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String>;

    /// Visit and potentially alter a variable declaration in the IR.
    fn visit_variable_declaration(
        &mut self,
        mode: TreeTraversalMode,
        var_dec: &mut VariableDeclaration,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String>;

    /// Visit and potentially alter an operation in the IR.
    fn visit_operation(
        &mut self,
        mode: TreeTraversalMode,
        operation: &mut Operation,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String>;

    /// Visit and potentially alter an instruction in the IR.
    fn visit_instruction(
        &mut self,
        mode: TreeTraversalMode,
        instruction: &mut Instruction,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String>;

    /// Visit and potentially alter a function block in the IR.
    fn visit_function_block(
        &mut self,
        mode: TreeTraversalMode,
        function_block: &mut FunctionBlock,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String>;

    /// Visit and potentially alter a function body in the IR.
    fn visit_function_body(
        &mut self,
        mode: TreeTraversalMode,
        function_body: &mut FunctionBody,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String>;

    /// Visit and potentially alter a concrete type in the IR.
    fn visit_concrete_type(
        &mut self,
        mode: TreeTraversalMode,
        con_type: &mut ConcreteType,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String>;

    /// Visit and potentially alter a contract field in the IR.
    fn visit_contract_field(
        &mut self,
        mode: TreeTraversalMode,
        function_kind: &mut ContractField,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String>;

    /// Visit and potentially alter a function kind in the IR.
    fn visit_function_kind(
        &mut self,
        mode: TreeTraversalMode,
        function_kind: &mut FunctionKind,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String>;

    /// Visit and potentially alter a concrete function in the IR.
    fn visit_concrete_function(
        &mut self,
        mode: TreeTraversalMode,
        con_function: &mut ConcreteFunction,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String>;

    /// Visit and potentially alter a case clause in the IR.
    fn visit_case_clause(
        &mut self,
        mode: TreeTraversalMode,
        con_function: &mut CaseClause,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String>;

    /// Visit and potentially alter primitives in the IR.
    fn visit_primitives(
        // TODO Remove
        &mut self,
        mode: TreeTraversalMode,
        primitives: &mut IntermediateRepresentation,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String>;

    /// Initiate the pass.
    fn initiate(&mut self);

    /// Finalize the pass.
    fn finalize(&mut self);
}
