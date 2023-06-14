use crate::ast::*;
use crate::type_classes::*;
use std::collections::HashMap;

// Placeholder functions for the top-level type checking and inference
pub trait TypeInference {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String>;
}

struct Workspace {
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

impl TypeInference for NodeVariableIdentifier {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        match self {
            NodeVariableIdentifier::VariableName(name) => match env.get(name) {
                Some(t) => Ok(t.get_instance()),
                None => Err(format!("{} is not defined", name)),
            },
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
                match env.get(&name) {
                    Some(t) => {
                        let qualified_name = format!("{}.{}", name, var_name);
                        match env.get(&qualified_name) {
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
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
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
            NodeTypeNameIdentifier::CustomType(custom_type) => match env.get(custom_type) {
                Some(t) => Ok(t.get_instance()),
                None => Err(format!("{} is not defined", custom_type)),
            },
        }
    }
}

impl TypeInference for NodeByteStr {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        match self {
            NodeByteStr::Constant(_) => Err(String::from(
                "TypeInference not supported for NodeByteStr::Constant",
            )),
            NodeByteStr::Type(t) => match env.get(t) {
                Some(p) => Ok(p.get_instance()),
                None => Err(format!("Type {} is not defined", t)),
            },
        }
    }
}

impl TypeInference for NodeTransitionDefinition {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        unimplemented!()
        // TODO: Implement TypeInference for NodeComponentBody then self.body.get_type(env)
    }
}

impl TypeInference for NodeTypeAlternativeClause {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        match self {
            NodeTypeAlternativeClause::ClauseType(name) => match env.get(&name.to_string()) {
                Some(t) => Ok(t.get_instance()),
                None => Err(format!("{:?} is not defined", name)),
            },
            NodeTypeAlternativeClause::ClauseTypeWithArgs(name, args) => {
                unimplemented!()
                /*
                TODO: Deal with type arguments
                match env.get(&name.to_string()) {
                    Some(t) => {
                        let expected_arg_count = t.arg_types.len();
                        if expected_arg_count != args.len() {
                            return Err(format!("Expected {} arguments, found {}", expected_arg_count, args.len()));
                        }
                        for (expected_type, actual_node) in t.arg_types.iter().zip(args.iter()) {
                            let actual_type = actual_node.get_type(env)?;
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
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        match self {
            NodeTypeMapValueArguments::EnclosedTypeMapValue(node) => node.get_type(env),
            NodeTypeMapValueArguments::GenericMapValueArgument(identifier) => {
                identifier.get_type(env)
            }
            NodeTypeMapValueArguments::MapKeyValueType(key, value) => {
                let key_type = key.get_type(env)?;
                let value_type = value.get_type(env)?;
                Ok(TypeAnnotation::TemplateType(TemplateType {
                    name: format!("Map[{}, {}]", key_type.to_string(), value_type.to_string()),
                    symbol: "Map".to_string(),
                }))
            }
        }
    }
}

impl TypeInference for NodeTypeMapValueAllowingTypeArguments {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        match self {
            NodeTypeMapValueAllowingTypeArguments::TypeMapValueNoArgs(node) => node.get_type(env),
            NodeTypeMapValueAllowingTypeArguments::TypeMapValueWithArgs(identifier, args) => {
                let id_type = identifier.get_type(env)?;
                match id_type {
                    TypeAnnotation::TemplateType(template_type) => {
                        // Perform type inference for arguments
                        let arg_types = args
                            .iter()
                            .map(|arg| arg.get_type(env))
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
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        match self {
            NodeImportedName::RegularImport(type_name_identifier) => {
                type_name_identifier.get_type(env)
            }
            NodeImportedName::AliasedImport(original_type_name, _alias_type_name) => {
                original_type_name.get_type(env)
            }
        }
    }
}

impl TypeInference for NodeImportDeclarations {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        // Since import declarations do not have an explicit type, we return an error.
        Err(String::from(
            "Import declarations do not have an associated type",
        ))
    }
}

impl TypeInference for NodeMetaIdentifier {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        match self {
            NodeMetaIdentifier::MetaName(name) => {
                if let NodeTypeNameIdentifier::CustomType(n) = name {
                    if let Some(t) = env.get(n) {
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
                if let Some(t) = env.get(&full_name) {
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
                if let Some(t) = env.get(&full_name) {
                    Ok(t.get_instance())
                } else {
                    Err(format!(
                        "type in hexspace {}.{} not found in environment",
                        hexspace, name
                    ))
                }
            }
            NodeMetaIdentifier::ByteString => {
                if let Some(t) = env.get("ByteString") {
                    Ok(t.get_instance())
                } else {
                    Err(format!("type ByteString not found in environment"))
                }
            }
        }
    }
}

impl TypeInference for NodeBuiltinArguments {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        if let Some(first_arg) = self.arguments.first() {
            first_arg.get_type(env)
        } else {
            Err(String::from("No arguments in NodeBuiltinArguments"))
        }
    }
}

impl TypeInference for NodeTypeMapKey {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        match self {
            NodeTypeMapKey::GenericMapKey(node_meta_identifier) => {
                node_meta_identifier.get_type(env)
            }
            NodeTypeMapKey::EnclosedGenericId(node_meta_identifier) => {
                node_meta_identifier.get_type(env)
            }
            NodeTypeMapKey::EnclosedAddressMapKeyType(node_address_type) => {
                node_address_type.get_type(env)
            }
            NodeTypeMapKey::AddressMapKeyType(node_address_type) => node_address_type.get_type(env),
        }
    }
}

impl TypeInference for NodeTypeMapValue {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        match self {
            NodeTypeMapValue::MapValueCustomType(meta_id) => meta_id.get_type(env),
            NodeTypeMapValue::MapKeyValue(node_type_map_entry) => node_type_map_entry.get_type(env),
            NodeTypeMapValue::MapValueParanthesizedType(node_type_map_value) => {
                node_type_map_value.get_type(env)
            }
            NodeTypeMapValue::MapValueAddressType(node_address_type) => {
                node_address_type.get_type(env)
            }
        }
    }
}

impl TypeInference for NodeTypeMapEntry {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        let key_type = self.key.get_type(env)?;
        let value_type = self.value.get_type(env)?;
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
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        match self {
            NodeTypeArgument::EnclosedTypeArgument(node_type) => node_type.get_type(env),
            NodeTypeArgument::GenericTypeArgument(meta_id) => meta_id.get_type(env),
            NodeTypeArgument::TemplateTypeArgument(template) => {
                if let Some(ty) = env.get(template) {
                    Ok(ty.get_instance())
                } else {
                    Err(format!(
                        "Template type `{}` not found in the environment",
                        template
                    ))
                }
            }
            NodeTypeArgument::AddressTypeArgument(node_addr_type) => node_addr_type.get_type(env),
            NodeTypeArgument::MapTypeArgument(map_key, map_value) => {
                let key_type = map_key.get_type(env)?;
                let value_type = map_value.get_type(env)?;
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
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        match self {
            NodeScillaType::GenericTypeWithArgs(meta_identifier, _type_args) => {
                // TODO: Check the how to handle meta_identifier and type_args to determine type
                unimplemented!()
            }
            NodeScillaType::MapType(map_key, map_value) => {
                let key_type = map_key.get_type(env)?;
                let value_type = map_value.get_type(env)?;
                Ok(TypeAnnotation::FunType(FunType {
                    template_types: vec![],
                    arg_types: vec![key_type],
                    to_type: Box::new(value_type),
                    symbol: "Map".to_string(),
                }))
            }
            NodeScillaType::FunctionType(arg_type, return_type) => {
                let arg_type = arg_type.get_type(env)?;
                let return_type = return_type.get_type(env)?;
                Ok(TypeAnnotation::FunType(FunType {
                    template_types: vec![],
                    arg_types: vec![arg_type],
                    to_type: Box::new(return_type),
                    symbol: "->".to_string(),
                }))
            }
            NodeScillaType::EnclosedType(inner_type) => inner_type.get_type(env),
            NodeScillaType::ScillaAddresseType(address_type) => address_type.get_type(env),
            NodeScillaType::PolyFunctionType(_param, return_type) => return_type.get_type(env),
            NodeScillaType::TypeVarType(name) => {
                // TODO: Check the how to handle this case to determine type
                unimplemented!()
            }
        }
    }
}

impl TypeInference for NodeAddressTypeField {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        self.type_name.get_type(env)
    }
}

impl TypeInference for NodeAddressType {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        match self.type_name.as_str() {
            "" => Err(String::from("No type name in NodeAddressType")),
            _ => Ok(env.get(self.type_name.as_str()).unwrap().get_instance()),
        }
    }
}

impl TypeInference for NodeFullExpression {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        match self {
            NodeFullExpression::LocalVariableDeclaration {
                identifier_name,
                expression,
                containing_expression,
                ..
            } => {
                // Assuming the expression has a type annotation
                let expr_type = expression.get_type(env)?;
                // Adding the expression type to the environment
                env.insert(identifier_name.clone(), Box::new(expr_type.clone()));
                // Returning the containing expression type
                containing_expression.get_type(env)
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
                    env.insert(identifier_value.clone(), Box::new(fun_type.get_instance()));
                }
                // Returning the type annotation of the declared function
                if let Some(fun_type) = env.get(identifier_value) {
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
                if let Some(function_type) = env.get(&function_name_str) {
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
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        match self {
            NodeMessageEntry::MessageLiteral(_, value_literal) => value_literal.get_type(env),
            NodeMessageEntry::MessageVariable(_, var2_id) => var2_id.get_type(env),
        }
    }
}

impl TypeInference for NodePatternMatchExpressionClause {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        // Get the type of the expression inside the pattern match clause
        self.expression.get_type(env)
    }
}

impl TypeInference for NodeAtomicExpression {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        match self {
            NodeAtomicExpression::AtomicSid(node) => node.get_type(env),
            NodeAtomicExpression::AtomicLit(node) => node.get_type(env),
        }
    }
}

impl TypeInference for NodeContractTypeArguments {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        if let Some(first_arg) = self.type_arguments.first() {
            first_arg.get_type(env)
        } else {
            Err(String::from(
                "No type arguments in NodeContractTypeArguments",
            ))
        }
    }
}

impl TypeInference for NodeValueLiteral {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        match self {
            NodeValueLiteral::LiteralInt(ty, _value) => ty.get_type(env),
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
                let key_ty_annotation = key_ty.get_type(env)?;
                let value_ty_annotation = value_ty.get_type(env)?;
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
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        let map_identifier_type = self.identifier_name.get_type(env)?;
        match map_identifier_type {
            TypeAnnotation::BuiltinType(BuiltinType {
                ref name,
                ref symbol,
            }) if symbol == "Map" => {
                // parse the name which should contain a tuple (key type, value type) for maps.
                let value_type = name // gets &(key_type, value_type)
                    .trim_start_matches('(') // Remove '(' , get key_type, value_type)
                    .trim_end_matches(')') // Remove ')' , get key_type, value_type
                    .split(',') // Split by comma, get [key_type, value_type]
                    .map(|s| s.trim()) // Trim the spaces, get [key_type, value_type]
                    .collect::<Vec<&str>>()
                    .get(1) // Fetch the second element value_type
                    .ok_or_else(|| String::from("Error while parsing map type."))?; // Return error if unable to fetch the value type.
                                                                                    // Assuming that `value_type` is a perfectly matchable string, we fetch the type out of `env`
                unimplemented!()
                /*
                TODO: this is not compiling
                env.get(value_type)
                   .ok_or_else(|| format!("Undefined type {}", value_type))
                   .map(|bt| bt.get_instance())  // Return the value type for map access
                */
            }
            _ => Err(format!(
                "Identifier should be of type Map, but found {:?}",
                map_identifier_type
            )),
        }
    }
}

impl TypeInference for NodePattern {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        match self {
            NodePattern::Wildcard => {
                Err("Wildcard pattern _ does not have a specific type".to_string())
            }
            NodePattern::Binder(b) => match env.get(b) {
                Some(t) => Ok(t.get_instance()),
                None => Err(format!("{} is not defined", b)),
            },
            NodePattern::Constructor(meta_id, arg_patterns) => {
                match env.get(&meta_id.to_string()) {
                    Some(t) => Ok(t.get_instance()),
                    None => Err(format!("{:?} is not defined", meta_id.to_string())),
                }
            }
        }
    }
}

impl TypeInference for NodeArgumentPattern {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        match self {
            NodeArgumentPattern::WildcardArgument => {
                Err("Type inference for WildcardArgument not supported".to_string())
            }
            NodeArgumentPattern::BinderArgument(binder) => match env.get(binder) {
                Some(t) => Ok(t.get_instance()),
                None => Err(format!("{} is not defined", binder)),
            },
            NodeArgumentPattern::ConstructorArgument(constructor) => constructor.get_type(env),
            NodeArgumentPattern::PatternArgument(pattern) => pattern.get_type(env),
        }
    }
}

impl TypeInference for NodePatternMatchClause {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        // Since a pattern match clause doesn't have a concrete type, we return the type of its statement block
        if let Some(ref statement_block) = self.statement_block {
            statement_block.get_type(env)
        } else {
            Err(String::from(
                "Empty statement block in pattern match clause",
            ))
        }
    }
}

// Add this implementation to your `/src/type_inference.rs`
impl TypeInference for NodeBlockchainFetchArguments {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        for argument in &self.arguments {
            argument.get_type(env)?; // Return an error if argument type can't be inferred
        }
        // Assuming the type of NodeBlockchainFetchArguments is always BuiltinType, replace with the correct type if necessary
        Ok(TypeAnnotation::BuiltinType(BuiltinType {
            name: "NodeBlockchainFetchArguments".to_string(),
            symbol: "NodeBlockchainFetchArguments".to_string(),
        }))
    }
}

impl TypeInference for NodeStatement {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        use NodeStatement::*;
        match self {
            Load {
                right_hand_side, ..
            }
            | Store {
                right_hand_side, ..
            }
            | MapUpdate {
                right_hand_side, ..
            } => right_hand_side.get_type(env),
            RemoteFetch(_inner) => {
                Err("Type inference for RemoteFetch is not supported".to_string())
            }
            Bind {
                right_hand_side, ..
            } => right_hand_side.get_type(env),
            ReadFromBC { type_name, .. } => type_name.get_type(env),
            MapGet { .. }
            | MapGetExists { .. }
            | MapUpdateDelete { .. }
            | Accept
            | Send { .. }
            | CreateEvnt { .. }
            | Throw { .. } => Err("Type inference for this statement is not supported".to_string()),
            MatchStmt { variable, .. } => variable.get_type(env),
            CallProc { .. } => Err("Type inference for CallProc is not supported".to_string()),
            Iterate {
                identifier_name, ..
            } => identifier_name.get_type(env),
        }
    }
}

impl TypeInference for NodeRemoteFetchStatement {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        match self {
            NodeRemoteFetchStatement::ReadStateMutable(_, _, var_id)
            | NodeRemoteFetchStatement::ReadStateMutableCastAddress(_, var_id, _) => {
                var_id.get_type(env)
            }
            _ => Err(format!(
                "Type inference for NodeRemoteFetchStatement {:?} is not supported.",
                self
            )),
        }
    }
}

impl TypeInference for NodeComponentId {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        match self {
            NodeComponentId::WithTypeLikeName(type_name_identifier) => {
                type_name_identifier.get_type(env)
            }
            NodeComponentId::WithRegularId(id_string) => match env.get(id_string) {
                Some(t) => Ok(t.get_instance()),
                None => Err(format!("{} is not defined", id_string)),
            },
        }
    }
}

impl TypeInference for NodeComponentParameters {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
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
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        // Delegating the type inference process to the identifier_with_type
        self.identifier_with_type.get_type(env)
    }
}

impl TypeInference for NodeComponentBody {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        match &self.type_annotation {
            Some(type_ann) => Ok(type_ann.clone()),
            None => Err("Type annotation not found for NodeComponentBody".to_string()),
        }
    }
}

impl TypeInference for NodeStatementBlock {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        let mut last_type_annotation = None;
        for statement in &self.statements {
            last_type_annotation = Some(statement.get_type(env)?);
        }
        match last_type_annotation {
            Some(annotation) => Ok(annotation),
            None => Err("No statements found in the block".to_string()),
        }
    }
}

impl TypeInference for NodeTypedIdentifier {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
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
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        match &self.type_annotation {
            Some(ty) => Ok(ty.clone()),
            None => self.type_name.get_type(env),
        }
    }
}

impl TypeInference for NodeProgram {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        // Type inference for NodeProgram is not very meaningful
        // since it contains many elements that may or may not
        // influence the type. However, for consistency reasons,
        // we can get the type of the contract definition
        // and return it as the type of the NodeProgram.
        self.contract_definition.get_type(env)
    }
}

impl TypeInference for NodeLibraryDefinition {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        match &self.type_annotation {
            Some(t) => Ok(t.clone()),
            None => Err(format!("Type not defined for library {:?}", self.name)),
        }
    }
}

impl TypeInference for NodeLibrarySingleDefinition {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        match self {
            NodeLibrarySingleDefinition::LetDefinition {
                variable_name,
                type_annotation: _,
                expression,
            } => expression.get_type(env),
            NodeLibrarySingleDefinition::TypeDefinition(type_name, opt_type_alternatives) => {
                let constructed_type = type_name.get_type(env)?;
                if let Some(type_alternatives) = opt_type_alternatives {
                    let types = type_alternatives
                        .iter()
                        .map(|ta| ta.get_type(env))
                        .collect::<Result<Vec<_>, _>>()?;
                    Ok(TypeAnnotation::UnionType(UnionType {
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
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
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
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        match &self.type_annotation {
            Some(t) => Ok(t.clone()),
            None => self.typed_identifier.get_type(env),
        }
    }
}

impl TypeInference for NodeWithConstraint {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        self.expression.get_type(env)
    }
}

impl TypeInference for NodeComponentDefinition {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        match self {
            NodeComponentDefinition::TransitionComponent(transition_def) => {
                transition_def.get_type(env)
            }
            NodeComponentDefinition::ProcedureComponent(procedure_def) => {
                procedure_def.get_type(env)
            }
        }
    }
}

impl TypeInference for NodeProcedureDefinition {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        let mut local_env = env
            .iter()
            .map(|(k, v)| (k.clone(), v.as_ref().clone_boxed()))
            .collect::<HashMap<String, Box<dyn BaseType>>>();
        for param_pair in &self.parameters.parameters {
            let identifier = &param_pair.identifier_with_type.identifier_name;
            let ty = &param_pair.identifier_with_type.type_annotation;
            if let Some(type_annotation) = ty {
                local_env.insert(identifier.to_string(), Box::new(type_annotation.clone()));
            } else {
                return Err(format!(
                    "Type annotation not found for parameter '{}'",
                    identifier
                ));
            }
        }
        match self.body.get_type(&mut local_env) {
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
