use crate::ast::*;
pub enum TreeTraversalMode {
    Enter, // Used when emit_... is invoked before children are visited
    Exit,  // User after children were visited
}

#[derive(PartialEq, Eq)]
pub enum TraversalResult {
    Ok,           // Returned when the visitor should continue tree traversal
    SkipChildren, // Returned when the visitor should skip the children and exit traversal
    Fail(String), // Returned when a failure occured.
}

pub trait CodeEmitter {
    fn emit_byte_str(&mut self, mode: TreeTraversalMode, node: &NodeByteStr) -> TraversalResult;
    fn emit_type_name_identifier(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeNameIdentifier,
    ) -> TraversalResult;
    fn emit_imported_name(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeImportedName,
    ) -> TraversalResult;
    fn emit_import_declarations(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeImportDeclarations,
    ) -> TraversalResult;
    fn emit_meta_identifier(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeMetaIdentifier,
    ) -> TraversalResult;
    fn emit_variable_identifier(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeVariableIdentifier,
    ) -> TraversalResult;
    fn emit_builtin_arguments(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeBuiltinArguments,
    ) -> TraversalResult;
    fn emit_type_map_key(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeMapKey,
    ) -> TraversalResult;
    fn emit_type_map_value(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeMapValue,
    ) -> TraversalResult;
    fn emit_type_argument(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeArgument,
    ) -> TraversalResult;
    fn emit_scilla_type(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeScillaType,
    ) -> TraversalResult;
    fn emit_type_map_entry(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeMapEntry,
    ) -> TraversalResult;
    fn emit_address_type_field(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeAddressTypeField,
    ) -> TraversalResult;
    fn emit_address_type(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeAddressType,
    ) -> TraversalResult;
    fn emit_full_expression(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeFullExpression,
    ) -> TraversalResult;
    fn emit_message_entry(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeMessageEntry,
    ) -> TraversalResult;
    fn emit_pattern_match_expression_clause(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodePatternMatchExpressionClause,
    ) -> TraversalResult;
    fn emit_atomic_expression(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeAtomicExpression,
    ) -> TraversalResult;
    fn emit_contract_type_arguments(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeContractTypeArguments,
    ) -> TraversalResult;
    fn emit_value_literal(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeValueLiteral,
    ) -> TraversalResult;
    fn emit_map_access(&mut self, mode: TreeTraversalMode, node: &NodeMapAccess)
        -> TraversalResult;
    fn emit_pattern(&mut self, mode: TreeTraversalMode, node: &NodePattern) -> TraversalResult;
    fn emit_argument_pattern(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeArgumentPattern,
    ) -> TraversalResult;
    fn emit_pattern_match_clause(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodePatternMatchClause,
    ) -> TraversalResult;
    fn emit_blockchain_fetch_arguments(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeBlockchainFetchArguments,
    ) -> TraversalResult;
    fn emit_statement(&mut self, mode: TreeTraversalMode, node: &NodeStatement) -> TraversalResult;
    fn emit_remote_fetch_statement(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeRemoteFetchStatement,
    ) -> TraversalResult;
    fn emit_component_id(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeComponentId,
    ) -> TraversalResult;
    fn emit_component_parameters(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeComponentParameters,
    ) -> TraversalResult;
    fn emit_parameter_pair(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeParameterPair,
    ) -> TraversalResult;
    fn emit_component_body(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeComponentBody,
    ) -> TraversalResult;
    fn emit_statement_block(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeStatementBlock,
    ) -> TraversalResult;
    fn emit_typed_identifier(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypedIdentifier,
    ) -> TraversalResult;
    fn emit_type_annotation(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeAnnotation,
    ) -> TraversalResult;
    fn emit_program(&mut self, mode: TreeTraversalMode, node: &NodeProgram) -> TraversalResult;
    fn emit_library_definition(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeLibraryDefinition,
    ) -> TraversalResult;
    fn emit_library_single_definition(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeLibrarySingleDefinition,
    ) -> TraversalResult;
    fn emit_contract_definition(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeContractDefinition,
    ) -> TraversalResult;
    fn emit_contract_field(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeContractField,
    ) -> TraversalResult;
    fn emit_with_constraint(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeWithConstraint,
    ) -> TraversalResult;
    fn emit_component_definition(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeComponentDefinition,
    ) -> TraversalResult;
    fn emit_procedure_definition(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeProcedureDefinition,
    ) -> TraversalResult;
    fn emit_transition_definition(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTransitionDefinition,
    ) -> TraversalResult;
    fn emit_type_alternative_clause(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeAlternativeClause,
    ) -> TraversalResult;
    fn emit_type_map_value_arguments(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeMapValueArguments,
    ) -> TraversalResult;
    fn emit_type_map_value_allowing_type_arguments(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeMapValueAllowingTypeArguments,
    ) -> TraversalResult;
}
