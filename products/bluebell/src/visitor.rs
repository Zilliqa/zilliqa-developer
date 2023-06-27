use crate::ast::*;
use crate::code_emitter::{CodeEmitter, TraversalResult, TreeTraversalMode};

pub trait Visitor {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult;
}

impl Visitor for NodeByteStr {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        let ret = emitter.emit_byte_str(TreeTraversalMode::Enter, self);

        // No children

        match ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => emitter.emit_byte_str(TreeTraversalMode::Exit, self),
            _ => TraversalResult::Ok,
        }
    }
}

impl Visitor for NodeTypeNameIdentifier {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        let ret = emitter.emit_type_name_identifier(TreeTraversalMode::Enter, self);

        let children_ret = if ret == TraversalResult::Ok {
            match self {
                NodeTypeNameIdentifier::ByteStringType(bs_type) => bs_type.visit(emitter),
                _ => TraversalResult::Ok,
            }
        } else {
            ret
        };

        match children_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => emitter.emit_type_name_identifier(TreeTraversalMode::Exit, self),
            _ => TraversalResult::Ok,
        }
    }
}

impl Visitor for NodeImportedName {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        let ret = emitter.emit_imported_name(TreeTraversalMode::Enter, self);

        let children_ret = if ret == TraversalResult::Ok {
            match self {
                NodeImportedName::RegularImport(name) => name.visit(emitter),
                NodeImportedName::AliasedImport(name, alias) => match name.visit(emitter) {
                    TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
                    _ => alias.visit(emitter),
                },
            }
        } else {
            ret
        };

        match children_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => emitter.emit_imported_name(TreeTraversalMode::Exit, self),
            _ => TraversalResult::Ok,
        }
    }
}

impl Visitor for NodeImportDeclarations {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        let ret = emitter.emit_import_declarations(TreeTraversalMode::Enter, self);
        let children_ret = if ret == TraversalResult::Ok {
            for import in &self.import_list {
                match import.visit(emitter) {
                    TraversalResult::Fail(msg) => return TraversalResult::Fail(msg),
                    _ => (),
                }
            }
            TraversalResult::Ok
        } else {
            ret
        };
        match children_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => emitter.emit_import_declarations(TreeTraversalMode::Exit, self),
            _ => TraversalResult::Ok,
        }
    }
}

impl Visitor for NodeMetaIdentifier {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        let ret = emitter.emit_meta_identifier(TreeTraversalMode::Enter, self);
        let children_ret = if ret == TraversalResult::Ok {
            match self {
                NodeMetaIdentifier::MetaName(name) => name.visit(emitter),
                NodeMetaIdentifier::MetaNameInNamespace(name, ns) => match name.visit(emitter) {
                    TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
                    _ => ns.visit(emitter),
                },
                NodeMetaIdentifier::MetaNameInHexspace(_, name) => name.visit(emitter),
                NodeMetaIdentifier::ByteString => TraversalResult::Ok,
            }
        } else {
            ret
        };
        match children_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => emitter.emit_meta_identifier(TreeTraversalMode::Exit, self),
            _ => TraversalResult::Ok,
        }
    }
}

impl Visitor for NodeVariableIdentifier {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        let ret = emitter.emit_variable_identifier(TreeTraversalMode::Enter, self);
        let children_ret = if ret == TraversalResult::Ok {
            match self {
                NodeVariableIdentifier::VariableInNamespace(type_name_identifier, _) => {
                    type_name_identifier.visit(emitter)
                }
                // Since VariableName and SpecialIdentifier don't have children
                // we can directly return ret here.
                _ => ret,
            }
        } else {
            ret
        };
        match children_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => emitter.emit_variable_identifier(TreeTraversalMode::Exit, self),
            _ => TraversalResult::Ok,
        }
    }
}

impl Visitor for NodeBuiltinArguments {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        // Call the code emitter at the entry of the NodeBuiltinArguments
        let ret = emitter.emit_builtin_arguments(TreeTraversalMode::Enter, self);
        // Call the visitor on all children of NodeBuiltinArguments if ret == TraversalResult::Ok
        let children_ret = if ret == TraversalResult::Ok {
            // Visit each of the arguments
            self.arguments
                .iter()
                .map(|argument| argument.visit(emitter))
                .find(|r| *r == TraversalResult::Fail(String::from("Failure")))
                .unwrap_or(TraversalResult::Ok)
        } else {
            ret
        };
        // Call the code emitter at the exit of the NodeBuiltinArguments
        match children_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => emitter.emit_builtin_arguments(TreeTraversalMode::Exit, self),
            _ => TraversalResult::Ok,
        }
    }
}

impl Visitor for NodeTypeMapKey {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        let ret = emitter.emit_type_map_key(TreeTraversalMode::Enter, self);
        let children_ret = if ret == TraversalResult::Ok {
            match self {
                NodeTypeMapKey::GenericMapKey(node_met_id) => node_met_id.visit(emitter),
                NodeTypeMapKey::EnclosedGenericId(node_met_id) => node_met_id.visit(emitter),
                NodeTypeMapKey::EnclosedAddressMapKeyType(node_address_type) => {
                    node_address_type.visit(emitter)
                }
                NodeTypeMapKey::AddressMapKeyType(node_address_type) => {
                    node_address_type.visit(emitter)
                }
            }
        } else {
            ret
        };
        match children_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => emitter.emit_type_map_key(TreeTraversalMode::Exit, self),
            _ => TraversalResult::Ok,
        }
    }
}

impl Visitor for NodeTypeMapValue {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        let ret = emitter.emit_type_map_value(TreeTraversalMode::Enter, self);
        let children_ret = if ret == TraversalResult::Ok {
            match self {
                NodeTypeMapValue::MapValueTypeOrEnumLikeIdentifier(meta_id) => {
                    meta_id.visit(emitter)
                }
                NodeTypeMapValue::MapKeyValue(entry) => entry.visit(emitter),
                NodeTypeMapValue::MapValueParanthesizedType(value) => value.visit(emitter),
                NodeTypeMapValue::MapValueAddressType(address_type) => address_type.visit(emitter),
            }
        } else {
            ret
        };
        match children_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => emitter.emit_type_map_value(TreeTraversalMode::Exit, self),
            _ => TraversalResult::Ok,
        }
    }
}

impl Visitor for NodeTypeArgument {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        let ret = emitter.emit_type_argument(TreeTraversalMode::Enter, self);
        let children_ret = if ret == TraversalResult::Ok {
            match self {
                NodeTypeArgument::EnclosedTypeArgument(node) => node.visit(emitter),
                NodeTypeArgument::GenericTypeArgument(node) => node.visit(emitter),
                NodeTypeArgument::TemplateTypeArgument(_) => TraversalResult::Ok,
                NodeTypeArgument::AddressTypeArgument(node) => node.visit(emitter),
                NodeTypeArgument::MapTypeArgument(key_node, value_node) => {
                    match key_node.visit(emitter) {
                        TraversalResult::Ok => value_node.visit(emitter),
                        TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
                        _ => TraversalResult::Ok,
                    }
                }
            }
        } else {
            ret
        };
        match children_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => emitter.emit_type_argument(TreeTraversalMode::Exit, self),
            _ => TraversalResult::Ok,
        }
    }
}

impl Visitor for NodeScillaType {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        let ret = emitter.emit_scilla_type(TreeTraversalMode::Enter, self);
        let children_ret = if ret == TraversalResult::Ok {
            match self {
                NodeScillaType::GenericTypeWithArgs(id, args) => match id.visit(emitter) {
                    TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
                    _ => {
                        for arg in args {
                            match arg.visit(emitter) {
                                TraversalResult::Fail(msg) => return TraversalResult::Fail(msg),
                                _ => continue,
                            }
                        }
                        TraversalResult::Ok
                    }
                },
                NodeScillaType::MapType(key, value) => match key.visit(emitter) {
                    TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
                    _ => value.visit(emitter),
                },
                NodeScillaType::FunctionType(t1, t2) => match t1.visit(emitter) {
                    TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
                    _ => t2.visit(emitter),
                },
                NodeScillaType::PolyFunctionType(_, t) => t.visit(emitter),
                NodeScillaType::EnclosedType(t) => t.visit(emitter),
                NodeScillaType::ScillaAddresseType(t) => t.visit(emitter),
                NodeScillaType::TypeVarType(_) => TraversalResult::Ok,
            }
        } else {
            ret
        };
        match children_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => emitter.emit_scilla_type(TreeTraversalMode::Exit, self),
            _ => TraversalResult::Ok,
        }
    }
}

impl Visitor for NodeTypeMapEntry {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        let ret = emitter.emit_type_map_entry(TreeTraversalMode::Enter, self);
        let children_ret = if ret == TraversalResult::Ok {
            match self.key.visit(emitter) {
                TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
                _ => self.value.visit(emitter),
            }
        } else {
            ret
        };
        match children_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => emitter.emit_type_map_entry(TreeTraversalMode::Exit, self),
            _ => TraversalResult::Ok,
        }
    }
}

impl Visitor for NodeAddressTypeField {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        let ret = emitter.emit_address_type_field(TreeTraversalMode::Enter, self);
        let children_ret = if ret == TraversalResult::Ok {
            match self.identifier.visit(emitter) {
                TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
                _ => self.type_name.visit(emitter),
            }
        } else {
            ret
        };
        match children_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => emitter.emit_address_type_field(TreeTraversalMode::Exit, self),
            _ => TraversalResult::Ok,
        }
    }
}

impl Visitor for NodeAddressType {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        let ret = emitter.emit_address_type(TreeTraversalMode::Enter, self);
        let children_ret = if ret == TraversalResult::Ok {
            let mut ret = self.identifier.visit(emitter);
            for field in &self.address_fields {
                match ret {
                    TraversalResult::Fail(msg) => return TraversalResult::Fail(msg),
                    _ => ret = field.visit(emitter),
                }
            }
            ret
        } else {
            ret
        };
        match children_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => emitter.emit_address_type(TreeTraversalMode::Exit, self),
            _ => TraversalResult::Ok,
        }
    }
}

impl Visitor for NodeFullExpression {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        let ret = emitter.emit_full_expression(TreeTraversalMode::Enter, self);
        let children_ret = if ret == TraversalResult::Ok {
            match self {
                NodeFullExpression::LocalVariableDeclaration {
                    expression,
                    containing_expression,
                    ..
                } => match expression.visit(emitter) {
                    TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
                    _ => containing_expression.visit(emitter),
                },
                NodeFullExpression::FunctionDeclaration { expression, .. } => {
                    expression.visit(emitter)
                }
                NodeFullExpression::FunctionCall {
                    function_name,
                    argument_list,
                    ..
                } => match function_name.visit(emitter) {
                    TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
                    _ => {
                        for arg in argument_list {
                            match arg.visit(emitter) {
                                TraversalResult::Fail(msg) => return TraversalResult::Fail(msg),
                                _ => continue,
                            }
                        }
                        TraversalResult::Ok
                    }
                },
                NodeFullExpression::ExpressionAtomic(atom_expr) => atom_expr.visit(emitter),
                NodeFullExpression::ExpressionBuiltin { xs, .. } => xs.visit(emitter),
                NodeFullExpression::Message(message_entries) => {
                    for entry in message_entries {
                        match entry.visit(emitter) {
                            TraversalResult::Fail(msg) => return TraversalResult::Fail(msg),
                            _ => continue,
                        }
                    }
                    TraversalResult::Ok
                }
                NodeFullExpression::Match {
                    match_expression,
                    clauses,
                    ..
                } => match match_expression.visit(emitter) {
                    TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
                    _ => {
                        for clause in clauses {
                            match clause.visit(emitter) {
                                TraversalResult::Fail(msg) => return TraversalResult::Fail(msg),
                                _ => continue,
                            }
                        }
                        TraversalResult::Ok
                    }
                },
                NodeFullExpression::ConstructorCall {
                    identifier_name,
                    argument_list,
                    ..
                } => match identifier_name.visit(emitter) {
                    TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
                    _ => {
                        for arg in argument_list {
                            match arg.visit(emitter) {
                                TraversalResult::Fail(msg) => return TraversalResult::Fail(msg),
                                _ => continue,
                            }
                        }
                        TraversalResult::Ok
                    }
                },
                NodeFullExpression::TemplateFunction { expression, .. } => {
                    expression.visit(emitter)
                }
                NodeFullExpression::TApp {
                    identifier_name,
                    type_arguments,
                    ..
                } => match identifier_name.visit(emitter) {
                    TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
                    _ => {
                        for targ in type_arguments {
                            match targ.visit(emitter) {
                                TraversalResult::Fail(msg) => return TraversalResult::Fail(msg),
                                _ => continue,
                            }
                        }
                        TraversalResult::Ok
                    }
                },
            }
        } else {
            ret
        };
        match children_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => emitter.emit_full_expression(TreeTraversalMode::Exit, self),
            _ => TraversalResult::Ok,
        }
    }
}

impl Visitor for NodeMessageEntry {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        let ret = emitter.emit_message_entry(TreeTraversalMode::Enter, self);
        let children_ret = if ret == TraversalResult::Ok {
            match self {
                NodeMessageEntry::MessageLiteral(var_identifier, value_literal) => {
                    match var_identifier.visit(emitter) {
                        TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
                        _ => value_literal.visit(emitter),
                    }
                }
                NodeMessageEntry::MessageVariable(var_identifier1, var_identifier2) => {
                    match var_identifier1.visit(emitter) {
                        TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
                        _ => var_identifier2.visit(emitter),
                    }
                }
            }
        } else {
            ret
        };
        match children_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => emitter.emit_message_entry(TreeTraversalMode::Exit, self),
            _ => TraversalResult::Ok,
        }
    }
}

impl Visitor for NodePatternMatchExpressionClause {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        let ret = emitter.emit_pattern_match_expression_clause(TreeTraversalMode::Enter, self);
        let pattern_ret = if ret == TraversalResult::Ok {
            self.pattern.visit(emitter)
        } else {
            ret
        };
        let expression_ret = if pattern_ret == TraversalResult::Ok {
            self.expression.visit(emitter)
        } else {
            pattern_ret
        };
        match expression_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => {
                emitter.emit_pattern_match_expression_clause(TreeTraversalMode::Exit, self)
            }
            _ => TraversalResult::Ok,
        }
    }
}

impl Visitor for NodeAtomicExpression {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        let ret = emitter.emit_atomic_expression(TreeTraversalMode::Enter, self);
        // Only visit children if entering was successful and did not result in skipping
        let children_ret = if ret == TraversalResult::Ok {
            match self {
                NodeAtomicExpression::AtomicSid(sid) => sid.visit(emitter),
                NodeAtomicExpression::AtomicLit(lit) => lit.visit(emitter),
            }
        } else {
            ret
        };
        match children_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => emitter.emit_atomic_expression(TreeTraversalMode::Exit, self),
            _ => TraversalResult::Ok,
        }
    }
}

impl Visitor for NodeContractTypeArguments {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        let ret = emitter.emit_contract_type_arguments(TreeTraversalMode::Enter, self);
        let children_ret = if ret == TraversalResult::Ok {
            self.type_arguments
                .iter()
                .map(|child| child.visit(emitter))
                .find(|result| *result == TraversalResult::Fail("".into()))
                .unwrap_or(TraversalResult::Ok)
        } else {
            ret
        };
        match children_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => {
                emitter.emit_contract_type_arguments(TreeTraversalMode::Exit, self)
            }
            _ => TraversalResult::Ok,
        }
    }
}

impl Visitor for NodeValueLiteral {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        let ret = emitter.emit_value_literal(TreeTraversalMode::Enter, self);
        let children_ret = if ret == TraversalResult::Ok {
            match self {
                NodeValueLiteral::LiteralInt(type_name, _) => type_name.visit(emitter),
                NodeValueLiteral::LiteralEmptyMap(type_map_key, type_map_value) => {
                    match type_map_key.visit(emitter) {
                        TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
                        _ => type_map_value.visit(emitter),
                    }
                }
                _ => TraversalResult::Ok,
            }
        } else {
            ret
        };
        match children_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => emitter.emit_value_literal(TreeTraversalMode::Exit, self),
            _ => TraversalResult::Ok,
        }
    }
}

impl Visitor for NodeMapAccess {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        let ret = emitter.emit_map_access(TreeTraversalMode::Enter, self);
        let children_ret = if ret == TraversalResult::Ok {
            self.identifier_name.visit(emitter)
        } else {
            ret
        };
        match children_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => emitter.emit_map_access(TreeTraversalMode::Exit, self),
            _ => TraversalResult::Ok,
        }
    }
}

impl Visitor for NodePattern {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        let ret = emitter.emit_pattern(TreeTraversalMode::Enter, self);
        let children_ret = if ret == TraversalResult::Ok {
            match self {
                NodePattern::Wildcard => TraversalResult::Ok,
                NodePattern::Binder(_) => TraversalResult::Ok,
                NodePattern::Constructor(identifier, argument_patterns) => {
                    match identifier.visit(emitter) {
                        TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
                        _ => {
                            for pattern in argument_patterns {
                                let result = pattern.visit(emitter);
                                if let TraversalResult::Fail(msg) = result {
                                    return TraversalResult::Fail(msg);
                                }
                            }
                            TraversalResult::Ok
                        }
                    }
                }
            }
        } else {
            ret
        };
        match children_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => emitter.emit_pattern(TreeTraversalMode::Exit, self),
            _ => TraversalResult::Ok,
        }
    }
}

impl Visitor for NodeArgumentPattern {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        let ret = emitter.emit_argument_pattern(TreeTraversalMode::Enter, self);
        let children_ret = if ret == TraversalResult::Ok {
            match self {
                NodeArgumentPattern::WildcardArgument => TraversalResult::Ok,
                NodeArgumentPattern::BinderArgument(_) => TraversalResult::Ok,
                NodeArgumentPattern::ConstructorArgument(meta_identifier) => {
                    meta_identifier.visit(emitter)
                }
                NodeArgumentPattern::PatternArgument(pattern) => pattern.visit(emitter),
            }
        } else {
            ret
        };
        match children_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => emitter.emit_argument_pattern(TreeTraversalMode::Exit, self),
            _ => TraversalResult::Ok,
        }
    }
}

impl Visitor for NodePatternMatchClause {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        let ret = emitter.emit_pattern_match_clause(TreeTraversalMode::Enter, self);
        let children_ret = if ret == TraversalResult::Ok {
            match self.pattern_expression.visit(emitter) {
                TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
                _ => match &self.statement_block {
                    Some(stmt_block) => stmt_block.visit(emitter),
                    None => TraversalResult::Ok,
                },
            }
        } else {
            ret
        };
        match children_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => emitter.emit_pattern_match_clause(TreeTraversalMode::Exit, self),
            _ => TraversalResult::Ok,
        }
    }
}

impl Visitor for NodeBlockchainFetchArguments {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        let ret = emitter.emit_blockchain_fetch_arguments(TreeTraversalMode::Enter, self);
        if let TraversalResult::Ok = ret {
            // Visit each argument
            for arg in &self.arguments {
                match arg.visit(emitter) {
                    TraversalResult::Fail(msg) => return TraversalResult::Fail(msg),
                    _ => {}
                }
            }
        }
        match ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => {
                emitter.emit_blockchain_fetch_arguments(TreeTraversalMode::Exit, self)
            }
            _ => TraversalResult::Ok,
        }
    }
}

impl Visitor for NodeStatement {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        let ret = emitter.emit_statement(TreeTraversalMode::Enter, self);
        let children_ret = if ret == TraversalResult::Ok {
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
                } => right_hand_side.visit(emitter),
                NodeStatement::ReadFromBC { arguments, .. } => {
                    if let Some(arg) = arguments {
                        arg.visit(emitter)
                    } else {
                        TraversalResult::Ok
                    }
                }
                NodeStatement::MapGet { keys, .. }
                | NodeStatement::MapUpdate { keys, .. }
                | NodeStatement::MapGetExists { keys, .. }
                | NodeStatement::MapUpdateDelete { keys, .. } => {
                    for key in keys {
                        let ret = key.visit(emitter);
                        if ret != TraversalResult::Ok {
                            return ret;
                        }
                    }
                    TraversalResult::Ok
                }
                NodeStatement::Send {
                    identifier_name, ..
                }
                | NodeStatement::CreateEvnt {
                    identifier_name, ..
                } => identifier_name.visit(emitter),
                NodeStatement::Throw { error_variable, .. } => {
                    if let Some(variable) = error_variable {
                        variable.visit(emitter)
                    } else {
                        TraversalResult::Ok
                    }
                }
                NodeStatement::MatchStmt {
                    variable, clauses, ..
                } => {
                    let ret = variable.visit(emitter);
                    if ret != TraversalResult::Ok {
                        return ret;
                    }
                    for clause in clauses {
                        let ret = clause.visit(emitter);
                        if ret != TraversalResult::Ok {
                            return ret;
                        }
                    }
                    TraversalResult::Ok
                }
                NodeStatement::CallProc {
                    component_id,
                    arguments,
                    ..
                } => {
                    let ret = component_id.visit(emitter);
                    if ret != TraversalResult::Ok {
                        return ret;
                    }
                    for argument in arguments {
                        let ret = argument.visit(emitter);
                        if ret != TraversalResult::Ok {
                            return ret;
                        }
                    }
                    TraversalResult::Ok
                }
                NodeStatement::Iterate {
                    identifier_name,
                    component_id,
                } => {
                    let ret = identifier_name.visit(emitter);
                    if ret != TraversalResult::Ok {
                        return ret;
                    }
                    component_id.visit(emitter)
                }
                _ => TraversalResult::Ok,
            }
        } else {
            ret
        };
        match children_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => emitter.emit_statement(TreeTraversalMode::Exit, self),
            _ => TraversalResult::Ok,
        }
    }
}

impl Visitor for NodeRemoteFetchStatement {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        let ret = emitter.emit_remote_fetch_statement(TreeTraversalMode::Enter, self);
        let children_ret = if ret == TraversalResult::Ok {
            match self {
                NodeRemoteFetchStatement::ReadStateMutable(_, _, variable) => {
                    variable.visit(emitter)
                }
                NodeRemoteFetchStatement::ReadStateMutableSpecialId(_, _, _) => TraversalResult::Ok,
                NodeRemoteFetchStatement::ReadStateMutableMapAccess(_, _, _, accesses) => {
                    for access in accesses {
                        if let TraversalResult::Fail(msg) = access.visit(emitter) {
                            return TraversalResult::Fail(msg);
                        }
                    }
                    TraversalResult::Ok
                }
                NodeRemoteFetchStatement::ReadStateMutableMapAccessExists(_, _, _, accesses) => {
                    for access in accesses {
                        if let TraversalResult::Fail(msg) = access.visit(emitter) {
                            return TraversalResult::Fail(msg);
                        }
                    }
                    TraversalResult::Ok
                }
                NodeRemoteFetchStatement::ReadStateMutableCastAddress(_, variable, address) => {
                    match variable.visit(emitter) {
                        TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
                        _ => address.visit(emitter),
                    }
                }
            }
        } else {
            ret
        };
        match children_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => {
                emitter.emit_remote_fetch_statement(TreeTraversalMode::Exit, self)
            }
            _ => TraversalResult::Ok,
        }
    }
}

impl Visitor for NodeComponentId {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        // Emit enter event
        let ret = emitter.emit_component_id(TreeTraversalMode::Enter, self);
        let children_ret = if ret == TraversalResult::Ok {
            // Handle child nodes
            match self {
                NodeComponentId::WithTypeLikeName(type_name_identifier) => {
                    type_name_identifier.visit(emitter)
                }
                NodeComponentId::WithRegularId(_) => TraversalResult::Ok,
            }
        } else {
            ret
        };
        match children_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => emitter.emit_component_id(TreeTraversalMode::Exit, self),
            _ => TraversalResult::Ok,
        }
    }
}

impl Visitor for NodeComponentParameters {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        let ret = emitter.emit_component_parameters(TreeTraversalMode::Enter, self);
        let children_ret = if ret == TraversalResult::Ok {
            for param in &self.parameters {
                match param.visit(emitter) {
                    TraversalResult::Fail(msg) => return TraversalResult::Fail(msg),
                    _ => {}
                }
            }
            TraversalResult::Ok
        } else {
            ret
        };
        match children_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => emitter.emit_component_parameters(TreeTraversalMode::Exit, self),
            _ => TraversalResult::Ok,
        }
    }
}

impl Visitor for NodeParameterPair {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        let ret = emitter.emit_parameter_pair(TreeTraversalMode::Enter, self);
        let children_ret = if ret == TraversalResult::Ok {
            self.identifier_with_type.visit(emitter)
        } else {
            ret
        };
        match children_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => emitter.emit_parameter_pair(TreeTraversalMode::Exit, self),
            _ => TraversalResult::Ok,
        }
    }
}

impl Visitor for NodeComponentBody {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        let ret = emitter.emit_component_body(TreeTraversalMode::Enter, self);
        let children_ret = if ret == TraversalResult::Ok {
            if let Some(statement_block) = &self.statement_block {
                statement_block.visit(emitter)
            } else {
                TraversalResult::Ok
            }
        } else {
            ret
        };
        match children_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => emitter.emit_component_body(TreeTraversalMode::Exit, self),
            _ => TraversalResult::Ok,
        }
    }
}

impl Visitor for NodeStatementBlock {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        let ret = emitter.emit_statement_block(TreeTraversalMode::Enter, self);
        // Visit each statement if not skipping children
        let children_ret = if ret == TraversalResult::Ok {
            for statement in &self.statements {
                match statement.visit(emitter) {
                    TraversalResult::Fail(msg) => return TraversalResult::Fail(msg),
                    _ => (),
                }
            }
            TraversalResult::Ok
        } else {
            ret
        };
        match children_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => emitter.emit_statement_block(TreeTraversalMode::Exit, self),
            _ => TraversalResult::Ok,
        }
    }
}

impl Visitor for NodeTypedIdentifier {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        let ret = emitter.emit_typed_identifier(TreeTraversalMode::Enter, self);
        // Visit the annotation child node if the enter phase didn't fail or skip children
        let children_ret = if ret == TraversalResult::Ok {
            self.annotation.visit(emitter)
        } else {
            ret
        };
        // Depending on the result of the children's visits, either fail or finish with the exit phase
        match children_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => emitter.emit_typed_identifier(TreeTraversalMode::Exit, self),
            _ => TraversalResult::Ok,
        }
    }
}

impl Visitor for NodeTypeAnnotation {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        let ret = emitter.emit_type_annotation(TreeTraversalMode::Enter, self);
        // Child element: self.type_name
        let children_ret = if ret == TraversalResult::Ok {
            self.type_name.visit(emitter)
        } else {
            ret
        };
        match children_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => emitter.emit_type_annotation(TreeTraversalMode::Exit, self),
            _ => TraversalResult::Ok,
        }
    }
}

impl Visitor for NodeProgram {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        // Emit enter event
        let ret = emitter.emit_program(TreeTraversalMode::Enter, self);
        let children_ret = if ret == TraversalResult::Ok {
            // Visit children nodes
            let mut result = TraversalResult::Ok;
            // Visit import_declarations if it's not None
            if let Some(import_declarations) = &self.import_declarations {
                result = import_declarations.visit(emitter);
                if result != TraversalResult::Ok {
                    return result;
                }
            }
            // Visit library_definition if it's not None
            if let Some(library_definition) = &self.library_definition {
                result = library_definition.visit(emitter);
                if result != TraversalResult::Ok {
                    return result;
                }
            }
            // Visit contract_definition

            result = self.contract_definition.visit(emitter);
            result
        } else {
            ret
        };
        // Emit exit event
        match children_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => emitter.emit_program(TreeTraversalMode::Exit, self),
            _ => TraversalResult::Ok,
        }
    }
}

impl Visitor for NodeLibraryDefinition {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        let ret = emitter.emit_library_definition(TreeTraversalMode::Enter, self);
        let children_ret = if ret == TraversalResult::Ok {
            for definition in &self.definitions {
                match definition.visit(emitter) {
                    TraversalResult::Fail(msg) => return TraversalResult::Fail(msg),
                    _ => continue,
                };
            }
            TraversalResult::Ok
        } else {
            ret
        };
        match children_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => emitter.emit_library_definition(TreeTraversalMode::Exit, self),
            _ => TraversalResult::Ok,
        }
    }
}
impl Visitor for NodeLibrarySingleDefinition {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        let ret = emitter.emit_library_single_definition(TreeTraversalMode::Enter, self);
        let children_ret = if ret == TraversalResult::Ok {
            match self {
                NodeLibrarySingleDefinition::LetDefinition {
                    variable_name,
                    type_annotation,
                    expression,
                } => expression.visit(emitter),
                NodeLibrarySingleDefinition::TypeDefinition(name, option_clause) => {
                    match name.visit(emitter) {
                        TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
                        _ => match option_clause {
                            Some(clauses) => {
                                for clause in clauses {
                                    if let TraversalResult::Fail(msg) = clause.visit(emitter) {
                                        return TraversalResult::Fail(msg);
                                    }
                                }
                                TraversalResult::Ok
                            }
                            None => TraversalResult::Ok,
                        },
                    }
                }
            }
        } else {
            ret
        };
        match children_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => {
                emitter.emit_library_single_definition(TreeTraversalMode::Exit, self)
            }
            _ => TraversalResult::Ok,
        }
    }
}
impl Visitor for NodeContractDefinition {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        println!("Visiting contract!");

        let ret = emitter.emit_contract_definition(TreeTraversalMode::Enter, self);
        let children_ret = if ret == TraversalResult::Ok {
            if let TraversalResult::Fail(msg) = self.parameters.visit(emitter) {
                return TraversalResult::Fail(msg);
            }
            if let Some(constraint) = &self.constraint {
                if let TraversalResult::Fail(msg) = constraint.visit(emitter) {
                    return TraversalResult::Fail(msg);
                }
            }
            for field in &self.fields {
                if let TraversalResult::Fail(msg) = field.visit(emitter) {
                    return TraversalResult::Fail(msg);
                }
            }
            for component in &self.components {
                if let TraversalResult::Fail(msg) = component.visit(emitter) {
                    return TraversalResult::Fail(msg);
                }
            }
            TraversalResult::Ok
        } else {
            ret
        };
        match children_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => emitter.emit_contract_definition(TreeTraversalMode::Exit, self),
            _ => TraversalResult::Ok,
        }
    }
}

impl Visitor for NodeContractField {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        let ret = emitter.emit_contract_field(TreeTraversalMode::Enter, self);
        let children_ret = if ret == TraversalResult::Ok {
            match self.typed_identifier.visit(emitter) {
                TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
                _ => self.right_hand_side.visit(emitter),
            }
        } else {
            ret
        };
        match children_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => emitter.emit_contract_field(TreeTraversalMode::Exit, self),
            _ => TraversalResult::Ok,
        }
    }
}
impl Visitor for NodeWithConstraint {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        let ret = emitter.emit_with_constraint(TreeTraversalMode::Enter, self);
        let children_ret = if ret == TraversalResult::Ok {
            self.expression.visit(emitter)
        } else {
            ret
        };
        match children_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => emitter.emit_with_constraint(TreeTraversalMode::Exit, self),
            _ => TraversalResult::Ok,
        }
    }
}
impl Visitor for NodeComponentDefinition {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        let ret = emitter.emit_component_definition(TreeTraversalMode::Enter, self);
        let children_ret = if ret == TraversalResult::Ok {
            match self {
                NodeComponentDefinition::TransitionComponent(transition_definition) => {
                    transition_definition.visit(emitter)
                }
                NodeComponentDefinition::ProcedureComponent(procedure_definition) => {
                    procedure_definition.visit(emitter)
                }
            }
        } else {
            ret
        };
        match children_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => emitter.emit_component_definition(TreeTraversalMode::Exit, self),
            _ => TraversalResult::Ok,
        }
    }
}
impl Visitor for NodeProcedureDefinition {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        let ret = emitter.emit_procedure_definition(TreeTraversalMode::Enter, self);
        let children_ret = if ret == TraversalResult::Ok {
            match self.name.visit(emitter) {
                TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
                _ => match self.parameters.visit(emitter) {
                    TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
                    _ => self.body.visit(emitter),
                },
            }
        } else {
            ret
        };
        match children_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => emitter.emit_procedure_definition(TreeTraversalMode::Exit, self),
            _ => TraversalResult::Ok,
        }
    }
}
impl Visitor for NodeTransitionDefinition {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        let ret = emitter.emit_transition_definition(TreeTraversalMode::Enter, self);
        let children_ret = if ret == TraversalResult::Ok {
            match self.name.visit(emitter) {
                TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
                _ => match self.parameters.visit(emitter) {
                    TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
                    _ => self.body.visit(emitter),
                },
            }
        } else {
            ret
        };
        match children_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => {
                emitter.emit_transition_definition(TreeTraversalMode::Exit, self)
            }
            _ => TraversalResult::Ok,
        }
    }
}

impl Visitor for NodeTypeAlternativeClause {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        match emitter.emit_type_alternative_clause(TreeTraversalMode::Enter, self) {
            TraversalResult::Fail(msg) => return TraversalResult::Fail(msg),
            TraversalResult::SkipChildren => return TraversalResult::Ok,
            TraversalResult::Ok => (),
        }
        let children_ret = match self {
            NodeTypeAlternativeClause::ClauseType(type_name) => type_name.visit(emitter),
            NodeTypeAlternativeClause::ClauseTypeWithArgs(type_name, type_args) => {
                match type_name.visit(emitter) {
                    TraversalResult::Fail(msg) => return TraversalResult::Fail(msg),
                    _ => (),
                }
                for type_arg in type_args {
                    match type_arg.visit(emitter) {
                        TraversalResult::Fail(msg) => return TraversalResult::Fail(msg),
                        _ => (),
                    }
                }
                TraversalResult::Ok
            }
        };
        match children_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => {
                emitter.emit_type_alternative_clause(TreeTraversalMode::Exit, self)
            }
            _ => TraversalResult::Ok,
        }
    }
}

impl Visitor for NodeTypeMapValueArguments {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        let ret = emitter.emit_type_map_value_arguments(TreeTraversalMode::Enter, self);
        let children_ret = if ret == TraversalResult::Ok {
            match self {
                NodeTypeMapValueArguments::EnclosedTypeMapValue(enclosed) => {
                    enclosed.visit(emitter)
                }
                NodeTypeMapValueArguments::GenericMapValueArgument(meta_identifier) => {
                    meta_identifier.visit(emitter)
                }
                NodeTypeMapValueArguments::MapKeyValueType(key_type, value_type) => {
                    match key_type.visit(emitter) {
                        TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
                        _ => value_type.visit(emitter),
                    }
                }
            }
        } else {
            ret
        };
        match children_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => {
                emitter.emit_type_map_value_arguments(TreeTraversalMode::Exit, self)
            }
            _ => TraversalResult::Ok,
        }
    }
}

impl Visitor for NodeTypeMapValueAllowingTypeArguments {
    fn visit(&self, emitter: &mut dyn CodeEmitter) -> TraversalResult {
        let ret =
            emitter.emit_type_map_value_allowing_type_arguments(TreeTraversalMode::Enter, self);
        let children_ret = if ret == TraversalResult::Ok {
            match self {
                NodeTypeMapValueAllowingTypeArguments::TypeMapValueNoArgs(type_map_value) => {
                    type_map_value.visit(emitter)
                }
                NodeTypeMapValueAllowingTypeArguments::TypeMapValueWithArgs(
                    meta_id,
                    value_args,
                ) => {
                    let mut ret = meta_id.visit(emitter);
                    if ret == TraversalResult::Ok {
                        for value_arg in value_args {
                            ret = value_arg.visit(emitter);
                            if ret != TraversalResult::Ok {
                                break;
                            }
                        }
                    }
                    ret
                }
            }
        } else {
            ret
        };
        match children_ret {
            TraversalResult::Fail(msg) => TraversalResult::Fail(msg),
            TraversalResult::Ok => {
                emitter.emit_type_map_value_allowing_type_arguments(TreeTraversalMode::Exit, self)
            }
            _ => TraversalResult::Ok,
        }
    }
}
