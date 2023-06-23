use crate::ast::*;
use crate::code_emitter::{CodeEmitter, TraversalResult, TreeTraversalMode};
use crate::visitor::Visitor;

pub struct ScillaCodeEmitter {
    indent_level: usize,
    script: String,
}

impl ScillaCodeEmitter {
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
        node.visit(self);

        self.script.clone()
    }
}

impl CodeEmitter for ScillaCodeEmitter {
    fn emit_byte_str(&mut self, mode: TreeTraversalMode, node: &NodeByteStr) -> TraversalResult {
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
        TraversalResult::Ok
    }

    fn emit_type_name_identifier(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeNameIdentifier,
    ) -> TraversalResult {
        match mode {
            TreeTraversalMode::Enter => match node {
                NodeTypeNameIdentifier::ByteStringType(_) => (),
                NodeTypeNameIdentifier::EventType => {
                    self.script.push_str("Event");
                }
                NodeTypeNameIdentifier::CustomType(n) => {
                    self.script.push_str(n);
                }
            },
            TreeTraversalMode::Exit => (),
        }
        TraversalResult::Ok
    }

    fn emit_imported_name(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeImportedName,
    ) -> TraversalResult {
        match mode {
            TreeTraversalMode::Enter => {
                self.add_newlines(1);
                self.script.push_str("import ");
            }
            TreeTraversalMode::Exit => (),
        }

        TraversalResult::Ok
    }

    fn emit_import_declarations(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeImportDeclarations,
    ) -> TraversalResult {
        match mode {
            TreeTraversalMode::Enter => {
                self.add_newlines(1);
            }
            TreeTraversalMode::Exit => (),
        }

        TraversalResult::Ok
    }

    fn emit_meta_identifier(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeMetaIdentifier,
    ) -> TraversalResult {
        match mode {
            TreeTraversalMode::Enter => match node {
                NodeMetaIdentifier::MetaNameInNamespace(l, r) => {
                    l.visit(self);
                    self.script.push_str(".");
                    r.visit(self);
                    return TraversalResult::SkipChildren;
                }
                NodeMetaIdentifier::MetaNameInHexspace(l, r) => {
                    self.script.push_str(&l);
                    self.script.push_str(".");
                    r.visit(self);
                    return TraversalResult::SkipChildren;
                }
                NodeMetaIdentifier::ByteString => {
                    self.script.push_str("ByStr");
                }
                NodeMetaIdentifier::MetaName(_) => (),
            },
            TreeTraversalMode::Exit => (),
        }
        TraversalResult::Ok
    }

    fn emit_variable_identifier(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeVariableIdentifier,
    ) -> TraversalResult {
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
        TraversalResult::Ok
    }

    fn emit_builtin_arguments(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeBuiltinArguments,
    ) -> TraversalResult {
        if node.arguments.len() == 0 {
            self.script.push_str("()");
        } else {
            for (i, arg) in node.arguments.iter().enumerate() {
                if i != 0 {
                    self.script.push_str(" ");
                }
                arg.visit(self);
            }
        }
        TraversalResult::SkipChildren
    }

    fn emit_type_map_key(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeMapKey,
    ) -> TraversalResult {
        match node {
            NodeTypeMapKey::GenericMapKey(value) => {
                value.visit(self);
            }
            NodeTypeMapKey::EnclosedGenericId(value) => {
                self.script.push_str("(");
                value.visit(self);
                self.script.push_str(")");
            }
            NodeTypeMapKey::EnclosedAddressMapKeyType(value) => {
                self.script.push_str("(");
                value.visit(self);
                self.script.push_str(")");
            }
            NodeTypeMapKey::AddressMapKeyType(value) => {
                value.visit(self);
            }
        }
        TraversalResult::SkipChildren
    }

    fn emit_type_map_value(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeMapValue,
    ) -> TraversalResult {
        match node {
            NodeTypeMapValue::MapValueCustomType(value) => {
                value.visit(self);
            }
            NodeTypeMapValue::MapKeyValue(value) => {
                (*value).visit(self);
            }
            NodeTypeMapValue::MapValueParanthesizedType(value) => {
                self.script.push_str("(");
                (*value).visit(self);
                self.script.push_str(")");
            }
            NodeTypeMapValue::MapValueAddressType(value) => {
                (*value).visit(self);
            }
        }
        TraversalResult::SkipChildren
    }

    fn emit_type_argument(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeArgument,
    ) -> TraversalResult {
        match mode {
            TreeTraversalMode::Enter => match node {
                NodeTypeArgument::MapTypeArgument(k, v) => {
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
        TraversalResult::Ok
    }

    fn emit_scilla_type(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeScillaType,
    ) -> TraversalResult {
        match node {
            NodeScillaType::GenericTypeWithArgs(lead, args) => {
                lead.visit(self);
                for arg in args.iter() {
                    self.script.push_str(" ");
                    arg.visit(self);
                }
            }
            NodeScillaType::MapType(key, value) => {
                self.script.push_str("Map ");
                key.visit(self);
                self.script.push_str(" ");
                value.visit(self);
            }
            NodeScillaType::FunctionType(a, b) => {
                (*a).visit(self);
                (*b).visit(self);
            }
            NodeScillaType::EnclosedType(a) => {
                self.script.push_str("( ");
                (*a).visit(self);
                self.script.push_str(" )");
            }
            NodeScillaType::ScillaAddresseType(a) => {
                (*a).visit(self);
            }
            NodeScillaType::PolyFunctionType(name, a) => {
                self.script.push_str(name);
                (*a).visit(self);
            }
            NodeScillaType::TypeVarType(name) => {
                self.script.push_str(name);
            }
        };
        TraversalResult::SkipChildren
    }

    fn emit_type_map_entry(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeMapEntry,
    ) -> TraversalResult {
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
        mode: TreeTraversalMode,
        node: &NodeAddressTypeField,
    ) -> TraversalResult {
        self.script.push_str("field ");
        node.identifier.visit(self);
        self.script.push_str(" : ");
        node.type_name.visit(self);
        TraversalResult::SkipChildren
    }

    fn emit_address_type(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeAddressType,
    ) -> TraversalResult {
        node.identifier.visit(self);
        self.script.push_str(" with ");

        if node.type_name.len() > 0 {
            self.script.push_str(&node.type_name);
            self.script.push_str(" ");
        }

        for field in node.address_fields.iter() {
            field.visit(self);
            self.script.push_str(" ");
        }

        self.script.push_str("end");

        TraversalResult::SkipChildren
    }

    fn emit_full_expression(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeFullExpression,
    ) -> TraversalResult {
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
                    t.visit(self);
                }
                self.script.push_str(" = ");
                (*expression).visit(self);
                self.script.push_str(" in ");
                (*containing_expression).visit(self);
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
                type_annotation.visit(self);
                self.script.push_str(") => ");

                (*expression).visit(self);
                self.indent_level -= 1;
            }
            NodeFullExpression::FunctionCall {
                function_name,
                argument_list,
            } => {
                function_name.visit(self);
                for arg in argument_list.iter() {
                    self.script.push_str(" ");
                    arg.visit(self);
                }
            }
            NodeFullExpression::ExpressionAtomic(expr) => {
                expr.visit(self);
            }
            NodeFullExpression::ExpressionBuiltin { b, targs, xs } => {
                self.script.push_str("builtin ");
                self.script.push_str(b);
                if let Some(args) = targs {
                    args.visit(self);
                }
                self.script.push_str(" ");
                xs.visit(self);
            }
            NodeFullExpression::Message(entries) => {
                self.script.push_str("{");
                self.indent_level += 1;
                for (i, message) in entries.iter().enumerate() {
                    message.visit(self);
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
                match_expression.visit(self);
                self.script.push_str(" with ");
                self.indent_level += 1;
                for clause in clauses.iter() {
                    clause.visit(self);
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
                identifier_name.visit(self);
                if let Some(cta) = contract_type_arguments {
                    self.script.push_str(" ");
                    cta.visit(self);
                }
                for a in argument_list.iter() {
                    self.script.push_str(" ");
                    a.visit(self);
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
                expression.visit(self);
            }
            NodeFullExpression::TApp {
                identifier_name,
                type_arguments,
            } => {
                self.script.push_str("@");
                identifier_name.visit(self);
                for arg in type_arguments.iter() {
                    self.script.push_str(" ");
                    arg.visit(self);
                }
            }
        }
        TraversalResult::SkipChildren
    }

    fn emit_message_entry(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeMessageEntry,
    ) -> TraversalResult {
        self.add_newlines(1);
        match node {
            NodeMessageEntry::MessageLiteral(var, val) => {
                // Converting the variable and value literals into Scilla code
                // Assuming the emit_variable_identifier and emit_value_literal are implemented
                var.visit(self);
                self.script.push_str(" : ");
                val.visit(self);
            }
            NodeMessageEntry::MessageVariable(var1, var2) => {
                var1.visit(self);
                self.script.push_str(" : ");
                var2.visit(self);
            }
        }
        TraversalResult::SkipChildren
    }
    fn emit_pattern_match_expression_clause(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodePatternMatchExpressionClause,
    ) -> TraversalResult {
        self.add_newlines(1);
        self.script.push_str("| ");
        node.pattern.visit(self);
        self.script.push_str(" => ");
        node.expression.visit(self);
        TraversalResult::SkipChildren
    }

    fn emit_atomic_expression(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeAtomicExpression,
    ) -> TraversalResult {
        // Pass through
        TraversalResult::Ok
    }

    fn emit_contract_type_arguments(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeContractTypeArguments,
    ) -> TraversalResult {
        self.script.push_str("{");
        for (i, arg) in node.type_arguments.iter().enumerate() {
            if i != 0 {
                self.script.push_str(" ");
            }

            arg.visit(self);
        }
        self.script.push_str("}");

        TraversalResult::SkipChildren
    }

    fn emit_value_literal(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeValueLiteral,
    ) -> TraversalResult {
        match mode {
            TreeTraversalMode::Enter => match node {
                NodeValueLiteral::LiteralInt(n, v) => {
                    n.visit(self);
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
                    key_type.visit(self);
                    self.script.push_str(" ");
                    value_type.visit(self);
                }
            },
            TreeTraversalMode::Exit => (),
        }
        TraversalResult::SkipChildren
    }

    fn emit_map_access(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeMapAccess,
    ) -> TraversalResult {
        match mode {
            TreeTraversalMode::Enter => self.script.push_str("["),
            TreeTraversalMode::Exit => self.script.push_str("]"),
        }
        TraversalResult::Ok
    }

    fn emit_pattern(&mut self, mode: TreeTraversalMode, node: &NodePattern) -> TraversalResult {
        match node {
            NodePattern::Wildcard => {
                self.script.push_str("_");
            }
            NodePattern::Binder(value) => {
                self.script.push_str(value);
            }
            NodePattern::Constructor(identifier, args) => {
                identifier.visit(self);

                for arg in args.iter() {
                    self.script.push_str(" ");
                    arg.visit(self);
                }
            }
        }
        TraversalResult::SkipChildren
    }

    fn emit_argument_pattern(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeArgumentPattern,
    ) -> TraversalResult {
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
        TraversalResult::Ok
    }

    fn emit_pattern_match_clause(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodePatternMatchClause,
    ) -> TraversalResult {
        match mode {
            TreeTraversalMode::Enter => {
                self.add_newlines(1);
                self.script.push_str("| ");
                node.pattern_expression.visit(self);
                self.script.push_str(" =>");
                if let Some(stmt) = &node.statement_block {
                    stmt.visit(self);
                }
            }
            TreeTraversalMode::Exit => {}
        }
        TraversalResult::SkipChildren
    }

    fn emit_blockchain_fetch_arguments(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeBlockchainFetchArguments,
    ) -> TraversalResult {
        self.script.push_str("(");
        for arg in node.arguments.iter() {
            self.script.push_str(" ");
            arg.visit(self);
        }
        self.script.push_str(" )");
        TraversalResult::SkipChildren
    }

    fn emit_statement(&mut self, mode: TreeTraversalMode, node: &NodeStatement) -> TraversalResult {
        self.add_newlines(1);
        match node {
            NodeStatement::Load {
                left_hand_side,
                right_hand_side,
            } => {
                self.script.push_str(left_hand_side);
                self.script.push_str(" <- ");
                right_hand_side.visit(self);
            }
            NodeStatement::RemoteFetch(fetch_statement) => {
                (*fetch_statement).visit(self);
            }
            NodeStatement::Store {
                left_hand_side,
                right_hand_side,
            } => {
                self.script.push_str(left_hand_side);
                self.script.push_str(" := ");
                right_hand_side.visit(self);
            }
            NodeStatement::Bind {
                left_hand_side,
                right_hand_side,
            } => {
                self.script.push_str(left_hand_side);
                self.script.push_str(" = ");
                right_hand_side.visit(self);
            }
            NodeStatement::ReadFromBC {
                left_hand_side,
                type_name,
                arguments,
            } => {
                self.script.push_str(left_hand_side);
                self.script.push_str(" <-& ");
                type_name.visit(self);
                if let Some(args) = arguments {
                    args.visit(self);
                }
            }
            NodeStatement::MapGet {
                left_hand_side,
                keys,
                right_hand_side,
            } => {
                unimplemented!();
            }
            NodeStatement::MapGetExists {
                left_hand_side,
                keys,
                right_hand_side,
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
                    key.visit(self);
                }
                self.script.push_str(" := ");
                right_hand_side.visit(self);
            }
            NodeStatement::MapUpdateDelete {
                left_hand_side,
                keys,
            } => {
                unimplemented!();
            }
            NodeStatement::Accept => self.script.push_str("accept"),
            NodeStatement::Send { identifier_name } => {
                self.script.push_str("send ");
                identifier_name.visit(self);
            }
            NodeStatement::CreateEvnt { identifier_name } => {
                self.script.push_str("event ");
                identifier_name.visit(self);
            }
            NodeStatement::Throw { error_variable } => {
                self.script.push_str("throw");
                if let Some(e) = error_variable {
                    self.script.push_str(" ");
                    e.visit(self);
                }
            }
            NodeStatement::MatchStmt { variable, clauses } => {
                self.script.push_str("match ");
                variable.visit(self);
                self.script.push_str(" with");
                self.indent_level += 1;
                for clause in clauses.iter() {
                    clause.visit(self);
                }
                self.indent_level -= 1;
                self.add_newlines(1);
                self.script.push_str("end");
            }
            NodeStatement::CallProc {
                component_id,
                arguments,
            } => {
                component_id.visit(self);
                for arg in arguments.iter() {
                    self.script.push_str(" ");
                    arg.visit(self);
                }
            }
            NodeStatement::Iterate {
                identifier_name,
                component_id,
            } => {
                self.script.push_str("forall ");
                identifier_name.visit(self);
                self.script.push_str(" ");
                component_id.visit(self);
            }
        }

        TraversalResult::SkipChildren
    }

    fn emit_remote_fetch_statement(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeRemoteFetchStatement,
    ) -> TraversalResult {
        match node {
            NodeRemoteFetchStatement::ReadStateMutable(lhs, address, identifier) => {
                self.script.push_str(&format!("{} <-& {}.", lhs, address));
                identifier.visit(self);
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
                    access.visit(self);
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
                    access.visit(self);
                }
            }
            NodeRemoteFetchStatement::ReadStateMutableCastAddress(
                lhs,
                address_id,
                address_type,
            ) => {
                self.script.push_str(&format!("{} <-& ", lhs));
                address_id.visit(self);
                self.script.push_str(" as ");

                address_type.visit(self);
            }
        }
        TraversalResult::SkipChildren
    }

    fn emit_component_id(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeComponentId,
    ) -> TraversalResult {
        match mode {
            TreeTraversalMode::Enter => match node {
                NodeComponentId::WithRegularId(name) => self.script.push_str(name),
                _ => (),
            },
            _ => (),
        }
        TraversalResult::Ok
    }

    fn emit_component_parameters(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeComponentParameters,
    ) -> TraversalResult {
        match mode {
            TreeTraversalMode::Enter => {
                self.script.push_str("(");
                for (i, parameter) in node.parameters.iter().enumerate() {
                    if i > 0 {
                        self.script.push_str(", ");
                    }
                    parameter.visit(self);
                }
                self.script.push_str(")");
            }
            TreeTraversalMode::Exit => (),
        }
        TraversalResult::SkipChildren
    }

    fn emit_parameter_pair(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeParameterPair,
    ) -> TraversalResult {
        // Pass through
        TraversalResult::Ok
    }

    fn emit_component_body(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeComponentBody,
    ) -> TraversalResult {
        // Pass through
        TraversalResult::Ok
    }

    fn emit_statement_block(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeStatementBlock,
    ) -> TraversalResult {
        self.indent_level += 1;
        for (i, stmt) in node.statements.iter().enumerate() {
            stmt.visit(self);
            if i != node.statements.len() - 1 {
                self.script.push_str(";");
            }
        }
        self.indent_level -= 1;
        TraversalResult::SkipChildren
    }

    fn emit_typed_identifier(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypedIdentifier,
    ) -> TraversalResult {
        match mode {
            TreeTraversalMode::Enter => {
                // Assuming that annotation type is of String
                self.script.push_str(&node.identifier_name);
            }
            TreeTraversalMode::Exit => (),
        }
        TraversalResult::Ok
    }

    fn emit_type_annotation(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeAnnotation,
    ) -> TraversalResult {
        match mode {
            TreeTraversalMode::Enter => self.script.push_str(" : "),
            _ => (),
        }
        TraversalResult::Ok
    }

    fn emit_program(&mut self, mode: TreeTraversalMode, node: &NodeProgram) -> TraversalResult {
        match mode {
            TreeTraversalMode::Enter => {
                self.script
                    .push_str(&format!("scilla_version {}", node.version));
            }
            TreeTraversalMode::Exit => {
                self.script.push_str("\n");
            }
        }
        TraversalResult::Ok
    }

    fn emit_library_definition(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeLibraryDefinition,
    ) -> TraversalResult {
        match mode {
            TreeTraversalMode::Enter => {
                // Add Indent
                self.add_newlines(2);
                self.script.push_str(&format!("library {}", node.name));
            }
            _ => (),
        }
        TraversalResult::Ok
    }

    fn emit_library_single_definition(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeLibrarySingleDefinition,
    ) -> TraversalResult {
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
                        v.visit(self);
                    }
                    self.script.push_str(" = ");
                    expression.visit(self);
                    self.indent_level -= 1;
                }
                NodeLibrarySingleDefinition::TypeDefinition(name, clauses) => {
                    self.add_newlines(1);
                    self.script.push_str("type ");
                    name.visit(self);
                    match clauses {
                        Some(clauses) => {
                            self.script.push_str(" =");
                            self.indent_level += 1;
                            for clause in clauses.iter() {
                                clause.visit(self);
                            }
                            self.indent_level -= 1;
                        }
                        None => (),
                    }
                }
            },
            _ => {}
        }
        TraversalResult::SkipChildren
    }

    fn emit_contract_definition(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeContractDefinition,
    ) -> TraversalResult {
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
        TraversalResult::Ok
    }

    fn emit_contract_field(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeContractField,
    ) -> TraversalResult {
        match mode {
            TreeTraversalMode::Enter => {
                self.add_newlines(1);
                self.script.push_str("field ");
                node.typed_identifier.visit(self);
                self.script.push_str(" = ");
                node.right_hand_side.visit(self);
            }
            _ => (),
        }
        TraversalResult::SkipChildren
    }

    fn emit_with_constraint(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeWithConstraint,
    ) -> TraversalResult {
        match mode {
            TreeTraversalMode::Enter => {
                self.script.push_str(" with ");
            }
            _ => {
                self.script.push_str(" =>");
            }
        }
        TraversalResult::Ok
    }

    fn emit_component_definition(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeComponentDefinition,
    ) -> TraversalResult {
        // Fall through to either Transition or Procedure
        TraversalResult::Ok
    }

    fn emit_procedure_definition(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeProcedureDefinition,
    ) -> TraversalResult {
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
        TraversalResult::Ok
    }

    fn emit_transition_definition(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTransitionDefinition,
    ) -> TraversalResult {
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
        TraversalResult::Ok
    }

    fn emit_type_alternative_clause(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeAlternativeClause,
    ) -> TraversalResult {
        self.add_newlines(1);
        self.script.push_str("| ");
        match node {
            NodeTypeAlternativeClause::ClauseType(v) => {
                v.visit(self);
            }
            NodeTypeAlternativeClause::ClauseTypeWithArgs(name, args) => {
                name.visit(self);
                self.script.push_str(" of");
                for arg in args.iter() {
                    self.script.push_str(" ");
                    arg.visit(self);
                }
            }
            _ => (),
        }

        TraversalResult::SkipChildren
    }

    fn emit_type_map_value_arguments(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeMapValueArguments,
    ) -> TraversalResult {
        unimplemented!()
    }

    fn emit_type_map_value_allowing_type_arguments(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeMapValueAllowingTypeArguments,
    ) -> TraversalResult {
        // Pass through
        TraversalResult::Ok
    }
}
