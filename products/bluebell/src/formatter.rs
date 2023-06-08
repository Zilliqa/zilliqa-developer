use crate::ast::*;
use std::fmt::Write;

pub trait ScillaFormatter {
    fn to_string(&self) -> String {
        self.to_string_with_indent(0)
    }

    fn indent(&self, level: usize) -> String {
        self.to_string_with_indent(level)
    }

    fn to_string_with_indent(&self, level: usize) -> String;
}

fn indentation(level: usize) -> String {
    "  ".repeat(level)
}

impl ScillaFormatter for NodeTypeNameIdentifier {
    // Reviewed and corrected
    fn to_string_with_indent(&self, level: usize) -> String {
        match self {
            NodeTypeNameIdentifier::ByteStringType(byte_str) => {
                format!("{}{}", indentation(level), byte_str.to_string())
            }
            NodeTypeNameIdentifier::EventType => format!("{}Event", indentation(level)),
            NodeTypeNameIdentifier::CustomType(custom_type) => {
                format!("{}{}", indentation(level), custom_type.clone())
            }
        }
    }
}

impl ScillaFormatter for NodeByteStr {
    // Reviewed and corrected
    fn to_string_with_indent(&self, _: usize) -> String {
        match self {
            NodeByteStr::Constant(s) => s.clone(),
            NodeByteStr::Type(t) => t.clone(),
        }
    }
}

impl ScillaFormatter for NodeTransitionDefinition {
    fn to_string_with_indent(&self, level: usize) -> String {
        let mut formatted = String::new();
        // Remove leading indentation, as the formatted string should not include any whitespace before "transition"
        // formatted.push_str(&indentation(level));
        formatted.push_str("transition ");
        formatted.push_str(&self.name.to_string());
        formatted.push_str(&self.parameters.to_string_with_indent(0)); // No indentation for the parameters
        formatted.push_str(" ");
        formatted.push_str(&self.body.to_string_with_indent(level + 1));
        // Add " end" to the end of the formatted string
        formatted.push_str(" end");
        formatted
    }
}
impl ScillaFormatter for NodeTypeAlternativeClause {
    fn to_string_with_indent(&self, level: usize) -> String {
        match self {
            NodeTypeAlternativeClause::ClauseType(name) => {
                format!("{}{}", indentation(level), name.indent(level))
            }
            NodeTypeAlternativeClause::ClauseTypeWithArgs(name, args) => {
                let indented_args = args
                    .iter()
                    .map(|arg| arg.to_string_with_indent(level + 1))
                    .collect::<Vec<_>>()
                    .join(", ");

                format!(
                    "{}{}({})",
                    indentation(level),
                    name.indent(level),
                    indented_args
                )
            }
        }
    }
}

impl ScillaFormatter for NodeTypeMapValueArguments {
    fn to_string_with_indent(&self, level: usize) -> String {
        match self {
            NodeTypeMapValueArguments::EnclosedTypeMapValue(node) => {
                node.to_string_with_indent(level)
            }
            NodeTypeMapValueArguments::GenericMapValueArgument(identifier) => {
                identifier.to_string_with_indent(level)
            }
            NodeTypeMapValueArguments::MapKeyValueType(key, value) => {
                let key_str = key.to_string_with_indent(level);
                let value_str = value.to_string_with_indent(level);
                format!("{}:{}", key_str, value_str)
            }
        }
    }
}

impl ScillaFormatter for NodeTypeMapValueAllowingTypeArguments {
    fn to_string_with_indent(&self, level: usize) -> String {
        match self {
            NodeTypeMapValueAllowingTypeArguments::TypeMapValueNoArgs(node) => {
                node.to_string_with_indent(level)
            }
            NodeTypeMapValueAllowingTypeArguments::TypeMapValueWithArgs(identifier, args) => {
                let id_str = identifier.to_string_with_indent(level);
                let args_str = args
                    .iter()
                    .map(|arg| arg.to_string_with_indent(level + 1))
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("{}({}, {})", id_str, args_str, indentation(level))
            }
        }
    }
}

impl ScillaFormatter for NodeImportedName {
    fn to_string_with_indent(&self, level: usize) -> String {
        match self {
            NodeImportedName::RegularImport(type_name_identifier) => {
                type_name_identifier.to_string_with_indent(level)
            }
            NodeImportedName::AliasedImport(original_type_name, alias_type_name) => {
                let original = original_type_name.to_string_with_indent(level);
                let alias = alias_type_name.to_string_with_indent(level);
                format!("{} as {}", original, alias)
            }
        }
    }
}

impl ScillaFormatter for NodeImportDeclarations {
    fn to_string_with_indent(&self, level: usize) -> String {
        let mut output = String::new();

        for (idx, import) in self.import_list.iter().enumerate() {
            if idx != 0 {
                output.push('\n');
            }
            write!(&mut output, "{}", import.to_string_with_indent(level)).unwrap();
        }

        output
    }
}

impl ScillaFormatter for NodeMetaIdentifier {
    fn to_string_with_indent(&self, level: usize) -> String {
        let ind = indentation(level);
        match self {
            NodeMetaIdentifier::MetaName(name) => {
                format!("{}{}", ind, name.to_string_with_indent(0))
            }
            NodeMetaIdentifier::MetaNameInNamespace(namespace, name) => {
                format!(
                    "{}{}::{}",
                    ind,
                    namespace.to_string_with_indent(0),
                    name.to_string_with_indent(0)
                )
            }
            NodeMetaIdentifier::MetaNameInHexspace(hexspace, name) => {
                format!("{}{}::{}", ind, hexspace, name.to_string_with_indent(0))
            }
            NodeMetaIdentifier::ByteString => format!("{}byte", ind),
        }
    }
}

impl ScillaFormatter for NodeVariableIdentifier {
    fn to_string_with_indent(&self, level: usize) -> String {
        let indent = indentation(level);
        match self {
            NodeVariableIdentifier::VariableName(name) => format!("{}{}", indent, name),
            NodeVariableIdentifier::SpecialIdentifier(id) => format!("{}{}", indent, id),
            NodeVariableIdentifier::VariableInNamespace(namespace, var_name) => format!(
                "{}{}.{}",
                indent,
                namespace.to_string_with_indent(level),
                var_name
            ),
        }
    }
}

impl ScillaFormatter for NodeBuiltinArguments {
    fn to_string_with_indent(&self, level: usize) -> String {
        let mut formatted_str = String::new();
        let indent = indentation(level);

        for (i, arg) in self.arguments.iter().enumerate() {
            formatted_str.push_str(&indent);
            formatted_str.push_str(&arg.to_string_with_indent(level + 1));

            if i < self.arguments.len() - 1 {
                formatted_str.push_str(",\n");
            }
        }

        formatted_str
    }
}

impl ScillaFormatter for NodeTypeMapKey {
    fn to_string_with_indent(&self, level: usize) -> String {
        match self {
            NodeTypeMapKey::GenericMapKey(node_meta_identifier) => {
                node_meta_identifier.to_string_with_indent(level)
            }
            NodeTypeMapKey::EnclosedGenericId(node_meta_identifier) => {
                format!("{{{}}}", node_meta_identifier.to_string_with_indent(level))
            }
            NodeTypeMapKey::EnclosedAddressMapKeyType(node_address_type) => {
                format!("{{{}}}", node_address_type.to_string_with_indent(level))
            }
            NodeTypeMapKey::AddressMapKeyType(node_address_type) => {
                node_address_type.to_string_with_indent(level)
            }
        }
    }
}

impl ScillaFormatter for NodeTypeMapValue {
    fn to_string_with_indent(&self, level: usize) -> String {
        let indent = indentation(level);

        match self {
            NodeTypeMapValue::MapValueCustomType(meta_id) => {
                format!("{}{}", indent, meta_id.to_string_with_indent(level))
            }
            NodeTypeMapValue::MapKeyValue(node_type_map_entry) => {
                format!(
                    "{}{}",
                    indent,
                    node_type_map_entry.to_string_with_indent(level)
                )
            }
            NodeTypeMapValue::MapValueParanthesizedType(node_type_map_value) => {
                format!("({})", node_type_map_value.to_string_with_indent(level))
            }
            NodeTypeMapValue::MapValueAddressType(node_address_type) => {
                format!(
                    "{}{}",
                    indent,
                    node_address_type.to_string_with_indent(level)
                )
            }
        }
    }
}

impl ScillaFormatter for NodeTypeArgument {
    fn to_string_with_indent(&self, level: usize) -> String {
        match self {
            NodeTypeArgument::EnclosedTypeArgument(node_type) => {
                format!("({})", node_type.to_string_with_indent(level))
            }
            NodeTypeArgument::GenericTypeArgument(meta_id) => meta_id.to_string_with_indent(level),
            NodeTypeArgument::TemplateTypeArgument(template) => format!("{}", template),
            NodeTypeArgument::AddressTypeArgument(node_addr_type) => {
                node_addr_type.to_string_with_indent(level)
            }
            NodeTypeArgument::MapTypeArgument(map_key, map_value) => {
                format!(
                    "({} => {})",
                    map_key.to_string_with_indent(level),
                    map_value.to_string_with_indent(level)
                )
            }
        }
    }
}

impl ScillaFormatter for NodeScillaType {
    fn to_string_with_indent(&self, level: usize) -> String {
        match self {
            NodeScillaType::GenericTypeWithArgs(meta_identifier, type_args) => {
                let type_args_str = type_args
                    .iter()
                    .map(|arg| arg.to_string_with_indent(level + 1))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!(
                    "{}<{}>",
                    meta_identifier.to_string_with_indent(level),
                    type_args_str
                )
            }
            NodeScillaType::MapType(map_key, map_value) => {
                format!(
                    "Map ({} -> {})",
                    map_key.to_string_with_indent(level + 1),
                    map_value.to_string_with_indent(level + 1)
                )
            }
            NodeScillaType::FunctionType(arg_type, return_type) => {
                format!(
                    "{} -> {}",
                    arg_type.to_string_with_indent(level),
                    return_type.to_string_with_indent(level)
                )
            }
            NodeScillaType::EnclosedType(inner_type) => {
                format!("({})", inner_type.to_string_with_indent(level + 1))
            }
            NodeScillaType::ScillaAddresseType(address_type) => {
                format!("{}", address_type.to_string_with_indent(level))
            }
            NodeScillaType::PolyFunctionType(param, return_type) => {
                format!(
                    "forall {} . {}",
                    param,
                    return_type.to_string_with_indent(level)
                )
            }
            NodeScillaType::TypeVarType(name) => name.clone(),
        }
    }
}

impl ScillaFormatter for NodeTypeMapEntry {
    fn to_string_with_indent(&self, level: usize) -> String {
        let key_string = self.key.to_string_with_indent(level + 1);
        let value_string = self.value.to_string_with_indent(level + 1);

        format!("{}{} : {}", indentation(level), key_string, value_string)
    }
}

impl ScillaFormatter for NodeAddressTypeField {
    fn to_string_with_indent(&self, level: usize) -> String {
        let id_str = self.identifier.to_string_with_indent(level);
        let type_str = self.type_name.to_string_with_indent(level);

        format!("{} : {}", id_str, type_str)
    }
}

impl ScillaFormatter for NodeAddressType {
    fn to_string_with_indent(&self, level: usize) -> String {
        let id_str = self.identifier.to_string_with_indent(level);
        let type_name_str = format!("({})", self.type_name);

        let fields_str = self
            .address_fields
            .iter()
            .map(|field| field.to_string_with_indent(level + 1))
            .collect::<Vec<String>>()
            .join(",\n");
        let fields_str = if fields_str.is_empty() {
            fields_str
        } else {
            format!("\n{}\n{}", indentation(level + 1), fields_str)
        };

        format!("{}{}{}", id_str, type_name_str, fields_str)
    }
}

impl ScillaFormatter for NodeFullExpression {
    fn to_string_with_indent(&self, level: usize) -> String {
        let ind = indentation(level);
        match self {
            NodeFullExpression::LocalVariableDeclaration {
                identifier_name,
                expression,
                containing_expression,
                ..
            } => format!(
                "{}let {} = {}\n{}in {}",
                ind,
                identifier_name,
                expression.to_string_with_indent(level + 1),
                ind,
                containing_expression.to_string_with_indent(level)
            ),
            NodeFullExpression::FunctionDeclaration {
                identier_value,
                type_annotation,
                expression,
            } => format!(
                "{}fun {} {} => {}",
                ind,
                identier_value,
                type_annotation.to_string(),
                expression.to_string_with_indent(level + 1)
            ),
            NodeFullExpression::FunctionCall {
                function_name,
                argument_list,
            } => format!(
                "{}{}({})",
                ind,
                function_name.to_string(),
                argument_list
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            NodeFullExpression::ExpressionAtomic(node) => node.to_string_with_indent(level),
            NodeFullExpression::ExpressionBuiltin { b, targs, xs } => {
                format!("{}{} {}", ind, b, xs.to_string_with_indent(level + 1))
            }
            NodeFullExpression::Message(entries) => format!(
                "{}{{\n{}\n{}}}",
                ind,
                entries
                    .iter()
                    .map(|entry| entry.to_string_with_indent(level + 1))
                    .collect::<Vec<String>>()
                    .join("\n"),
                ind
            ),
            NodeFullExpression::Match {
                match_expression,
                clauses,
            } => format!(
                "{}match {} with\n{}",
                ind,
                match_expression.to_string(),
                clauses
                    .iter()
                    .map(|clause| clause.to_string_with_indent(level + 1))
                    .collect::<Vec<String>>()
                    .join("\n")
            ),
            NodeFullExpression::ConstructorCall {
                identifier_name,
                contract_type_arguments,
                argument_list,
            } => {
                let type_args = match contract_type_arguments {
                    Some(args) => format!("<{}>", args.to_string()),
                    None => "".to_string(),
                };

                format!(
                    "{}{}{}({})",
                    ind,
                    identifier_name.to_string(),
                    type_args,
                    argument_list
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            NodeFullExpression::TemplateFunction {
                identifier_name,
                expression,
            } => format!(
                "{}tfun {} => {}",
                ind,
                identifier_name,
                expression.to_string_with_indent(level + 1)
            ),
            NodeFullExpression::TApp {
                identifier_name,
                type_arguments,
            } => format!(
                "{}{}<{}>",
                ind,
                identifier_name.to_string(),
                type_arguments
                    .iter()
                    .map(|ta| ta.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        }
    }
}

impl ScillaFormatter for NodeMessageEntry {
    fn to_string_with_indent(&self, level: usize) -> String {
        let indent_str = indentation(level);
        match self {
            NodeMessageEntry::MessageLiteral(var_id, value_literal) => format!(
                "{}{}_tag:{} = {}",
                indent_str,
                var_id.to_string_with_indent(level),
                var_id.to_string_with_indent(level),
                value_literal.to_string_with_indent(level)
            ),
            NodeMessageEntry::MessageVariable(var1_id, var2_id) => format!(
                "{}{}_tag:{} = {}",
                indent_str,
                var1_id.to_string_with_indent(level),
                var1_id.to_string_with_indent(level),
                var2_id.to_string_with_indent(level)
            ),
        }
    }
}

impl ScillaFormatter for NodePatternMatchExpressionClause {
    fn to_string_with_indent(&self, level: usize) -> String {
        let indent = indentation(level);
        let pattern_str = self.pattern.to_string_with_indent(level + 1);
        let expression_str = self.expression.to_string_with_indent(level + 1);

        format!("{}{} => {}", indent, pattern_str, expression_str)
    }
}

impl ScillaFormatter for NodeAtomicExpression {
    fn to_string_with_indent(&self, level: usize) -> String {
        match self {
            NodeAtomicExpression::AtomicSid(node) => {
                // No need to increase the indentation level since it's an atomic expression
                node.to_string_with_indent(level)
            }
            NodeAtomicExpression::AtomicLit(node) => {
                // No need to increase the indentation level since it's an atomic expression
                node.to_string_with_indent(level)
            }
        }
    }
}

impl ScillaFormatter for NodeContractTypeArguments {
    fn to_string_with_indent(&self, level: usize) -> String {
        // To get proper indentation string for the current level
        let indent_str = indentation(level);

        // Traversing Vec<NodeTypeArgument> and invoking to_string_with_indent for each element
        let type_arg_strings = self
            .type_arguments
            .iter()
            .map(|arg| arg.to_string_with_indent(level + 1))
            .collect::<Vec<String>>()
            .join("\n");

        // Returning the formatted string
        format!("{}TypeArguments:\n{}", indent_str, type_arg_strings)
    }
}

impl ScillaFormatter for NodeValueLiteral {
    fn to_string_with_indent(&self, level: usize) -> String {
        let indent = indentation(level);
        match self {
            NodeValueLiteral::LiteralInt(ty, value) => format!(
                "{}{}_{}",
                indent,
                ty.to_string_with_indent(level + 1),
                value
            ),
            NodeValueLiteral::LiteralHex(value) => format!("{}0x{}", indent, value),
            NodeValueLiteral::LiteralString(value) => format!("{}\"{}\"", indent, value),
            NodeValueLiteral::LiteralEmptyMap(key_ty, value_ty) => {
                format!(
                    "{}EmpMap {{{}: {}}}",
                    indent,
                    key_ty.to_string_with_indent(level + 1),
                    value_ty.to_string_with_indent(level + 1)
                )
            }
        }
    }
}

impl ScillaFormatter for NodeMapAccess {
    fn to_string_with_indent(&self, level: usize) -> String {
        // 1. Use the `indentation` function to indent according to the given level
        let indent = indentation(level);

        // 2. Obtain the formatted string representation of the `identifier_name`
        let identifier_str = self.identifier_name.to_string_with_indent(level);

        // 3. Since `type_annotation` is an internal variable, ignore it
        // 4. Build the final output string
        format!("{}{}", indent, identifier_str)
    }
}

impl ScillaFormatter for NodePattern {
    fn to_string_with_indent(&self, level: usize) -> String {
        match self {
            NodePattern::Wildcard => "_".to_string(),
            NodePattern::Binder(b) => b.clone(),
            NodePattern::Constructor(meta_id, arg_patterns) => {
                let mut formatted = format!("{}(", meta_id.to_string_with_indent(level));
                for (i, pattern) in arg_patterns.iter().enumerate() {
                    if i > 0 {
                        formatted.push_str(", ");
                    }
                    formatted.push_str(&pattern.to_string_with_indent(level + 1));
                }
                formatted.push(')');
                formatted
            }
        }
    }
}

impl ScillaFormatter for NodeArgumentPattern {
    fn to_string_with_indent(&self, level: usize) -> String {
        match self {
            NodeArgumentPattern::WildcardArgument => "_".to_string(),
            NodeArgumentPattern::BinderArgument(binder) => binder.clone(),
            NodeArgumentPattern::ConstructorArgument(constructor) => {
                constructor.to_string_with_indent(level)
            }
            NodeArgumentPattern::PatternArgument(pattern) => {
                format!("({})", pattern.to_string_with_indent(level))
            }
        }
    }
}

impl ScillaFormatter for NodePatternMatchClause {
    fn to_string_with_indent(&self, level: usize) -> String {
        let mut formatted: String = String::new();
        formatted.push_str(&indentation(level));
        formatted.push_str("match ");
        formatted.push_str(&self.pattern_expression.to_string_with_indent(level + 1));

        if let Some(ref statement_block) = self.statement_block {
            formatted.push_str(" => ");
            formatted.push_str(&statement_block.to_string_with_indent(level + 1));
        }

        formatted
    }
}

impl ScillaFormatter for NodeBlockchainFetchArguments {
    fn to_string_with_indent(&self, level: usize) -> String {
        let args = self
            .arguments
            .iter()
            .map(|arg| arg.to_string_with_indent(level))
            .collect::<Vec<_>>()
            .join(", ");

        format!("{{{} FETCH_BLOCKCHAIN_ARGUMENTS }}", args)
    }
}

impl ScillaFormatter for NodeStatement {
    fn to_string_with_indent(&self, level: usize) -> String {
        use NodeStatement::*;
        let indent = indentation(level);

        match self {
            Load {
                left_hand_side,
                right_hand_side,
            } => {
                format!(
                    "{}load {} <- {};\n",
                    indent,
                    left_hand_side,
                    right_hand_side.to_string()
                )
            }
            RemoteFetch(inner) => {
                format!("{}{}\n", indent, inner.to_string_with_indent(level))
            }
            Store {
                left_hand_side,
                right_hand_side,
            } => {
                format!(
                    "{}store {} <- {};\n",
                    indent,
                    left_hand_side,
                    right_hand_side.to_string()
                )
            }
            Bind {
                left_hand_side,
                right_hand_side,
            } => {
                format!(
                    "{}{} <- {};\n",
                    indent,
                    left_hand_side,
                    right_hand_side.to_string_with_indent(level)
                )
            }
            ReadFromBC {
                left_hand_side,
                type_name,
                arguments,
            } => {
                let args = match arguments {
                    Some(arg) => arg.to_string_with_indent(level),
                    None => "".to_string(),
                };
                format!(
                    "{}{} <- {}\n({});\n",
                    indent,
                    left_hand_side,
                    type_name.to_string(),
                    args
                )
            }
            MapGet {
                left_hand_side,
                keys,
                right_hand_side,
            } => {
                let key_str = keys
                    .iter()
                    .map(|k| k.to_string_with_indent(level))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!(
                    "{}{} <- {} [{}];\n",
                    indent, left_hand_side, right_hand_side, key_str
                )
            }
            MapGetExists {
                left_hand_side,
                keys,
                right_hand_side,
            } => {
                let key_str = keys
                    .iter()
                    .map(|k| k.to_string_with_indent(level))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!(
                    "{}{} <- exists {} [{}];\n",
                    indent, left_hand_side, right_hand_side, key_str
                )
            }
            MapUpdate {
                left_hand_side,
                keys,
                right_hand_side,
            } => {
                let key_str = keys
                    .iter()
                    .map(|k| k.to_string_with_indent(level))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!(
                    "{}{} [{}] <- {};\n",
                    indent,
                    left_hand_side,
                    key_str,
                    right_hand_side.to_string()
                )
            }
            MapUpdateDelete {
                left_hand_side,
                keys,
            } => {
                let key_str = keys
                    .iter()
                    .map(|k| k.to_string_with_indent(level))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}{} [{}] <- delete;\n", indent, left_hand_side, key_str)
            }
            Accept => format!("{}accept;\n", indent),
            Send { identifier_name } => {
                format!("{}send({});\n", indent, identifier_name.to_string())
            }
            CreateEvnt { identifier_name } => {
                format!("{}event({});\n", indent, identifier_name.to_string())
            }
            Throw { error_variable } => {
                let error_var = error_variable
                    .as_ref()
                    .map_or("".to_string(), |v| v.to_string());
                format!("{}throw {};\n", indent, error_var)
            }
            MatchStmt { variable, clauses } => {
                let clause_str = clauses
                    .iter()
                    .map(|c| c.to_string_with_indent(level + 1))
                    .collect::<Vec<_>>()
                    .join("\n");
                format!(
                    "{}match {} with\n{}end\n",
                    indent,
                    variable.to_string(),
                    clause_str
                )
            }
            CallProc {
                component_id,
                arguments,
            } => {
                let args_str = arguments
                    .iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}{}({});\n", indent, component_id.to_string(), args_str)
            }
            Iterate {
                identifier_name,
                component_id,
            } => {
                format!(
                    "{}forall {} {}\n",
                    indent,
                    identifier_name.to_string(),
                    component_id.to_string()
                )
            }
        }
    }
}

impl ScillaFormatter for NodeRemoteFetchStatement {
    fn to_string_with_indent(&self, level: usize) -> String {
        let ind = indentation(level);
        match self {
            NodeRemoteFetchStatement::ReadStateMutable(id1, id2, var_id) => {
                format!(
                    "{}fetch {}: {} <-{}\n",
                    ind,
                    id1,
                    id2,
                    var_id.to_string_with_indent(level)
                )
            }
            NodeRemoteFetchStatement::ReadStateMutableSpecialId(id1, id2, id3) => {
                format!("{}fetch {}: {} {}\n", ind, id1, id2, id3)
            }
            NodeRemoteFetchStatement::ReadStateMutableMapAccess(id1, id2, id3, map_accesses) => {
                let accesses_str = map_accesses
                    .iter()
                    .map(|access| access.to_string_with_indent(level + 1))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!(
                    "{}fetch {}: {}[{}]<-({})\n",
                    ind, id1, id2, id3, accesses_str
                )
            }
            NodeRemoteFetchStatement::ReadStateMutableMapAccessExists(
                id1,
                id2,
                id3,
                map_accesses,
            ) => {
                let accesses_str = map_accesses
                    .iter()
                    .map(|access| access.to_string_with_indent(level + 1))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!(
                    "{}fetch {}: {}[{}]<-(exists {})\n",
                    ind, id1, id2, id3, accesses_str
                )
            }
            NodeRemoteFetchStatement::ReadStateMutableCastAddress(id1, var_id, addr_type) => {
                format!(
                    "{}fetch {}: {} := {} {}\n",
                    ind,
                    id1,
                    var_id.to_string_with_indent(level),
                    "cast_to",
                    addr_type.to_string_with_indent(level)
                )
            }
        }
    }
}

impl ScillaFormatter for NodeComponentId {
    fn to_string_with_indent(&self, level: usize) -> String {
        match self {
            NodeComponentId::WithTypeLikeName(type_name_identifier) => {
                type_name_identifier.to_string_with_indent(level)
            }
            NodeComponentId::WithRegularId(id_string) => {
                let indented_string = indentation(level);
                format!("{}{}", indented_string, id_string)
            }
        }
    }
}

impl ScillaFormatter for NodeComponentParameters {
    fn to_string_with_indent(&self, level: usize) -> String {
        let indent = indentation(level);
        let formatted_parameters = self
            .parameters
            .iter()
            .map(|param| param.to_string_with_indent(level + 1))
            .collect::<Vec<String>>()
            .join(",\n");

        if formatted_parameters.is_empty() {
            format!("{}()", indent)
        } else {
            format!(
                "{}(\n{}\n{})",
                indent,
                formatted_parameters,
                indentation(level)
            )
        }
    }
}

impl ScillaFormatter for NodeParameterPair {
    fn to_string_with_indent(&self, level: usize) -> String {
        let indent = indentation(level);
        let param_str = self.identifier_with_type.to_string_with_indent(level);

        format!("{}{}", indent, param_str)
    }
}

impl ScillaFormatter for NodeComponentBody {
    fn to_string_with_indent(&self, level: usize) -> String {
        match &self.statement_block {
            Some(statement_block) => statement_block.to_string_with_indent(level),
            None => String::new(),
        }
    }
}

impl ScillaFormatter for NodeStatementBlock {
    fn to_string_with_indent(&self, level: usize) -> String {
        let mut formatted_statements = String::new();
        for (i, statement) in self.statements.iter().enumerate() {
            let mut formatted_statement = statement.to_string_with_indent(level);
            if i > 0 {
                // Properly separate statements with a newline
                formatted_statements.push_str("\n");
                // Indent the statements as needed
                formatted_statements.push_str(&indentation(level));
            }
            formatted_statements.push_str(&formatted_statement);
        }
        formatted_statements
    }
}

impl ScillaFormatter for NodeTypedIdentifier {
    fn to_string_with_indent(&self, level: usize) -> String {
        let id = &self.identifier_name;
        let annotation = self.annotation.to_string_with_indent(level);
        format!("{}{} : {}", indentation(level), id, annotation)
    }
}

impl ScillaFormatter for NodeTypeAnnotation {
    fn to_string_with_indent(&self, level: usize) -> String {
        let mut output = String::new();
        output.push_str(&indentation(level));
        output.push_str(&self.type_name.to_string_with_indent(level));
        // No need to handle `type_annotation` as mentioned in the instructions.
        output
    }
}

impl ScillaFormatter for NodeProgram {
    fn to_string_with_indent(&self, level: usize) -> String {
        let mut formatted_code = format!("scilla_version {}\n", self.version);
        if let Some(ref import_declarations) = self.import_declarations {
            formatted_code.push_str(&import_declarations.to_string_with_indent(level));
            formatted_code.push_str("\n");
        }
        if let Some(ref library_definition) = self.library_definition {
            formatted_code.push_str(&library_definition.to_string_with_indent(level));
            formatted_code.push_str("\n");
        }
        formatted_code.push_str(&self.contract_definition.to_string_with_indent(level));
        formatted_code
    }
}

impl ScillaFormatter for NodeLibraryDefinition {
    fn to_string_with_indent(&self, level: usize) -> String {
        let mut result = String::new();
        let indent = indentation(level);
        // Add library keyword and name
        result.push_str(&format!(
            "{}library {}\n",
            indent,
            self.name.to_string_with_indent(level)
        ));
        // Add opening brace
        result.push_str(&format!("{}{{\n", indent));
        // Add definitions with proper indentation
        let definition_strings: Vec<String> = self
            .definitions
            .iter()
            .map(|definition| definition.to_string_with_indent(level + 1))
            .collect();
        result.push_str(&definition_strings.join("\n"));
        // Add closing brace
        result.push_str(&format!("\n{}}}", indent));
        result
    }
}

impl ScillaFormatter for NodeLibrarySingleDefinition {
    fn to_string_with_indent(&self, level: usize) -> String {
        match self {
            NodeLibrarySingleDefinition::LetDefinition {
                variable_name,
                type_annotation: _,
                expression,
            } => {
                format!(
                    "{}let {} = {}\n",
                    indentation(level),
                    variable_name,
                    expression.to_string_with_indent(level + 1)
                )
            }
            NodeLibrarySingleDefinition::TypeDefinition(type_name, opt_type_alternatives) => {
                let type_alternatives_str = match opt_type_alternatives {
                    Some(type_alternatives) => type_alternatives
                        .iter()
                        .map(|alternative| alternative.to_string_with_indent(level + 1))
                        .collect::<Vec<String>>()
                        .join("\n"),
                    None => "".to_string(),
                };
                format!(
                    "{}type {}{}\n",
                    indentation(level),
                    type_name.to_string_with_indent(level + 1),
                    if !type_alternatives_str.is_empty() {
                        format!("\n{}", type_alternatives_str)
                    } else {
                        "".to_string()
                    }
                )
            }
        }
    }
}

impl ScillaFormatter for NodeContractDefinition {
    fn to_string_with_indent(&self, level: usize) -> String {
        let contract_name = self.contract_name.to_string_with_indent(level);
        let parameters = self.parameters.to_string_with_indent(level);
        let constraint = self
            .constraint
            .as_ref()
            .map_or("".to_string(), |c| c.indent(level + 1));
        let fields = self
            .fields
            .iter()
            .map(|field| field.to_string_with_indent(level + 1))
            .collect::<Vec<String>>()
            .join("\n");
        let components = self
            .components
            .iter()
            .map(|component| component.to_string_with_indent(level + 1))
            .collect::<Vec<String>>()
            .join("\n");
        let indent = indentation(level);
        format!(
            "{indent}contract {contract_name} ({parameters}){constraint}\n{indent}{{\n{fields}\n\n{components}\n{indent}}}",
            indent=indent,
            contract_name=contract_name,
            parameters=parameters,
            constraint=constraint,
            fields=fields,
            components=components,
        )
    }
}

impl ScillaFormatter for NodeContractField {
    fn to_string_with_indent(&self, level: usize) -> String {
        let indent = indentation(level);
        let typed_identifier_str = self.typed_identifier.to_string_with_indent(level);
        let right_hand_side_str = self.right_hand_side.to_string_with_indent(level + 1);
        format!(
            "{}{} =\n{}",
            indent, typed_identifier_str, right_hand_side_str
        )
    }
}

impl ScillaFormatter for NodeWithConstraint {
    fn to_string_with_indent(&self, level: usize) -> String {
        let indent = indentation(level);
        let expression_string = self.expression.to_string_with_indent(level + 1);
        format!("{}{}", indent, expression_string)
    }
}

impl ScillaFormatter for NodeComponentDefinition {
    fn to_string_with_indent(&self, level: usize) -> String {
        match self {
            NodeComponentDefinition::TransitionComponent(transition_def) => {
                transition_def.to_string_with_indent(level)
            }
            NodeComponentDefinition::ProcedureComponent(procedure_def) => {
                procedure_def.to_string_with_indent(level)
            }
        }
    }
}

impl ScillaFormatter for NodeProcedureDefinition {
    fn to_string_with_indent(&self, level: usize) -> String {
        let indent = indentation(level);
        let name_str = self.name.to_string_with_indent(0);
        let params_str = self.parameters.to_string_with_indent(0);
        let body_str = self.body.to_string_with_indent(level + 1);
        format!(
            "{}procedure {} ({})\n{}=\n{}{}",
            indent, name_str, params_str, indent, body_str, indent
        )
    }
}
