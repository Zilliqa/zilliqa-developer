use crate::ast::*;
use crate::ast_converting::AstConverting;
use crate::ast_visitor::AstVisitor;
use crate::constants::{TreeTraversalMode,TraversalResult};

pub struct BluebellFormatter {
    indent_level: usize,
    script: String,
}

impl BluebellFormatter {
    pub fn new() -> Self {
        Self {
            indent_level: 0,
            script: "".to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        self.script.clone()
    }

    pub fn add_newlines(&mut self, count: usize) {
        self.script.push_str(&"\n".repeat(count));
        self.script.push_str(&" ".repeat(self.indent_level * 2));
    }

    pub fn emit(&mut self, node: &mut NodeProgram) -> String {
        self.script = "".to_string();
        // TODO: Handle errors
        // Consider adding an error logger and be greedy
        // when collecting errors
        let _ = node.visit(self);

        self.script.clone()
    }
}

impl AstConverting for BluebellFormatter {
    fn emit_byte_str(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeByteStr,
    ) -> Result<TraversalResult, String> {
        match mode {
            TreeTraversalMode::Enter => match node {
                NodeByteStr::Constant(s) => {
                    self.script.push_str(&format!("\"{}\"", s)); // Push a constant byte string to the script
                }
                NodeByteStr::Type(s) => {
                    self.script.push_str(&format!("{}", s)); // Push a byte string type definition to the script
                }
            },
            TreeTraversalMode::Exit => (),
        }
        Ok(TraversalResult::Continue)
    }

    fn emit_type_name_identifier(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeNameIdentifier,
    ) -> Result<TraversalResult, String> {
        match mode {
            TreeTraversalMode::Enter => match node {
                NodeTypeNameIdentifier::ByteStringType(_) => (),
                NodeTypeNameIdentifier::EventType => {
                    self.script.push_str("Event");
                }
                NodeTypeNameIdentifier::TypeOrEnumLikeIdentifier(n) => {
                    self.script.push_str(n);
                }
            },
            TreeTraversalMode::Exit => (),
        }
        Ok(TraversalResult::Continue)
    }

    fn emit_imported_name(
        &mut self,
        mode: TreeTraversalMode,
        __node: &NodeImportedName,
    ) -> Result<TraversalResult, String> {
        match mode {
            TreeTraversalMode::Enter => {
                self.add_newlines(1);
                self.script.push_str("import ");
            }
            TreeTraversalMode::Exit => (),
        }

        Ok(TraversalResult::Continue)
    }

    fn emit_import_declarations(
        &mut self,
        mode: TreeTraversalMode,
        __node: &NodeImportDeclarations,
    ) -> Result<TraversalResult, String> {
        match mode {
            TreeTraversalMode::Enter => {
                self.add_newlines(1);
            }
            TreeTraversalMode::Exit => (),
        }

        Ok(TraversalResult::Continue)
    }

    fn emit_meta_identifier(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeMetaIdentifier,
    ) -> Result<TraversalResult, String> {
        match mode {
            TreeTraversalMode::Enter => match node {
                NodeMetaIdentifier::MetaNameInNamespace(l, r) => {
                    let _ = l.visit(self)?;
                    self.script.push_str(".");
                    let _ = r.visit(self)?;
                    return Ok(TraversalResult::SkipChildren);
                }
                NodeMetaIdentifier::MetaNameInHexspace(l, r) => {
                    self.script.push_str(&l);
                    self.script.push_str(".");
                    let _ = r.visit(self)?;
                    return Ok(TraversalResult::SkipChildren);
                }
                NodeMetaIdentifier::ByteString => {
                    self.script.push_str("ByStr");
                }
                NodeMetaIdentifier::MetaName(_) => (),
            },
            TreeTraversalMode::Exit => (),
        }
        Ok(TraversalResult::Continue)
    }

    fn emit_variable_identifier(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeVariableIdentifier,
    ) -> Result<TraversalResult, String> {
        match mode {
            TreeTraversalMode::Enter => (),
            TreeTraversalMode::Exit => match node {
                NodeVariableIdentifier::VariableName(v) => self.script.push_str(v),
                NodeVariableIdentifier::SpecialIdentifier(v) => self.script.push_str(v),
                NodeVariableIdentifier::VariableInNamespace(_, v) => {
                    self.script.push_str(".");
                    self.script.push_str(v);
                }
            },
        }
        Ok(TraversalResult::Continue)
    }

    fn emit_builtin_arguments(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodeBuiltinArguments,
    ) -> Result<TraversalResult, String> {
        if node.arguments.len() == 0 {
            self.script.push_str("()");
        } else {
            for (i, arg) in node.arguments.iter().enumerate() {
                if i != 0 {
                    self.script.push_str(" ");
                }
                let _ = arg.visit(self)?;
            }
        }
        Ok(TraversalResult::SkipChildren)
    }

    fn emit_type_map_key(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodeTypeMapKey,
    ) -> Result<TraversalResult, String> {
        match node {
            NodeTypeMapKey::GenericMapKey(value) => {
                let _ = value.visit(self)?;
            }
            NodeTypeMapKey::EnclosedGenericId(value) => {
                self.script.push_str("(");
                let _ = value.visit(self)?;
                self.script.push_str(")");
            }
            NodeTypeMapKey::EnclosedAddressMapKeyType(value) => {
                self.script.push_str("(");
                let _ = value.visit(self)?;
                self.script.push_str(")");
            }
            NodeTypeMapKey::AddressMapKeyType(value) => {
                let _ = value.visit(self)?;
            }
        }
        Ok(TraversalResult::SkipChildren)
    }

    fn emit_type_map_value(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodeTypeMapValue,
    ) -> Result<TraversalResult, String> {
        match node {
            NodeTypeMapValue::MapValueTypeOrEnumLikeIdentifier(value) => {
                let _ = value.visit(self)?;
            }
            NodeTypeMapValue::MapKeyValue(value) => {
                let _ = (*value).visit(self)?;
            }
            NodeTypeMapValue::MapValueParanthesizedType(value) => {
                self.script.push_str("(");
                let _ = (*value).visit(self)?;
                self.script.push_str(")");
            }
            NodeTypeMapValue::MapValueAddressType(value) => {
                let _ = (*value).visit(self)?;
            }
        }
        Ok(TraversalResult::SkipChildren)
    }

    fn emit_type_argument(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeArgument,
    ) -> Result<TraversalResult, String> {
        match mode {
            TreeTraversalMode::Enter => match node {
                NodeTypeArgument::MapTypeArgument(_, _) => {
                    self.script.push_str("Map ");
                }
                NodeTypeArgument::EnclosedTypeArgument(_) => {
                    self.script.push_str("(");
                }
                NodeTypeArgument::TemplateTypeArgument(var) => {
                    self.script.push_str(var);
                }
                _ => (),
            },
            TreeTraversalMode::Exit => match node {
                NodeTypeArgument::EnclosedTypeArgument(_) => {
                    self.script.push_str(")");
                }
                _ => (),
            },
        }
        Ok(TraversalResult::Continue)
    }

    fn emit_scilla_type(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodeScillaType,
    ) -> Result<TraversalResult, String> {
        match node {
            NodeScillaType::GenericTypeWithArgs(lead, args) => {
                let _ = lead.visit(self)?;
                for arg in args.iter() {
                    self.script.push_str(" ");
                    let _ = arg.visit(self)?;
                }
            }
            NodeScillaType::MapType(key, value) => {
                self.script.push_str("Map ");
                let _ = key.visit(self)?;
                self.script.push_str(" ");
                let _ = value.visit(self)?;
            }
            NodeScillaType::FunctionType(a, b) => {
                let _ = (*a).visit(self)?;
                let _ = (*b).visit(self)?;
            }
            NodeScillaType::EnclosedType(a) => {
                self.script.push_str("( ");
                let _ = (*a).visit(self)?;
                self.script.push_str(" )");
            }
            NodeScillaType::ScillaAddresseType(a) => {
                let _ = (*a).visit(self)?;
            }
            NodeScillaType::PolyFunctionType(name, a) => {
                self.script.push_str(name);
                let _ = (*a).visit(self)?;
            }
            NodeScillaType::TypeVarType(name) => {
                self.script.push_str(name);
            }
        };
        Ok(TraversalResult::SkipChildren)
    }

    fn emit_type_map_entry(
        &mut self,
        _mode: TreeTraversalMode,
        __node: &NodeTypeMapEntry,
    ) -> Result<TraversalResult, String> {
        /*
        #[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
        pub struct NodeTypeMapEntry {
            pub key: NodeTypeMapKey,
            pub value: NodeTypeMapValue,
            pub type_annotation: Option<TypeAnnotation>,
        }
        */

        unimplemented!()
    }

    fn emit_address_type_field(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodeAddressTypeField,
    ) -> Result<TraversalResult, String> {
        self.script.push_str("field ");
        let _ = node.identifier.visit(self)?;
        self.script.push_str(" : ");
        let _ = node.type_name.visit(self)?;
        Ok(TraversalResult::SkipChildren)
    }

    fn emit_address_type(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodeAddressType,
    ) -> Result<TraversalResult, String> {
        let _ = node.identifier.visit(self)?;
        self.script.push_str(" with ");

        if node.type_name.len() > 0 {
            self.script.push_str(&node.type_name);
            self.script.push_str(" ");
        }

        for field in node.address_fields.iter() {
            let _ = field.visit(self)?;
            self.script.push_str(" ");
        }

        self.script.push_str("end");

        Ok(TraversalResult::SkipChildren)
    }

    fn emit_full_expression(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodeFullExpression,
    ) -> Result<TraversalResult, String> {
        match node {
            NodeFullExpression::LocalVariableDeclaration {
                identifier_name,
                expression,
                type_annotation,
                containing_expression,
            } => {
                self.add_newlines(1);
                self.script.push_str("let ");
                self.script.push_str(&identifier_name);
                self.indent_level += 1;
                if let Some(t) = type_annotation {
                    let _ = t.visit(self)?;
                }
                self.script.push_str(" = ");
                let _ = (*expression).visit(self)?;
                self.script.push_str(" in ");
                let _ = (*containing_expression).visit(self)?;
                self.indent_level -= 1;
            }
            NodeFullExpression::FunctionDeclaration {
                identier_value, // TODO: Missing spelling - global replacement
                type_annotation,
                expression,
            } => {
                self.add_newlines(1);
                self.script.push_str("fun ");
                self.indent_level += 1;
                self.script.push_str("(");
                self.script.push_str(&identier_value);
                let _ = type_annotation.visit(self)?;
                self.script.push_str(") => ");

                let _ = (*expression).visit(self)?;
                self.indent_level -= 1;
            }
            NodeFullExpression::FunctionCall {
                function_name,
                argument_list,
            } => {
                let _ = function_name.visit(self)?;
                for arg in argument_list.iter() {
                    self.script.push_str(" ");
                    let _ = arg.visit(self)?;
                }
            }
            NodeFullExpression::ExpressionAtomic(expr) => {
                let _ = expr.visit(self)?;
            }
            NodeFullExpression::ExpressionBuiltin { b, targs, xs } => {
                self.script.push_str("builtin ");
                self.script.push_str(b);
                if let Some(args) = targs {
                    let _ = args.visit(self)?;
                }
                self.script.push_str(" ");
                let _ = xs.visit(self)?;
            }
            NodeFullExpression::Message(entries) => {
                self.script.push_str("{");
                self.indent_level += 1;
                for (i, message) in entries.iter().enumerate() {
                    let _ = message.visit(self)?;
                    if i != entries.len() - 1 {
                        self.script.push_str(";")
                    }
                }
                self.indent_level -= 1;
                self.add_newlines(1);
                self.script.push_str("}");
            }
            NodeFullExpression::Match {
                match_expression,
                clauses,
            } => {
                self.add_newlines(1);
                self.script.push_str("match ");
                let _ = match_expression.visit(self)?;
                self.script.push_str(" with ");
                self.indent_level += 1;
                for clause in clauses.iter() {
                    let _ = clause.visit(self)?;
                }
                self.indent_level -= 1;
                self.add_newlines(1);
                self.script.push_str("end");
            }
            NodeFullExpression::ConstructorCall {
                identifier_name,
                contract_type_arguments,
                argument_list,
            } => {
                let _ = identifier_name.visit(self)?;
                if let Some(cta) = contract_type_arguments {
                    self.script.push_str(" ");
                    let _ = cta.visit(self)?;
                }
                for a in argument_list.iter() {
                    self.script.push_str(" ");
                    let _ = a.visit(self)?;
                }
            }
            NodeFullExpression::TemplateFunction {
                identifier_name,
                expression,
            } => {
                self.add_newlines(1);
                self.script.push_str("tfun ");
                self.script.push_str(identifier_name);
                self.script.push_str(" => ");
                let _ = expression.visit(self)?;
            }
            NodeFullExpression::TApp {
                identifier_name,
                type_arguments,
            } => {
                self.script.push_str("@");
                let _ = identifier_name.visit(self)?;
                for arg in type_arguments.iter() {
                    self.script.push_str(" ");
                    let _ = arg.visit(self)?;
                }
            }
        }
        Ok(TraversalResult::SkipChildren)
    }

    fn emit_message_entry(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodeMessageEntry,
    ) -> Result<TraversalResult, String> {
        self.add_newlines(1);
        match node {
            NodeMessageEntry::MessageLiteral(var, val) => {
                // Converting the variable and value literals into Scilla code
                // Assuming the emit_variable_identifier and emit_value_literal are implemented
                let _ = var.visit(self)?;
                self.script.push_str(" : ");
                let _ = val.visit(self)?;
            }
            NodeMessageEntry::MessageVariable(var1, var2) => {
                let _ = var1.visit(self)?;
                self.script.push_str(" : ");
                let _ = var2.visit(self)?;
            }
        }
        Ok(TraversalResult::SkipChildren)
    }
    fn emit_pattern_match_expression_clause(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodePatternMatchExpressionClause,
    ) -> Result<TraversalResult, String> {
        self.add_newlines(1);
        self.script.push_str("| ");
        let _ = node.pattern.visit(self)?;
        self.script.push_str(" => ");
        let _ = node.expression.visit(self)?;
        Ok(TraversalResult::SkipChildren)
    }

    fn emit_atomic_expression(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeAtomicExpression,
    ) -> Result<TraversalResult, String> {
        // Pass through
        Ok(TraversalResult::Continue)
    }

    fn emit_contract_type_arguments(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodeContractTypeArguments,
    ) -> Result<TraversalResult, String> {
        self.script.push_str("{");
        for (i, arg) in node.type_arguments.iter().enumerate() {
            if i != 0 {
                self.script.push_str(" ");
            }

            let _ = arg.visit(self)?;
        }
        self.script.push_str("}");

        Ok(TraversalResult::SkipChildren)
    }

    fn emit_value_literal(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeValueLiteral,
    ) -> Result<TraversalResult, String> {
        match mode {
            TreeTraversalMode::Enter => match node {
                NodeValueLiteral::LiteralInt(n, v) => {
                    let _ = n.visit(self)?;
                    self.script.push_str(" ");
                    self.script.push_str(&v);
                }
                NodeValueLiteral::LiteralHex(h) => {
                    self.script.push_str(&format!("0x{}", h)); // Push the literal hexadecimal type definition to the script
                }
                NodeValueLiteral::LiteralString(s) => {
                    self.script.push_str(&format!("\"{}\"", s)); // Push the literal string type definition to the script
                }
                NodeValueLiteral::LiteralEmptyMap(key_type, value_type) => {
                    self.script.push_str("Emp ");
                    let _ = key_type.visit(self)?;
                    self.script.push_str(" ");
                    let _ = value_type.visit(self)?;
                }
            },
            TreeTraversalMode::Exit => (),
        }
        Ok(TraversalResult::SkipChildren)
    }

    fn emit_map_access(
        &mut self,
        mode: TreeTraversalMode,
        _node: &NodeMapAccess,
    ) -> Result<TraversalResult, String> {
        match mode {
            TreeTraversalMode::Enter => self.script.push_str("["),
            TreeTraversalMode::Exit => self.script.push_str("]"),
        }
        Ok(TraversalResult::Continue)
    }

    fn emit_pattern(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodePattern,
    ) -> Result<TraversalResult, String> {
        match node {
            NodePattern::Wildcard => {
                self.script.push_str("_");
            }
            NodePattern::Binder(value) => {
                self.script.push_str(value);
            }
            NodePattern::Constructor(identifier, args) => {
                let _ = identifier.visit(self)?;

                for arg in args.iter() {
                    self.script.push_str(" ");
                    let _ = arg.visit(self)?;
                }
            }
        }
        Ok(TraversalResult::SkipChildren)
    }

    fn emit_argument_pattern(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeArgumentPattern,
    ) -> Result<TraversalResult, String> {
        match mode {
            TreeTraversalMode::Enter => match node {
                NodeArgumentPattern::BinderArgument(s) => self.script.push_str(s),
                NodeArgumentPattern::WildcardArgument => self.script.push_str("_"),
                NodeArgumentPattern::PatternArgument(_) => self.script.push_str("("),
                _ => (),
            },
            TreeTraversalMode::Exit => match node {
                NodeArgumentPattern::PatternArgument(_) => self.script.push_str(")"),
                _ => (),
            },
        }
        // Pass through
        Ok(TraversalResult::Continue)
    }

    fn emit_pattern_match_clause(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodePatternMatchClause,
    ) -> Result<TraversalResult, String> {
        match mode {
            TreeTraversalMode::Enter => {
                self.add_newlines(1);
                self.script.push_str("| ");
                let _ = node.pattern_expression.visit(self)?;
                self.script.push_str(" =>");
                if let Some(stmt) = &node.statement_block {
                    let _ = stmt.visit(self)?;
                }
            }
            TreeTraversalMode::Exit => {}
        }
        Ok(TraversalResult::SkipChildren)
    }

    fn emit_blockchain_fetch_arguments(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodeBlockchainFetchArguments,
    ) -> Result<TraversalResult, String> {
        self.script.push_str("(");
        for arg in node.arguments.iter() {
            self.script.push_str(" ");
            let _ = arg.visit(self)?;
        }
        self.script.push_str(" )");
        Ok(TraversalResult::SkipChildren)
    }

    fn emit_statement(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodeStatement,
    ) -> Result<TraversalResult, String> {
        self.add_newlines(1);
        match node {
            NodeStatement::Load {
                left_hand_side,
                right_hand_side,
            } => {
                self.script.push_str(left_hand_side);
                self.script.push_str(" <- ");
                let _ = right_hand_side.visit(self)?;
            }
            NodeStatement::RemoteFetch(fetch_statement) => {
                let _ = (*fetch_statement).visit(self)?;
            }
            NodeStatement::Store {
                left_hand_side,
                right_hand_side,
            } => {
                self.script.push_str(left_hand_side);
                self.script.push_str(" := ");
                let _ = right_hand_side.visit(self)?;
            }
            NodeStatement::Bind {
                left_hand_side,
                right_hand_side,
            } => {
                self.script.push_str(left_hand_side);
                self.script.push_str(" = ");
                let _ = right_hand_side.visit(self)?;
            }
            NodeStatement::ReadFromBC {
                left_hand_side,
                type_name,
                arguments,
            } => {
                self.script.push_str(left_hand_side);
                self.script.push_str(" <-& ");
                let _ = type_name.visit(self)?;
                if let Some(args) = arguments {
                    let _ = args.visit(self)?;
                }
            }
            NodeStatement::MapGet {
                left_hand_side: _,
                keys: _,
                right_hand_side: _,
            } => {
                unimplemented!();
            }
            NodeStatement::MapGetExists {
                left_hand_side: _,
                keys: _,
                right_hand_side: _,
            } => {
                unimplemented!();
            }
            NodeStatement::MapUpdate {
                left_hand_side,
                keys,
                right_hand_side,
            } => {
                self.script.push_str(left_hand_side);
                for key in keys.iter() {
                    let _ = key.visit(self)?;
                }
                self.script.push_str(" := ");
                let _ = right_hand_side.visit(self)?;
            }
            NodeStatement::MapUpdateDelete {
                left_hand_side: _,
                keys: _,
            } => {
                unimplemented!();
            }
            NodeStatement::Accept => self.script.push_str("accept"),
            NodeStatement::Send { identifier_name } => {
                self.script.push_str("send ");
                let _ = identifier_name.visit(self)?;
            }
            NodeStatement::CreateEvnt { identifier_name } => {
                self.script.push_str("event ");
                let _ = identifier_name.visit(self)?;
            }
            NodeStatement::Throw { error_variable } => {
                self.script.push_str("throw");
                if let Some(e) = error_variable {
                    self.script.push_str(" ");
                    let _ = e.visit(self)?;
                }
            }
            NodeStatement::MatchStmt { variable, clauses } => {
                self.script.push_str("match ");
                let _ = variable.visit(self)?;
                self.script.push_str(" with");
                self.indent_level += 1;
                for clause in clauses.iter() {
                    let _ = clause.visit(self)?;
                }
                self.indent_level -= 1;
                self.add_newlines(1);
                self.script.push_str("end");
            }
            NodeStatement::CallProc {
                component_id,
                arguments,
            } => {
                let _ = component_id.visit(self)?;
                for arg in arguments.iter() {
                    self.script.push_str(" ");
                    let _ = arg.visit(self)?;
                }
            }
            NodeStatement::Iterate {
                identifier_name,
                component_id,
            } => {
                self.script.push_str("forall ");
                let _ = identifier_name.visit(self)?;
                self.script.push_str(" ");
                let _ = component_id.visit(self)?;
            }
        }

        Ok(TraversalResult::SkipChildren)
    }

    fn emit_remote_fetch_statement(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodeRemoteFetchStatement,
    ) -> Result<TraversalResult, String> {
        match node {
            NodeRemoteFetchStatement::ReadStateMutable(lhs, address, identifier) => {
                self.script.push_str(&format!("{} <-& {}.", lhs, address));
                let _ = identifier.visit(self)?;
            }
            NodeRemoteFetchStatement::ReadStateMutableSpecialId(lhs, address, identifier) => {
                self.script
                    .push_str(&format!("{} <-& {}.{}", lhs, address, identifier));
            }
            NodeRemoteFetchStatement::ReadStateMutableMapAccess(
                lhs,
                address,
                member_id,
                map_accesses,
            ) => {
                self.script
                    .push_str(&format!("{} <-& {}.{} ", lhs, address, member_id));
                for access in map_accesses.iter() {
                    let _ = access.visit(self)?;
                }
            }
            NodeRemoteFetchStatement::ReadStateMutableMapAccessExists(
                lhs,
                address,
                member_id,
                map_accesses,
            ) => {
                self.script
                    .push_str(&format!("{} <-& exists {}.{} ", lhs, address, member_id));
                for access in map_accesses.iter() {
                    let _ = access.visit(self)?;
                }
            }
            NodeRemoteFetchStatement::ReadStateMutableCastAddress(
                lhs,
                address_id,
                address_type,
            ) => {
                self.script.push_str(&format!("{} <-& ", lhs));
                let _ = address_id.visit(self)?;
                self.script.push_str(" as ");

                let _ = address_type.visit(self)?;
            }
        }
        Ok(TraversalResult::SkipChildren)
    }

    fn emit_component_id(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeComponentId,
    ) -> Result<TraversalResult, String> {
        match mode {
            TreeTraversalMode::Enter => match node {
                NodeComponentId::WithRegularId(name) => self.script.push_str(name),
                _ => (),
            },
            _ => (),
        }
        Ok(TraversalResult::Continue)
    }

    fn emit_component_parameters(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeComponentParameters,
    ) -> Result<TraversalResult, String> {
        match mode {
            TreeTraversalMode::Enter => {
                self.script.push_str("(");
                for (i, parameter) in node.parameters.iter().enumerate() {
                    if i > 0 {
                        self.script.push_str(", ");
                    }
                    let _ = parameter.visit(self)?;
                }
                self.script.push_str(")");
            }
            TreeTraversalMode::Exit => (),
        }
        Ok(TraversalResult::SkipChildren)
    }

    fn emit_parameter_pair(
        &mut self,
        __mode: TreeTraversalMode,
        __node: &NodeParameterPair,
    ) -> Result<TraversalResult, String> {
        // Pass through
        Ok(TraversalResult::Continue)
    }

    fn emit_component_body(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeComponentBody,
    ) -> Result<TraversalResult, String> {
        // Pass through
        Ok(TraversalResult::Continue)
    }

    fn emit_statement_block(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodeStatementBlock,
    ) -> Result<TraversalResult, String> {
        self.indent_level += 1;
        for (i, stmt) in node.statements.iter().enumerate() {
            let _ = stmt.visit(self)?;
            if i != node.statements.len() - 1 {
                self.script.push_str(";");
            }
        }
        self.indent_level -= 1;
        Ok(TraversalResult::SkipChildren)
    }

    fn emit_typed_identifier(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypedIdentifier,
    ) -> Result<TraversalResult, String> {
        match mode {
            TreeTraversalMode::Enter => {
                // Assuming that annotation type is of String
                self.script.push_str(&node.identifier_name);
            }
            TreeTraversalMode::Exit => (),
        }
        Ok(TraversalResult::Continue)
    }

    fn emit_type_annotation(
        &mut self,
        mode: TreeTraversalMode,
        _node: &NodeTypeAnnotation,
    ) -> Result<TraversalResult, String> {
        match mode {
            TreeTraversalMode::Enter => self.script.push_str(" : "),
            _ => (),
        }
        Ok(TraversalResult::Continue)
    }

    fn emit_program(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeProgram,
    ) -> Result<TraversalResult, String> {
        match mode {
            TreeTraversalMode::Enter => {
                self.script
                    .push_str(&format!("scilla_version {}", node.version));
            }
            TreeTraversalMode::Exit => {
                self.script.push_str("\n");
            }
        }
        Ok(TraversalResult::Continue)
    }

    fn emit_library_definition(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeLibraryDefinition,
    ) -> Result<TraversalResult, String> {
        match mode {
            TreeTraversalMode::Enter => {
                // Add Indent
                self.add_newlines(2);
                self.script.push_str(&format!("library {}", node.name));
            }
            _ => (),
        }
        Ok(TraversalResult::Continue)
    }

    fn emit_library_single_definition(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeLibrarySingleDefinition,
    ) -> Result<TraversalResult, String> {
        match mode {
            TreeTraversalMode::Enter => match node {
                NodeLibrarySingleDefinition::LetDefinition {
                    variable_name,
                    type_annotation,
                    expression,
                } => {
                    self.add_newlines(1);
                    self.script.push_str("let ");
                    self.script.push_str(&variable_name);
                    self.indent_level += 1;
                    if let Some(v) = type_annotation {
                        let _ = v.visit(self)?;
                    }
                    self.script.push_str(" = ");
                    let _ = expression.visit(self)?;
                    self.indent_level -= 1;
                }
                NodeLibrarySingleDefinition::TypeDefinition(name, clauses) => {
                    self.add_newlines(1);
                    self.script.push_str("type ");
                    let _ = name.visit(self)?;
                    match clauses {
                        Some(clauses) => {
                            self.script.push_str(" =");
                            self.indent_level += 1;
                            for clause in clauses.iter() {
                                let _ = clause.visit(self)?;
                            }
                            self.indent_level -= 1;
                        }
                        None => (),
                    }
                }
            },
            _ => {}
        }
        Ok(TraversalResult::SkipChildren)
    }

    fn emit_contract_definition(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeContractDefinition,
    ) -> Result<TraversalResult, String> {
        match mode {
            TreeTraversalMode::Enter => {
                // Add Indent
                self.add_newlines(2);

                // Add Contract definition, contract name and open parentheses
                self.script
                    .push_str(&format!("contract {}", node.contract_name));
            }
            _ => (),
        }
        Ok(TraversalResult::Continue)
    }

    fn emit_contract_field(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeContractField,
    ) -> Result<TraversalResult, String> {
        match mode {
            TreeTraversalMode::Enter => {
                self.add_newlines(1);
                self.script.push_str("field ");
                let _ = node.typed_identifier.visit(self)?;
                self.script.push_str(" = ");
                let _ = node.right_hand_side.visit(self)?;
            }
            _ => (),
        }
        Ok(TraversalResult::SkipChildren)
    }

    fn emit_with_constraint(
        &mut self,
        mode: TreeTraversalMode,
        _node: &NodeWithConstraint,
    ) -> Result<TraversalResult, String> {
        match mode {
            TreeTraversalMode::Enter => {
                self.script.push_str(" with ");
            }
            _ => {
                self.script.push_str(" =>");
            }
        }
        Ok(TraversalResult::Continue)
    }

    fn emit_component_definition(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeComponentDefinition,
    ) -> Result<TraversalResult, String> {
        // Fall through to either Transition or Procedure
        Ok(TraversalResult::Continue)
    }

    fn emit_procedure_definition(
        &mut self,
        mode: TreeTraversalMode,
        _node: &NodeProcedureDefinition,
    ) -> Result<TraversalResult, String> {
        match mode {
            TreeTraversalMode::Enter => {
                self.add_newlines(2);
                self.script.push_str("procedure ");
            }
            TreeTraversalMode::Exit => {
                self.add_newlines(1);
                self.script.push_str("end");
            }
        }
        Ok(TraversalResult::Continue)
    }

    fn emit_transition_definition(
        &mut self,
        mode: TreeTraversalMode,
        _node: &NodeTransitionDefinition,
    ) -> Result<TraversalResult, String> {
        match mode {
            TreeTraversalMode::Enter => {
                self.add_newlines(2);
                self.script.push_str("transition ");
            }
            TreeTraversalMode::Exit => {
                self.add_newlines(1);
                self.script.push_str("end");
            }
        }
        Ok(TraversalResult::Continue)
    }

    fn emit_type_alternative_clause(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodeTypeAlternativeClause,
    ) -> Result<TraversalResult, String> {
        self.add_newlines(1);
        self.script.push_str("| ");
        match node {
            NodeTypeAlternativeClause::ClauseType(v) => {
                let _ = v.visit(self)?;
            }
            NodeTypeAlternativeClause::ClauseTypeWithArgs(name, args) => {
                let _ = name.visit(self)?;
                self.script.push_str(" of");
                for arg in args.iter() {
                    self.script.push_str(" ");
                    let _ = arg.visit(self)?;
                }
            }
        }

        Ok(TraversalResult::SkipChildren)
    }

    fn emit_type_map_value_arguments(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeTypeMapValueArguments,
    ) -> Result<TraversalResult, String> {
        unimplemented!()
    }

    fn emit_type_map_value_allowing_type_arguments(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeTypeMapValueAllowingTypeArguments,
    ) -> Result<TraversalResult, String> {
        // Pass through
        Ok(TraversalResult::Continue)
    }
}
