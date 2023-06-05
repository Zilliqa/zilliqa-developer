use crate::ast::*;
use crate::type_classes::*;
use std::collections::HashMap;

// Placeholder functions for the top-level type checking and inference

pub fn type_of_stmt(
    stmt: &NodeStatement,
    env: &HashMap<String, Box<dyn BaseType>>,
) -> Result<TypeAnnotation, String> {
    unimplemented!()
}

pub fn type_of_variable_identifier(
    identifier: &NodeVariableIdentifier,
    env: &HashMap<String, Box<dyn BaseType>>,
) -> Result<TypeAnnotation, String> {
    match identifier {
        NodeVariableIdentifier::VariableName(name) => {
            match env.get(name) {
                Some(t) => Ok(t.get_instance()), // Return the type of the variable if found in the environment
                None => Err(format!("{} is not defined", name)), // Return an error if the variable is not defined in the environment
            }
        }
        // You may need to add more match patterns once you add more enums to NodeVariableIdentifier
        _ => Err(String::from("Unsupported NodeVariableIdentifier variant")),
    }
}

pub fn type_of_node_scilla_type(
    scilla_type: &NodeScillaType,
    env: &HashMap<String, Box<dyn BaseType>>,
) -> Result<TypeAnnotation, String> {
    match scilla_type {
        NodeScillaType::GenericTypeWithArgs(meta_identifier, type_arguments) => {
            // TODO: Handle GenericTypeWithArgs
            unimplemented!()
        }
        NodeScillaType::MapType(map_key, map_value) => {
            // TODO: Handle MapType
            unimplemented!()
        }
        NodeScillaType::FunctionType(from_type, to_type) => {
            // TODO: Handle FunctionType
            unimplemented!()
        }
        NodeScillaType::EnclosedType(enclosed_type) => {
            // TODO: Handle EnclosedType
            unimplemented!()
        }
        NodeScillaType::ScillaAddresseType(address_type) => {
            // TODO: Handle ScillaAddresseType
            unimplemented!()
        }
        NodeScillaType::PolyFunctionType(template_name, poly_function_type) => {
            // TODO: Handle PolyFunctionType
            unimplemented!()
        }
        NodeScillaType::TypeVarType(type_var_type) => {
            // TODO: Handle TypeVarType
            unimplemented!()
        }
    }
}

pub fn type_of_expression(
    expr: &NodeFullExpression,
    env: &HashMap<String, Box<dyn BaseType>>,
) -> Result<TypeAnnotation, String> {
    match expr {
        NodeFullExpression::LocalVariableDeclaration { .. } => {
            unimplemented!()
            // Handle the LocalVariableDeclaration case
        }
        NodeFullExpression::FunctionDeclaration { .. } => {
            unimplemented!()
            // Handle the FunctionDeclaration case
        }
        NodeFullExpression::FunctionCall { .. } => {
            unimplemented!()
            // Handle the FunctionCall case
        }
        // Add more cases for other variants of NodeFullExpression.
        _ => unimplemented!(),
    }
}

pub fn type_of_node_message_entry(
    message_entry: &NodeMessageEntry,
    env: &HashMap<String, Box<dyn BaseType>>,
) -> Result<TypeAnnotation, String> {
    match message_entry {
        NodeMessageEntry::MessageLiteral(variable_identifier, value_literal) => {
            // Infer the types of the variable_identifier and value_literal.
            // Check for compatibility of types and return the appropriate type.
            unimplemented!()
        }
        NodeMessageEntry::MessageVariable(variable_identifier1, variable_identifier2) => {
            // Infer the types of both variable_identifiers.
            // Check for compatibility of types and return the appropriate type.
            unimplemented!()
        }
    }
}

pub fn type_of_node_pattern(
    pattern: &NodePattern,
    env: &HashMap<String, Box<dyn BaseType>>,
) -> Result<TypeAnnotation, String> {
    match pattern {
        NodePattern::Wildcard => {
            // Handle Wildcard pattern
            unimplemented!()
        }
        NodePattern::Binder(ref binder) => {
            // Handle Binder pattern
            unimplemented!()
        }
        NodePattern::Constructor(ref constructor, ref arg_patterns) => {
            // Handle Constructor pattern
            unimplemented!()
        }
    }
}

pub fn type_of_node_statement(
    stmt: &NodeStatement,
    env: &HashMap<String, Box<dyn BaseType>>,
) -> Result<TypeAnnotation, String> {
    match stmt {
        NodeStatement::Accept => {
            // Implement logic for the 'Accept' statement
            unimplemented!()
        }
        NodeStatement::Send { identifier_name } => {
            // Implement logic for the 'Send' statement using the variable identifier
            let identifier_type = type_of_variable_identifier(identifier_name, &env)?;
            // Handle type checking or inference for the 'Send' statement here
            unimplemented!()
        }
        NodeStatement::Load {
            left_hand_side,
            right_hand_side,
        } => {
            // Implement logic for the 'Load' statement
            /*
            TODO: Not working
            let lhs_type = type_of_variable_identifier(left_hand_side, &env)?;
            let rhs_type = type_of_variable_identifier(right_hand_side, &env)?;
            if lhs_type != rhs_type {
                return Err(format!("Type mismatch: {:?} != {:?}", lhs_type, rhs_type));
            }
            */
            // Handle type checking or inference for the 'Load' statement here
            unimplemented!()
        }
        _ => unimplemented!(), // Handle other statement types separately once you become comfortable with implementing the easier ones
    }
}

pub fn type_check_func(func: &NodeProcedureDefinition) -> Result<FunType, String> {
    // Create an environment mapping for argument names to their types

    // TODO: Allocate this type: HashMap<String, Box<dyn BaseType>>
    // let mut env = HashMap::new();
    /*
    TODO: Fix this
    for (param, param_type) in &func.parameters {
        env.insert(param.name.clone(), param_type.clone());
    }
    */

    // Check the types of each statement in the function body
    /*
    TODO: Fix this
    for stmt in &func.body.statement_block.statements {
        let stmt_type = type_of_stmt(stmt, &env)?;
        // Perform any additional type checks required for the statement here
    }
    */

    // Determine the return type of the function by examining the (potentially) last return statement
    /*
    TODO: Fix this
    let ret_type = if let Some(last_stmt) = func.body.statement_block.statements.last() {
        match last_stmt {
            NodeStatement::Return { value, .. } => type_of(value, &env)?,
            _ => return Err("Function does not end with a return statement".to_string()),
        }
    } else {
        return Err("Function body is empty".to_string());
    };
    */

    /*
    TODO: OK
    Ok(FunType {
        // Fill in the necessary fields for FunType struct based on the gathered information
    })
    */

    unimplemented!();
}

pub fn type_check_transition(
    transition: &NodeTransitionDefinition,
    global_env: &HashMap<String, Box<dyn BaseType>>,
) -> Result<(), String> {
    /*
    // Create an environment mapping for parameter names to their types
    let mut env: HashMap<String, Box<dyn BaseType>> = transition
        .parameters
        .parameters
        .iter()
        .map(|param| (param.identifier_with_type.identifier_name.clone(), param.identifier_with_type.type_annotation.get_instance()))
        .collect();

    // Merge the local environment with the global environment
    for (k, v) in global_env.iter() {
        env.entry(k.clone()).or_insert(v.clone());
    }

    // Type check each statement in the transition
    if let Some(statement_block) = &transition.body.statement_block {
        for stmt in &statement_block.statements {
            type_of_stmt(stmt, &env)?;
        }
    }
    */
    Ok(())
}

pub fn type_of(
    expr: &dyn AnyKind,
    env: &HashMap<String, Box<dyn BaseType>>,
) -> Result<TypeAnnotation, String> {
    let any = expr.to_any();
    match any {
        NodeAny::NodeByteStr(_) => unimplemented!(),
        NodeAny::NodeTypeNameIdentifier(_) => unimplemented!(),
        NodeAny::NodeImportedName(_) => unimplemented!(),
        NodeAny::NodeImportDeclarations(_) => unimplemented!(),
        NodeAny::NodeMetaIdentifier(_) => unimplemented!(),
        NodeAny::NodeVariableIdentifier(_) => unimplemented!(),
        NodeAny::NodeBuiltinArguments(_) => unimplemented!(),
        NodeAny::NodeTypeMapKey(_) => unimplemented!(),
        NodeAny::NodeTypeMapValue(_) => unimplemented!(),
        NodeAny::NodeTypeArgument(_) => unimplemented!(),
        NodeAny::NodeScillaType(_) => unimplemented!(),
        NodeAny::NodeTypeMapEntry(_) => unimplemented!(),

        NodeAny::NodeAddressTypeField(_) => unimplemented!(),
        NodeAny::NodeAddressType(_) => unimplemented!(),
        NodeAny::NodeFullExpression(full_expr) => match full_expr {
            NodeFullExpression::LocalVariableDeclaration { .. } => unimplemented!(),
            NodeFullExpression::FunctionDeclaration { .. } => unimplemented!(),
            // ... More cases for other variants of NodeFullExpression
            _ => unimplemented!(),
        },
        NodeAny::NodeMessageEntry(_) => unimplemented!(),
        NodeAny::NodePatternMatchExpressionClause(_) => unimplemented!(),
        NodeAny::NodeAtomicExpression(_) => unimplemented!(),
        NodeAny::NodeContractTypeArguments(_) => unimplemented!(),
        NodeAny::NodeValueLiteral(_) => unimplemented!(),
        NodeAny::NodeMapAccess(_) => unimplemented!(),
        NodeAny::NodePattern(_) => unimplemented!(),
        NodeAny::NodeArgumentPattern(_) => unimplemented!(),
        NodeAny::NodePatternMatchClause(_) => unimplemented!(),
        NodeAny::NodeBlockchainFetchArguments(_) => unimplemented!(),
        NodeAny::NodeStatement(_) => unimplemented!(),
        NodeAny::NodeRemoteFetchStatement(_) => unimplemented!(),
        NodeAny::NodeComponentId(_) => unimplemented!(),
        NodeAny::NodeComponentParameters(_) => unimplemented!(),
        NodeAny::NodeParameterPair(_) => unimplemented!(),
        NodeAny::NodeComponentBody(_) => unimplemented!(),
        NodeAny::NodeStatementBlock(_) => unimplemented!(),
        NodeAny::NodeTypedIdentifier(_) => unimplemented!(),
        NodeAny::NodeTypeAnnotation(_) => unimplemented!(),
        NodeAny::NodeProgram(_) => unimplemented!(),
        NodeAny::NodeLibraryDefinition(_) => unimplemented!(),
        NodeAny::NodeLibrarySingleDefinition(_) => unimplemented!(),
        NodeAny::NodeContractDefinition(_) => unimplemented!(),
        NodeAny::NodeContractField(_) => unimplemented!(),
        NodeAny::NodeWithConstraint(_) => unimplemented!(),
        NodeAny::NodeComponentDefinition(_) => unimplemented!(),
        NodeAny::NodeProcedureDefinition(_) => unimplemented!(),
        NodeAny::NodeTransitionDefinition(_) => unimplemented!(),
        NodeAny::NodeTypeAlternativeClause(_) => unimplemented!(),
        NodeAny::NodeTypeMapValueArguments(_) => unimplemented!(),
        NodeAny::NodeTypeMapValueAllowingTypeArguments(_) => unimplemented!(),
    }
}
