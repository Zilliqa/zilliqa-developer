use scilla_parser::{
    ast::{
        converting::AstConverting, nodes::*, visitor::AstVisitor, TraversalResult,
        TreeTraversalMode,
    },
    parser::lexer::SourcePosition,
};

/// `BluebellFormatter` is a structure responsible for generating a formatted script from an AST.
/// It stores the current indentation level for the partially generated script `script`.
pub struct BluebellFormatter {
    /// `indent_level` keeps track of the current indentation level in the script.
    /// It will typically increase with each nested construct.
    indent_level: usize,

    /// `script` is a string representing the partially generated script.
    script: String,
}

impl BluebellFormatter {
    /// This constructs a new `BluebellFormatter` with an initial `indent_level` of 0 and an empty `script`.
    pub fn new() -> Self {
        Self {
            indent_level: 0,
            script: "".to_string(),
        }
    }

    /// This function returns the current state of the script as a `String`.
    pub fn to_string(&self) -> &String {
        &self.script
    }

    /// After resetting the current `script`, this function makes `ast` visit the instance of `BluebellFormatter`.
    /// The `NodeProgram`'s `visit` method will walk down the AST rooted at `ast`, and mutate the `script`
    /// and `indent_level` as it sees fit. After visiting the `ast`, it returns the current state of the `script`.
    pub fn emit(&mut self, ast: &mut NodeProgram) -> String {
        self.script = "".to_string();
        // TODO: Handle errors
        // Consider adding an error logger and be greedy
        // when collecting errors
        let _ = ast.visit(self);
        self.script.clone()
    }

    /// This function adds newlines to the `script`. The number of newlines to be added is specified by the
    /// `count` parameter. After adding the newlines, it appends spaces equivalent to twice the current
    /// `indent_level` to maintain indentation.
    fn add_newlines(&mut self, count: usize) {
        self.script.push_str(&"\n".repeat(count));
        // TODO: Consider making indentation configurable from constants.rs
        self.script.push_str(&" ".repeat(self.indent_level * 2));
    }
}

impl AstConverting for BluebellFormatter {
    fn push_source_position(&mut self, _start: &SourcePosition, _end: &SourcePosition) {}

    fn pop_source_position(&mut self) {}

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
                    self.script.push_str(&n.node);
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
                    let _ = l.node.visit(self)?;
                    self.script.push_str(".");
                    let _ = r.node.visit(self)?;
                    return Ok(TraversalResult::SkipChildren);
                }
                NodeMetaIdentifier::MetaNameInHexspace(l, r) => {
                    self.script.push_str(&l.node);
                    self.script.push_str(".");
                    let _ = r.node.visit(self)?;
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
                NodeVariableIdentifier::VariableName(v) => self.script.push_str(&v.node),
                NodeVariableIdentifier::SpecialIdentifier(v) => self.script.push_str(&v.node),
                NodeVariableIdentifier::VariableInNamespace(_, v) => {
                    self.script.push_str(".");
                    self.script.push_str(&v.node);
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
                let _ = arg.node.visit(self)?;
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
                let _ = value.node.visit(self)?;
            }
            NodeTypeMapKey::EnclosedGenericId(value) => {
                self.script.push_str("(");
                let _ = value.node.visit(self)?;
                self.script.push_str(")");
            }
            NodeTypeMapKey::EnclosedAddressMapKeyType(value) => {
                self.script.push_str("(");
                let _ = value.node.visit(self)?;
                self.script.push_str(")");
            }
            NodeTypeMapKey::AddressMapKeyType(value) => {
                let _ = value.node.visit(self)?;
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
                let _ = value.node.visit(self)?;
            }
            NodeTypeMapValue::MapKeyValue(value) => {
                let _ = (*value).node.visit(self)?;
            }
            NodeTypeMapValue::MapValueParanthesizedType(value) => {
                self.script.push_str("(");
                let _ = (*value).node.visit(self)?;
                self.script.push_str(")");
            }
            NodeTypeMapValue::MapValueAddressType(value) => {
                let _ = (*value).node.visit(self)?;
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
                    self.script.push_str(&var.node);
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
                let _ = lead.node.visit(self)?;
                for arg in args.iter() {
                    self.script.push_str(" ");
                    let _ = arg.node.visit(self)?;
                }
            }
            NodeScillaType::MapType(key, value) => {
                self.script.push_str("Map ");
                let _ = key.node.visit(self)?;
                self.script.push_str(" ");
                let _ = value.node.visit(self)?;
            }
            NodeScillaType::FunctionType(a, b) => {
                let _ = (*a).node.visit(self)?;
                let _ = (*b).node.visit(self)?;
            }
            NodeScillaType::EnclosedType(a) => {
                self.script.push_str("( ");
                let _ = (*a).node.visit(self)?;
                self.script.push_str(" )");
            }
            NodeScillaType::ScillaAddresseType(a) => {
                let _ = (*a).node.visit(self)?;
            }
            NodeScillaType::PolyFunctionType(name, a) => {
                self.script.push_str(&name.node);
                let _ = (*a).node.visit(self)?;
            }
            NodeScillaType::TypeVarType(name) => {
                self.script.push_str(&name.node);
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
        let _ = node.identifier.node.visit(self)?;
        self.script.push_str(" : ");
        let _ = node.type_name.node.visit(self)?;
        Ok(TraversalResult::SkipChildren)
    }

    fn emit_address_type(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodeAddressType,
    ) -> Result<TraversalResult, String> {
        let _ = node.identifier.node.visit(self)?;
        self.script.push_str(" with ");

        if node.type_name.node.len() > 0 {
            self.script.push_str(&node.type_name.node);
            self.script.push_str(" ");
        }

        for field in node.address_fields.iter() {
            let _ = field.node.visit(self)?;
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
                self.script.push_str(&identifier_name.node);
                self.indent_level += 1;
                if let Some(t) = type_annotation {
                    let _ = t.node.visit(self)?;
                }
                self.script.push_str(" = ");
                let _ = (*expression).node.visit(self)?;
                self.script.push_str(" in ");
                let _ = (*containing_expression).node.visit(self)?;
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
                self.script.push_str(&identier_value.node);
                let _ = type_annotation.node.visit(self)?;
                self.script.push_str(") => ");

                let _ = (*expression).node.visit(self)?;
                self.indent_level -= 1;
            }
            NodeFullExpression::FunctionCall {
                function_name,
                argument_list,
            } => {
                let _ = function_name.node.visit(self)?;
                for arg in argument_list.iter() {
                    self.script.push_str(" ");
                    let _ = arg.node.visit(self)?;
                }
            }
            NodeFullExpression::ExpressionAtomic(expr) => {
                let _ = expr.node.visit(self)?;
            }
            NodeFullExpression::ExpressionBuiltin { b, targs, xs } => {
                self.script.push_str("builtin ");
                self.script.push_str(&b.node);
                if let Some(args) = targs {
                    let _ = args.node.visit(self)?;
                }
                self.script.push_str(" ");
                let _ = xs.node.visit(self)?;
            }
            NodeFullExpression::Message(entries) => {
                self.script.push_str("{");
                self.indent_level += 1;
                for (i, message) in entries.iter().enumerate() {
                    let _ = message.node.visit(self)?;
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
                let _ = match_expression.node.visit(self)?;
                self.script.push_str(" with ");
                self.indent_level += 1;
                for clause in clauses.iter() {
                    let _ = clause.node.visit(self)?;
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
                let _ = identifier_name.node.visit(self)?;
                if let Some(cta) = contract_type_arguments {
                    self.script.push_str(" ");
                    let _ = cta.node.visit(self)?;
                }
                for a in argument_list.iter() {
                    self.script.push_str(" ");
                    let _ = a.node.visit(self)?;
                }
            }
            NodeFullExpression::TemplateFunction {
                identifier_name,
                expression,
            } => {
                self.add_newlines(1);
                self.script.push_str("tfun ");
                self.script.push_str(&identifier_name.node);
                self.script.push_str(" => ");
                let _ = expression.node.visit(self)?;
            }
            NodeFullExpression::TApp {
                identifier_name,
                type_arguments,
            } => {
                self.script.push_str("@");
                let _ = identifier_name.node.visit(self)?;
                for arg in type_arguments.iter() {
                    self.script.push_str(" ");
                    let _ = arg.node.visit(self)?;
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
                let _ = var.node.visit(self)?;
                self.script.push_str(" : ");
                let _ = val.node.visit(self)?;
            }
            NodeMessageEntry::MessageVariable(var1, var2) => {
                let _ = var1.node.visit(self)?;
                self.script.push_str(" : ");
                let _ = var2.node.visit(self)?;
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
        let _ = node.pattern.node.visit(self)?;
        self.script.push_str(" => ");
        let _ = node.expression.node.visit(self)?;
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

            let _ = arg.node.visit(self)?;
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
                    let _ = n.node.visit(self)?;
                    self.script.push_str(" ");
                    self.script.push_str(&v.node);
                }
                NodeValueLiteral::LiteralHex(h) => {
                    self.script.push_str(&format!("0x{}", h)); // Push the literal hexadecimal type definition to the script
                }
                NodeValueLiteral::LiteralString(s) => {
                    self.script.push_str(&format!("\"{}\"", s)); // Push the literal string type definition to the script
                }
                NodeValueLiteral::LiteralEmptyMap(key_type, value_type) => {
                    self.script.push_str("Emp ");
                    let _ = key_type.node.visit(self)?;
                    self.script.push_str(" ");
                    let _ = value_type.node.visit(self)?;
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
                self.script.push_str(&value.node);
            }
            NodePattern::Constructor(identifier, args) => {
                let _ = identifier.node.visit(self)?;

                for arg in args.iter() {
                    self.script.push_str(" ");
                    let _ = arg.node.visit(self)?;
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
                NodeArgumentPattern::BinderArgument(s) => self.script.push_str(&s.node),
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
                let _ = node.pattern_expression.node.visit(self)?;
                self.script.push_str(" =>");
                if let Some(stmt) = &node.statement_block {
                    let _ = stmt.node.visit(self)?;
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
            let _ = arg.node.visit(self)?;
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
                self.script.push_str(&left_hand_side.node);
                self.script.push_str(" <- ");
                let _ = right_hand_side.node.visit(self)?;
            }
            NodeStatement::RemoteFetch(fetch_statement) => {
                let _ = (*fetch_statement).visit(self)?;
            }
            NodeStatement::Store {
                left_hand_side,
                right_hand_side,
            } => {
                self.script.push_str(&left_hand_side.node);
                self.script.push_str(" := ");
                let _ = right_hand_side.node.visit(self)?;
            }
            NodeStatement::Bind {
                left_hand_side,
                right_hand_side,
            } => {
                self.script.push_str(&left_hand_side.node);
                self.script.push_str(" = ");
                let _ = right_hand_side.node.visit(self)?;
            }
            NodeStatement::ReadFromBC {
                left_hand_side,
                type_name,
                arguments,
            } => {
                self.script.push_str(&left_hand_side.node);
                self.script.push_str(" <-& ");
                let _ = type_name.node.visit(self)?;
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
                self.script.push_str(&left_hand_side.node);
                for key in keys.iter() {
                    let _ = key.node.visit(self)?;
                }
                self.script.push_str(" := ");
                let _ = right_hand_side.node.visit(self)?;
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
                let _ = identifier_name.node.visit(self)?;
            }
            NodeStatement::CreateEvnt { identifier_name } => {
                self.script.push_str("event ");
                let _ = identifier_name.node.visit(self)?;
            }
            NodeStatement::Throw { error_variable } => {
                self.script.push_str("throw");
                if let Some(e) = error_variable {
                    self.script.push_str(" ");
                    let _ = e.node.visit(self)?;
                }
            }
            NodeStatement::MatchStmt { variable, clauses } => {
                self.script.push_str("match ");
                let _ = variable.node.visit(self)?;
                self.script.push_str(" with");
                self.indent_level += 1;
                for clause in clauses.iter() {
                    let _ = clause.node.visit(self)?;
                }
                self.indent_level -= 1;
                self.add_newlines(1);
                self.script.push_str("end");
            }
            NodeStatement::CallProc {
                component_id,
                arguments,
            } => {
                let _ = component_id.node.visit(self)?;
                for arg in arguments.iter() {
                    self.script.push_str(" ");
                    let _ = arg.node.visit(self)?;
                }
            }
            NodeStatement::Iterate {
                identifier_name,
                component_id,
            } => {
                self.script.push_str("forall ");
                let _ = identifier_name.node.visit(self)?;
                self.script.push_str(" ");
                let _ = component_id.node.visit(self)?;
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
                let _ = identifier.node.visit(self)?;
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
                    let _ = access.node.visit(self)?;
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
                    let _ = access.node.visit(self)?;
                }
            }
            NodeRemoteFetchStatement::ReadStateMutableCastAddress(
                lhs,
                address_id,
                address_type,
            ) => {
                self.script.push_str(&format!("{} <-& ", lhs));
                let _ = address_id.node.visit(self)?;
                self.script.push_str(" as ");

                let _ = address_type.node.visit(self)?;
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
                NodeComponentId::WithRegularId(name) => self.script.push_str(&name.node),
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
                    let _ = parameter.node.visit(self)?;
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
                self.script.push_str(&node.identifier_name.node);
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
                    self.script.push_str(&variable_name.node);
                    self.indent_level += 1;
                    if let Some(v) = type_annotation {
                        let _ = v.node.visit(self)?;
                    }
                    self.script.push_str(" = ");
                    let _ = expression.node.visit(self)?;
                    self.indent_level -= 1;
                }
                NodeLibrarySingleDefinition::TypeDefinition(name, clauses) => {
                    self.add_newlines(1);
                    self.script.push_str("type ");
                    let _ = name.node.visit(self)?;
                    match clauses {
                        Some(clauses) => {
                            self.script.push_str(" =");
                            self.indent_level += 1;
                            for clause in clauses.iter() {
                                let _ = clause.node.visit(self)?;
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
                let _ = node.typed_identifier.node.visit(self)?;
                self.script.push_str(" = ");
                let _ = node.right_hand_side.node.visit(self)?;
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
                let _ = v.node.visit(self)?;
            }
            NodeTypeAlternativeClause::ClauseTypeWithArgs(name, args) => {
                let _ = name.node.visit(self)?;
                self.script.push_str(" of");
                for arg in args.iter() {
                    self.script.push_str(" ");
                    let _ = arg.node.visit(self)?;
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
