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

pub fn infer_types(program: &NodeProgram) -> Result<TypeInferenceState, String> {
    let mut type_inference_state = TypeInferenceState {
        type_environment: HashMap::new(),
        substitution: HashMap::new(),
        counter: 0,
    };

    traverse_nodeprogram(program, &mut type_inference_state);

    // TODO: apply the substitutions to the type_environment
    // You may need to create a helper function that applies the substitutions recursively

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
        // Perform traversal or type inference on the imported_name, if necessary
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
                // Perform traversal or type inference on the let definition elements, if necessary
            }
            NodeLibrarySingleDefinition::TypeDefinition(name, alternative_clauses_opt) => {
                // Perform traversal or type inference on the type definition elements, if necessary
                if let Some(alternative_clauses) = alternative_clauses_opt {
                    for alternative_clause in alternative_clauses {
                        // Perform traversal or type inference on the alternative_clause, if necessary
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
    // TODO: traverse_nodecomponentparameters(&node.parameters);

    if let Some(constraint) = &node.constraint {
        // TODO: traverse_nodewithconstraint(constraint);
    }

    for field in &node.fields {
        // TODO: traverse_nodecontractfield(field);
    }

    for component in &node.components {
        match component {
            NodeComponentDefinition::TransitionComponent(transition_definition) => {
                // traverse_nodetransitiondefinition(transition_definition);
            }
            NodeComponentDefinition::ProcedureComponent(procedure_definition) => {
                // traverse_nodeproceduredefinition(procedure_definition);
            }
        }
    }
}

// Add implementations for traversing the remaining AST nodes, following the same pattern
