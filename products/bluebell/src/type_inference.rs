use crate::ast::*;
use crate::constants::*;
use crate::type_classes::*;
use std::collections::HashMap;

// Placeholder functions for the top-level type checking and inference
pub trait TypeInference {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String>;
}

pub struct Workspace {
    pub env: HashMap<String, Box<dyn BaseType>>,
    pub namespace: String,
}

impl Clone for Workspace {
    fn clone(&self) -> Self {
        Workspace {
            env: self
                .env
                .iter()
                .map(|(key, val)| (key.clone(), val.clone_boxed()))
                .collect(),
            namespace: self.namespace.clone(),
        }
    }
}

fn resolve_qualified_name(
    workspace: &mut Workspace,
    basename: &String,
) -> Option<Box<dyn BaseType>> {
    let mut namespaces = workspace
        .namespace
        .split(NAMESPACE_SEPARATOR)
        .collect::<Vec<&str>>();
    while !namespaces.is_empty() {
        let full_name = format!(
            "{}{}{}",
            namespaces.join(NAMESPACE_SEPARATOR),
            NAMESPACE_SEPARATOR,
            basename
        );
        println!("Trying name {} {}", full_name, workspace.namespace);
        if let Some(var) = workspace.env.get(&full_name) {
            println!("Found name {} {}", full_name, workspace.namespace);
            return Some((*var).clone_boxed());
        }
        // Remove the last level of the namespace
        namespaces.pop();
    }
    // Try with just the basename, with no namespace
    if let Some(var) = workspace.env.get(basename) {
        println!("Found name {}", basename);
        return Some((*var).clone_boxed());
    }
    // If nothing was found, return None
    None
}

impl TypeInference for NodeVariableIdentifier {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        match self {
            NodeVariableIdentifier::VariableName(name) => {
                match resolve_qualified_name(workspace, &name) {
                    Some(t) => Ok(t.get_instance()),
                    None => Err(format!("{} is not defined", name)),
                }
            }
            NodeVariableIdentifier::SpecialIdentifier(id) => Err(format!(
                "Type inference for SpecialIdentifier '{}' not supported",
                id
            )),
            NodeVariableIdentifier::VariableInNamespace(namespace, var_name) => {
                let name = match namespace {
                    NodeTypeNameIdentifier::ByteStringType(bystr) => {
                        return Err("Namespace cannot be a ByteStringType".to_string());
                    }
                    NodeTypeNameIdentifier::EventType => {
                        return Err("Namespace cannot be an EventType".to_string());
                    }
                    NodeTypeNameIdentifier::CustomType(s) => s.to_string(),
                };
                match resolve_qualified_name(workspace, &name) {
                    Some(t) => {
                        let qualified_name = format!("{}{}{}", name, NAMESPACE_SEPARATOR, var_name);
                        match workspace.env.get(&qualified_name) {
                            Some(ty) => Ok(ty.get_instance()),
                            None => Err(format!("{} is not defined", qualified_name)),
                        }
                    }
                    None => Err(format!("Namespace {} is not defined", name)),
                }
            }
        }
    }
}

impl TypeInference for NodeTypeNameIdentifier {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        match self {
            NodeTypeNameIdentifier::ByteStringType(byte_str) => {
                Ok(TypeAnnotation::BuiltinType(BuiltinType {
                    name: "ByStr".to_string(),
                    symbol: "ByStr".to_string(),
                }))
            }
            NodeTypeNameIdentifier::EventType => Ok(TypeAnnotation::BuiltinType(BuiltinType {
                name: "Event".to_string(),
                symbol: "Event".to_string(),
            })),
            NodeTypeNameIdentifier::CustomType(custom_type) => match workspace.env.get(custom_type)
            {
                Some(t) => Ok(t.get_instance()),
                None => Err(format!("{} is not defined", custom_type)),
            },
        }
    }
}

impl TypeInference for NodeByteStr {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        match self {
            NodeByteStr::Constant(_) => Err(String::from(
                "TypeInference not supported for NodeByteStr::Constant",
            )),
            NodeByteStr::Type(t) => match workspace.env.get(t) {
                Some(p) => Ok(p.get_instance()),
                None => Err(format!("Type {} is not defined", t)),
            },
        }
    }
}

impl TypeInference for NodeTransitionDefinition {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        unimplemented!()
        // TODO: Implement TypeInference for NodeComponentBody then self.body.get_type(workspace)
    }
}

impl TypeInference for NodeTypeAlternativeClause {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        match self {
            NodeTypeAlternativeClause::ClauseType(name) => {
                match workspace.env.get(&name.to_string()) {
                    Some(t) => Ok(t.get_instance()),
                    None => Err(format!("{:?} is not defined", name)),
                }
            }
            NodeTypeAlternativeClause::ClauseTypeWithArgs(name, args) => {
                unimplemented!()
                /*
                TODO: Deal with type arguments
                match workspace.env.get(&name.to_string()) {
                    Some(t) => {
                        let expected_arg_count = t.arg_types.len();
                        if expected_arg_count != args.len() {
                            return Err(format!("Expected {} arguments, found {}", expected_arg_count, args.len()));
                        }
                        for (expected_type, actual_node) in t.arg_types.iter().zip(args.iter()) {
                            let actual_type = actual_node.get_type(workspace)?;
                            if actual_type != *expected_type {
                                return Err(format!("Expected type {:?}, found {:?}", expected_type, actual_type));
                            }
                        }
                        Ok(t.get_instance())
                    },
                    None => Err(format!("{:?} is not defined", name.to_string())),
                }
                */
            }
        }
    }
}

impl TypeInference for NodeTypeMapValueArguments {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        match self {
            NodeTypeMapValueArguments::EnclosedTypeMapValue(node) => node.get_type(workspace),
            NodeTypeMapValueArguments::GenericMapValueArgument(identifier) => {
                identifier.get_type(workspace)
            }
            NodeTypeMapValueArguments::MapKeyValueType(key, value) => {
                let key_type = key.get_type(workspace)?;
                let value_type = value.get_type(workspace)?;
                Ok(TypeAnnotation::TemplateType(TemplateType {
                    name: format!("Map[{}, {}]", key_type.to_string(), value_type.to_string()),
                    symbol: "Map".to_string(),
                }))
            }
        }
    }
}

impl TypeInference for NodeTypeMapValueAllowingTypeArguments {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        match self {
            NodeTypeMapValueAllowingTypeArguments::TypeMapValueNoArgs(node) => {
                node.get_type(workspace)
            }
            NodeTypeMapValueAllowingTypeArguments::TypeMapValueWithArgs(identifier, args) => {
                let id_type = identifier.get_type(workspace)?;
                match id_type {
                    TypeAnnotation::TemplateType(template_type) => {
                        // Perform type inference for arguments
                        let arg_types = args
                            .iter()
                            .map(|arg| arg.get_type(workspace))
                            .collect::<Result<Vec<TypeAnnotation>, String>>()
                            .map_err(|err| format!("Error in type arguments: {}", err))?;
                        // Do something with arg_types and template_type here.
                        // For now, just return the template_type Annotation
                        // TODO:
                        Ok(TypeAnnotation::TemplateType(template_type)) // TODO: Placeholder until the new type is derived
                    }
                    _ => Err(format!(
                        "Node {:?} expected to be a TemplateType but got {:?}",
                        identifier, id_type
                    )),
                }
            }
        }
    }
}

impl TypeInference for NodeImportedName {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        match self {
            NodeImportedName::RegularImport(type_name_identifier) => {
                type_name_identifier.get_type(workspace)
            }
            NodeImportedName::AliasedImport(original_type_name, _alias_type_name) => {
                original_type_name.get_type(workspace)
            }
        }
    }
}

impl TypeInference for NodeImportDeclarations {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        // Since import declarations do not have an explicit type, we return an error.
        Err(String::from(
            "Import declarations do not have an associated type",
        ))
    }
}

impl TypeInference for NodeMetaIdentifier {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        match self {
            NodeMetaIdentifier::MetaName(name) => {
                if let NodeTypeNameIdentifier::CustomType(n) = name {
                    if let Some(t) = workspace.env.get(n) {
                        Ok(t.get_instance())
                    } else {
                        Err(format!("type {} not found in environment", n))
                    }
                } else {
                    Err(format!("expected CustomType, got {:?}", name))
                }
            }
            NodeMetaIdentifier::MetaNameInNamespace(namespace, name) => {
                let full_name = format!("{}.{}", namespace, name);
                if let Some(t) = workspace.env.get(&full_name) {
                    Ok(t.get_instance())
                } else {
                    Err(format!(
                        "type {}.{} not found in environment",
                        namespace, name
                    ))
                }
            }
            NodeMetaIdentifier::MetaNameInHexspace(hexspace, name) => {
                let full_name = format!("{}::{}", hexspace, name);
                if let Some(t) = workspace.env.get(&full_name) {
                    Ok(t.get_instance())
                } else {
                    Err(format!(
                        "type in hexspace {}.{} not found in environment",
                        hexspace, name
                    ))
                }
            }
            NodeMetaIdentifier::ByteString => {
                if let Some(t) = workspace.env.get("ByteString") {
                    Ok(t.get_instance())
                } else {
                    Err(format!("type ByteString not found in environment"))
                }
            }
        }
    }
}

impl TypeInference for NodeBuiltinArguments {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        if let Some(first_arg) = self.arguments.first() {
            first_arg.get_type(workspace)
        } else {
            Err(String::from("No arguments in NodeBuiltinArguments"))
        }
    }
}

impl TypeInference for NodeTypeMapKey {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        match self {
            NodeTypeMapKey::GenericMapKey(node_meta_identifier) => {
                node_meta_identifier.get_type(workspace)
            }
            NodeTypeMapKey::EnclosedGenericId(node_meta_identifier) => {
                node_meta_identifier.get_type(workspace)
            }
            NodeTypeMapKey::EnclosedAddressMapKeyType(node_address_type) => {
                node_address_type.get_type(workspace)
            }
            NodeTypeMapKey::AddressMapKeyType(node_address_type) => {
                node_address_type.get_type(workspace)
            }
        }
    }
}

impl TypeInference for NodeTypeMapValue {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        match self {
            NodeTypeMapValue::MapValueCustomType(meta_id) => meta_id.get_type(workspace),
            NodeTypeMapValue::MapKeyValue(node_type_map_entry) => {
                node_type_map_entry.get_type(workspace)
            }
            NodeTypeMapValue::MapValueParanthesizedType(node_type_map_value) => {
                node_type_map_value.get_type(workspace)
            }
            NodeTypeMapValue::MapValueAddressType(node_address_type) => {
                node_address_type.get_type(workspace)
            }
        }
    }
}

impl TypeInference for NodeTypeMapEntry {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        let key_type = self.key.get_type(workspace)?;
        let value_type = self.value.get_type(workspace)?;
        // Handling type inference error
        if key_type != value_type {
            return Err(format!(
                "Type Error: expected key and value to have the same type but got {:?} and {:?}",
                key_type, value_type
            ));
        }

        unimplemented!()
        // TODO: Implement MapType Ok(TypeAnnotation::MapType(key_type))
    }
}

impl TypeInference for NodeTypeArgument {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        match self {
            NodeTypeArgument::EnclosedTypeArgument(node_type) => node_type.get_type(workspace),
            NodeTypeArgument::GenericTypeArgument(meta_id) => meta_id.get_type(workspace),
            NodeTypeArgument::TemplateTypeArgument(template) => {
                if let Some(ty) = workspace.env.get(template) {
                    Ok(ty.get_instance())
                } else {
                    Err(format!(
                        "Template type `{}` not found in the environment",
                        template
                    ))
                }
            }
            NodeTypeArgument::AddressTypeArgument(node_addr_type) => {
                node_addr_type.get_type(workspace)
            }
            NodeTypeArgument::MapTypeArgument(map_key, map_value) => {
                let key_type = map_key.get_type(workspace)?;
                let value_type = map_value.get_type(workspace)?;
                Ok(TypeAnnotation::FunType(FunType {
                    template_types: vec![],
                    arg_types: vec![key_type, value_type],
                    to_type: Box::new(TypeAnnotation::TemplateType(TemplateType {
                        name: "Map".to_string(),
                        symbol: "Map".to_string(),
                    })),
                    symbol: "Map".to_string(),
                }))
            }
        }
    }
}

impl TypeInference for NodeScillaType {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        match self {
            NodeScillaType::GenericTypeWithArgs(meta_identifier, _type_args) => {
                // TODO: Check the how to handle meta_identifier and type_args to determine type
                unimplemented!()
            }
            NodeScillaType::MapType(map_key, map_value) => {
                let key_type = map_key.get_type(workspace)?;
                let value_type = map_value.get_type(workspace)?;
                Ok(TypeAnnotation::FunType(FunType {
                    template_types: vec![],
                    arg_types: vec![key_type],
                    to_type: Box::new(value_type),
                    symbol: "Map".to_string(),
                }))
            }
            NodeScillaType::FunctionType(arg_type, return_type) => {
                let arg_type = arg_type.get_type(workspace)?;
                let return_type = return_type.get_type(workspace)?;
                Ok(TypeAnnotation::FunType(FunType {
                    template_types: vec![],
                    arg_types: vec![arg_type],
                    to_type: Box::new(return_type),
                    symbol: "->".to_string(),
                }))
            }
            NodeScillaType::EnclosedType(inner_type) => inner_type.get_type(workspace),
            NodeScillaType::ScillaAddresseType(address_type) => address_type.get_type(workspace),
            NodeScillaType::PolyFunctionType(_param, return_type) => {
                return_type.get_type(workspace)
            }
            NodeScillaType::TypeVarType(name) => {
                // TODO: Check the how to handle this case to determine type
                unimplemented!()
            }
        }
    }
}

impl TypeInference for NodeAddressTypeField {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        self.type_name.get_type(workspace)
    }
}

impl TypeInference for NodeAddressType {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        match self.type_name.as_str() {
            "" => Err(String::from("No type name in NodeAddressType")),
            _ => Ok(workspace
                .env
                .get(self.type_name.as_str())
                .unwrap()
                .get_instance()),
        }
    }
}

impl TypeInference for NodeFullExpression {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        match self {
            NodeFullExpression::LocalVariableDeclaration {
                identifier_name,
                expression,
                containing_expression,
                ..
            } => {
                // Assuming the expression has a type annotation
                let expr_type = expression.get_type(workspace)?;
                // Adding the expression type to the environment
                workspace
                    .env
                    .insert(identifier_name.clone(), Box::new(expr_type.clone()));
                // Returning the containing expression type
                containing_expression.get_type(workspace)
            }
            NodeFullExpression::FunctionDeclaration {
                identier_value, // TODO: Miss-spelled - search and replace
                type_annotation,
                ..
            } => {
                unimplemented!()
                /*
                TODO: Does not compile
                // Adding the type of the function to environment
                if let TypeAnnotation::FunType(fun_type) = &type_annotation.type_name {
                    workspace.env.insert(identifier_value.clone(), Box::new(fun_type.get_instance()));
                }
                // Returning the type annotation of the declared function
                if let Some(fun_type) = workspace.env.get(identifier_value) {
                    Ok(fun_type.get_instance())
                } else {
                    Err(String::from("Function type not found in environment"))
                }
                */
            }
            NodeFullExpression::FunctionCall { function_name, .. } => {
                // Use identifier to_string() to print the function name as string
                let function_name_str = function_name.to_string();
                // If function is in the environment, return the function type
                if let Some(function_type) = workspace.env.get(&function_name_str) {
                    Ok(function_type.get_instance())
                } else {
                    Err(String::from("Function not found in environment"))
                }
            }
            _ => Err(String::from(
                "No type inference available for this node variant",
            )),
        }
    }
}

impl TypeInference for NodeMessageEntry {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        match self {
            NodeMessageEntry::MessageLiteral(_, value_literal) => value_literal.get_type(workspace),
            NodeMessageEntry::MessageVariable(_, var2_id) => var2_id.get_type(workspace),
        }
    }
}

impl TypeInference for NodePatternMatchExpressionClause {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        // Get the type of the expression inside the pattern match clause
        self.expression.get_type(workspace)
    }
}

impl TypeInference for NodeAtomicExpression {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        match self {
            NodeAtomicExpression::AtomicSid(node) => node.get_type(workspace),
            NodeAtomicExpression::AtomicLit(node) => node.get_type(workspace),
        }
    }
}

impl TypeInference for NodeContractTypeArguments {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        if let Some(first_arg) = self.type_arguments.first() {
            first_arg.get_type(workspace)
        } else {
            Err(String::from(
                "No type arguments in NodeContractTypeArguments",
            ))
        }
    }
}

impl TypeInference for NodeValueLiteral {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        match self {
            NodeValueLiteral::LiteralInt(ty, _value) => ty.get_type(workspace),
            NodeValueLiteral::LiteralHex(_value) => Ok(TypeAnnotation::BuiltinType(BuiltinType {
                name: "Uint128".to_string(),
                symbol: "Uint128".to_string(),
            })),
            NodeValueLiteral::LiteralString(_value) => {
                Ok(TypeAnnotation::BuiltinType(BuiltinType {
                    name: "String".to_string(),
                    symbol: "String".to_string(),
                }))
            }
            NodeValueLiteral::LiteralEmptyMap(key_ty, value_ty) => {
                let key_ty_annotation = key_ty.get_type(workspace)?;
                let value_ty_annotation = value_ty.get_type(workspace)?;
                let map_symbol = format!(
                    "Map ({} : {})",
                    key_ty_annotation.to_string(),
                    value_ty_annotation.to_string()
                );
                Ok(TypeAnnotation::BuiltinType(BuiltinType {
                    name: map_symbol.clone(),
                    symbol: map_symbol,
                }))
            }
        }
    }
}

impl TypeInference for NodeMapAccess {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        Err(format!(
                "Internal error: attempted to get type of MapAccess {:?}, but this object does not have a type in its own right.",
                self))
    }
}

impl TypeInference for NodePattern {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        match self {
            NodePattern::Wildcard => {
                Err("Wildcard pattern _ does not have a specific type".to_string())
            }
            NodePattern::Binder(b) => match workspace.env.get(b) {
                Some(t) => Ok(t.get_instance()),
                None => Err(format!("{} is not defined", b)),
            },
            NodePattern::Constructor(meta_id, arg_patterns) => {
                match workspace.env.get(&meta_id.to_string()) {
                    Some(t) => Ok(t.get_instance()),
                    None => Err(format!("{:?} is not defined", meta_id.to_string())),
                }
            }
        }
    }
}

impl TypeInference for NodeArgumentPattern {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        match self {
            NodeArgumentPattern::WildcardArgument => {
                Err("Type inference for WildcardArgument not supported".to_string())
            }
            NodeArgumentPattern::BinderArgument(binder) => match workspace.env.get(binder) {
                Some(t) => Ok(t.get_instance()),
                None => Err(format!("{} is not defined", binder)),
            },
            NodeArgumentPattern::ConstructorArgument(constructor) => {
                constructor.get_type(workspace)
            }
            NodeArgumentPattern::PatternArgument(pattern) => pattern.get_type(workspace),
        }
    }
}

impl TypeInference for NodePatternMatchClause {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        // Since a pattern match clause doesn't have a concrete type, we return the type of its statement block
        if let Some(ref statement_block) = self.statement_block {
            statement_block.get_type(workspace)
        } else {
            Err(String::from(
                "Empty statement block in pattern match clause",
            ))
        }
    }
}

// Add this implementation to your `/src/type_inference.rs`
impl TypeInference for NodeBlockchainFetchArguments {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        for argument in &self.arguments {
            argument.get_type(workspace)?; // Return an error if argument type can't be inferred
        }
        // Assuming the type of NodeBlockchainFetchArguments is always BuiltinType, replace with the correct type if necessary
        Ok(TypeAnnotation::BuiltinType(BuiltinType {
            name: "NodeBlockchainFetchArguments".to_string(),
            symbol: "NodeBlockchainFetchArguments".to_string(),
        }))
    }
}

fn resolve_map_access_type(
    workspace: &mut Workspace,
    right_hand_type: TypeAnnotation,
    left_hand_side: &String,
    keys: &Vec<NodeMapAccess>,
) -> Result<TypeAnnotation, String> {
    let mut resolved_type = right_hand_type;

    let mut map_type = match &resolved_type {
        TypeAnnotation::MapType(value) => value.clone(),
        _ => return Err(format!("Expected Map type, but found {:?}", &resolved_type)),
    };

    for (i, key) in keys.iter().enumerate() {
        // TODO: Check key type against supplied key type
        // TODO: Use type_of_key(key.identifier_name.to_string()); in the event of multimap
        resolved_type = match &map_type.value_type {
            Some(v) => Ok(v.get_instance()),
            None => Err(format!("Map '{:?}; does not have a value.", map_type)),
        }?;
        if i != keys.len() - 1 {
            map_type = match &resolved_type {
                TypeAnnotation::MapType(value) => value.clone(),
                _ => {
                    return Err(format!(
                        "Expected Map type '{:?}', but found {:?}",
                        &map_type, &resolved_type
                    ))
                }
            };
        }
    }

    if workspace.env.contains_key(left_hand_side) {
        Err(format!("'{}' already defined", left_hand_side))
    } else {
        // TODO: Qualify with namespace?
        workspace
            .env
            .insert(left_hand_side.to_string(), Box::new(resolved_type.clone()));
        Ok(resolved_type)
    }
}

impl TypeInference for NodeStatement {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        use NodeStatement::*;
        match self {
            MapUpdate {
                left_hand_side,
                keys,
                right_hand_side,
            } => {
                let resolved_type = right_hand_side.get_type(workspace)?;
                resolve_map_access_type(workspace, resolved_type, left_hand_side, keys)
            }
            MapGetExists {
                left_hand_side,
                keys,
                right_hand_side,
            }
            | MapGet {
                left_hand_side,
                keys,
                right_hand_side,
            } => {
                let resolved_type = match workspace.env.get(right_hand_side) {
                    Some(t) => Ok(t.get_instance()),
                    None => Err(format!("'{}' is not defined", right_hand_side)),
                }?;

                resolve_map_access_type(workspace, resolved_type, left_hand_side, keys)
            }
            Accept | Send { .. } | CreateEvnt { .. } | Throw { .. } =>
            // TODO: Consider whether it is needed to visit the children
            {
                Ok(TypeAnnotation::Void)
            }

            // TODO: Implement those below
            MapUpdateDelete { .. } => {
                unimplemented!()
            }

            Load { .. } | Store { .. } => {
                unimplemented!()
            }
            RemoteFetch(_inner) => {
                Err("Type inference for RemoteFetch is not supported".to_string())
            }
            Bind {
                right_hand_side, ..
            } => right_hand_side.get_type(workspace),
            ReadFromBC { type_name, .. } => type_name.get_type(workspace),

            MatchStmt { variable, .. } => variable.get_type(workspace),
            CallProc { .. } => Err("Type inference for CallProc is not supported".to_string()),
            Iterate {
                identifier_name, ..
            } => identifier_name.get_type(workspace),
        }
    }
}

impl TypeInference for NodeRemoteFetchStatement {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        match self {
            NodeRemoteFetchStatement::ReadStateMutable(_, _, var_id)
            | NodeRemoteFetchStatement::ReadStateMutableCastAddress(_, var_id, _) => {
                var_id.get_type(workspace)
            }
            _ => Err(format!(
                "Type inference for NodeRemoteFetchStatement {:?} is not supported.",
                self
            )),
        }
    }
}

impl TypeInference for NodeComponentId {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        match self {
            NodeComponentId::WithTypeLikeName(type_name_identifier) => {
                type_name_identifier.get_type(workspace)
            }
            NodeComponentId::WithRegularId(id_string) => match workspace.env.get(id_string) {
                Some(t) => Ok(t.get_instance()),
                None => Err(format!("{} is not defined", id_string)),
            },
        }
    }
}

impl TypeInference for NodeComponentParameters {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        if let Some(ref type_annotation) = self.type_annotation {
            Ok(type_annotation.clone())
        } else {
            Err(
                "Unable to infer type for NodeComponentParameters without type annotation"
                    .to_string(),
            )
        }
    }
}

impl TypeInference for NodeParameterPair {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        // Delegating the type inference process to the identifier_with_type
        self.identifier_with_type.get_type(workspace)
    }
}

impl TypeInference for NodeComponentBody {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        match &self.type_annotation {
            Some(type_ann) => Ok(type_ann.clone()),
            None => Err("Type annotation not found for NodeComponentBody".to_string()),
        }
    }
}

impl TypeInference for NodeStatementBlock {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        let mut last_type_annotation = None;
        for statement in &self.statements {
            last_type_annotation = Some(statement.get_type(workspace)?);
        }
        match last_type_annotation {
            Some(annotation) => Ok(annotation),
            None => Err("No statements found in the block".to_string()),
        }
    }
}

impl TypeInference for NodeTypedIdentifier {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        match &self.type_annotation {
            Some(ty) => Ok(ty.clone()),
            None => Err(format!(
                "Type annotation is missing for {}",
                self.identifier_name
            )),
        }
    }
}

impl TypeInference for NodeTypeAnnotation {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        match &self.type_annotation {
            Some(ty) => Ok(ty.clone()),
            None => self.type_name.get_type(workspace),
        }
    }
}

impl TypeInference for NodeProgram {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        // Type inference for NodeProgram is not very meaningful
        // since it contains many elements that may or may not
        // influence the type. However, for consistency reasons,
        // we can get the type of the contract definition
        // and return it as the type of the NodeProgram.
        self.contract_definition.get_type(workspace)
    }
}

impl TypeInference for NodeLibraryDefinition {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        match &self.type_annotation {
            Some(t) => Ok(t.clone()),
            None => Err(format!("Type not defined for library {:?}", self.name)),
        }
    }
}

impl TypeInference for NodeLibrarySingleDefinition {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        match self {
            NodeLibrarySingleDefinition::LetDefinition {
                variable_name,
                type_annotation: _,
                expression,
            } => expression.get_type(workspace),
            NodeLibrarySingleDefinition::TypeDefinition(type_name, opt_type_alternatives) => {
                let constructed_type = type_name.get_type(workspace)?;
                if let Some(type_alternatives) = opt_type_alternatives {
                    let types = type_alternatives
                        .iter()
                        .map(|ta| ta.get_type(workspace))
                        .collect::<Result<Vec<_>, _>>()?;
                    Ok(TypeAnnotation::UnionType(UnionType {
                        name: type_name.to_string(),
                        types,
                        symbol: constructed_type.to_string(),
                    }))
                } else {
                    Ok(constructed_type)
                }
            }
        }
    }
}

impl TypeInference for NodeContractDefinition {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        match self.type_annotation {
            Some(ref annotation) => Ok(annotation.clone()),
            None => Err(format!(
                "Type annotation not found for contract definition '{:?}'",
                self.contract_name
            )),
        }
    }
}

impl TypeInference for NodeContractField {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        match &self.type_annotation {
            Some(t) => Ok(t.clone()),
            None => self.typed_identifier.get_type(workspace),
        }
    }
}

impl TypeInference for NodeWithConstraint {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        self.expression.get_type(workspace)
    }
}

impl TypeInference for NodeComponentDefinition {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        match self {
            NodeComponentDefinition::TransitionComponent(transition_def) => {
                transition_def.get_type(workspace)
            }
            NodeComponentDefinition::ProcedureComponent(procedure_def) => {
                procedure_def.get_type(workspace)
            }
        }
    }
}

impl TypeInference for NodeProcedureDefinition {
    fn get_type(&self, workspace: &mut Workspace) -> Result<TypeAnnotation, String> {
        let mut local_workspace = workspace.clone();
        for param_pair in &self.parameters.parameters {
            let identifier = &param_pair.identifier_with_type.identifier_name;
            let ty = &param_pair.identifier_with_type.type_annotation;
            if let Some(type_annotation) = ty {
                local_workspace
                    .env
                    .insert(identifier.to_string(), Box::new(type_annotation.clone()));
            } else {
                return Err(format!(
                    "Type annotation not found for parameter '{}'",
                    identifier
                ));
            }
        }
        match self.body.get_type(&mut local_workspace) {
            Ok(_) => {
                let return_type = TypeAnnotation::FunType(FunType {
                    template_types: vec![],
                    arg_types: self
                        .parameters
                        .parameters
                        .iter()
                        .filter_map(|p| p.identifier_with_type.type_annotation.clone())
                        .collect(),
                    to_type: Box::new(TypeAnnotation::BuiltinType(BuiltinType {
                        name: "Procedure".to_string(),
                        symbol: "Procedure".to_string(),
                    })),
                    symbol: "".to_string(),
                });
                Ok(return_type)
            }
            Err(err) => Err(err),
        }
    }
}
