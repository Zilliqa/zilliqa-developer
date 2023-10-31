use crate::ast::converting::AstConverting;
use crate::ast::nodes::*;
use crate::constants::{TraversalResult, TreeTraversalMode};

/// The `AstVisitor` trait is used for implementing the visiting behaviour for each AST node of the Scilla AST.
/// Each node in the AST implements this trait to define how it should be visited during the tree traversal.
/// The `visit` method is called with an `emitter` that implements the `AstConverting` trait, which is responsible for converting the AST to some other form.
/// The `visit` method returns a `Result` with a `TraversalResult` that informs the visitor algorithm how to proceed, or a `String` in case of an error.
pub trait AstVisitor {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String>;
}

impl<T: AstVisitor> AstVisitor for WithMetaData<T> {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        emitter.push_source_position(&self.start, &self.end);
        let ret = self.node.visit(emitter);
        emitter.pop_source_position();

        ret
    }
}

impl AstVisitor for NodeByteStr {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_byte_str(TreeTraversalMode::Enter, self)?;

        // No children

        match ret {
            TraversalResult::Continue => emitter.emit_byte_str(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeTypeNameIdentifier {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_type_name_identifier(TreeTraversalMode::Enter, self);

        let children_ret = if ret == Ok(TraversalResult::Continue) {
            match self {
                NodeTypeNameIdentifier::ByteStringType(bs_type) => {
                    let ret = bs_type.visit(emitter);
                    ret
                }
                _ => Ok(TraversalResult::Continue),
            }
        } else {
            ret
        }?;

        match children_ret {
            TraversalResult::Continue => {
                emitter.emit_type_name_identifier(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeImportedName {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_imported_name(TreeTraversalMode::Enter, self);

        let children_ret = if ret == Ok(TraversalResult::Continue) {
            match self {
                NodeImportedName::RegularImport(name) => {
                    let ret = name.visit(emitter);

                    ret
                }
                NodeImportedName::AliasedImport(name, alias) => {
                    let ret = match name.visit(emitter) {
                        Err(msg) => Err(msg),
                        _ => alias.visit(emitter),
                    };
                    ret
                }
            }
        } else {
            ret
        }?;

        match children_ret {
            TraversalResult::Continue => emitter.emit_imported_name(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeImportDeclarations {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_import_declarations(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            for import in &self.import_list {
                match import.visit(emitter) {
                    Err(msg) => {
                        return Err(msg);
                    }
                    _ => (),
                }
            }
            Ok(TraversalResult::Continue)
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => {
                emitter.emit_import_declarations(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeMetaIdentifier {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_meta_identifier(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            match self {
                NodeMetaIdentifier::MetaName(name) => {
                    let ret = name.visit(emitter);
                    ret
                }
                NodeMetaIdentifier::MetaNameInNamespace(name, ns) => {
                    let ret = match name.visit(emitter) {
                        Err(msg) => Err(msg),
                        _ => ns.visit(emitter),
                    };
                    ret
                }
                NodeMetaIdentifier::MetaNameInHexspace(_, name) => {
                    let ret = name.visit(emitter);
                    ret
                }
                NodeMetaIdentifier::ByteString => Ok(TraversalResult::Continue),
            }
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => {
                emitter.emit_meta_identifier(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeVariableIdentifier {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_variable_identifier(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            match self {
                NodeVariableIdentifier::VariableInNamespace(type_name_identifier, _) => {
                    let ret = type_name_identifier.visit(emitter);
                    ret
                }
                // Since VariableName and SpecialIdentifier don't have children
                // we can directly return ret here.
                _ => ret,
            }
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => {
                emitter.emit_variable_identifier(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeBuiltinArguments {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        // Call the code emitter at the entry of the NodeBuiltinArguments
        let ret = emitter.emit_builtin_arguments(TreeTraversalMode::Enter, self);
        // Call the visitor on all children of NodeBuiltinArguments if ret == Ok(TraversalResult::Continue)
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            // Visit each of the arguments
            self.arguments
                .iter()
                .map(|argument| {
                    let ret = argument.visit(emitter);
                    ret
                })
                .find(|r| *r == Err(String::from("Failure")))
                .unwrap_or(Ok(TraversalResult::Continue))
        } else {
            ret
        }?;
        // Call the code emitter at the exit of the NodeBuiltinArguments
        match children_ret {
            TraversalResult::Continue => {
                emitter.emit_builtin_arguments(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeTypeMapKey {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_type_map_key(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            match self {
                NodeTypeMapKey::GenericMapKey(node_met_id) => {
                    let ret = node_met_id.visit(emitter);
                    ret
                }
                NodeTypeMapKey::EnclosedGenericId(node_met_id) => {
                    let ret = node_met_id.visit(emitter);
                    ret
                }
                NodeTypeMapKey::EnclosedAddressMapKeyType(node_address_type) => {
                    let ret = node_address_type.visit(emitter);
                    ret
                }
                NodeTypeMapKey::AddressMapKeyType(node_address_type) => {
                    let ret = node_address_type.visit(emitter);
                    ret
                }
            }
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => emitter.emit_type_map_key(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeTypeMapValue {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_type_map_value(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            match self {
                NodeTypeMapValue::MapValueTypeOrEnumLikeIdentifier(meta_id) => {
                    let ret = meta_id.visit(emitter);

                    ret
                }
                NodeTypeMapValue::MapKeyValue(entry) => {
                    let ret = entry.visit(emitter);

                    ret
                }
                NodeTypeMapValue::MapValueParanthesizedType(value) => {
                    let ret = value.visit(emitter);

                    ret
                }
                NodeTypeMapValue::MapValueAddressType(address_type) => {
                    let ret = address_type.visit(emitter);

                    ret
                }
            }
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => emitter.emit_type_map_value(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeTypeArgument {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_type_argument(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            match self {
                NodeTypeArgument::EnclosedTypeArgument(node) => {
                    let ret = node.visit(emitter);
                    ret
                }
                NodeTypeArgument::GenericTypeArgument(node) => {
                    let ret = node.visit(emitter);
                    ret
                }
                NodeTypeArgument::TemplateTypeArgument(_) => Ok(TraversalResult::Continue),
                NodeTypeArgument::AddressTypeArgument(node) => {
                    let ret = node.visit(emitter);
                    ret
                }
                NodeTypeArgument::MapTypeArgument(key_node, value_node) => {
                    let ret = match key_node.visit(emitter) {
                        Ok(TraversalResult::Continue) => value_node.visit(emitter),
                        Err(msg) => Err(msg),
                        _ => Ok(TraversalResult::Continue),
                    };

                    ret
                }
            }
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => emitter.emit_type_argument(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeScillaType {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_scilla_type(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            match self {
                NodeScillaType::GenericTypeWithArgs(id, args) => {
                    let ret = match id.visit(emitter) {
                        Err(msg) => Err(msg),
                        _ => {
                            for arg in args {
                                match arg.visit(emitter) {
                                    Err(msg) => {
                                        return Err(msg);
                                    }
                                    _ => continue,
                                }
                            }
                            Ok(TraversalResult::Continue)
                        }
                    };

                    ret
                }
                NodeScillaType::MapType(key, value) => {
                    let ret = match key.visit(emitter) {
                        Err(msg) => Err(msg),
                        _ => value.visit(emitter),
                    };
                    ret
                }
                NodeScillaType::FunctionType(t1, t2) => {
                    let ret = match t1.visit(emitter) {
                        Err(msg) => Err(msg),
                        _ => t2.visit(emitter),
                    };
                    ret
                }
                NodeScillaType::PolyFunctionType(_, t) => {
                    let ret = t.visit(emitter);
                    ret
                }
                NodeScillaType::EnclosedType(t) => {
                    let ret = t.visit(emitter);
                    ret
                }
                NodeScillaType::ScillaAddresseType(t) => {
                    let ret = t.visit(emitter);
                    ret
                }
                NodeScillaType::TypeVarType(_) => Ok(TraversalResult::Continue),
            }
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => emitter.emit_scilla_type(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeTypeMapEntry {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_type_map_entry(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            let ret = match self.key.visit(emitter) {
                Err(msg) => Err(msg),
                _ => self.value.visit(emitter),
            };

            ret
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => emitter.emit_type_map_entry(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeAddressTypeField {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_address_type_field(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            let ret = match self.identifier.visit(emitter) {
                Err(msg) => Err(msg),
                _ => self.type_name.visit(emitter),
            };
            ret
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => {
                emitter.emit_address_type_field(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeAddressType {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_address_type(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            let ret = self.identifier.visit(emitter);

            if ret.is_err() {
                return ret;
            }

            for field in &self.address_fields {
                let ret = field.visit(emitter);

                if ret.is_err() {
                    return ret;
                }
            }
            ret
        } else {
            Ok(TraversalResult::Continue)
        }?;
        match children_ret {
            TraversalResult::Continue => emitter.emit_address_type(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeFullExpression {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_full_expression(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            match self {
                NodeFullExpression::LocalVariableDeclaration {
                    expression,
                    containing_expression,
                    ..
                } => {
                    let ret = match expression.visit(emitter) {
                        Err(msg) => Err(msg),
                        _ => containing_expression.visit(emitter),
                    };
                    ret
                }
                NodeFullExpression::FunctionDeclaration { expression, .. } => {
                    let ret = expression.visit(emitter);

                    ret
                }
                NodeFullExpression::FunctionCall {
                    function_name,
                    argument_list,
                    ..
                } => {
                    let ret = match function_name.visit(emitter) {
                        Err(msg) => Err(msg),
                        _ => {
                            for arg in argument_list {
                                match arg.visit(emitter) {
                                    Err(msg) => {
                                        return Err(msg);
                                    }
                                    _ => continue,
                                }
                            }
                            Ok(TraversalResult::Continue)
                        }
                    };

                    ret
                }
                NodeFullExpression::ExpressionAtomic(atom_expr) => {
                    let ret = atom_expr.visit(emitter);

                    ret
                }
                NodeFullExpression::ExpressionBuiltin { xs, .. } => {
                    let ret = xs.visit(emitter);

                    ret
                }
                NodeFullExpression::Message(message_entries) => {
                    for entry in message_entries {
                        match entry.visit(emitter) {
                            Err(msg) => {
                                return Err(msg);
                            }
                            _ => (),
                        }
                    }

                    Ok(TraversalResult::Continue)
                }
                NodeFullExpression::Match {
                    match_expression,
                    clauses,
                    ..
                } => {
                    let ret = match match_expression.visit(emitter) {
                        Err(msg) => Err(msg),
                        _ => {
                            for clause in clauses {
                                match clause.visit(emitter) {
                                    Err(msg) => {
                                        return Err(msg);
                                    }
                                    _ => continue,
                                }
                            }

                            Ok(TraversalResult::Continue)
                        }
                    };

                    ret
                }
                NodeFullExpression::ConstructorCall {
                    identifier_name,
                    argument_list,
                    ..
                } => {
                    let ret = match identifier_name.visit(emitter) {
                        Err(msg) => Err(msg),
                        _ => {
                            for arg in argument_list {
                                match arg.visit(emitter) {
                                    Err(msg) => {
                                        return Err(msg);
                                    }
                                    _ => continue,
                                }
                            }
                            Ok(TraversalResult::Continue)
                        }
                    };

                    ret
                }
                NodeFullExpression::TemplateFunction { expression, .. } => {
                    let ret = expression.visit(emitter);

                    ret
                }
                NodeFullExpression::TApp {
                    identifier_name,
                    type_arguments,
                    ..
                } => {
                    let ret = match identifier_name.visit(emitter) {
                        Err(msg) => Err(msg),
                        _ => {
                            for targ in type_arguments {
                                match targ.visit(emitter) {
                                    Err(msg) => {
                                        return Err(msg);
                                    }
                                    _ => continue,
                                }
                            }
                            Ok(TraversalResult::Continue)
                        }
                    };
                    ret
                }
            }
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => {
                emitter.emit_full_expression(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeMessageEntry {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_message_entry(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            match self {
                NodeMessageEntry::MessageLiteral(var_identifier, value_literal) => {
                    let ret = match var_identifier.visit(emitter) {
                        Err(msg) => Err(msg),
                        _ => value_literal.visit(emitter),
                    };

                    ret
                }
                NodeMessageEntry::MessageVariable(var_identifier1, var_identifier2) => {
                    let ret = match var_identifier1.visit(emitter) {
                        Err(msg) => Err(msg),
                        _ => {
                            emitter
                                .push_source_position(&var_identifier2.start, &var_identifier2.end);
                            var_identifier2.visit(emitter)
                        }
                    };
                    ret
                }
            }
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => emitter.emit_message_entry(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodePatternMatchExpressionClause {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_pattern_match_expression_clause(TreeTraversalMode::Enter, self);
        let pattern_ret = if ret == Ok(TraversalResult::Continue) {
            self.pattern.visit(emitter)
        } else {
            ret
        };
        let expression_ret = if pattern_ret == Ok(TraversalResult::Continue) {
            let ret = self.expression.visit(emitter);
            ret
        } else {
            pattern_ret
        }?;
        match expression_ret {
            TraversalResult::Continue => {
                emitter.emit_pattern_match_expression_clause(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeAtomicExpression {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_atomic_expression(TreeTraversalMode::Enter, self);
        // Only visit children if entering was successful and did not result in skipping
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            match self {
                NodeAtomicExpression::AtomicSid(sid) => {
                    let ret = sid.visit(emitter);
                    ret
                }
                NodeAtomicExpression::AtomicLit(lit) => {
                    let ret = lit.visit(emitter);
                    ret
                }
            }
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => {
                emitter.emit_atomic_expression(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeContractTypeArguments {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_contract_type_arguments(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            self.type_arguments
                .iter()
                .map(|child| {
                    let ret = child.visit(emitter);
                    ret
                })
                .find(|result| *result == Err("".into()))
                .unwrap_or(Ok(TraversalResult::Continue))
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => {
                emitter.emit_contract_type_arguments(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeValueLiteral {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_value_literal(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            match self {
                NodeValueLiteral::LiteralInt(type_name, _) => {
                    let ret = type_name.visit(emitter);
                    ret
                }
                NodeValueLiteral::LiteralEmptyMap(type_map_key, type_map_value) => {
                    let ret = match type_map_key.visit(emitter) {
                        Err(msg) => Err(msg),
                        _ => {
                            emitter
                                .push_source_position(&type_map_value.start, &type_map_value.end);
                            type_map_value.visit(emitter)
                        }
                    };
                    ret
                }
                _ => Ok(TraversalResult::Continue),
            }
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => emitter.emit_value_literal(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeMapAccess {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_map_access(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            let ret = self.identifier_name.visit(emitter);
            ret
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => emitter.emit_map_access(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodePattern {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_pattern(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            match self {
                NodePattern::Wildcard => Ok(TraversalResult::Continue),
                NodePattern::Binder(_) => Ok(TraversalResult::Continue),
                NodePattern::Constructor(identifier, argument_patterns) => {
                    let ret = match identifier.visit(emitter) {
                        Err(msg) => Err(msg),
                        _ => {
                            for pattern in argument_patterns {
                                let result = pattern.visit(emitter);
                                if let Err(msg) = result {
                                    return Err(msg);
                                }
                            }
                            Ok(TraversalResult::Continue)
                        }
                    };

                    ret
                }
            }
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => emitter.emit_pattern(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeArgumentPattern {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_argument_pattern(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            match self {
                NodeArgumentPattern::WildcardArgument => Ok(TraversalResult::Continue),
                NodeArgumentPattern::BinderArgument(_) => Ok(TraversalResult::Continue),
                NodeArgumentPattern::ConstructorArgument(meta_identifier) => {
                    let ret = meta_identifier.visit(emitter);

                    ret
                }
                NodeArgumentPattern::PatternArgument(pattern) => {
                    let ret = pattern.visit(emitter);
                    ret
                }
            }
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => {
                emitter.emit_argument_pattern(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodePatternMatchClause {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_pattern_match_clause(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            emitter
                .push_source_position(&self.pattern_expression.start, &self.pattern_expression.end);
            let ret = match self.pattern_expression.visit(emitter) {
                Err(msg) => Err(msg),
                _ => match &self.statement_block {
                    Some(stmt_block) => stmt_block.visit(emitter),
                    None => Ok(TraversalResult::Continue),
                },
            };

            ret
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => {
                emitter.emit_pattern_match_clause(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeBlockchainFetchArguments {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_blockchain_fetch_arguments(TreeTraversalMode::Enter, self);
        if let Ok(TraversalResult::Continue) = ret {
            // Visit each argument
            for arg in &self.arguments {
                match arg.visit(emitter) {
                    Err(msg) => {
                        return Err(msg);
                    }
                    _ => {}
                }
            }
        }
        match ret? {
            TraversalResult::Continue => {
                emitter.emit_blockchain_fetch_arguments(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeStatement {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_statement(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            match self {
                NodeStatement::Load {
                    right_hand_side, ..
                }
                | NodeStatement::Store {
                    right_hand_side, ..
                } => right_hand_side.visit(emitter),
                NodeStatement::RemoteFetch(statement) => statement.visit(emitter),
                NodeStatement::Bind {
                    right_hand_side, ..
                } => {
                    let ret = right_hand_side.visit(emitter);
                    ret
                }
                NodeStatement::ReadFromBC { arguments, .. } => {
                    if let Some(arg) = arguments {
                        let ret = arg.visit(emitter);

                        ret
                    } else {
                        Ok(TraversalResult::Continue)
                    }
                }
                NodeStatement::MapGet { keys, .. }
                | NodeStatement::MapUpdate { keys, .. }
                | NodeStatement::MapGetExists { keys, .. }
                | NodeStatement::MapUpdateDelete { keys, .. } => {
                    for key in keys {
                        let ret = key.visit(emitter);

                        if ret != Ok(TraversalResult::Continue) {
                            return ret;
                        }
                    }

                    Ok(TraversalResult::Continue)
                }
                NodeStatement::Send {
                    identifier_name, ..
                }
                | NodeStatement::CreateEvnt {
                    identifier_name, ..
                } => identifier_name.visit(emitter),
                NodeStatement::Throw { error_variable, .. } => {
                    if let Some(variable) = error_variable {
                        let ret = variable.visit(emitter);

                        ret
                    } else {
                        Ok(TraversalResult::Continue)
                    }
                }
                NodeStatement::MatchStmt {
                    variable, clauses, ..
                } => {
                    let ret = variable.visit(emitter);

                    if ret != Ok(TraversalResult::Continue) {
                        return ret;
                    }
                    for clause in clauses {
                        let ret = clause.visit(emitter);

                        if ret != Ok(TraversalResult::Continue) {
                            return ret;
                        }
                    }
                    Ok(TraversalResult::Continue)
                }
                NodeStatement::CallProc {
                    component_id,
                    arguments,
                    ..
                } => {
                    let ret = component_id.visit(emitter);
                    if ret != Ok(TraversalResult::Continue) {
                        return ret;
                    }
                    for argument in arguments {
                        let ret = argument.visit(emitter);
                        if ret != Ok(TraversalResult::Continue) {
                            return ret;
                        }
                    }
                    Ok(TraversalResult::Continue)
                }
                NodeStatement::Iterate {
                    identifier_name,
                    component_id,
                } => {
                    let ret = identifier_name.visit(emitter);
                    if ret != Ok(TraversalResult::Continue) {
                        return ret;
                    }

                    let ret = component_id.visit(emitter);

                    ret
                }
                _ => Ok(TraversalResult::Continue),
            }
        } else {
            ret
        };

        match children_ret? {
            TraversalResult::Continue => emitter.emit_statement(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeRemoteFetchStatement {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_remote_fetch_statement(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            match self {
                NodeRemoteFetchStatement::ReadStateMutable(_, _, variable) => {
                    let ret = variable.visit(emitter);

                    ret
                }
                NodeRemoteFetchStatement::ReadStateMutableSpecialId(_, _, _) => {
                    Ok(TraversalResult::Continue)
                }
                NodeRemoteFetchStatement::ReadStateMutableMapAccess(_, _, _, accesses) => {
                    for access in accesses {
                        let ret = access.visit(emitter);

                        if let Err(msg) = ret {
                            return Err(msg);
                        }
                    }
                    Ok(TraversalResult::Continue)
                }
                NodeRemoteFetchStatement::ReadStateMutableMapAccessExists(_, _, _, accesses) => {
                    for access in accesses {
                        let ret = access.visit(emitter);

                        if let Err(msg) = ret {
                            return Err(msg);
                        }
                    }
                    Ok(TraversalResult::Continue)
                }
                NodeRemoteFetchStatement::ReadStateMutableCastAddress(_, variable, address) => {
                    let ret = match variable.visit(emitter) {
                        Err(msg) => Err(msg),
                        _ => address.visit(emitter),
                    };

                    ret
                }
            }
        } else {
            ret
        };
        match children_ret? {
            TraversalResult::Continue => {
                emitter.emit_remote_fetch_statement(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeComponentId {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        // Emit enter event
        let ret = emitter.emit_component_id(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            // Handle child nodes
            match self {
                NodeComponentId::WithTypeLikeName(type_name_identifier) => {
                    let ret = type_name_identifier.visit(emitter);

                    ret
                }
                NodeComponentId::WithRegularId(_) => Ok(TraversalResult::Continue),
            }
        } else {
            ret
        };
        match children_ret? {
            TraversalResult::Continue => emitter.emit_component_id(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeComponentParameters {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_component_parameters(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            for param in &self.parameters {
                let ret = param.visit(emitter);

                match ret {
                    Err(msg) => return Err(msg),
                    _ => {}
                }
            }
            Ok(TraversalResult::Continue)
        } else {
            ret
        };
        match children_ret? {
            TraversalResult::Continue => {
                emitter.emit_component_parameters(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeParameterPair {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_parameter_pair(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            let ret = self.identifier_with_type.visit(emitter);

            ret
        } else {
            ret
        };
        match children_ret? {
            TraversalResult::Continue => emitter.emit_parameter_pair(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeComponentBody {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_component_body(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            if let Some(statement_block) = &self.statement_block {
                let ret = statement_block.visit(emitter);

                ret
            } else {
                Ok(TraversalResult::Continue)
            }
        } else {
            ret
        };
        match children_ret? {
            TraversalResult::Continue => emitter.emit_component_body(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeStatementBlock {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_statement_block(TreeTraversalMode::Enter, self);
        // Visit each statement if not skipping children
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            for statement in &self.statements {
                let ret = statement.visit(emitter);

                match ret {
                    Err(msg) => return Err(msg),
                    _ => (),
                }
            }
            Ok(TraversalResult::Continue)
        } else {
            ret
        };
        match children_ret? {
            TraversalResult::Continue => {
                emitter.emit_statement_block(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeTypedIdentifier {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_typed_identifier(TreeTraversalMode::Enter, self);
        // Visit the annotation child node if the enter phase didn't fail or skip children
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            let ret = self.annotation.visit(emitter);

            ret
        } else {
            ret
        };
        // Depending on the result of the children's visits, either fail or finish with the exit phase
        match children_ret? {
            TraversalResult::Continue => {
                emitter.emit_typed_identifier(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeTypeAnnotation {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_type_annotation(TreeTraversalMode::Enter, self);
        // Child element: self.type_name
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            let ret = self.type_name.visit(emitter);

            ret
        } else {
            ret
        };
        match children_ret? {
            TraversalResult::Continue => {
                emitter.emit_type_annotation(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeProgram {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        // Emit enter event
        let ret = emitter.emit_program(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            // Visit import_declarations if it's not None
            if let Some(import_declarations) = &self.import_declarations {
                let result = import_declarations.visit(emitter);

                if result != Ok(TraversalResult::Continue) {
                    return result;
                }
            }
            // Visit library_definition if it's not None
            if let Some(library_definition) = &self.library_definition {
                let result = library_definition.visit(emitter);

                if result != Ok(TraversalResult::Continue) {
                    return result;
                }
            }
            // Visit contract_definition

            let result = self.contract_definition.visit(emitter);
            result
        } else {
            ret
        };
        // Emit exit event
        match children_ret? {
            TraversalResult::Continue => emitter.emit_program(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeLibraryDefinition {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_library_definition(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            for definition in &self.definitions {
                let result = definition.visit(emitter);

                match result {
                    Err(msg) => return Err(msg),
                    _ => continue,
                };
            }
            Ok(TraversalResult::Continue)
        } else {
            ret
        };
        match children_ret? {
            TraversalResult::Continue => {
                emitter.emit_library_definition(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}
impl AstVisitor for NodeLibrarySingleDefinition {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_library_single_definition(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            match self {
                NodeLibrarySingleDefinition::LetDefinition {
                    variable_name: _,
                    type_annotation: _,
                    expression,
                } => {
                    // TODO: Unused variables aboce
                    let _ = expression.visit(emitter)?;

                    unimplemented!()
                }
                NodeLibrarySingleDefinition::TypeDefinition(name, option_clause) => {
                    let result = name.visit(emitter);

                    match result {
                        Err(msg) => Err(msg),
                        _ => match option_clause {
                            Some(clauses) => {
                                for clause in clauses {
                                    let result = clause.visit(emitter);

                                    if let Err(msg) = result {
                                        return Err(msg);
                                    }
                                }
                                Ok(TraversalResult::Continue)
                            }
                            None => Ok(TraversalResult::Continue),
                        },
                    }
                }
            }
        } else {
            ret
        };
        match children_ret? {
            TraversalResult::Continue => {
                emitter.emit_library_single_definition(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}
impl AstVisitor for NodeContractDefinition {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_contract_definition(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            if let Err(msg) = self.parameters.visit(emitter) {
                return Err(msg);
            }

            if let Some(constraint) = &self.constraint {
                if let Err(msg) = constraint.visit(emitter) {
                    return Err(msg);
                }
            }

            for field in &self.fields {
                if let Err(msg) = field.visit(emitter) {
                    return Err(msg);
                }
            }

            for component in &self.components {
                if let Err(msg) = component.visit(emitter) {
                    return Err(msg);
                }
            }

            Ok(TraversalResult::Continue)
        } else {
            ret
        };
        match children_ret? {
            TraversalResult::Continue => {
                emitter.emit_contract_definition(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeContractField {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_contract_field(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            let ret = match self.typed_identifier.visit(emitter) {
                Err(msg) => Err(msg),
                _ => self.right_hand_side.visit(emitter),
            };

            ret
        } else {
            ret
        };
        match children_ret? {
            TraversalResult::Continue => emitter.emit_contract_field(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}
impl AstVisitor for NodeWithConstraint {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_with_constraint(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            let ret = self.expression.visit(emitter);

            ret
        } else {
            ret
        };
        match children_ret? {
            TraversalResult::Continue => {
                emitter.emit_with_constraint(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}
impl AstVisitor for NodeComponentDefinition {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_component_definition(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            match self {
                NodeComponentDefinition::TransitionComponent(transition_definition) => {
                    let ret = transition_definition.visit(emitter);
                    ret
                }
                NodeComponentDefinition::ProcedureComponent(procedure_definition) => {
                    let ret = procedure_definition.visit(emitter);
                    ret
                }
            }
        } else {
            ret
        };
        match children_ret? {
            TraversalResult::Continue => {
                emitter.emit_component_definition(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}
impl AstVisitor for NodeProcedureDefinition {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_procedure_definition(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            let result = self.name.visit(emitter);
            let ret = match result {
                Err(msg) => Err(msg),
                _ => match self.parameters.visit(emitter) {
                    Err(msg) => Err(msg),
                    _ => self.body.visit(emitter),
                },
            };
            ret
        } else {
            ret
        };
        match children_ret? {
            TraversalResult::Continue => {
                emitter.emit_procedure_definition(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}
impl AstVisitor for NodeTransitionDefinition {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_transition_definition(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            let ret = match self.name.visit(emitter) {
                Err(msg) => Err(msg),
                _ => match self.parameters.visit(emitter) {
                    Err(msg) => Err(msg),
                    _ => self.body.visit(emitter),
                },
            };
            ret
        } else {
            ret
        };
        match children_ret? {
            TraversalResult::Continue => {
                emitter.emit_transition_definition(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeTypeAlternativeClause {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        match emitter.emit_type_alternative_clause(TreeTraversalMode::Enter, self)? {
            TraversalResult::SkipChildren => return Ok(TraversalResult::Continue),
            TraversalResult::Continue => (),
        }
        let children_ret = match self {
            NodeTypeAlternativeClause::ClauseType(type_name) => {
                let ret = type_name.visit(emitter);

                ret
            }
            NodeTypeAlternativeClause::ClauseTypeWithArgs(type_name, type_args) => {
                match type_name.visit(emitter) {
                    Err(msg) => {
                        return Err(msg);
                    }
                    _ => (),
                }

                for type_arg in type_args {
                    match type_arg.visit(emitter) {
                        Err(msg) => {
                            return Err(msg);
                        }
                        _ => (),
                    }
                }

                Ok(TraversalResult::Continue)
            }
        };
        match children_ret? {
            TraversalResult::Continue => {
                emitter.emit_type_alternative_clause(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeTypeMapValueArguments {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_type_map_value_arguments(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            match self {
                NodeTypeMapValueArguments::EnclosedTypeMapValue(enclosed) => {
                    let ret = enclosed.visit(emitter);

                    ret
                }
                NodeTypeMapValueArguments::GenericMapValueArgument(meta_identifier) => {
                    let ret = meta_identifier.visit(emitter);

                    ret
                }
                NodeTypeMapValueArguments::MapKeyValueType(key_type, value_type) => {
                    let ret = match key_type.visit(emitter) {
                        Err(msg) => Err(msg),
                        _ => value_type.visit(emitter),
                    };

                    ret
                }
            }
        } else {
            ret
        };
        match children_ret? {
            TraversalResult::Continue => {
                emitter.emit_type_map_value_arguments(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeTypeMapValueAllowingTypeArguments {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret =
            emitter.emit_type_map_value_allowing_type_arguments(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            match self {
                NodeTypeMapValueAllowingTypeArguments::TypeMapValueNoArgs(type_map_value) => {
                    let ret = type_map_value.visit(emitter);

                    ret
                }
                NodeTypeMapValueAllowingTypeArguments::TypeMapValueWithArgs(
                    meta_id,
                    value_args,
                ) => {
                    let mut ret = meta_id.visit(emitter);

                    if ret == Ok(TraversalResult::Continue) {
                        for value_arg in value_args {
                            ret = value_arg.visit(emitter);
                            if ret != Ok(TraversalResult::Continue) {
                                break;
                            }
                        }
                    }

                    ret
                }
            }
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => {
                emitter.emit_type_map_value_allowing_type_arguments(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}
