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

    pub fn newline(&mut self) {
        self.script.push_str("\n");
        self.script.push_str(&" ".repeat(self.indent_level * 2));
    }

    pub fn emit(&mut self, node: &mut NodeProgram) {
        node.visit(self);
        println!("Formatted: {:?}", self.script);
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
                    self.script.push_str(&format!("ByStr{}", s)); // Push a byte string type definition to the script
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
        unimplemented!()
    }

    fn emit_import_declarations(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeImportDeclarations,
    ) -> TraversalResult {
        unimplemented!()
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
        unimplemented!()
    }

    fn emit_type_map_key(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeMapKey,
    ) -> TraversalResult {
        unimplemented!()
    }

    fn emit_type_map_value(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeMapValue,
    ) -> TraversalResult {
        unimplemented!()
    }

    fn emit_type_argument(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeArgument,
    ) -> TraversalResult {
        unimplemented!()
    }

    fn emit_scilla_type(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeScillaType,
    ) -> TraversalResult {
        match mode {
            TreeTraversalMode::Enter => match node {
                _ => (),
                NodeScillaType::TypeVarType(value)
                | NodeScillaType::PolyFunctionType(value, ..) => {
                    self.script.push_str(&value);
                }
            },
            _ => (),
        }
        TraversalResult::Ok
    }

    fn emit_type_map_entry(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeMapEntry,
    ) -> TraversalResult {
        unimplemented!()
    }

    fn emit_address_type_field(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeAddressTypeField,
    ) -> TraversalResult {
        unimplemented!()
    }

    fn emit_address_type(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeAddressType,
    ) -> TraversalResult {
        unimplemented!()
    }

    fn emit_full_expression(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeFullExpression,
    ) -> TraversalResult {
        // TODO: Pass through for now
        TraversalResult::Ok
    }

    fn emit_message_entry(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeMessageEntry,
    ) -> TraversalResult {
        match mode {
            TreeTraversalMode::Enter => match node {
                NodeMessageEntry::MessageLiteral(var, val) => {
                    // Converting the variable and value literals into Scilla code
                    // Assuming the emit_variable_identifier and emit_value_literal are implemented
                    var.visit(self);
                    self.script.push_str(":");
                    val.visit(self);
                }
                NodeMessageEntry::MessageVariable(var1, var2) => {
                    var1.visit(self);
                    self.script.push_str(":");
                    var2.visit(self);
                }
            },
            TreeTraversalMode::Exit => (),
        }
        TraversalResult::SkipChildren
    }
    fn emit_pattern_match_expression_clause(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodePatternMatchExpressionClause,
    ) -> TraversalResult {
        unimplemented!()
    }

    fn emit_atomic_expression(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeAtomicExpression,
    ) -> TraversalResult {
        unimplemented!()
    }

    fn emit_contract_type_arguments(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeContractTypeArguments,
    ) -> TraversalResult {
        unimplemented!()
    }

    fn emit_value_literal(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeValueLiteral,
    ) -> TraversalResult {
        unimplemented!()
    }

    fn emit_map_access(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeMapAccess,
    ) -> TraversalResult {
        unimplemented!()
    }

    fn emit_pattern(&mut self, mode: TreeTraversalMode, node: &NodePattern) -> TraversalResult {
        unimplemented!()
    }

    fn emit_argument_pattern(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeArgumentPattern,
    ) -> TraversalResult {
        unimplemented!()
    }

    fn emit_pattern_match_clause(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodePatternMatchClause,
    ) -> TraversalResult {
        unimplemented!()
    }

    fn emit_blockchain_fetch_arguments(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeBlockchainFetchArguments,
    ) -> TraversalResult {
        unimplemented!()
    }

    fn emit_statement(&mut self, mode: TreeTraversalMode, node: &NodeStatement) -> TraversalResult {
        // TODO: Pass through for now
        TraversalResult::Ok
    }

    fn emit_remote_fetch_statement(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeRemoteFetchStatement,
    ) -> TraversalResult {
        unimplemented!()
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
                self.newline();
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
        match mode {
            TreeTraversalMode::Enter => (),
            TreeTraversalMode::Exit => (),
        }
        // TODO: Pass through for now
        TraversalResult::Ok
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
            TreeTraversalMode::Enter => self.script.push_str(": "),
            _ => (),
        }
        TraversalResult::Ok
    }

    fn emit_program(&mut self, mode: TreeTraversalMode, node: &NodeProgram) -> TraversalResult {
        match mode {
            TreeTraversalMode::Enter => {
                self.script
                    .push_str(&format!("scilla_version {}", node.version));
                self.newline();
                self.newline();
            }
            TreeTraversalMode::Exit => (),
        }
        TraversalResult::Ok
    }

    fn emit_library_definition(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeLibraryDefinition,
    ) -> TraversalResult {
        unimplemented!()
    }

    fn emit_library_single_definition(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeLibrarySingleDefinition,
    ) -> TraversalResult {
        unimplemented!()
    }

    fn emit_contract_definition(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeContractDefinition,
    ) -> TraversalResult {
        match mode {
            TreeTraversalMode::Enter => {
                // Add Indent
                println!("Entering in contract!");
                let indent = "    ".repeat(self.indent_level as usize);
                self.script.push_str(&indent);
                // Add Contract definition, contract name and open parentheses
                self.script
                    .push_str(&format!("contract {}", node.contract_name));
                self.indent_level += 1;
            }
            _ => {
                self.indent_level -= 1;
            }
        }
        TraversalResult::Ok
    }

    fn emit_contract_field(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeContractField,
    ) -> TraversalResult {
        unimplemented!()
    }

    fn emit_with_constraint(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeWithConstraint,
    ) -> TraversalResult {
        unimplemented!()
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
        unimplemented!()
    }

    fn emit_transition_definition(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTransitionDefinition,
    ) -> TraversalResult {
        match mode {
            TreeTraversalMode::Enter => {
                self.indent_level += 1;
                self.script.push_str("transition ");
            }
            TreeTraversalMode::Exit => {
                self.indent_level -= 1;
            }
        }
        TraversalResult::Ok
    }

    fn emit_type_alternative_clause(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeAlternativeClause,
    ) -> TraversalResult {
        unimplemented!()
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
        unimplemented!()
    }
}
