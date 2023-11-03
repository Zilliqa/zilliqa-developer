use crate::{
    ast::nodes::*,
    constants::{TraversalResult, TreeTraversalMode},
    parser::lexer::SourcePosition,
};

/// The `AstConverting` trait is used for converting an Abstract Syntax Tree (AST)
/// to some other form, such as an internal or intermediate representation.
/// Each method corresponds to a specific node in the AST and is responsible for
/// converting that node and its children. The methods are called upon entering and
/// exiting the tree traversal and the return result informs the visitor algorithm
/// how to proceed.
pub trait AstConverting {
    /// Pushes the source position of the current node onto a stack.
    fn push_source_position(&mut self, start: &SourcePosition, end: &SourcePosition) -> ();

    /// Pops the source position of the current node from the stack.
    fn pop_source_position(&mut self) -> ();

    /// Converts a `NodeByteStr` node.
    fn emit_byte_str(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeByteStr,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodeTypeNameIdentifier` node.
    fn emit_type_name_identifier(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeNameIdentifier,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodeImportedName` node.
    fn emit_imported_name(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeImportedName,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodeImportDeclarations` node.
    fn emit_import_declarations(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeImportDeclarations,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodeMetaIdentifier` node.
    fn emit_meta_identifier(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeMetaIdentifier,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodeVariableIdentifier` node.
    fn emit_variable_identifier(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeVariableIdentifier,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodeBuiltinArguments` node.
    fn emit_builtin_arguments(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeBuiltinArguments,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodeTypeMapKey` node.
    fn emit_type_map_key(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeMapKey,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodeTypeMapValue` node.
    fn emit_type_map_value(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeMapValue,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodeTypeArgument` node.
    fn emit_type_argument(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeArgument,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodeScillaType` node.
    fn emit_scilla_type(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeScillaType,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodeTypeMapEntry` node.
    fn emit_type_map_entry(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeMapEntry,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodeAddressTypeField` node.
    fn emit_address_type_field(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeAddressTypeField,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodeAddressType` node.
    fn emit_address_type(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeAddressType,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodeFullExpression` node.
    fn emit_full_expression(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeFullExpression,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodeMessageEntry` node.
    fn emit_message_entry(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeMessageEntry,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodePatternMatchExpressionClause` node.
    fn emit_pattern_match_expression_clause(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodePatternMatchExpressionClause,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodeAtomicExpression` node.
    fn emit_atomic_expression(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeAtomicExpression,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodeContractTypeArguments` node.
    fn emit_contract_type_arguments(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeContractTypeArguments,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodeValueLiteral` node.
    fn emit_value_literal(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeValueLiteral,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodeMapAccess` node.
    fn emit_map_access(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeMapAccess,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodePattern` node.
    fn emit_pattern(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodePattern,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodeArgumentPattern` node.
    fn emit_argument_pattern(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeArgumentPattern,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodePatternMatchClause` node.
    fn emit_pattern_match_clause(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodePatternMatchClause,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodeBlockchainFetchArguments` node.
    fn emit_blockchain_fetch_arguments(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeBlockchainFetchArguments,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodeStatement` node.
    fn emit_statement(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeStatement,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodeRemoteFetchStatement` node.
    fn emit_remote_fetch_statement(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeRemoteFetchStatement,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodeComponentId` node.
    fn emit_component_id(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeComponentId,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodeComponentParameters` node.
    fn emit_component_parameters(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeComponentParameters,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodeParameterPair` node.
    fn emit_parameter_pair(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeParameterPair,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodeComponentBody` node.
    fn emit_component_body(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeComponentBody,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodeStatementBlock` node.
    fn emit_statement_block(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeStatementBlock,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodeTypedIdentifier` node.
    fn emit_typed_identifier(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypedIdentifier,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodeTypeAnnotation` node.
    fn emit_type_annotation(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeAnnotation,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodeProgram` node.
    fn emit_program(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeProgram,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodeLibraryDefinition` node.
    fn emit_library_definition(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeLibraryDefinition,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodeLibrarySingleDefinition` node.
    fn emit_library_single_definition(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeLibrarySingleDefinition,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodeContractDefinition` node.
    fn emit_contract_definition(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeContractDefinition,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodeContractField` node.
    fn emit_contract_field(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeContractField,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodeWithConstraint` node.
    fn emit_with_constraint(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeWithConstraint,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodeComponentDefinition` node.
    fn emit_component_definition(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeComponentDefinition,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodeProcedureDefinition` node.
    fn emit_procedure_definition(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeProcedureDefinition,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodeTransitionDefinition` node.
    fn emit_transition_definition(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTransitionDefinition,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodeTypeAlternativeClause` node.
    fn emit_type_alternative_clause(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeAlternativeClause,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodeTypeMapValueArguments` node.
    fn emit_type_map_value_arguments(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeMapValueArguments,
    ) -> Result<TraversalResult, String>;

    /// Converts a `NodeTypeMapValueAllowingTypeArguments` node.
    fn emit_type_map_value_allowing_type_arguments(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeMapValueAllowingTypeArguments,
    ) -> Result<TraversalResult, String>;
}
