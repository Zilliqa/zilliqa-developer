use crate::ast::converting::AstConverting;
use crate::ast::nodes::*;
use crate::constants::{TraversalResult, TreeTraversalMode};

pub trait AstVisitor {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String>;
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
                    emitter.push_source_position(&bs_type.start, &bs_type.end);
                    let ret = bs_type.node.visit(emitter);
                    emitter.pop_source_position();
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
                    emitter.push_source_position(&name.start, &name.end);
                    let ret = name.node.visit(emitter);
                    emitter.pop_source_position();

                    ret
                }
                NodeImportedName::AliasedImport(name, alias) => {
                    emitter.push_source_position(&name.start, &name.end);
                    let ret = match name.node.visit(emitter) {
                        Err(msg) => Err(msg),
                        _ => {
                            emitter.pop_source_position();
                            emitter.push_source_position(&alias.start, &alias.end);
                            alias.node.visit(emitter)
                        }
                    };
                    emitter.pop_source_position();
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
                emitter.push_source_position(&import.start, &import.end);
                match import.node.visit(emitter) {
                    Err(msg) => {
                        emitter.pop_source_position();
                        return Err(msg);
                    }
                    _ => (),
                }
                emitter.pop_source_position();
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
                    emitter.push_source_position(&name.start, &name.end);
                    let ret = name.node.visit(emitter);
                    emitter.pop_source_position();
                    ret
                }
                NodeMetaIdentifier::MetaNameInNamespace(name, ns) => {
                    emitter.push_source_position(&name.start, &name.end);
                    let ret = match name.node.visit(emitter) {
                        Err(msg) => Err(msg),
                        _ => {
                            emitter.pop_source_position();
                            emitter.push_source_position(&ns.start, &ns.end);
                            ns.node.visit(emitter)
                        }
                    };
                    emitter.pop_source_position();
                    ret
                }
                NodeMetaIdentifier::MetaNameInHexspace(_, name) => {
                    emitter.push_source_position(&name.start, &name.end);
                    let ret = name.node.visit(emitter);
                    emitter.pop_source_position();
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
                    emitter.push_source_position(
                        &type_name_identifier.start,
                        &type_name_identifier.end,
                    );
                    let ret = type_name_identifier.node.visit(emitter);
                    emitter.pop_source_position();
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
                    emitter.push_source_position(&argument.start, &argument.end);
                    let ret = argument.node.visit(emitter);
                    emitter.pop_source_position();
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
                    emitter.push_source_position(&node_met_id.start, &node_met_id.end);
                    let ret = node_met_id.node.visit(emitter);
                    emitter.pop_source_position();
                    ret
                }
                NodeTypeMapKey::EnclosedGenericId(node_met_id) => {
                    emitter.push_source_position(&node_met_id.start, &node_met_id.end);
                    let ret = node_met_id.node.visit(emitter);
                    emitter.pop_source_position();
                    ret
                }
                NodeTypeMapKey::EnclosedAddressMapKeyType(node_address_type) => {
                    emitter.push_source_position(&node_address_type.start, &node_address_type.end);
                    let ret = node_address_type.node.visit(emitter);
                    emitter.pop_source_position();
                    ret
                }
                NodeTypeMapKey::AddressMapKeyType(node_address_type) => {
                    emitter.push_source_position(&node_address_type.start, &node_address_type.end);
                    let ret = node_address_type.node.visit(emitter);
                    emitter.pop_source_position();
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
                    emitter.push_source_position(&meta_id.start, &meta_id.end);
                    let ret = meta_id.node.visit(emitter);
                    emitter.pop_source_position();

                    ret
                }
                NodeTypeMapValue::MapKeyValue(entry) => {
                    emitter.push_source_position(&entry.start, &entry.end);
                    let ret = entry.node.visit(emitter);
                    emitter.pop_source_position();

                    ret
                }
                NodeTypeMapValue::MapValueParanthesizedType(value) => {
                    emitter.push_source_position(&value.start, &value.end);
                    let ret = value.node.visit(emitter);
                    emitter.pop_source_position();

                    ret
                }
                NodeTypeMapValue::MapValueAddressType(address_type) => {
                    emitter.push_source_position(&address_type.start, &address_type.end);
                    let ret = address_type.node.visit(emitter);
                    emitter.pop_source_position();

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
                    emitter.push_source_position(&node.start, &node.end);
                    let ret = node.node.visit(emitter);
                    emitter.pop_source_position();
                    ret
                }
                NodeTypeArgument::GenericTypeArgument(node) => {
                    emitter.push_source_position(&node.start, &node.end);
                    let ret = node.node.visit(emitter);
                    emitter.pop_source_position();
                    ret
                }
                NodeTypeArgument::TemplateTypeArgument(_) => Ok(TraversalResult::Continue),
                NodeTypeArgument::AddressTypeArgument(node) => {
                    emitter.push_source_position(&node.start, &node.end);
                    let ret = node.node.visit(emitter);
                    emitter.pop_source_position();
                    ret
                }
                NodeTypeArgument::MapTypeArgument(key_node, value_node) => {
                    emitter.push_source_position(&key_node.start, &key_node.end);
                    let ret = match key_node.node.visit(emitter) {
                        Ok(TraversalResult::Continue) => {
                            emitter.pop_source_position();
                            emitter.push_source_position(&value_node.start, &value_node.end);
                            value_node.node.visit(emitter)
                        }
                        Err(msg) => Err(msg),
                        _ => Ok(TraversalResult::Continue),
                    };
                    emitter.pop_source_position();

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
                    emitter.push_source_position(&id.start, &id.end);
                    let ret = match id.node.visit(emitter) {
                        Err(msg) => Err(msg),
                        _ => {
                            for arg in args {
                                emitter.pop_source_position();
                                emitter.push_source_position(&arg.start, &arg.end);
                                match arg.node.visit(emitter) {
                                    Err(msg) => {
                                        emitter.pop_source_position();
                                        return Err(msg);
                                    }
                                    _ => continue,
                                }
                            }
                            Ok(TraversalResult::Continue)
                        }
                    };
                    emitter.pop_source_position();

                    ret
                }
                NodeScillaType::MapType(key, value) => {
                    emitter.push_source_position(&key.start, &key.end);
                    let ret = match key.node.visit(emitter) {
                        Err(msg) => Err(msg),
                        _ => {
                            emitter.pop_source_position();
                            emitter.push_source_position(&value.start, &value.end);
                            value.node.visit(emitter)
                        }
                    };
                    emitter.pop_source_position();
                    ret
                }
                NodeScillaType::FunctionType(t1, t2) => {
                    emitter.push_source_position(&t1.start, &t1.end);
                    let ret = match t1.node.visit(emitter) {
                        Err(msg) => Err(msg),
                        _ => {
                            emitter.pop_source_position();
                            emitter.push_source_position(&t2.start, &t2.end);
                            t2.node.visit(emitter)
                        }
                    };
                    emitter.pop_source_position();
                    ret
                }
                NodeScillaType::PolyFunctionType(_, t) => {
                    emitter.push_source_position(&t.start, &t.end);
                    let ret = t.node.visit(emitter);
                    emitter.pop_source_position();
                    ret
                }
                NodeScillaType::EnclosedType(t) => {
                    emitter.push_source_position(&t.start, &t.end);
                    let ret = t.node.visit(emitter);
                    emitter.pop_source_position();
                    ret
                }
                NodeScillaType::ScillaAddresseType(t) => {
                    emitter.push_source_position(&t.start, &t.end);
                    let ret = t.node.visit(emitter);
                    emitter.pop_source_position();
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
            emitter.push_source_position(&self.key.start, &self.key.end);
            let ret = match self.key.node.visit(emitter) {
                Err(msg) => Err(msg),
                _ => {
                    emitter.pop_source_position();
                    emitter.push_source_position(&self.value.start, &self.value.end);
                    self.value.node.visit(emitter)
                }
            };

            emitter.pop_source_position();

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
            emitter.push_source_position(&self.identifier.start, &self.identifier.end);
            let ret = match self.identifier.node.visit(emitter) {
                Err(msg) => Err(msg),
                _ => {
                    emitter.pop_source_position();
                    emitter.push_source_position(&self.type_name.start, &self.type_name.end);
                    self.type_name.node.visit(emitter)
                }
            };
            emitter.pop_source_position();
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
            emitter.push_source_position(&self.identifier.start, &self.identifier.end);
            let ret = self.identifier.node.visit(emitter);
            emitter.pop_source_position();

            if ret.is_err() {
                return ret;
            }

            for field in &self.address_fields {
                emitter.push_source_position(&field.start, &field.end);
                let ret = field.node.visit(emitter);
                emitter.pop_source_position();

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
                    emitter.push_source_position(&expression.start, &expression.end);
                    let ret = match expression.node.visit(emitter) {
                        Err(msg) => Err(msg),
                        _ => {
                            emitter.pop_source_position();
                            emitter.push_source_position(
                                &containing_expression.start,
                                &containing_expression.end,
                            );
                            containing_expression.node.visit(emitter)
                        }
                    };
                    emitter.pop_source_position();
                    ret
                }
                NodeFullExpression::FunctionDeclaration { expression, .. } => {
                    emitter.push_source_position(&expression.start, &expression.end);
                    let ret = expression.node.visit(emitter);
                    emitter.pop_source_position();

                    ret
                }
                NodeFullExpression::FunctionCall {
                    function_name,
                    argument_list,
                    ..
                } => {
                    emitter.push_source_position(&function_name.start, &function_name.end);
                    let ret = match function_name.node.visit(emitter) {
                        Err(msg) => Err(msg),
                        _ => {
                            for arg in argument_list {
                                emitter.pop_source_position();
                                emitter.push_source_position(&arg.start, &arg.end);
                                match arg.node.visit(emitter) {
                                    Err(msg) => {
                                        emitter.pop_source_position();
                                        return Err(msg);
                                    }
                                    _ => continue,
                                }
                            }
                            Ok(TraversalResult::Continue)
                        }
                    };
                    emitter.pop_source_position();

                    ret
                }
                NodeFullExpression::ExpressionAtomic(atom_expr) => {
                    emitter.push_source_position(&atom_expr.start, &atom_expr.end);
                    let ret = atom_expr.node.visit(emitter);
                    emitter.pop_source_position();

                    ret
                }
                NodeFullExpression::ExpressionBuiltin { xs, .. } => {
                    emitter.push_source_position(&xs.start, &xs.end);
                    let ret = xs.node.visit(emitter);
                    emitter.pop_source_position();

                    ret
                }
                NodeFullExpression::Message(message_entries) => {
                    for entry in message_entries {
                        emitter.push_source_position(&entry.start, &entry.end);
                        match entry.node.visit(emitter) {
                            Err(msg) => {
                                emitter.pop_source_position();
                                return Err(msg);
                            }
                            _ => (),
                        }
                        emitter.pop_source_position();
                    }

                    Ok(TraversalResult::Continue)
                }
                NodeFullExpression::Match {
                    match_expression,
                    clauses,
                    ..
                } => {
                    emitter.push_source_position(&match_expression.start, &match_expression.end);
                    let ret = match match_expression.node.visit(emitter) {
                        Err(msg) => Err(msg),
                        _ => {
                            for clause in clauses {
                                emitter.pop_source_position();
                                emitter.push_source_position(&clause.start, &clause.end);
                                match clause.node.visit(emitter) {
                                    Err(msg) => {
                                        emitter.pop_source_position();
                                        return Err(msg);
                                    }
                                    _ => continue,
                                }
                            }

                            Ok(TraversalResult::Continue)
                        }
                    };

                    emitter.pop_source_position();
                    ret
                }
                NodeFullExpression::ConstructorCall {
                    identifier_name,
                    argument_list,
                    ..
                } => {
                    emitter.push_source_position(&identifier_name.start, &identifier_name.end);
                    let ret = match identifier_name.node.visit(emitter) {
                        Err(msg) => Err(msg),
                        _ => {
                            for arg in argument_list {
                                emitter.pop_source_position();
                                emitter.push_source_position(&arg.start, &arg.end);
                                match arg.node.visit(emitter) {
                                    Err(msg) => {
                                        emitter.pop_source_position();
                                        return Err(msg);
                                    }
                                    _ => continue,
                                }
                            }
                            Ok(TraversalResult::Continue)
                        }
                    };
                    emitter.pop_source_position();

                    ret
                }
                NodeFullExpression::TemplateFunction { expression, .. } => {
                    emitter.push_source_position(&expression.start, &expression.end);
                    let ret = expression.node.visit(emitter);
                    emitter.pop_source_position();

                    ret
                }
                NodeFullExpression::TApp {
                    identifier_name,
                    type_arguments,
                    ..
                } => {
                    emitter.push_source_position(&identifier_name.start, &identifier_name.end);
                    let ret = match identifier_name.node.visit(emitter) {
                        Err(msg) => Err(msg),
                        _ => {
                            for targ in type_arguments {
                                emitter.pop_source_position();
                                emitter.push_source_position(&targ.start, &targ.end);
                                match targ.node.visit(emitter) {
                                    Err(msg) => {
                                        emitter.pop_source_position();
                                        return Err(msg);
                                    }
                                    _ => continue,
                                }
                            }
                            Ok(TraversalResult::Continue)
                        }
                    };
                    emitter.pop_source_position();
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
                    emitter.push_source_position(&var_identifier.start, &var_identifier.end);
                    let ret = match var_identifier.node.visit(emitter) {
                        Err(msg) => Err(msg),
                        _ => {
                            emitter.pop_source_position();
                            emitter.push_source_position(&value_literal.start, &value_literal.end);
                            value_literal.node.visit(emitter)
                        }
                    };
                    emitter.pop_source_position();

                    ret
                }
                NodeMessageEntry::MessageVariable(var_identifier1, var_identifier2) => {
                    emitter.push_source_position(&var_identifier1.start, &var_identifier1.end);
                    let ret = match var_identifier1.node.visit(emitter) {
                        Err(msg) => Err(msg),
                        _ => {
                            emitter.pop_source_position();
                            emitter
                                .push_source_position(&var_identifier2.start, &var_identifier2.end);
                            var_identifier2.node.visit(emitter)
                        }
                    };
                    emitter.pop_source_position();
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
            self.pattern.node.visit(emitter)
        } else {
            ret
        };
        let expression_ret = if pattern_ret == Ok(TraversalResult::Continue) {
            emitter.push_source_position(&self.expression.start, &self.expression.end);
            let ret = self.expression.node.visit(emitter);
            emitter.pop_source_position();
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
                    emitter.push_source_position(&sid.start, &sid.end);
                    let ret = sid.node.visit(emitter);
                    emitter.pop_source_position();
                    ret
                }
                NodeAtomicExpression::AtomicLit(lit) => {
                    emitter.push_source_position(&lit.start, &lit.end);
                    let ret = lit.node.visit(emitter);
                    emitter.pop_source_position();
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
                    emitter.push_source_position(&child.start, &child.end);
                    let ret = child.node.visit(emitter);
                    emitter.pop_source_position();
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
                    emitter.push_source_position(&type_name.start, &type_name.end);
                    let ret = type_name.node.visit(emitter);
                    emitter.pop_source_position();
                    ret
                }
                NodeValueLiteral::LiteralEmptyMap(type_map_key, type_map_value) => {
                    emitter.push_source_position(&type_map_key.start, &type_map_key.end);
                    let ret = match type_map_key.node.visit(emitter) {
                        Err(msg) => Err(msg),
                        _ => {
                            emitter.pop_source_position();
                            emitter
                                .push_source_position(&type_map_value.start, &type_map_value.end);
                            type_map_value.node.visit(emitter)
                        }
                    };
                    emitter.pop_source_position();
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
            emitter.push_source_position(&self.identifier_name.start, &self.identifier_name.end);
            let ret = self.identifier_name.node.visit(emitter);
            emitter.pop_source_position();
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
                    emitter.push_source_position(&identifier.start, &identifier.end);
                    let ret = match identifier.node.visit(emitter) {
                        Err(msg) => Err(msg),
                        _ => {
                            for pattern in argument_patterns {
                                emitter.pop_source_position();
                                emitter.push_source_position(&pattern.start, &pattern.end);
                                let result = pattern.node.visit(emitter);
                                if let Err(msg) = result {
                                    emitter.pop_source_position();
                                    return Err(msg);
                                }
                            }
                            Ok(TraversalResult::Continue)
                        }
                    };
                    emitter.pop_source_position();

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
                    emitter.push_source_position(&meta_identifier.start, &meta_identifier.end);
                    let ret = meta_identifier.node.visit(emitter);

                    ret
                }
                NodeArgumentPattern::PatternArgument(pattern) => {
                    emitter.push_source_position(&pattern.start, &pattern.end);
                    let ret = pattern.node.visit(emitter);
                    emitter.pop_source_position();
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
            let ret = match self.pattern_expression.node.visit(emitter) {
                Err(msg) => Err(msg),
                _ => match &self.statement_block {
                    Some(stmt_block) => {
                        emitter.pop_source_position();
                        emitter.push_source_position(&stmt_block.start, &stmt_block.end);
                        stmt_block.node.visit(emitter)
                    }
                    None => Ok(TraversalResult::Continue),
                },
            };
            emitter.pop_source_position();

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
                emitter.push_source_position(&arg.start, &arg.end);
                match arg.node.visit(emitter) {
                    Err(msg) => {
                        emitter.pop_source_position();
                        return Err(msg);
                    }
                    _ => {}
                }
                emitter.pop_source_position();
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
                } => right_hand_side.node.visit(emitter),
                NodeStatement::RemoteFetch(statement) => statement.visit(emitter),
                NodeStatement::Bind {
                    right_hand_side, ..
                } => {
                    emitter.push_source_position(&right_hand_side.start, &right_hand_side.end);
                    let ret = right_hand_side.node.visit(emitter);
                    emitter.pop_source_position();
                    ret
                }
                NodeStatement::ReadFromBC { arguments, .. } => {
                    if let Some(arg) = arguments {
                        // TODO: emitter.push_source_position(&arg.start, &arg.end);

                        let ret = arg.visit(emitter);
                        // TODO: emitter.pop_source_position();

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
                        emitter.push_source_position(&key.start, &key.end);

                        let ret = key.node.visit(emitter);
                        emitter.pop_source_position();

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
                } => identifier_name.node.visit(emitter),
                NodeStatement::Throw { error_variable, .. } => {
                    if let Some(variable) = error_variable {
                        emitter.push_source_position(&variable.start, &variable.end);

                        let ret = variable.node.visit(emitter);
                        emitter.pop_source_position();

                        ret
                    } else {
                        Ok(TraversalResult::Continue)
                    }
                }
                NodeStatement::MatchStmt {
                    variable, clauses, ..
                } => {
                    emitter.push_source_position(&variable.start, &variable.end);
                    let ret = variable.node.visit(emitter);
                    emitter.pop_source_position();

                    if ret != Ok(TraversalResult::Continue) {
                        return ret;
                    }
                    for clause in clauses {
                        emitter.push_source_position(&clause.start, &clause.end);

                        let ret = clause.node.visit(emitter);
                        emitter.pop_source_position();

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
                    emitter.push_source_position(&component_id.start, &component_id.end);
                    let ret = component_id.node.visit(emitter);
                    emitter.pop_source_position();
                    if ret != Ok(TraversalResult::Continue) {
                        return ret;
                    }
                    for argument in arguments {
                        emitter.push_source_position(&argument.start, &argument.end);
                        let ret = argument.node.visit(emitter);
                        emitter.pop_source_position();
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
                    emitter.push_source_position(&identifier_name.start, &identifier_name.end);
                    let ret = identifier_name.node.visit(emitter);
                    emitter.pop_source_position();
                    if ret != Ok(TraversalResult::Continue) {
                        return ret;
                    }

                    emitter.push_source_position(&component_id.start, &component_id.end);
                    let ret = component_id.node.visit(emitter);
                    emitter.pop_source_position();

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
                    let ret = variable.node.visit(emitter);

                    ret
                }
                NodeRemoteFetchStatement::ReadStateMutableSpecialId(_, _, _) => {
                    Ok(TraversalResult::Continue)
                }
                NodeRemoteFetchStatement::ReadStateMutableMapAccess(_, _, _, accesses) => {
                    for access in accesses {
                        emitter.push_source_position(&access.start, &access.end);
                        let ret = access.node.visit(emitter);
                        emitter.pop_source_position();

                        if let Err(msg) = ret {
                            return Err(msg);
                        }
                    }
                    Ok(TraversalResult::Continue)
                }
                NodeRemoteFetchStatement::ReadStateMutableMapAccessExists(_, _, _, accesses) => {
                    for access in accesses {
                        emitter.push_source_position(&access.start, &access.end);
                        let ret = access.node.visit(emitter);
                        emitter.pop_source_position();

                        if let Err(msg) = ret {
                            return Err(msg);
                        }
                    }
                    Ok(TraversalResult::Continue)
                }
                NodeRemoteFetchStatement::ReadStateMutableCastAddress(_, variable, address) => {
                    emitter.push_source_position(&variable.start, &variable.end);
                    let ret = match variable.node.visit(emitter) {
                        Err(msg) => Err(msg),
                        _ => {
                            emitter.pop_source_position();
                            emitter.push_source_position(&address.start, &address.end);
                            address.node.visit(emitter)
                        }
                    };
                    emitter.pop_source_position();

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
                    emitter.push_source_position(
                        &type_name_identifier.start,
                        &type_name_identifier.end,
                    );
                    let ret = type_name_identifier.node.visit(emitter);
                    emitter.pop_source_position();

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
                emitter.push_source_position(&param.start, &param.end);
                let ret = param.node.visit(emitter);
                emitter.pop_source_position();

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
            emitter.push_source_position(
                &self.identifier_with_type.start,
                &self.identifier_with_type.end,
            );
            let ret = self.identifier_with_type.node.visit(emitter);
            emitter.pop_source_position();

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
                emitter.push_source_position(&statement_block.start, &statement_block.end);
                let ret = statement_block.node.visit(emitter);
                emitter.pop_source_position();

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
                // TODO: emitter.push_source_position(&statement.start, &statement.end);
                let ret = statement.visit(emitter);
                // TODO: emitter.pop_source_position();

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
            emitter.push_source_position(&self.annotation.start, &self.annotation.end);
            let ret = self.annotation.node.visit(emitter);
            emitter.pop_source_position();

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
            emitter.push_source_position(&self.type_name.start, &self.type_name.end);
            let ret = self.type_name.node.visit(emitter);
            emitter.pop_source_position();

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
                emitter.push_source_position(&import_declarations.start, &import_declarations.end);
                let result = import_declarations.node.visit(emitter);
                emitter.pop_source_position();

                if result != Ok(TraversalResult::Continue) {
                    return result;
                }
            }
            // Visit library_definition if it's not None
            if let Some(library_definition) = &self.library_definition {
                emitter.push_source_position(&library_definition.start, &library_definition.end);
                let result = library_definition.node.visit(emitter);
                emitter.pop_source_position();

                if result != Ok(TraversalResult::Continue) {
                    return result;
                }
            }
            // Visit contract_definition

            let result = self.contract_definition.node.visit(emitter);
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
                emitter.push_source_position(&definition.start, &definition.end);
                let result = definition.node.visit(emitter);
                emitter.pop_source_position();

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
                    emitter.push_source_position(&expression.start, &expression.end);
                    let _ = expression.node.visit(emitter)?;
                    emitter.pop_source_position();

                    unimplemented!()
                }
                NodeLibrarySingleDefinition::TypeDefinition(name, option_clause) => {
                    emitter.push_source_position(&name.start, &name.end);
                    let result = name.node.visit(emitter);
                    emitter.pop_source_position();

                    match result {
                        Err(msg) => Err(msg),
                        _ => match option_clause {
                            Some(clauses) => {
                                for clause in clauses {
                                    emitter.push_source_position(&clause.start, &clause.end);
                                    let result = clause.node.visit(emitter);
                                    emitter.pop_source_position();

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
            emitter.push_source_position(&self.parameters.start, &self.parameters.end);

            if let Err(msg) = self.parameters.node.visit(emitter) {
                emitter.pop_source_position();
                return Err(msg);
            }

            if let Some(constraint) = &self.constraint {
                emitter.pop_source_position();
                emitter.push_source_position(&constraint.start, &constraint.end);
                if let Err(msg) = constraint.node.visit(emitter) {
                    return Err(msg);
                }
            }

            for field in &self.fields {
                emitter.pop_source_position();
                emitter.push_source_position(&field.start, &field.end);
                if let Err(msg) = field.node.visit(emitter) {
                    return Err(msg);
                }
            }

            for component in &self.components {
                emitter.pop_source_position();
                emitter.push_source_position(&component.start, &component.end);
                if let Err(msg) = component.node.visit(emitter) {
                    return Err(msg);
                }
            }

            emitter.pop_source_position();
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
            emitter.push_source_position(&self.typed_identifier.start, &self.typed_identifier.end);
            let ret = match self.typed_identifier.node.visit(emitter) {
                Err(msg) => Err(msg),
                _ => {
                    emitter.pop_source_position();
                    emitter.push_source_position(
                        &self.right_hand_side.start,
                        &self.right_hand_side.end,
                    );
                    self.right_hand_side.node.visit(emitter)
                }
            };

            emitter.pop_source_position();
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
            emitter.push_source_position(&self.expression.start, &self.expression.end);
            let ret = self.expression.node.visit(emitter);
            emitter.pop_source_position();

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
                    emitter.push_source_position(
                        &transition_definition.start,
                        &transition_definition.end,
                    );
                    let ret = transition_definition.node.visit(emitter);
                    emitter.pop_source_position();
                    ret
                }
                NodeComponentDefinition::ProcedureComponent(procedure_definition) => {
                    emitter.push_source_position(
                        &procedure_definition.start,
                        &procedure_definition.end,
                    );
                    let ret = procedure_definition.node.visit(emitter);
                    emitter.pop_source_position();
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
            emitter.push_source_position(&self.name.start, &self.name.end);
            let result = self.name.node.visit(emitter);
            let ret = match result {
                Err(msg) => Err(msg),
                _ => {
                    emitter.pop_source_position();
                    emitter.push_source_position(&self.parameters.start, &self.parameters.end);
                    match self.parameters.node.visit(emitter) {
                        Err(msg) => Err(msg),
                        _ => {
                            emitter.pop_source_position();
                            emitter.push_source_position(&self.body.start, &self.body.end);
                            self.body.node.visit(emitter)
                        }
                    }
                }
            };
            emitter.pop_source_position();
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
            emitter.push_source_position(&self.name.start, &self.name.end);
            let ret = match self.name.node.visit(emitter) {
                Err(msg) => Err(msg),
                _ => {
                    emitter.pop_source_position();
                    emitter.push_source_position(&self.parameters.start, &self.parameters.end);
                    match self.parameters.node.visit(emitter) {
                        Err(msg) => Err(msg),
                        _ => {
                            emitter.pop_source_position();
                            emitter.push_source_position(&self.body.start, &self.body.end);
                            self.body.node.visit(emitter)
                        }
                    }
                }
            };
            emitter.pop_source_position();
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
                emitter.push_source_position(&type_name.start, &type_name.end);
                let ret = type_name.node.visit(emitter);

                emitter.pop_source_position();
                ret
            }
            NodeTypeAlternativeClause::ClauseTypeWithArgs(type_name, type_args) => {
                emitter.push_source_position(&type_name.start, &type_name.end);
                match type_name.node.visit(emitter) {
                    Err(msg) => {
                        emitter.pop_source_position();
                        return Err(msg);
                    }
                    _ => (),
                }

                for type_arg in type_args {
                    emitter.pop_source_position();
                    emitter.push_source_position(&type_arg.start, &type_arg.end);
                    match type_arg.node.visit(emitter) {
                        Err(msg) => {
                            emitter.pop_source_position();
                            return Err(msg);
                        }
                        _ => (),
                    }
                }

                emitter.pop_source_position();
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
                    emitter.push_source_position(&enclosed.start, &enclosed.end);
                    let ret = enclosed.node.visit(emitter);
                    emitter.pop_source_position();

                    ret
                }
                NodeTypeMapValueArguments::GenericMapValueArgument(meta_identifier) => {
                    emitter.push_source_position(&meta_identifier.start, &meta_identifier.end);
                    let ret = meta_identifier.node.visit(emitter);
                    emitter.pop_source_position();

                    ret
                }
                NodeTypeMapValueArguments::MapKeyValueType(key_type, value_type) => {
                    emitter.push_source_position(&key_type.start, &key_type.end);
                    let ret = match key_type.node.visit(emitter) {
                        Err(msg) => Err(msg),
                        _ => {
                            emitter.pop_source_position();
                            emitter.push_source_position(&value_type.start, &value_type.end);
                            value_type.node.visit(emitter)
                        }
                    };
                    emitter.pop_source_position();

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
                    let ret = type_map_value.node.visit(emitter);

                    ret
                }
                NodeTypeMapValueAllowingTypeArguments::TypeMapValueWithArgs(
                    meta_id,
                    value_args,
                ) => {
                    emitter.push_source_position(&meta_id.start, &meta_id.end);
                    let mut ret = meta_id.node.visit(emitter);

                    if ret == Ok(TraversalResult::Continue) {
                        for value_arg in value_args {
                            emitter.pop_source_position();
                            emitter.push_source_position(&value_arg.start, &value_arg.end);
                            ret = value_arg.node.visit(emitter);
                            if ret != Ok(TraversalResult::Continue) {
                                break;
                            }
                        }
                    }

                    emitter.pop_source_position();

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
