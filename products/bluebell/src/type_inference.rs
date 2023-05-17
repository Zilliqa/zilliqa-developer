use crate::ast::*;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TypeVariable {
    Named(String),
    Generated(usize),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeInfo {
    TypeVar(TypeVariable),
    Function(Box<TypeInfo>, Box<TypeInfo>),
    ScillaType(NodeScillaType),
}

pub type TypeEnvironment = HashMap<String, TypeInfo>;
pub type Substitution = HashMap<TypeVariable, TypeInfo>;

pub struct TypeInferenceState {
    pub type_environment: TypeEnvironment,
    pub substitution: Substitution,
    pub counter: usize,
}

impl TypeInferenceState {
    pub fn new() -> Self {
        TypeInferenceState {
            type_environment: TypeEnvironment::new(),
            substitution: Substitution::new(),
            counter: 1,
        }
    }
}

/*
fn unify(t1: &TypeInfo, t2: &TypeInfo, type_inference_state: &mut TypeInferenceState) -> Result<(), String> {
    // Implement the unification algorithm here.
    // Update the type_inference_state.substitution map accordingly.
    // Return an error in case the types cannot be unified.
}

*/

fn extend_type_environment(
    identifier: &str,
    type_info: TypeInfo,
    type_inference_state: &mut TypeInferenceState,
) {
    type_inference_state
        .type_environment
        .insert(identifier.to_owned(), type_info);
}

fn generate_fresh_type_variable(type_inference_state: &mut TypeInferenceState) -> TypeInfo {
    let type_variable = TypeVariable::Generated(type_inference_state.counter);
    type_inference_state.counter += 1;
    TypeInfo::TypeVar(type_variable)
}

fn apply_substitutions(type_info: &TypeInfo, substitution: &Substitution) -> TypeInfo {
    match type_info {
        TypeInfo::TypeVar(type_variable) => {
            substitution.get(type_variable).unwrap_or(type_info).clone()
        }
        TypeInfo::Function(input, output) => TypeInfo::Function(
            Box::new(apply_substitutions(input, substitution)),
            Box::new(apply_substitutions(output, substitution)),
        ),
        _ => type_info.clone(),
    }
}

// Apply substitutions and update the type_environment
fn apply_substitutions_to_type_environment(type_inference_state: &mut TypeInferenceState) {
    for (_, type_info) in type_inference_state.type_environment.iter_mut() {
        *type_info = apply_substitutions(type_info, &type_inference_state.substitution);
    }
}

pub fn infer_types(program: &NodeProgram) -> Result<TypeInferenceState, String> {
    let mut type_inference_state = TypeInferenceState {
        type_environment: HashMap::new(),
        substitution: HashMap::new(),
        counter: 0,
    };

    traverse_nodeprogram(program, &mut type_inference_state);

    // Apply the substitutions to the type_environment
    apply_substitutions_to_type_environment(&mut type_inference_state);

    Ok(type_inference_state)
}

fn traverse_nodeprogram(node: &NodeProgram, type_inference_state: &mut TypeInferenceState) {
    if let Some(import_declarations) = &node.import_declarations {
        traverse_nodeimportdeclarations(import_declarations, type_inference_state);
    }

    if let Some(library_definition) = &node.library_definition {
        traverse_nodelibrarydefinition(library_definition, type_inference_state);
    }

    traverse_nodecontractdefinition(&node.contract_definition, type_inference_state);
}

fn traverse_nodeimportdeclarations(
    node: &NodeImportDeclarations,
    type_inference_state: &mut TypeInferenceState,
) {
    for imported_name in &node.import_list {
        match imported_name {
            NodeImportedName::RegularImport(type_name_id) => {
                traverse_nodetypenameidentifier(type_name_id, type_inference_state);
            }
            NodeImportedName::AliasedImport(original_type_name_id, alias_type_name_id) => {
                traverse_nodetypenameidentifier(original_type_name_id, type_inference_state);
                traverse_nodetypenameidentifier(alias_type_name_id, type_inference_state);
            }
        }
    }
}

fn traverse_nodelibrarydefinition(
    node: &NodeLibraryDefinition,
    type_inference_state: &mut TypeInferenceState,
) {
    for definition in &node.definitions {
        match definition {
            NodeLibrarySingleDefinition::LetDefinition {
                variable_name,
                type_annotation,
                expression,
            } => {
                // TODO: Perform traversal or type inference on the let definition elements, if necessary
            }
            NodeLibrarySingleDefinition::TypeDefinition(name, alternative_clauses_opt) => {
                // TODO: Perform traversal or type inference on the type definition elements, if necessary
                if let Some(alternative_clauses) = alternative_clauses_opt {
                    for alternative_clause in alternative_clauses {
                        // TODO: Perform traversal or type inference on the alternative_clause, if necessary
                    }
                }
            }
        }
    }
}

fn traverse_nodecontractdefinition(
    node: &NodeContractDefinition,
    type_inference_state: &mut TypeInferenceState,
) {
    traverse_nodecomponentparameters(&node.parameters, type_inference_state);

    if let Some(constraint) = &node.constraint {
        traverse_nodewithconstraint(constraint, type_inference_state);
    }

    for field in &node.fields {
        traverse_nodecontractfield(field, type_inference_state);
    }

    for component in &node.components {
        match component {
            NodeComponentDefinition::TransitionComponent(transition_definition) => {
                traverse_nodetransitiondefinition(transition_definition, type_inference_state);
            }
            NodeComponentDefinition::ProcedureComponent(procedure_definition) => {
                traverse_nodeproceduredefinition(procedure_definition, type_inference_state);
            }
        }
    }
}

fn traverse_nodecomponentparameters(
    node: &NodeComponentParameters,
    type_inference_state: &mut TypeInferenceState,
) {
    for parameter in &node.parameters {
        traverse_nodeparameterpair(parameter, type_inference_state);
    }
}

fn traverse_nodewithconstraint(
    node: &NodeWithConstraint,
    type_inference_state: &mut TypeInferenceState,
) {
    // Perform traversal or type inference on the node's expression
    traverse_nodefull_expression(&node.expression, type_inference_state);
}

fn traverse_nodecontractfield(
    node: &NodeContractField,
    type_inference_state: &mut TypeInferenceState,
) {
    // Perform traversal or type inference on the node's elements
    // TODO: Function not found traverse_nodetyped_identifier(&node.typed_identifier, type_inference_state);
    traverse_nodefull_expression(&node.right_hand_side, type_inference_state);
}

fn traverse_nodetransitiondefinition(
    node: &NodeTransitionDefinition,
    type_inference_state: &mut TypeInferenceState,
) {
    traverse_nodecomponentparameters(&node.parameters, type_inference_state);
    traverse_nodecomponentbody(&node.body, type_inference_state);
}

fn traverse_nodeproceduredefinition(
    node: &NodeProcedureDefinition,
    type_inference_state: &mut TypeInferenceState,
) {
    traverse_nodecomponentparameters(&node.parameters, type_inference_state);
    traverse_nodecomponentbody(&node.body, type_inference_state);
}

fn traverse_nodeparameterpair(
    node: &NodeParameterPair,
    type_inference_state: &mut TypeInferenceState,
) {
    // Traverse the identifier_with_type field
    traverse_nodetypedidentifier(&node.identifier_with_type, type_inference_state);

    // Traverse the annotation field
    // TODO: Implement this
}

fn traverse_nodecomponentbody(
    node: &NodeComponentBody,
    type_inference_state: &mut TypeInferenceState,
) {
    // TODO: Perform traversal or type inference on the node's optional statement block, if necessary
    if let Some(statement_block) = &node.statement_block {
        traverse_nodestatementblock(statement_block, type_inference_state);
    }
}

fn traverse_nodetypedidentifier(
    node: &NodeTypedIdentifier,
    type_inference_state: &mut TypeInferenceState,
) {
    // TODO: Perform traversal or type inference on the node's elements, if necessary
    traverse_nodetypeannotation(&node.annotation, type_inference_state);
}

// Traverse NodeTypeAnnotation and perform the necessary traversal or type inference
fn traverse_nodetypeannotation(
    node: &NodeTypeAnnotation,
    type_inference_state: &mut TypeInferenceState,
) {
    traverse_nodescilla_type(&node.type_name, type_inference_state);
}

fn traverse_nodestatementblock(
    node: &NodeStatementBlock,
    type_inference_state: &mut TypeInferenceState,
) {
    for statement in &node.statements {
        // TODO: Perform traversal or type inference on each statement, if necessary
    }
}

// Add implementations for traversing the remaining AST nodes, following the same pattern

fn traverse_node_statement(node: &NodeStatement, type_inference_state: &mut TypeInferenceState) {
    match node {
        NodeStatement::Load {
            left_hand_side,
            right_hand_side,
        } => {
            // TODO: Perform traversal or type inference here
        }
        NodeStatement::RemoteFetch(remote_fetch_statement) => {
            traverse_node_remote_fetch_statement(remote_fetch_statement, type_inference_state);
        }
        NodeStatement::Store {
            left_hand_side,
            right_hand_side,
        } => {
            // TODO: Perform traversal or type inference here
        }
        NodeStatement::Bind {
            left_hand_side,
            right_hand_side,
        } => {
            // TODO: Perform traversal or type inference here
        }
        NodeStatement::ReadFromBC {
            left_hand_side,
            type_name,
            arguments,
        } => {
            // TODO: Perform traversal or type inference here
            if let Some(argument_list) = arguments {
                traverse_arguments_list(&argument_list.arguments, type_inference_state);
            }
        }
        NodeStatement::MapGet {
            left_hand_side,
            keys,
            right_hand_side,
        } => {
            // TODO: Perform traversal or type inference here
        }
        NodeStatement::MapGetExists {
            left_hand_side,
            keys,
            right_hand_side,
        } => {
            // TODO: Perform traversal or type inference here
        }
        NodeStatement::MapUpdate {
            left_hand_side,
            keys,
            right_hand_side,
        } => {
            // TODO: Perform traversal or type inference here
        }
        NodeStatement::MapUpdateDelete {
            left_hand_side,
            keys,
        } => {
            // TODO: Perform traversal or type inference here
        }
        NodeStatement::Accept => {
            // Nothing to do here, since Accept has no parameters
        }
        NodeStatement::Send { identifier_name } => {
            // TODO: Perform traversal or type inference here
            traverse_node_variable_identifier(identifier_name, type_inference_state);
        }
        NodeStatement::CreateEvnt { identifier_name } => {
            // TODO: Perform traversal or type inference here
            traverse_node_variable_identifier(identifier_name, type_inference_state);
        }
        NodeStatement::Throw { error_variable } => {
            if let Some(error_var) = error_variable {
                // TODO: Perform traversal or type inference here
                traverse_node_variable_identifier(error_var, type_inference_state);
            }
        }
        NodeStatement::MatchStmt { variable, clauses } => {
            traverse_node_variable_identifier(variable, type_inference_state);
            for clause in clauses {
                traverse_node_pattern_match_clause(clause, type_inference_state);
            }
        }
        NodeStatement::CallProc {
            component_id,
            arguments,
        } => {
            // TODO: Perform traversal or type inference here
            traverse_arguments_list(arguments, type_inference_state);
        }
        NodeStatement::Iterate {
            identifier_name,
            component_id,
        } => {
            traverse_node_variable_identifier(identifier_name, type_inference_state);
            // TODO: Perform traversal or type inference here for component_id if needed
        }
    }
}

fn traverse_node_remote_fetch_statement(
    node: &NodeRemoteFetchStatement,
    type_inference_state: &mut TypeInferenceState,
) {
    match node {
        NodeRemoteFetchStatement::ReadStateMutable(_, _, _) => { /* TODO: Perform traversal or type inference */
        }
        NodeRemoteFetchStatement::ReadStateMutableSpecialId(_, _, _) => { /* TODO: Perform traversal or type inference */
        }
        NodeRemoteFetchStatement::ReadStateMutableMapAccess(_, _, _, _) => { /* TODO: Perform traversal or type inference */
        }
        NodeRemoteFetchStatement::ReadStateMutableMapAccessExists(_, _, _, _) => { /* TODO: Perform traversal or type inference */
        }
        NodeRemoteFetchStatement::ReadStateMutableCastAddress(_, _, _) => { /* TODO: Perform traversal or type inference */
        }
    }
}

fn traverse_pattern_match_expression_clause(
    node: &NodePatternMatchExpressionClause,
    type_inference_state: &mut TypeInferenceState,
) {
    // TODO: Perform traversal or type inference on the node's elements, if necessary
}

fn traverse_arguments_list(
    node: &[NodeVariableIdentifier],
    type_inference_state: &mut TypeInferenceState,
) {
    // TODO: Perform traversal or type inference on the elements of arguments list
    for arg in node {
        traverse_node_variable_identifier(arg, type_inference_state);
    }
}

fn traverse_node_full_expression(
    node: &NodeFullExpression,
    type_inference_state: &mut TypeInferenceState,
) {
    // TODO: Match each variant of NodeFullExpression and perform traversal or type inference
}

fn traverse_node_atomic_expression(
    node: &NodeAtomicExpression,
    type_inference_state: &mut TypeInferenceState,
) {
    // TODO: Perform traversal or type inference on the elements of NodeAtomicExpression
}

fn traverse_node_value_literal(
    node: &NodeValueLiteral,
    type_inference_state: &mut TypeInferenceState,
) {
    // TODO: Perform traversal or type inference on the elements of NodeValueLiteral
}

// Traverse NodeScillaType and perform the necessary traversal or type inference
fn traverse_nodescilla_type(node: &NodeScillaType, type_inference_state: &mut TypeInferenceState) {
    match node {
        NodeScillaType::GenericTypeWithArgs(meta_identifier, type_arguments) => {
            // TODO: Perform traversal or type inference on meta_identifier and type_arguments
            traverse_node_meta_identifier(meta_identifier, type_inference_state);
            for type_argument in type_arguments {
                traverse_nodescilla_type_argument(type_argument, type_inference_state);
            }
        }
        NodeScillaType::MapType(map_key, map_value) => {
            // TODO: Perform traversal or type inference on map_key and map_value
            traverse_nodetype_map_key(map_key, type_inference_state);
            traverse_nodetype_map_value(map_value, type_inference_state);
        }
        NodeScillaType::FunctionType(dom, codom) => {
            // TODO: Perform traversal or type inference on dom and codom
            traverse_nodescilla_type(dom, type_inference_state);
            traverse_nodescilla_type(codom, type_inference_state);
        }
        NodeScillaType::EnclosedType(enclosed_type) => {
            // TODO: Perform traversal or type inference on enclosed_type
            traverse_nodescilla_type(enclosed_type, type_inference_state);
        }
        NodeScillaType::ScillaAddresseType(address_type) => {
            // TODO: Perform traversal or type inference on address_type
            traverse_node_address_type(address_type, type_inference_state);
        }
        NodeScillaType::PolyFunctionType(_, inner_type) => {
            // TODO: Perform traversal or type inference on inner_type
            traverse_nodescilla_type(inner_type, type_inference_state);
        }
        NodeScillaType::TypeVarType(_) => {
            // No traversal or type inference needed as there is no nested type
        }
    }
}

// Traverse NodeVariableIdentifier and perform the necessary traversal or type inference
fn traverse_node_variable_identifier(
    node: &NodeVariableIdentifier,
    type_inference_state: &mut TypeInferenceState,
) {
    match node {
        NodeVariableIdentifier::VariableName(name) => {
            // TODO: Perform traversal or type inference on name if needed
        }
        NodeVariableIdentifier::SpecialIdentifier(special_id) => {
            // TODO: Perform traversal or type inference on special_id if needed
        }
        NodeVariableIdentifier::VariableInNamespace(type_name_id, var_name) => {
            // TODO: Perform traversal or type inference on type_name_id and var_name if needed
            traverse_nodetypenameidentifier(type_name_id, type_inference_state);
        }
    }
}

// Traverse NodeScillaTypeArgument and perform the necessary traversal or type inference
fn traverse_nodescilla_type_argument(
    node: &NodeTypeArgument,
    type_inference_state: &mut TypeInferenceState,
) {
    match node {
        NodeTypeArgument::EnclosedTypeArgument(scilla_type) => {
            traverse_nodescilla_type(scilla_type, type_inference_state);
        }
        NodeTypeArgument::GenericTypeArgument(meta_identifier) => {
            traverse_node_meta_identifier(meta_identifier, type_inference_state);
        }
        NodeTypeArgument::TemplateTypeArgument(_)
        | NodeTypeArgument::AddressTypeArgument(_)
        | NodeTypeArgument::MapTypeArgument(_, _) => {
            // If needed, handle template and address type arguments and map key-value type arguments
        }
    }
}

// Traverse NodeTypeMapKey and perform the necessary traversal or type inference
fn traverse_nodetype_map_key(node: &NodeTypeMapKey, type_inference_state: &mut TypeInferenceState) {
    match node {
        NodeTypeMapKey::GenericMapKey(meta_identifier)
        | NodeTypeMapKey::EnclosedGenericId(meta_identifier) => {
            traverse_node_meta_identifier(meta_identifier, type_inference_state);
        }
        NodeTypeMapKey::EnclosedAddressMapKeyType(address_type)
        | NodeTypeMapKey::AddressMapKeyType(address_type) => {
            traverse_node_address_type(address_type, type_inference_state);
        }
    }
}

// Traverse NodeTypeMapValue and perform the necessary traversal or type inference
fn traverse_nodetype_map_value(
    node: &NodeTypeMapValue,
    type_inference_state: &mut TypeInferenceState,
) {
    match node {
        NodeTypeMapValue::MapValueCustomType(meta_identifier) => {
            traverse_node_meta_identifier(meta_identifier, type_inference_state);
        }
        NodeTypeMapValue::MapKeyValue(entry) => {
            traverse_nodetype_map_entry(entry, type_inference_state);
        }
        NodeTypeMapValue::MapValueParanthesizedType(value_type) => {
            // TODO:            traverse_nodetype_map_value(value_type, type_inference_state);
        }
        NodeTypeMapValue::MapValueAddressType(address_type) => {
            traverse_node_address_type(address_type, type_inference_state);
        }
    }
}

// Traverse NodeAddressType and perform the necessary traversal or type inference
fn traverse_node_address_type(
    node: &NodeAddressType,
    type_inference_state: &mut TypeInferenceState,
) {
    for field in &node.address_fields {
        // TODO:        traverse_nodecontractfield(field, type_inference_state);
    }
}

// Traverse NodeTypeMapEntry and perform the necessary traversal or type inference
fn traverse_nodetype_map_entry(
    node: &NodeTypeMapEntry,
    type_inference_state: &mut TypeInferenceState,
) {
    traverse_nodetype_map_key(&node.key, type_inference_state);
    traverse_nodetype_map_value(&node.value, type_inference_state);
}

// Traverse NodeMetaIdentifier and perform the necessary traversal or type inference
fn traverse_node_meta_identifier(
    node: &NodeMetaIdentifier,
    type_inference_state: &mut TypeInferenceState,
) {
    match node {
        NodeMetaIdentifier::MetaName(type_name) => {
            traverse_nodetypenameidentifier(type_name, type_inference_state)
        }
        // Add more match branches for other NodeMetaIdentifier variants as needed
        _ => (),
    }
}

// Traverse NodeTypeNameIdentifier and perform the necessary traversal or type inference
fn traverse_nodetypenameidentifier(
    node: &NodeTypeNameIdentifier,
    type_inference_state: &mut TypeInferenceState,
) {
    match node {
        NodeTypeNameIdentifier::ByteStringType(byte_str) => {
            // TODO: Perform traversal or type inference on byte_str, if needed
        }
        // Add more match branches for other NodeTypeNameIdentifier variants as needed
        _ => (),
    }
}

// Traverse NodePatternMatchClause and perform the necessary traversal or type inference
fn traverse_node_pattern_match_clause(
    node: &NodePatternMatchClause,
    type_inference_state: &mut TypeInferenceState,
) {
    traverse_node_pattern(&node.pattern_expression, type_inference_state);

    if let Some(statement_block) = &node.statement_block {
        traverse_nodestatementblock(statement_block, type_inference_state);
    }
}

// Traverse NodePattern and perform the necessary traversal or type inference
fn traverse_node_pattern(node: &NodePattern, type_inference_state: &mut TypeInferenceState) {
    match node {
        NodePattern::Wildcard => {
            // No traversal or type inference needed for Wildcard
        }
        NodePattern::Binder(binder_name) => {
            // TODO: Perform traversal or type inference on binder_name, if needed
        }
        NodePattern::Constructor(meta_identifier, argument_patterns) => {
            traverse_node_meta_identifier(meta_identifier, type_inference_state);

            for arg_pattern in argument_patterns {
                traverse_node_argument_pattern(arg_pattern, type_inference_state);
            }
        }
    }
}

// Traverse NodeArgumentPattern and perform the necessary traversal or type inference
fn traverse_node_argument_pattern(
    node: &NodeArgumentPattern,
    type_inference_state: &mut TypeInferenceState,
) {
    match node {
        NodeArgumentPattern::WildcardArgument => {
            // No traversal or type inference needed for WildcardArgument
        }
        NodeArgumentPattern::BinderArgument(binder_name) => {
            // TODO: Perform traversal or type inference on binder_name, if needed
        }
        NodeArgumentPattern::ConstructorArgument(meta_identifier) => {
            traverse_node_meta_identifier(meta_identifier, type_inference_state);
        }
        NodeArgumentPattern::PatternArgument(pattern) => {
            traverse_node_pattern(pattern, type_inference_state);
        }
    }
}

fn traverse_nodefull_expression(
    node: &NodeFullExpression,
    type_inference_state: &mut TypeInferenceState,
) {
    match node {
        NodeFullExpression::LocalVariableDeclaration {
            identifier_name,
            expression,
            type_annotation,
            containing_expression,
        } => {
            // TODO: Perform traversal or type inference on the elements, if necessary
        }
        NodeFullExpression::FunctionDeclaration {
            identier_value,
            type_annotation,
            expression,
        } => {
            // TODO: Perform traversal or type inference on the elements, if necessary
        }
        NodeFullExpression::FunctionCall {
            function_name,
            argument_list,
        } => {
            // TODO: Perform traversal or type inference on the elements, if necessary
        }
        NodeFullExpression::ExpressionAtomic(atomic_expr) => {
            // TODO: Perform traversal or type inference on atomic_expr
            traverse_node_atomic_expression(atomic_expr, type_inference_state);
        }
        // Add more match arms for other NodeFullExpression variants
        // and perform traversal or type inference as needed
        _ => (),
    }
}
