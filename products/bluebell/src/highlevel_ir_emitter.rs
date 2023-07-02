use crate::ast::*;
use crate::ast_converting::AstConverting;
use crate::ast_visitor::AstVisitor;
use crate::constants::{TraversalResult, TreeTraversalMode};
use crate::highlevel_ir::*;
use std::mem;

#[derive(Debug, Clone)]
enum StackObject {
    EnumValue(EnumValue),

    IrIdentifier(IrIdentifier),
    Instruction(Box<Instruction>),

    VariableDeclaration(VariableDeclaration),
    FunctionBody(Box<FunctionBody>),
    FunctionBlock(Box<FunctionBlock>),
}

pub struct HighlevelIrEmitter {
    stack: Vec<StackObject>,

    anonymous_type_number: u64,
    intermediate_counter: u64,
    block_counter: u64,

    ///
    current_block: Box<FunctionBlock>,
    current_body: Box<FunctionBody>,

    ir: Box<HighlevelIr>,
}

impl HighlevelIrEmitter {
    pub fn new() -> Self {
        let current_block = FunctionBlock::new("dummy".to_string());
        let current_body = FunctionBody::new();
        // TODO: Repeat similar code for all literals
        HighlevelIrEmitter {
            stack: Vec::new(),
            anonymous_type_number: 0,
            intermediate_counter: 0,
            block_counter: 0,
            current_block,
            current_body,
            ir: Box::new(HighlevelIr::new()),
        }
    }

    fn new_intermediate(&mut self) -> IrIdentifier {
        let n = self.intermediate_counter;
        self.intermediate_counter += 1;
        IrIdentifier {
            unresolved: format!("__imm_{}", n),
            resolved: None,
            type_reference: None,
            kind: IrIndentifierKind::VirtualRegisterIntermediate,
        }
    }

    fn new_block_label(&mut self, prefix: &str) -> IrIdentifier {
        let n = self.block_counter;
        self.block_counter += 1;
        let label = format!("{}_{}", prefix, n);
        FunctionBlock::new_label(label)
    }

    fn convert_instruction_to_symbol(&mut self, mut instruction: Box<Instruction>) -> IrIdentifier {
        // Optimisation: If previous instruction was "ResolveSymbol",
        // we avoid creating an intermediate
        let symbol = match instruction.operation {
            Operation::ResolveSymbol { symbol } => symbol,
            _ => {
                let symbol = if let Some(s) = instruction.ssa_name {
                    s
                } else {
                    self.new_intermediate()
                };
                instruction.ssa_name = Some(symbol.clone());
                self.current_block.instructions.push(instruction);
                symbol
            }
        };

        symbol
    }

    fn pop_function_block(&mut self) -> Result<Box<FunctionBlock>, String> {
        let ret = if let Some(candidate) = self.stack.pop() {
            match candidate {
                StackObject::FunctionBlock(n) => n,
                _ => {
                    return Err(format!(
                        "Expected function block, but found {:?}.",
                        candidate
                    ));
                }
            }
        } else {
            return Err("Expected function block, but found nothing.".to_string());
        };

        Ok(ret)
    }

    fn pop_symbol_name(&mut self) -> Result<IrIdentifier, String> {
        let ret = if let Some(candidate) = self.stack.pop() {
            match candidate {
                StackObject::IrIdentifier(n) => n,
                _ => {
                    return Err(format!("Expected symbol name, but found {:?}.", candidate));
                }
            }
        } else {
            return Err("Expected symbol name, but found nothing.".to_string());
        };

        Ok(ret)
    }

    fn pop_instruction(&mut self) -> Result<Box<Instruction>, String> {
        let ret = if let Some(candidate) = self.stack.pop() {
            match candidate {
                StackObject::Instruction(n) => n,
                _ => {
                    return Err(format!("Expected instruction, but found {:?}.", candidate));
                }
            }
        } else {
            return Err("Expected instruction, but found nothing.".to_string());
        };

        Ok(ret)
    }

    fn pop_enum_value(&mut self) -> Result<EnumValue, String> {
        let ret = if let Some(candidate) = self.stack.pop() {
            match candidate {
                StackObject::EnumValue(n) => n,
                _ => {
                    return Err(format!("Expected enum value, but found {:?}.", candidate));
                }
            }
        } else {
            return Err("Expected enum value, but found nothing.".to_string());
        };

        Ok(ret)
    }

    fn pop_variable_declaration(&mut self) -> Result<VariableDeclaration, String> {
        let ret = if let Some(candidate) = self.stack.pop() {
            match candidate {
                StackObject::VariableDeclaration(n) => n,
                _ => {
                    return Err(format!(
                        "Expected variable declaration, but found {:?}.",
                        candidate
                    ));
                }
            }
        } else {
            return Err("Expected variable declaration, but found nothing.".to_string());
        };

        Ok(ret)
    }

    fn pop_function_body(&mut self) -> Result<Box<FunctionBody>, String> {
        let ret = if let Some(candidate) = self.stack.pop() {
            match candidate {
                StackObject::FunctionBody(n) => n,
                _ => {
                    return Err(format!(
                        "Expected function body, but found {:?}.",
                        candidate
                    ));
                }
            }
        } else {
            return Err("Expected function body, but found nothing.".to_string());
        };

        Ok(ret)
    }

    fn generate_anonymous_type_id(&mut self, prefix: String) -> IrIdentifier {
        let n = self.anonymous_type_number;
        self.anonymous_type_number += 1;

        IrIdentifier {
            unresolved: format!("{}{}", prefix, n).to_string(),
            resolved: None,
            type_reference: None,
            kind: IrIndentifierKind::TypeName,
        }
    }

    pub fn emit(&mut self, node: &mut NodeProgram) -> Result<Box<HighlevelIr>, String> {
        let result = node.visit(self);
        match result {
            Err(m) => panic!("{}", m),
            _ => (),
        }

        // Creating type table

        // Annotating symbols with types

        // Returning
        let mut ret = Box::new(HighlevelIr::new());
        mem::swap(&mut self.ir, &mut ret);

        Ok(ret)
    }
}

impl AstConverting for HighlevelIrEmitter {
    fn emit_byte_str(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeByteStr,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
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
                    /*
                    self.stack.push(StackObject::Identifier(Identifier::Event(
                        "Event".to_string(),
                    )));
                    */
                    unimplemented!()
                }
                NodeTypeNameIdentifier::TypeOrEnumLikeIdentifier(name) => {
                    let symbol = IrIdentifier::new(name.to_string(), IrIndentifierKind::Unknown);

                    self.stack.push(StackObject::IrIdentifier(symbol));
                }
            },
            TreeTraversalMode::Exit => (),
        }
        Ok(TraversalResult::Continue)
    }
    fn emit_imported_name(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeImportedName,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_import_declarations(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeImportDeclarations,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_meta_identifier(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeMetaIdentifier,
    ) -> Result<TraversalResult, String> {
        // TODO:
        Ok(TraversalResult::Continue)
        //        unimplemented!();
    }
    fn emit_variable_identifier(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodeVariableIdentifier,
    ) -> Result<TraversalResult, String> {
        match node {
            NodeVariableIdentifier::VariableName(name) => {
                let operation = Operation::ResolveSymbol {
                    symbol: IrIdentifier::new(name.to_string(), IrIndentifierKind::VirtualRegister),
                };
                let instr = Box::new(Instruction {
                    ssa_name: None,
                    result_type: None,
                    operation,
                });
                self.stack.push(StackObject::Instruction(instr));
            }
            NodeVariableIdentifier::SpecialIdentifier(_identifier) => unimplemented!(),
            NodeVariableIdentifier::VariableInNamespace(_namespace, _identifier) => {
                unimplemented!()
            }
        }
        Ok(TraversalResult::SkipChildren)
    }
    fn emit_builtin_arguments(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeBuiltinArguments,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_type_map_key(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeTypeMapKey,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_type_map_value(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeTypeMapValue,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_type_argument(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodeTypeArgument,
    ) -> Result<TraversalResult, String> {
        match node {
            NodeTypeArgument::EnclosedTypeArgument(_) => {
                unimplemented!();
            }
            NodeTypeArgument::GenericTypeArgument(n) => {
                let _ = n.visit(self)?;
            }
            NodeTypeArgument::TemplateTypeArgument(_) => {
                unimplemented!();
            }
            NodeTypeArgument::AddressTypeArgument(_) => {
                unimplemented!();
            }
            NodeTypeArgument::MapTypeArgument(_, _) => {
                unimplemented!();
            }
        }
        Ok(TraversalResult::SkipChildren)
    }
    fn emit_scilla_type(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodeScillaType,
    ) -> Result<TraversalResult, String> {
        match node {
            NodeScillaType::GenericTypeWithArgs(lead, args) => {
                let _ = lead.visit(self)?;
                if args.len() > 0 {
                    // TODO: Deal with arguments
                    unimplemented!()
                }
            }
            NodeScillaType::MapType(key, value) => {
                let _ = key.visit(self)?;
                let _ = value.visit(self)?;
                // TODO: Pop the two and create type Map<X,Y>
                unimplemented!()
            }
            NodeScillaType::FunctionType(a, b) => {
                let _ = (*a).visit(self)?;
                let _ = (*b).visit(self)?;
                // TODO: Implement the function type
                unimplemented!()
            }

            NodeScillaType::PolyFunctionType(_name, a) => {
                // TODO: What to do with name
                let _ = (*a).visit(self)?;
                unimplemented!()
            }
            NodeScillaType::EnclosedType(a) => {
                let _ = (*a).visit(self)?;
            }
            NodeScillaType::ScillaAddresseType(a) => {
                let _ = (*a).visit(self)?;
            }
            NodeScillaType::TypeVarType(_name) => {
                /*
                self.stack
                    .push(StackObject::Identifier(Identifier::TypeName(
                        name.to_string(),
                    )));
                    */
                unimplemented!()
            }
        };
        Ok(TraversalResult::SkipChildren)
    }

    fn emit_type_map_entry(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeTypeMapEntry,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_address_type_field(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeAddressTypeField,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_address_type(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeAddressType,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }

    fn emit_full_expression(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodeFullExpression,
    ) -> Result<TraversalResult, String> {
        match node {
            NodeFullExpression::LocalVariableDeclaration {
                identifier_name: _,
                expression: _,
                type_annotation: _,
                containing_expression: _,
            } => {
                unimplemented!();
            }
            NodeFullExpression::FunctionDeclaration {
                identier_value: _, // TODO: Missing spelling - global replacement
                type_annotation: _,
                expression: _,
            } => {
                unimplemented!();
            }
            NodeFullExpression::FunctionCall {
                function_name: _,
                argument_list: _,
            } => {
                unimplemented!();
            }
            NodeFullExpression::ExpressionAtomic(expr) => match &**expr {
                NodeAtomicExpression::AtomicSid(identifier) => {
                    let _ = identifier.visit(self)?;
                }
                NodeAtomicExpression::AtomicLit(literal) => {
                    let _ = literal.visit(self)?;
                }
            },
            NodeFullExpression::ExpressionBuiltin { b, targs, xs } => {
                if let Some(targs) = targs {
                    unimplemented!();
                }

                let mut arguments: Vec<IrIdentifier> = [].to_vec();

                for arg in xs.arguments.iter() {
                    // TODO: xs should be rename .... not clear what this is, but it means function arguments
                    let _ = arg.visit(self)?;
                    let instruction = self.pop_instruction()?;

                    let symbol = self.convert_instruction_to_symbol(instruction);

                    arguments.push(symbol);
                }

                let name = IrIdentifier {
                    unresolved: b.to_string(),
                    resolved: None,
                    type_reference: None,
                    kind: IrIndentifierKind::ExternalFunctionName,
                };

                let operation = Operation::CallExternalFunction { name, arguments };

                let instr = Box::new(Instruction {
                    ssa_name: None,
                    result_type: None,
                    operation,
                });

                self.stack.push(StackObject::Instruction(instr));
            }
            NodeFullExpression::Message(_entries) => {
                unimplemented!();
            }
            NodeFullExpression::Match {
                match_expression,
                clauses,
            } => {
                let _ = match_expression.visit(self)?;
                let expression = self.pop_instruction()?;

                let main_expression_symbol = self.convert_instruction_to_symbol(expression);

                let finally_exit_label = self.new_block_label("match_finally");

                // Checking for catch all
                let mut catch_all: Option<&NodePatternMatchExpressionClause> = None;
                for clause in clauses.iter() {
                    match clause.pattern {
                        NodePattern::Wildcard => {
                            catch_all = Some(clause);
                            break;
                        }
                        _ => {}
                    }
                }

                let mut phi_results: Vec<IrIdentifier> = Vec::new();

                for clause in clauses.iter() {
                    match &clause.pattern {
                        NodePattern::Wildcard => continue,
                        NodePattern::Binder(_name) => {
                            unimplemented!()
                        }
                        NodePattern::Constructor(name, args) => {
                            if args.len() > 0 {
                                println!("Name: {:#?}", name);
                                println!("Args: {:#?}", args);

                                unimplemented!();
                            }

                            let _ = name.visit(self);
                        }
                    }

                    // Creating compare instruction
                    // TODO: Pop instruction or symbol
                    let expected_value = self.pop_symbol_name()?;
                    let compare_instr = Box::new(Instruction {
                        ssa_name: None,
                        result_type: None,
                        operation: Operation::IsEqual {
                            left: main_expression_symbol.clone(),
                            right: expected_value,
                        },
                    });
                    let case = self.convert_instruction_to_symbol(compare_instr);

                    // Blocks for success and fail
                    let fail_label = self.new_block_label("match_fail");

                    let success_label = self.new_block_label("match_success");
                    let mut success_block = FunctionBlock::new_from_symbol(success_label.clone());

                    // Terminating current block
                    let op = Operation::ConditionalJump {
                        expression: case,
                        on_success: success_label,
                        on_failure: fail_label.clone(),
                    };
                    self.current_block.instructions.push(Box::new(Instruction {
                        ssa_name: None,
                        result_type: None,
                        operation: op,
                    }));
                    self.current_block.terminated = true;

                    // Finishing current_block and moving it onto
                    // to the current body while preparing the success block
                    // as current
                    mem::swap(&mut success_block, &mut self.current_block);
                    self.current_body.blocks.push(success_block);

                    let _ = clause.expression.visit(self)?;
                    let expr_instr = self.pop_instruction()?;
                    let result_sym = self.convert_instruction_to_symbol(expr_instr);
                    phi_results.push(result_sym);

                    let exit_instruction = Box::new(Instruction {
                        ssa_name: None,
                        result_type: None,
                        operation: Operation::Jump(finally_exit_label.clone()),
                    });
                    self.current_block.instructions.push(exit_instruction);

                    // Pushing sucess block and creating fail block

                    let mut fail_block = FunctionBlock::new_from_symbol(fail_label.clone());
                    mem::swap(&mut fail_block, &mut self.current_block);
                    self.current_body.blocks.push(fail_block);

                    // let fail_label = self.new_block_label("match_case");
                    // let fail_block = FunctionBlock::new_from_symbol(fail_label);
                }

                // Currently in the last fail block
                // TODO: Catch all if needed

                let exit_instruction = Box::new(Instruction {
                    ssa_name: None,
                    result_type: None,
                    operation: Operation::Jump(finally_exit_label.clone()),
                });
                self.current_block.instructions.push(exit_instruction);

                // Attaching exit block
                let mut finally_exit_block =
                    FunctionBlock::new_from_symbol(finally_exit_label.clone());
                mem::swap(&mut finally_exit_block, &mut self.current_block);
                self.current_body.blocks.push(finally_exit_block);

                self.stack
                    .push(StackObject::Instruction(Box::new(Instruction {
                        ssa_name: None,
                        result_type: None,
                        operation: Operation::PhiNode(phi_results),
                    })));
                // unimplemented!();
            }
            NodeFullExpression::ConstructorCall {
                identifier_name,
                contract_type_arguments,
                argument_list,
            } => {
                let _ = identifier_name.visit(self)?;

                // Expecting function name symbol
                let mut name = self.pop_symbol_name()?;
                assert!(name.kind == IrIndentifierKind::Unknown);
                name.kind = IrIndentifierKind::FunctionName;

                let arguments: Vec<IrIdentifier> = [].to_vec();

                if let Some(test) = contract_type_arguments {
                    unimplemented!()
                }
                if argument_list.len() > 0 {
                    unimplemented!()
                }

                let operation = Operation::CallStaticFunction {
                    name,
                    owner: None, // We cannot deduce the type from the AST
                    arguments,
                };
                let instr = Box::new(Instruction {
                    ssa_name: None,
                    result_type: None,
                    operation,
                });

                self.stack.push(StackObject::Instruction(instr));
            }
            NodeFullExpression::TemplateFunction {
                identifier_name: _,
                expression: _,
            } => {
                unimplemented!();
            }
            NodeFullExpression::TApp {
                identifier_name: _,
                type_arguments: _,
            } => {
                unimplemented!();
            }
        }
        Ok(TraversalResult::SkipChildren)
    }

    fn emit_message_entry(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeMessageEntry,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_pattern_match_expression_clause(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodePatternMatchExpressionClause,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_atomic_expression(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeAtomicExpression,
    ) -> Result<TraversalResult, String> {
        // TODO:
        Ok(TraversalResult::Continue)
        //        unimplemented!();
    }
    fn emit_contract_type_arguments(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeContractTypeArguments,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_value_literal(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodeValueLiteral,
    ) -> Result<TraversalResult, String> {
        match node {
            NodeValueLiteral::LiteralInt(typename, value) => {
                let _ = typename.visit(self)?;
                let mut typename = self.pop_symbol_name()?;
                assert!(typename.kind == IrIndentifierKind::Unknown);
                typename.kind = IrIndentifierKind::TypeName;
                let operation = Operation::Literal {
                    data: value.to_string(),
                    typename,
                };
                let instr = Box::new(Instruction {
                    ssa_name: None,
                    result_type: None,
                    operation,
                });
                self.stack.push(StackObject::Instruction(instr));
            }
            NodeValueLiteral::LiteralHex(_value) => {
                unimplemented!();
            }
            NodeValueLiteral::LiteralString(_value) => {
                unimplemented!();
            }
            NodeValueLiteral::LiteralEmptyMap(_key, _value) => {
                unimplemented!();
            }
        }
        Ok(TraversalResult::SkipChildren)
    }
    fn emit_map_access(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeMapAccess,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_pattern(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodePattern,
    ) -> Result<TraversalResult, String> {
        /*
        match node {

        }
        */
        unimplemented!();
    }
    fn emit_argument_pattern(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeArgumentPattern,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_pattern_match_clause(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodePatternMatchClause,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_blockchain_fetch_arguments(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeBlockchainFetchArguments,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }

    fn emit_statement(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodeStatement,
    ) -> Result<TraversalResult, String> {
        let instr = match node {
            NodeStatement::Load {
                left_hand_side: _,
                right_hand_side: _,
            } => {
                unimplemented!()
            }
            NodeStatement::RemoteFetch(_remote_stmt) => {
                unimplemented!()
            }
            NodeStatement::Store {
                left_hand_side: _,
                right_hand_side: _,
            } => {
                unimplemented!()
            }
            NodeStatement::Bind {
                left_hand_side,
                right_hand_side,
            } => {
                // Generating instruction and setting its name
                let _ = right_hand_side.visit(self)?;

                let mut right_hand_side = self.pop_instruction()?;
                let symbol = IrIdentifier {
                    unresolved: left_hand_side.to_string(),
                    resolved: None,
                    type_reference: None,
                    kind: IrIndentifierKind::Unknown,
                };
                (*right_hand_side).ssa_name = Some(symbol);

                right_hand_side
            }
            NodeStatement::ReadFromBC {
                left_hand_side: _,
                type_name: _,
                arguments: _,
            } => {
                unimplemented!()
            }
            NodeStatement::MapGet {
                left_hand_side: _,
                keys: _,
                right_hand_side: _,
            } => {
                unimplemented!()
            }
            NodeStatement::MapGetExists {
                left_hand_side: _,
                keys: _,
                right_hand_side: _,
            } => {
                unimplemented!()
            }
            NodeStatement::MapUpdate {
                left_hand_side: _,
                keys: _,
                right_hand_side: _,
            } => {
                unimplemented!()
            }
            NodeStatement::MapUpdateDelete {
                left_hand_side: _,
                keys: _,
            } => {
                unimplemented!()
            }
            NodeStatement::Accept => Box::new(Instruction {
                ssa_name: None,
                result_type: None,
                operation: Operation::AcceptTransfer,
            }),
            NodeStatement::Send { identifier_name: _ } => {
                unimplemented!()
            }
            NodeStatement::CreateEvnt { identifier_name: _ } => {
                unimplemented!()
            }
            NodeStatement::Throw { error_variable: _ } => {
                unimplemented!()
            }
            NodeStatement::MatchStmt {
                variable: _,
                clauses: _,
            } => {
                unimplemented!()
            }
            NodeStatement::CallProc {
                component_id: _,
                arguments: _,
            } => {
                unimplemented!()
            }
            NodeStatement::Iterate {
                identifier_name: _,
                component_id: _,
            } => {
                unimplemented!()
            }
        };

        self.current_block.instructions.push(instr);
        Ok(TraversalResult::SkipChildren)
    }

    fn emit_remote_fetch_statement(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeRemoteFetchStatement,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_component_id(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodeComponentId,
    ) -> Result<TraversalResult, String> {
        match node {
            NodeComponentId::WithRegularId(name) => {
                self.stack.push(StackObject::IrIdentifier(IrIdentifier {
                    unresolved: name.to_string(),
                    resolved: None,
                    type_reference: None,
                    kind: IrIndentifierKind::ComponentName,
                }));
            }
            NodeComponentId::WithTypeLikeName(name) => {
                self.stack.push(StackObject::IrIdentifier(IrIdentifier {
                    unresolved: name.to_string(), // TODO: Travese the tree first and then construct the name
                    resolved: None,
                    type_reference: None,
                    kind: IrIndentifierKind::ComponentName,
                }));
            }
        }

        Ok(TraversalResult::SkipChildren)
    }

    fn emit_component_parameters(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeComponentParameters,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
        // TODO:        unimplemented!();
    }

    fn emit_parameter_pair(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodeParameterPair,
    ) -> Result<TraversalResult, String> {
        // Delibarate pass through
        Ok(TraversalResult::Continue)
    }

    fn emit_component_body(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodeComponentBody,
    ) -> Result<TraversalResult, String> {
        // Creating a new function body
        let mut new_body = FunctionBody::new();
        mem::swap(&mut new_body, &mut self.current_body);
        self.stack.push(StackObject::FunctionBody(new_body));

        // Visiting blocks
        if let Some(block) = &node.statement_block {
            let _ = block.visit(self)?;
        }

        let last_block = self.pop_function_block()?;
        // Restoring the old body as current
        let mut body = self.pop_function_body()?;
        mem::swap(&mut body, &mut self.current_body);

        // Pushing the current body onto the stack
        (*body).blocks.push(last_block);
        self.stack.push(StackObject::FunctionBody(body));
        Ok(TraversalResult::SkipChildren)
    }

    fn emit_statement_block(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeStatementBlock,
    ) -> Result<TraversalResult, String> {
        match mode {
            TreeTraversalMode::Enter => {
                // self.stack.push( FunctionBlock::new_stack_object("entry".to_string()) );
                let mut new_entry = FunctionBlock::new("entry".to_string());
                mem::swap(&mut new_entry, &mut self.current_block);
                self.stack.push(StackObject::FunctionBlock(new_entry));
            }
            _ => {
                // Restoring the current block and pushing the WiP onto the stack
                let mut body = self.pop_function_block()?;
                mem::swap(&mut body, &mut self.current_block);
                self.stack.push(StackObject::FunctionBlock(body));
            }
        }

        Ok(TraversalResult::Continue)
    }
    fn emit_typed_identifier(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodeTypedIdentifier,
    ) -> Result<TraversalResult, String> {
        let name = node.identifier_name.clone();
        let _ = node.annotation.visit(self)?;

        let mut typename = self.pop_symbol_name()?;
        assert!(typename.kind == IrIndentifierKind::Unknown);
        typename.kind = IrIndentifierKind::TypeName;

        let s = StackObject::VariableDeclaration(VariableDeclaration::new(name, false, typename));
        self.stack.push(s);

        Ok(TraversalResult::SkipChildren)
    }
    fn emit_type_annotation(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeTypeAnnotation,
    ) -> Result<TraversalResult, String> {
        // Pass through
        Ok(TraversalResult::Continue)
        //        unimplemented!();
    }

    fn emit_program(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeProgram,
    ) -> Result<TraversalResult, String> {
        match mode {
            TreeTraversalMode::Enter => {
                /*
                TODO: Move to LLVM emitter
                // Parse the version string to u64
                let version = match node.version.parse::<u64>() {
                    Ok(v) => v,
                    Err(_) => {
                        eprintln!("Failed to parse version");
                        return Err("Scilla version must be an integer".to_string());
                    }
                };
                let node_version_value = self.context.i64_type().const_int(version, false);
                // Add a global constant named `scilla_version` to the module
                let addr_space = inkwell::AddressSpace::from(2u16);
                let scilla_version = self.module.add_global(
                    self.context.i64_type(),
                    Some(addr_space),
                    "scilla_version",
                );
                scilla_version.set_initializer(&node_version_value);
                scilla_version.set_constant(true);
                */
            }
            TreeTraversalMode::Exit => {
                // Not sure on what's to be done during exit
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
                // TODO: Push namespace: node.name
            }
            TreeTraversalMode::Exit => {
                // TODO: Pop namespace
            }
        }
        Ok(TraversalResult::Continue)
    }

    fn emit_library_single_definition(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodeLibrarySingleDefinition,
    ) -> Result<TraversalResult, String> {
        match node {
            NodeLibrarySingleDefinition::LetDefinition {
                variable_name: _,
                type_annotation: _,
                expression: _,
            } => {
                unimplemented!();
            }
            NodeLibrarySingleDefinition::TypeDefinition(name, clauses) => {
                let _ = name.visit(self)?;
                let mut name = self.pop_symbol_name()?;
                assert!(name.kind == IrIndentifierKind::Unknown);
                name.kind = IrIndentifierKind::TypeName;

                let mut user_type = Variant::new();

                if let Some(clauses) = clauses {
                    for clause in clauses.iter() {
                        let _ = clause.visit(self)?;
                        let field = self.pop_enum_value()?;
                        user_type.add_field(field);
                    }
                }

                self.ir.type_definitions.push(ConcreteType::Variant {
                    name,
                    data_layout: Box::new(user_type),
                });
            }
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
                /*
                TODO: Move to LLVM emitter
                let void_type = self.context.void_type();
                let fn_type = void_type.fn_type(&[], false);
                let contract_initiator_name =
                    format!("Initiator_{}", &node.contract_name.to_string());
                // TODO: Add annotations to indidate that this is a contract constructor
                let function =
                    self.module
                        .add_function(&contract_initiator_name.to_string(), fn_type, None);
                let basic_block = self.context.append_basic_block(function, "entry");
                self.builder.position_at_end(basic_block);
                // TODO - you have to implement the contract definition here
                // in the form of IR instructions for LLVM. Also replace the return type or parameters in fn_type
                // if your contract requires different types
                // ...
                */
            }
            _ => {}
        }
        Ok(TraversalResult::Continue)
    }

    fn emit_contract_field(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeContractField,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_with_constraint(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeWithConstraint,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_component_definition(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeComponentDefinition,
    ) -> Result<TraversalResult, String> {
        // TODO:
        Ok(TraversalResult::Continue)
        //        unimplemented!();
    }
    fn emit_procedure_definition(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeProcedureDefinition,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }

    fn emit_transition_definition(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodeTransitionDefinition,
    ) -> Result<TraversalResult, String> {
        // Enter
        let _ = node.name.visit(self)?;

        let mut arguments: Vec<VariableDeclaration> = [].to_vec();
        for arg in node.parameters.parameters.iter() {
            let _ = arg.visit(self)?;
            let ir_arg = self.pop_variable_declaration()?;
            arguments.push(ir_arg);
        }

        // Function body
        let _ = node.body.visit(self)?;

        // Exit
        let mut body = self.pop_function_body()?;
        let mut function_name = self.pop_symbol_name()?;
        assert!(function_name.kind == IrIndentifierKind::ComponentName);
        function_name.kind = IrIndentifierKind::TransitionName;

        // TODO: Decude return type from body

        let function = ConcreteFunction {
            name: function_name,
            function_kind: FunctionKind::Transition,
            return_type: None, // TODO: Pop of the stack
            arguments,
            body,
        };

        self.ir.function_definitions.push(function);

        Ok(TraversalResult::SkipChildren)
    }

    fn emit_type_alternative_clause(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodeTypeAlternativeClause,
    ) -> Result<TraversalResult, String> {
        match node {
            NodeTypeAlternativeClause::ClauseType(identifier) => {
                let _ = identifier.visit(self)?;
                let mut enum_name = self.pop_symbol_name()?;
                assert!(enum_name.kind == IrIndentifierKind::Unknown);
                enum_name.kind = IrIndentifierKind::TypeName;

                self.stack
                    .push(StackObject::EnumValue(EnumValue::new(enum_name, None)));
            }
            NodeTypeAlternativeClause::ClauseTypeWithArgs(identifier, children) => {
                let _ = identifier.visit(self)?;
                let mut member_name = self.pop_symbol_name()?;
                assert!(member_name.kind == IrIndentifierKind::Unknown);
                member_name.kind = IrIndentifierKind::TypeName;

                let mut tuple = Tuple::new();
                for child in children.iter() {
                    let _ = child.visit(self)?;

                    let mut item = self.pop_symbol_name()?;
                    assert!(item.kind == IrIndentifierKind::Unknown);
                    item.kind = IrIndentifierKind::TypeName;

                    tuple.add_field(item)
                }

                let refid = self.generate_anonymous_type_id("Tuple".to_string());

                self.ir.type_definitions.push(ConcreteType::Tuple {
                    name: refid.clone(),
                    data_layout: Box::new(tuple),
                });

                self.stack.push(StackObject::EnumValue(EnumValue::new(
                    member_name,
                    Some(refid),
                )));
            }
        }
        Ok(TraversalResult::SkipChildren)
    }
    fn emit_type_map_value_arguments(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeTypeMapValueArguments,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_type_map_value_allowing_type_arguments(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeTypeMapValueAllowingTypeArguments,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
}
