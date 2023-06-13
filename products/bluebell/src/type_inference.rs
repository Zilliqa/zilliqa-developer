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

// In `src/type_inference.rs`
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
                // Assuming that the namespace implements the BaseType trait itself,
                // otherwise, replace with an appropriate subtype or handle accordingly
                unimplemented!();
                /* TDODO:
                match env.get(&namespace.to_string()) {
                    Some(t) => {
                        let qualified_name = format!("{}.{}", t.to_string(), var_name);
                        match env.get(&qualified_name) {
                            Some(ty) => Ok(ty.get_instance()),
                            None => Err(format!("{} is not defined", qualified_name)),
                        }
                    }
                    None =>Reading the code in Attachment #3 and Attachment #4, could you implement the type inference trait and its corresponding test?
                }
                */
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
        // As the NodeTransitionDefinition itself does not have a type, let's infer the type
        // of its body, since that's a meaningful subexpression.
        unimplemented!();
        // TODO: self.body.get_type(env)
    }
}

impl TypeInference for NodeTypeAlternativeClause {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        unimplemented!();

        /*
        match self {
            NodeTypeAlternativeClause::ClauseType(name) => {
                match env.get(name) {
                    Some(t) => Ok(t.get_instance()),
                    None => Err(format!("{} is not defined", name)),
                }
            }
            NodeTypeAlternativeClause::ClauseTypeWithArgs(name, args) => {
                match env.get(name) {
                    Some(t) => {
                        // Make sure the number of arguments matches.
                        let expected_arg_count = t.arg_types.len();
                        if expected_arg_count != args.len() {
                            return Err(format!("Expected {} arguments, found {}", expected_arg_count, args.len()));
                        }
                        // Check the types of the arguments.
                        for (expected_type, actual_node) in t.arg_types.iter().zip(args.iter()) {
                            let actual_type = actual_node.get_type(env)?;
                            if actual_type != *expected_type {
                                return Err(format!("Expected type {:?}, found {:?}", expected_type, actual_type));
                            }
                        }
                        Ok(t.get_instance())
                    },
                    None => Err(format!("{} is not defined", name)),
                }
            }
        }
        */
    }
}

impl TypeInference for NodeTypeMapValueArguments {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        unimplemented!();

        /*
        TODO:
        match self {
            NodeTypeMapValueArguments::EnclosedTypeMapValue(node) => {
                node.get_type(env)
            }
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
        */
    }
}

// type_inference.rs
impl TypeInference for NodeTypeMapValueAllowingTypeArguments {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        unimplemented!();
        /* TODO:
        match self {
            NodeTypeMapValueAllowingTypeArguments::TypeMapValueNoArgs(node) => {
                node.get_type(env)
            }
            NodeTypeMapValueAllowingTypeArguments::TypeMapValueWithArgs(identifier, args) => {
                let id_type = identifier.get_type(env)?;
                match id_type {
                    TypeAnnotation::TemplateType(template_type) => {
                        // Perform type inference for arguments
                        args.iter()
                            .map(|arg| arg.get_type(env))
                            .collect::<Result<Vec<TypeAnnotation>, String>>()
                            .map_err(|err| format!("Error in type arguments: {}", err))
                    }
                    _ => Err(format!("Node {} expected to be a TemplateType but got {:?}", identifier, id_type)),
                }
            }
        }
        */
    }
}

// Add this to `src/type_inference.rs`
impl TypeInference for NodeImportedName {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        unimplemented!()
        /*
        TODO:
         match self {
            NodeImportedName::RegularImport(type_name_identifier) => type_name_identifier.get_type(env),
            NodeImportedName::AliasedImport(original_type_name, _alias_type_name) => {
                original_type_name.get_type(env)
            }
        }
        */
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

// In `src/type_inference.rs`:
impl TypeInference for NodeMetaIdentifier {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        unimplemented!();
        /*
        TODO:
        match self {
            NodeMetaIdentifier::MetaName(name) => {
                name.get_type(env)
            }
            NodeMetaIdentifier::MetaNameInNamespace(namespace, name) => {
                // If the namespace and the name both have types in the environment,
                // concatenate them with a dot as a symbol and return the result.
                // Otherwise, return an error.
                let namespace_type = namespace.get_type(env)?;
                let name_type = name.get_type(env)?;
                let symbol = format!("{}.{}", namespace.to_string(), name.to_string());
                Ok(TypeAnnotation::TemplateType(TemplateType { name: symbol.clone(), symbol }))
            }
            NodeMetaIdentifier::MetaNameInHexspace(_, name) => {
                name.get_type(env)
            }
            NodeMetaIdentifier::ByteString => {
                Ok(TypeAnnotation::BuiltinType(BuiltinType { name: "ByStr".to_string(), symbol: "ByStr".to_string() }))
            }
        }
        */
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
        unimplemented!();
        /* TODO:
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
            NodeTypeMapKey::AddressMapKeyType(node_address_type) => {
                node_address_type.get_type(env)
            }
        }
        */
    }
}

impl TypeInference for NodeTypeMapValue {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        unimplemented!();
        /*
        TODO:
        match self {
            NodeTypeMapValue::MapValueCustomType(meta_id) => meta_id.get_type(env),
            NodeTypeMapValue::MapKeyValue(node_type_map_entry) => {
                node_type_map_entry.get_type(env)
            }
            NodeTypeMapValue::MapValueParanthesizedType(node_type_map_value) => {
                node_type_map_value.get_type(env)
            }
            NodeTypeMapValue::MapValueAddressType(node_address_type) => {
                node_address_type.get_type(env)
            }
        }
        */
    }
}

impl TypeInference for NodeTypeArgument {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        unimplemented!();
        /* TODO:
        match self {
            NodeTypeArgument::EnclosedTypeArgument(node_type) => node_type.get_type(env),
            NodeTypeArgument::GenericTypeArgument(meta_id) => meta_id.get_type(env),
            NodeTypeArgument::TemplateTypeArgument(template) => {
                if let Some(ty) = env.get(template) {
                    Ok(ty.get_instance())
                } else {
                    Err(format!("Template type `{}` not found in the environment", template))
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
        */
    }
}

impl TypeInference for NodeScillaType {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        unimplemented!()
        /* TODO:
        match self {
            NodeScillaType::GenericTypeWithArgs(meta_identifier, _type_args) => meta_identifier.get_type(env),
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
            NodeScillaType::TypeVarType(name) => env.get(name).cloned().ok_or_else(|| format!("Type variable not found in the environment: {}", name)),
        }
        */
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
        unimplemented!();
        /* TODO:
          match self {
                NodeFullExpression::LocalVariableDeclaration {
                    expression,
                    containing_expression,
                    ..
                } => containing_expression.get_type(env),
                NodeFullExpression::FunctionDeclaration {
                    type_annotation,
                    ..
                } => Ok(type_annotation.get_instance()),
                NodeFullExpression::FunctionCall {
                    function_name,
                    ..
                } => function_name.get_type(env),
                NodeFullExpression::ExpressionAtomic(node) => node.get_type(env),
                NodeFullExpression::ExpressionBuiltin { b, .. } => b.get_type(env),
                NodeFullExpression::Message(entries) => {
                    if let Some(first_entry) = entries.first() {
                        first_entry.get_type(env)
                    } else {
                        Err(String::from("No entries in NodeFullExpression::Message"))
                    }
                }
                NodeFullExpression::Match {
                    match_expression,
                    ..
                } => match_expression.get_type(env),
                NodeFullExpression::ConstructorCall {
                    identifier_name,
                    ..
                } => identifier_name.get_type(env),
                NodeFullExpression::TemplateFunction {
                    expression,
                    ..
                } => expression.get_type(env),
                NodeFullExpression::TApp {
                    identifier_name,
                    ..
                } => identifier_name.get_type(env),
            }
        */
    }
}

impl TypeInference for NodeMessageEntry {
    fn get_type(
        &self,
        env: &mut HashMap<String, Box<dyn BaseType>>,
    ) -> Result<TypeAnnotation, String> {
        match self {
            NodeMessageEntry::MessageLiteral(_, value_literal) => {
                unimplemented!()
                // TODO: value_literal.get_type(env)
            }
            NodeMessageEntry::MessageVariable(_, var2_id) => var2_id.get_type(env),
        }
    }
}

// Type Inference implementation for NodePatternMatchExpressionClause
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
            NodeAtomicExpression::AtomicLit(node) => {
                unimplemented!();
                // TODO: node.get_type(env)
            }
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
            NodeValueLiteral::LiteralInt(ty, _value) => {
                unimplemented!();
                //ty.get_type(env)
            }
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
                unimplemented!();
                /*
                let key_ty_annotation = key_ty.get_type(env)?;
                let value_ty_annotation = value_ty.get_type(env)?;
                let map_symbol = format!("Map ({} : {})", key_ty.to_string_with_indent(0), value_ty.to_string_with_indent(0));
                Ok(TypeAnnotation::BuiltinType(BuiltinType { name: map_symbol.clone(), symbol: map_symbol }))
                */
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
                unimplemented!();
                /*
                // Assuming name contains a tuple (key type, value type) for maps.
                let (_, value_type) = name; // You may need to update this part based on your map representation
                Ok(value_type.clone())      // Return the value type for map access
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
                unimplemented!();
                /*
                match env.get(&meta_id.to_string()) {
                    Some(t) => Ok(t.get_instance()),
                    None => Err(format!("{:?} is not defined", meta_id),
                }
                */
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
            unimplemented!()
            // statement_block.get_type(env)
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
            RemoteFetch(inner) => unimplemented!(), // inner.get_type(env),
            Bind {
                right_hand_side, ..
            } => right_hand_side.get_type(env),
            ReadFromBC { type_name, .. } => unimplemented!(), // type_name.get_type(env),
            MapGet { .. }
            | MapGetExists { .. }
            | MapUpdateDelete { .. }
            | Accept
            | Send { .. }
            | CreateEvnt { .. }
            | Throw { .. } => Err("Type inference for this statement is not supported".to_string()),
            MatchStmt { variable, .. } => variable.get_type(env),
            CallProc { component_id, .. } => unimplemented!(), //component_id.get_type(env),
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
                unimplemented!();
                // type_name_identifier.get_type(env)
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
        unimplemented!()
        // self.identifier_with_type.get_type(env)
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
        // self.contract_definition.get_type(env)
        unimplemented!();
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

pub fn type_of_stmt(
    stmt: &NodeStatement,
    env: &mut HashMap<String, Box<dyn BaseType>>,
) -> Result<TypeAnnotation, String> {
    unimplemented!()
}

pub fn type_of_variable_identifier(
    identifier: &NodeVariableIdentifier,
    env: &mut HashMap<String, Box<dyn BaseType>>,
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
    env: &mut HashMap<String, Box<dyn BaseType>>,
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
    env: &mut HashMap<String, Box<dyn BaseType>>,
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
    env: &mut HashMap<String, Box<dyn BaseType>>,
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
    env: &mut HashMap<String, Box<dyn BaseType>>,
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
    env: &mut HashMap<String, Box<dyn BaseType>>,
) -> Result<TypeAnnotation, String> {
    match stmt {
        NodeStatement::Accept => {
            // Implement logic for the 'Accept' statement
            unimplemented!()
        }
        NodeStatement::Send { identifier_name } => {
            // Implement logic for the 'Send' statement using the variable identifier
            let identifier_type = type_of_variable_identifier(identifier_name, env)?;
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

    unimplemented!();
}

pub fn type_check_transition(
    transition: &NodeTransitionDefinition,
    global_env: &mut HashMap<String, Box<dyn BaseType>>,
) -> Result<(), String> {
    unimplemented!()
}

pub fn type_of(
    expr: &dyn AnyKind,
    env: &mut HashMap<String, Box<dyn BaseType>>,
) -> Result<TypeAnnotation, String> {
    let any = expr.to_any();
    match any {
        NodeAny::NodeByteStr(_) => unimplemented!(),
        NodeAny::NodeTypeNameIdentifier(_) => unimplemented!(),
        NodeAny::NodeImportedName(_) => unimplemented!(),
        NodeAny::NodeImportDeclarations(_) => unimplemented!(),
        NodeAny::NodeMetaIdentifier(_) => unimplemented!(),
        NodeAny::NodeVariableIdentifier(v) => type_of_variable_identifier(v, env),
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
