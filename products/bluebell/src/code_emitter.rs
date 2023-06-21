use crate::ast::*;
pub enum TreeTraversalMode {
    Enter,
    Exit,
}

#[derive(PartialEq, Eq)]
pub enum TraversalResult {
    Ok,
    SkipChildren,
    Fail(String),
}

pub trait CodeEmitter {
    fn emit_byte_str(&self, mode: TreeTraversalMode, node: &NodeByteStr) -> TraversalResult;
    fn emit_type_name_identifier(
        &self,
        mode: TreeTraversalMode,
        node: &NodeTypeNameIdentifier,
    ) -> TraversalResult;
    fn emit_imported_name(
        &self,
        mode: TreeTraversalMode,
        node: &NodeImportedName,
    ) -> TraversalResult;
    fn emit_import_declarations(
        &self,
        mode: TreeTraversalMode,
        node: &NodeImportDeclarations,
    ) -> TraversalResult;
    fn emit_meta_identifier(
        &self,
        mode: TreeTraversalMode,
        node: &NodeMetaIdentifier,
    ) -> TraversalResult;
    fn emit_variable_identifier(
        &self,
        mode: TreeTraversalMode,
        node: &NodeVariableIdentifier,
    ) -> TraversalResult;
    fn emit_builtin_arguments(
        &self,
        mode: TreeTraversalMode,
        node: &NodeBuiltinArguments,
    ) -> TraversalResult;
    fn emit_type_map_key(&self, mode: TreeTraversalMode, node: &NodeTypeMapKey) -> TraversalResult;
    fn emit_type_map_value(
        &self,
        mode: TreeTraversalMode,
        node: &NodeTypeMapValue,
    ) -> TraversalResult;
    fn emit_type_argument(
        &self,
        mode: TreeTraversalMode,
        node: &NodeTypeArgument,
    ) -> TraversalResult;
    fn emit_scilla_type(&self, mode: TreeTraversalMode, node: &NodeScillaType) -> TraversalResult;
    fn emit_type_map_entry(
        &self,
        mode: TreeTraversalMode,
        node: &NodeTypeMapEntry,
    ) -> TraversalResult;
    fn emit_address_type_field(
        &self,
        mode: TreeTraversalMode,
        node: &NodeAddressTypeField,
    ) -> TraversalResult;
    fn emit_address_type(&self, mode: TreeTraversalMode, node: &NodeAddressType)
        -> TraversalResult;
    fn emit_full_expression(
        &self,
        mode: TreeTraversalMode,
        node: &NodeFullExpression,
    ) -> TraversalResult;
    fn emit_message_entry(
        &self,
        mode: TreeTraversalMode,
        node: &NodeMessageEntry,
    ) -> TraversalResult;
    fn emit_pattern_match_expression_clause(
        &self,
        mode: TreeTraversalMode,
        node: &NodePatternMatchExpressionClause,
    ) -> TraversalResult;
    fn emit_atomic_expression(
        &self,
        mode: TreeTraversalMode,
        node: &NodeAtomicExpression,
    ) -> TraversalResult;
    fn emit_contract_type_arguments(
        &self,
        mode: TreeTraversalMode,
        node: &NodeContractTypeArguments,
    ) -> TraversalResult;
    fn emit_value_literal(
        &self,
        mode: TreeTraversalMode,
        node: &NodeValueLiteral,
    ) -> TraversalResult;
    fn emit_map_access(&self, mode: TreeTraversalMode, node: &NodeMapAccess) -> TraversalResult;
    fn emit_pattern(&self, mode: TreeTraversalMode, node: &NodePattern) -> TraversalResult;
    fn emit_argument_pattern(
        &self,
        mode: TreeTraversalMode,
        node: &NodeArgumentPattern,
    ) -> TraversalResult;
    fn emit_pattern_match_clause(
        &self,
        mode: TreeTraversalMode,
        node: &NodePatternMatchClause,
    ) -> TraversalResult;
    fn emit_blockchain_fetch_arguments(
        &self,
        mode: TreeTraversalMode,
        node: &NodeBlockchainFetchArguments,
    ) -> TraversalResult;
    fn emit_statement(&self, mode: TreeTraversalMode, node: &NodeStatement) -> TraversalResult;
    fn emit_remote_fetch_statement(
        &self,
        mode: TreeTraversalMode,
        node: &NodeRemoteFetchStatement,
    ) -> TraversalResult;
    fn emit_component_id(&self, mode: TreeTraversalMode, node: &NodeComponentId)
        -> TraversalResult;
    fn emit_component_parameters(
        &self,
        mode: TreeTraversalMode,
        node: &NodeComponentParameters,
    ) -> TraversalResult;
    fn emit_parameter_pair(
        &self,
        mode: TreeTraversalMode,
        node: &NodeParameterPair,
    ) -> TraversalResult;
    fn emit_component_body(
        &self,
        mode: TreeTraversalMode,
        node: &NodeComponentBody,
    ) -> TraversalResult;
    fn emit_statement_block(
        &self,
        mode: TreeTraversalMode,
        node: &NodeStatementBlock,
    ) -> TraversalResult;
    fn emit_typed_identifier(
        &self,
        mode: TreeTraversalMode,
        node: &NodeTypedIdentifier,
    ) -> TraversalResult;
    fn emit_type_annotation(
        &self,
        mode: TreeTraversalMode,
        node: &NodeTypeAnnotation,
    ) -> TraversalResult;
    fn emit_program(&self, mode: TreeTraversalMode, node: &NodeProgram) -> TraversalResult;
    fn emit_library_definition(
        &self,
        mode: TreeTraversalMode,
        node: &NodeLibraryDefinition,
    ) -> TraversalResult;
    fn emit_library_single_definition(
        &self,
        mode: TreeTraversalMode,
        node: &NodeLibrarySingleDefinition,
    ) -> TraversalResult;
    fn emit_contract_definition(
        &self,
        mode: TreeTraversalMode,
        node: &NodeContractDefinition,
    ) -> TraversalResult;
    fn emit_contract_field(
        &self,
        mode: TreeTraversalMode,
        node: &NodeContractField,
    ) -> TraversalResult;
    fn emit_with_constraint(
        &self,
        mode: TreeTraversalMode,
        node: &NodeWithConstraint,
    ) -> TraversalResult;
    fn emit_component_definition(
        &self,
        mode: TreeTraversalMode,
        node: &NodeComponentDefinition,
    ) -> TraversalResult;
    fn emit_procedure_definition(
        &self,
        mode: TreeTraversalMode,
        node: &NodeProcedureDefinition,
    ) -> TraversalResult;
    fn emit_transition_definition(
        &self,
        mode: TreeTraversalMode,
        node: &NodeTransitionDefinition,
    ) -> TraversalResult;
    fn emit_type_alternative_clause(
        &self,
        mode: TreeTraversalMode,
        node: &NodeTypeAlternativeClause,
    ) -> TraversalResult;
    fn emit_type_map_value_arguments(
        &self,
        mode: TreeTraversalMode,
        node: &NodeTypeMapValueArguments,
    ) -> TraversalResult;
    fn emit_type_map_value_allowing_type_arguments(
        &self,
        mode: TreeTraversalMode,
        node: &NodeTypeMapValueAllowingTypeArguments,
    ) -> TraversalResult;
}
