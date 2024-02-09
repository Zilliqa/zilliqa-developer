use std::mem;

use log::info;
use scilla_parser::{
    ast::{
        converting::AstConverting, nodes::*, visitor::AstVisitor, TraversalResult,
        TreeTraversalMode,
    },
    parser::lexer::SourcePosition,
};

use crate::intermediate_representation::{primitives::*, symbol_table::SymbolTable};

/// Byte Code Generation Process
///
/// The process of generating byte code from Scilla source code involves several steps and transformations.
/// Here is a high-level overview of the process:
///
/// ```plaintext
/// [Scilla source code]
///        |
///        v
/// [Abstract Syntax Tree (AST)]
///        |
///        | (AstConverting)
///        v
/// [Intermediate Representation (IR)]
///        |
///        | (PassManager)
///        v
/// [Optimized Intermediate Representation]
///        |
///        | (EvmBytecodeGenerator)
///        v
/// [EVM Bytecode]
/// ```
///
/// Each arrow (`|        v`) represents a transformation or a step in the process.
/// The name in parentheses (e.g., `(AstConverting)`) is the component or the process that performs the transformation.
///
/// 1. Scilla source code is parsed into an Abstract Syntax Tree (AST).
/// 2. The AST is converted into an Intermediate Representation (IR) using the `AstConverting` trait.
/// 3. The IR is optimized using the `PassManager`.
/// 4. The optimized IR is then converted into EVM bytecode using the `EvmBytecodeGenerator`.
///

/// `StackObject` is an enum representing the different types of objects that can be placed on the stack during the conversion process.
/// It includes EnumValue, IrIdentifier, Instruction, VariableDeclaration, FunctionBody, and FunctionBlock.
#[derive(Debug, Clone)]
enum StackObject {
    /// Represents an EnumValue object on the stack.
    EnumValue(EnumValue),

    /// Represents an IrIdentifier object on the stack.
    IrIdentifier(IrIdentifier),

    /// Represents an Instruction object on the stack.
    Instruction(Box<Instruction>),

    /// Represents a VariableDeclaration object on the stack.
    VariableDeclaration(VariableDeclaration),

    /// Represents a FunctionBody object on the stack.
    FunctionBody(Box<FunctionBody>),

    /// Represents a FunctionBlock object on the stack.
    FunctionBlock(Box<FunctionBlock>),
}

/// The `IrEmitter` struct is used for bookkeeping during the conversion of a Scilla AST to an intermediate representation.
/// It implements the `AstConverting` trait, which is a generic trait for AST conversions.
pub struct IrEmitter {
    /// Stack of objects used during the conversion process.
    stack: Vec<StackObject>,

    /// Current function block being processed.
    current_block: Box<FunctionBlock>,

    /// Current function body being processed.
    current_body: Box<FunctionBody>,

    /// Current namespace being processed.
    current_namespace: IrIdentifier,

    /// Stack of namespaces used during the conversion process.
    namespace_stack: Vec<IrIdentifier>,

    /// Intermediate representation of the AST.
    ir: Box<IntermediateRepresentation>,

    /// Source positions of the AST nodes.
    source_positions: Vec<(SourcePosition, SourcePosition)>,
}

impl IrEmitter {
    pub fn new(symbol_table: SymbolTable) -> Self {
        let current_block = FunctionBlock::new("dummy".to_string());
        let current_body = FunctionBody::new();
        let ns = IrIdentifier {
            unresolved: "".to_string(),
            resolved: None,
            type_reference: None,
            kind: IrIndentifierKind::Namespace,
            is_definition: false,
            source_location: (
                SourcePosition::start_position(),
                SourcePosition::start_position(),
            ),
        };
        // TODO: Repeat similar code for all literals
        IrEmitter {
            stack: Vec::new(),
            current_block,
            current_body,
            current_namespace: ns.clone(),
            namespace_stack: [ns].to_vec(),
            /// current_function: None,
            ir: Box::new(IntermediateRepresentation::new(symbol_table)),
            source_positions: [(
                SourcePosition::invalid_position(),
                SourcePosition::invalid_position(),
            )]
            .to_vec(), // TODO: this should not be necessary
        }
    }

    fn current_location(&self) -> (SourcePosition, SourcePosition) {
        self.source_positions
            .last()
            .expect("Unable to determine source location")
            .clone()
    }

    fn push_namespace(&mut self, mut ns: IrIdentifier) {
        // TODO: Update ns to use nested namespaces
        ns.kind = IrIndentifierKind::Namespace;
        self.namespace_stack.push(ns.clone());
        self.current_namespace = ns;
    }

    fn pop_namespace(&mut self) {
        self.namespace_stack.pop();
        if let Some(ns) = self.namespace_stack.last() {
            self.current_namespace = ns.clone();
        } else {
            panic!("Namespace stack is empty.");
        }
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
                    self.ir.symbol_table.name_generator.new_intermediate()
                };
                instruction.ssa_name = Some(symbol.clone());
                self.current_block.instructions.push_back(instruction);
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

    fn pop_ir_identifier(&mut self) -> Result<IrIdentifier, String> {
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

    pub fn emit(&mut self, node: &NodeProgram) -> Result<Box<IntermediateRepresentation>, String> {
        // Copying original symbol table to create a new instance of the IR at the end
        // of traversing
        let symbol_table = self.ir.symbol_table.clone();

        let result = node.visit(self);
        match result {
            Err(m) => panic!("{}", m),
            _ => (),
        }

        // Creating type table

        // Annotating symbols with types

        // Returning
        let mut ret = Box::new(IntermediateRepresentation::new(symbol_table));
        mem::swap(&mut self.ir, &mut ret);

        Ok(ret)
    }
}

impl AstConverting for IrEmitter {
    fn push_source_position(&mut self, start: &SourcePosition, end: &SourcePosition) {
        self.source_positions.push((start.clone(), end.clone()));
    }

    fn pop_source_position(&mut self) {
        self.source_positions.pop();
    }

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
                    let symbol = IrIdentifier::new(
                        name.to_string(),
                        IrIndentifierKind::Unknown,
                        self.current_location(),
                    );

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
        Ok(TraversalResult::Continue)
    }
    fn emit_variable_identifier(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodeVariableIdentifier,
    ) -> Result<TraversalResult, String> {
        match node {
            NodeVariableIdentifier::VariableName(name) => {
                let operation = Operation::ResolveSymbol {
                    symbol: IrIdentifier::new(
                        name.to_string(),
                        IrIndentifierKind::VirtualRegister,
                        self.current_location(),
                    ),
                };
                let instr = Box::new(Instruction {
                    ssa_name: None,
                    result_type: None,
                    operation,
                    source_location: self.current_location(),
                });
                self.stack.push(StackObject::Instruction(instr));
            }
            NodeVariableIdentifier::SpecialIdentifier(identifier) => {
                let operation = Operation::ResolveContextResource {
                    symbol: IrIdentifier::new(
                        identifier.to_string(),
                        IrIndentifierKind::ContextResource,
                        self.current_location(),
                    ),
                };
                let instr = Box::new(Instruction {
                    ssa_name: None,
                    result_type: None,
                    operation,
                    source_location: self.current_location(),
                });
                self.stack.push(StackObject::Instruction(instr));
            }
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
                expression,
                type_annotation: _,
                containing_expression,
            } => {
                expression.visit(self)?;
                containing_expression.visit(self)?;
                unimplemented!();
            }
            NodeFullExpression::FunctionDeclaration {
                identier_value: _, // TODO: Missing spelling - global replacement
                type_annotation,
                expression,
            } => {
                // identier_value.visit(self)?;
                type_annotation.visit(self)?;
                expression.visit(self)?;

                unimplemented!();
            }
            NodeFullExpression::FunctionCall {
                function_name: _,
                argument_list: _,
            } => {
                unimplemented!();
            }
            NodeFullExpression::ExpressionAtomic(expr) => match &(**expr).node {
                NodeAtomicExpression::AtomicSid(identifier) => {
                    let _ = identifier.visit(self)?;
                }
                NodeAtomicExpression::AtomicLit(literal) => {
                    let _ = literal.visit(self)?;
                }
            },
            NodeFullExpression::ExpressionBuiltin { b, targs, xs } => {
                if let Some(_targs) = targs {
                    unimplemented!();
                }

                let mut arguments: Vec<IrIdentifier> = [].to_vec();
                for arg in xs.node.arguments.iter() {
                    // TODO: xs should be rename .... not clear what this is, but it means function arguments
                    let _ = arg.visit(self)?;
                    let instruction = self.pop_instruction()?;

                    let symbol = self.convert_instruction_to_symbol(instruction);
                    arguments.push(symbol);
                }

                let name = IrIdentifier {
                    unresolved: format!("builtin__{}", b).to_string(), // TODO: Use name generator
                    resolved: None,
                    type_reference: None,
                    kind: IrIndentifierKind::TemplateFunctionName,
                    is_definition: false,
                    source_location: self.current_location(),
                };

                let operation = Operation::CallExternalFunction { name, arguments };

                let instr = Box::new(Instruction {
                    ssa_name: None,
                    result_type: None,
                    operation,
                    source_location: self.current_location(),
                });

                self.stack.push(StackObject::Instruction(instr));
            }
            NodeFullExpression::Message(_entries) => {
                unimplemented!();
            }
            NodeFullExpression::Match {
                match_expression: _,
                clauses: _,
            } => {
                unimplemented!();
            } /* TODO: {

            info!("Match statement");
            let _ = match_expression.visit(self)?;
            let expression = self.pop_instruction()?;
            let source_location = expression.source_location.clone();

            let main_expression_symbol = self.convert_instruction_to_symbol(expression);

            let finally_exit_label = self
            .ir
            .symbol_table
            .name_generator
            .new_block_label("match_finally");

            let mut phi_results: Vec<IrIdentifier> = Vec::new();

            for clause in clauses.iter() {
            info!("Next clause");
            let fail_label = self
            .ir
            .symbol_table
            .name_generator
            .new_block_label("match_fail");
            todo!("Catch all is untested."); //

            match &clause.node.pattern.node {
            NodePattern::Wildcard => {
            info!("Dealing with wildcard");
            // Doing nothing as we will just write the instructions to the current block
            }
            NodePattern::Binder(_) => {
            unimplemented!()
            }
            NodePattern::Constructor(name, args) => {
            info!("Setting {} up", name);
            clause.node.pattern.visit(self)?;

            // Creating compare instruction
            // TODO: Pop instruction or symbol
            let expected_value = self.pop_ir_identifier()?;
            assert!(expected_value.kind == IrIndentifierKind::Unknown);

            let source_location = expected_value.source_location.clone();

            let compare_instr = Box::new(Instruction {
            ssa_name: None,
            result_type: None,
            operation: Operation::IsEqual {
            left: main_expression_symbol.clone(),
            right: expected_value,
            },
            source_location: source_location.clone(),
            });
            let case = self.convert_instruction_to_symbol(compare_instr);

            // Blocks for success

            let success_label = self
            .ir
            .symbol_table
            .name_generator
            .new_block_label("match_success");
            let mut success_block =
            FunctionBlock::new_from_symbol(success_label.clone());

            // Terminating current block
            let op = Operation::ConditionalJump {
            expression: case,
            on_success: success_label,
            on_failure: fail_label.clone(),
            };
            self.current_block
            .instructions
            .push_back(Box::new(Instruction {
            ssa_name: None,
            result_type: None,
            operation: op,
            source_location,
            }));

            // Finishing current_block and moving it onto
            // to the current body while preparing the success block
            // as current
            mem::swap(&mut success_block, &mut self.current_block);
            self.current_body.blocks.push(success_block);
            }
            }

            let _ = clause.node.expression.visit(self)?;
            let expr_instr = self.pop_instruction()?;
            let source_location = expr_instr.source_location.clone();

            let result_sym = self.convert_instruction_to_symbol(expr_instr);
            phi_results.push(result_sym);

            let exit_instruction = Box::new(Instruction {
            ssa_name: None,
            result_type: None,
            operation: Operation::Jump(finally_exit_label.clone()),
            source_location,
            });
            self.current_block.instructions.push_back(exit_instruction);
            self.current_block.terminated = true;
            // Pushing sucess block and creating fail block

            let mut fail_block = FunctionBlock::new_from_symbol(fail_label.clone());
            mem::swap(&mut fail_block, &mut self.current_block);
            self.current_body.blocks.push(fail_block);

            // let fail_label = self.ir.symbol_table.name_generator.new_block_label("match_case");
            // let fail_block = FunctionBlock::new_from_symbol(fail_label);
            }

            // TODO: Catch all if needed

            // Exiting
            let exit_instruction = Box::new(Instruction {
            ssa_name: None,
            result_type: None,
            operation: Operation::Jump(finally_exit_label.clone()),
            source_location: source_location.clone(),
            });
            self.current_block.instructions.push_back(exit_instruction);

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
            source_location: source_location.clone(),
            })));
            // unimplemented!();
            }
             */
            NodeFullExpression::ConstructorCall {
                identifier_name,
                contract_type_arguments,
                argument_list,
            } => {
                self.push_source_position(&identifier_name.start, &identifier_name.end);

                let _ = identifier_name.visit(self)?;

                // Expecting function name symbol
                let mut name = self.pop_ir_identifier()?;
                assert!(name.kind == IrIndentifierKind::Unknown);
                name.kind = IrIndentifierKind::FunctionName;

                let arguments: Vec<IrIdentifier> = [].to_vec();

                if let Some(_test) = contract_type_arguments {
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
                    source_location: self.current_location(),
                });
                self.pop_source_position();
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
                let mut typename = self.pop_ir_identifier()?;
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
                    source_location: self.current_location(),
                });
                self.stack.push(StackObject::Instruction(instr));
            }
            NodeValueLiteral::LiteralHex(value) => {
                let typename = self.ir.symbol_table.name_generator.hex_type();
                let operation = Operation::Literal {
                    data: value.to_string(),
                    typename,
                };
                let instr = Box::new(Instruction {
                    ssa_name: None,
                    result_type: None,
                    operation,
                    source_location: self.current_location(),
                });
                self.stack.push(StackObject::Instruction(instr));
            }
            NodeValueLiteral::LiteralString(value) => {
                let typename = self.ir.symbol_table.name_generator.string_type();
                let operation = Operation::Literal {
                    data: value.to_string(),
                    typename,
                };
                let instr = Box::new(Instruction {
                    ssa_name: None,
                    result_type: None,
                    operation,
                    source_location: self.current_location(),
                });
                self.stack.push(StackObject::Instruction(instr));
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
        node: &NodePattern,
    ) -> Result<TraversalResult, String> {
        match &node {
            NodePattern::Wildcard => {
                info!("Visiting wildcard!");
                // Wild card does not change anything
            }
            NodePattern::Binder(_name) => {
                unimplemented!()
            }
            NodePattern::Constructor(name, args) => {
                if args.len() > 0 {
                    unimplemented!();
                }

                let _ = name.visit(self);
            }
        }

        Ok(TraversalResult::SkipChildren)
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
                left_hand_side,
                right_hand_side,
            } => {
                let symbol = IrIdentifier {
                    unresolved: left_hand_side.to_string(),
                    resolved: None,
                    type_reference: None,
                    kind: IrIndentifierKind::VirtualRegister,
                    is_definition: true,
                    source_location: self.current_location(),
                };

                let right_hand_side = match &right_hand_side.node {
                    NodeVariableIdentifier::VariableName(name) => name,
                    _ => panic!("Load of state {:#?}", right_hand_side.node),
                };

                let ret = Box::new(Instruction {
                    ssa_name: Some(symbol),
                    result_type: None,
                    operation: Operation::StateLoad {
                        address: FieldAddress {
                            name: IrIdentifier {
                                unresolved: right_hand_side.to_string(),
                                resolved: None,
                                type_reference: None,
                                kind: IrIndentifierKind::State,
                                is_definition: false,
                                source_location: self.current_location(),
                            },
                            value: None,
                        },
                    },
                    source_location: self.current_location(),
                });

                Some(ret)
            }
            NodeStatement::RemoteFetch(_remote_stmt) => {
                unimplemented!()
            }
            NodeStatement::Store {
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
                    kind: IrIndentifierKind::VirtualRegister,
                    is_definition: false,
                    source_location: self.current_location(),
                };
                (*right_hand_side).ssa_name = Some(symbol.clone());
                self.current_block.instructions.push_back(right_hand_side);

                let ret = Box::new(Instruction {
                    ssa_name: None,
                    result_type: None,
                    operation: Operation::StateStore {
                        address: FieldAddress {
                            name: IrIdentifier {
                                unresolved: left_hand_side.to_string(),
                                resolved: None,
                                type_reference: None,
                                kind: IrIndentifierKind::State,
                                is_definition: false,
                                source_location: self.current_location(),
                            },
                            value: None,
                        },
                        value: symbol,
                    },
                    source_location: self.current_location(),
                });

                Some(ret)
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
                    kind: IrIndentifierKind::VirtualRegister,
                    is_definition: false,
                    source_location: self.current_location(),
                };
                (*right_hand_side).ssa_name = Some(symbol);

                Some(right_hand_side)
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
            NodeStatement::Accept => {
                let arguments: Vec<IrIdentifier> = [].to_vec();
                let name = IrIdentifier {
                    unresolved: "__intrinsic_accept_transfer".to_string(), // TODO: Register somewhere globally
                    resolved: None,
                    type_reference: None,
                    kind: IrIndentifierKind::ProcedureName,
                    is_definition: false,
                    source_location: self.current_location(),
                };

                let operation = Operation::CallFunction { name, arguments };
                // TODO: Location from component_id
                let instr = Box::new(Instruction {
                    ssa_name: None,
                    result_type: None,
                    operation,
                    source_location: self.current_location(),
                });

                Some(instr)
            }
            NodeStatement::Send { identifier_name: _ } => {
                unimplemented!()
            }
            NodeStatement::CreateEvnt { identifier_name: _ } => {
                unimplemented!()
            }
            NodeStatement::Throw { error_variable: _ } => {
                unimplemented!()
            }
            NodeStatement::MatchStmt { variable, clauses } => {
                let _ = variable.visit(self)?;
                let expression = self.pop_instruction()?;
                let source_location = expression.source_location.clone();
                let main_expression_symbol = self.convert_instruction_to_symbol(expression);

                let match_exit = self
                    .ir
                    .symbol_table
                    .name_generator
                    .new_block_label("match_exit");

                // Termingating current block with placeholder label
                let jump = Box::new(Instruction {
                    ssa_name: None,
                    result_type: None,
                    operation: Operation::Jump(match_exit.clone()),
                    source_location: source_location.clone(),
                });
                self.current_block.instructions.push_back(jump);

                for (i, clause) in clauses.iter().enumerate() {
                    // Terminating previous block
                    let label_condition = self
                        .ir
                        .symbol_table
                        .name_generator
                        .new_block_label(&format!("clause_{}_condition", i));
                    let label_block = self
                        .ir
                        .symbol_table
                        .name_generator
                        .new_block_label(&format!("clause_{}_block", i));

                    let next_jump_label = match &clause.node.pattern_expression.node {
                        NodePattern::Wildcard => label_block.clone(),
                        NodePattern::Binder(_) => {
                            unimplemented!()
                        }
                        NodePattern::Constructor(_, _) => label_condition.clone(),
                    };

                    let last_instruction = &mut self.current_block.instructions.back_mut().unwrap();

                    match &mut last_instruction.operation {
                        Operation::Jump(ref mut value) => {
                            *value = label_condition.clone();
                        }
                        Operation::ConditionalJump {
                            expression: _,
                            on_success: _,
                            ref mut on_failure,
                        } => {
                            *on_failure = next_jump_label;
                        }
                        _ => {
                            panic!("Expected previous block to be a terminating jump.");
                        }
                    }

                    match &clause.node.pattern_expression.node {
                        NodePattern::Wildcard => {
                            // In the event of a wildcard, we jump right to the clause block.
                            // TODO: Check that the wildcard is last block in the match statement.
                        }
                        NodePattern::Binder(_) => {
                            unimplemented!()
                        }
                        NodePattern::Constructor(_, _) => {
                            // Instating condition checking block as self.current_block
                            let mut clause_condition_block =
                                FunctionBlock::new_from_symbol(label_condition);
                            mem::swap(&mut clause_condition_block, &mut self.current_block);
                            self.current_body.blocks.push(clause_condition_block);

                            clause.node.pattern_expression.visit(self)?;
                            let expected_value = self.pop_ir_identifier()?;
                            assert!(expected_value.kind == IrIndentifierKind::Unknown);
                            let source_location = expected_value.source_location.clone();

                            let jump_condition = Box::new(Instruction {
                                ssa_name: None,
                                result_type: None,
                                operation: Operation::IsEqual {
                                    left: main_expression_symbol.clone(),
                                    right: expected_value,
                                },
                                source_location: source_location.clone(),
                            });

                            let jump_if = Box::new(Instruction {
                                ssa_name: None,
                                result_type: None,
                                operation: Operation::ConditionalJump {
                                    expression: self.convert_instruction_to_symbol(jump_condition),
                                    on_success: label_block.clone(),
                                    on_failure: match_exit.clone(), // Exit or Placeholder - will be overwritten in next cycle
                                },
                                source_location: source_location.clone(),
                            });
                            self.current_block.instructions.push_back(jump_if);
                        }
                    };

                    let mut clause_block = match &clause.node.statement_block {
                        Some(statement_block) => {
                            // Condition block
                            statement_block.visit(self)?;
                            self.pop_function_block()?
                        }
                        None => FunctionBlock::new("empty_block".to_string()),
                    };
                    // TODO: Get source location properly
                    let source_location = source_location.clone();
                    clause_block.name = label_block.clone();

                    let terminator_instr = Box::new(Instruction {
                        ssa_name: None,
                        result_type: None,
                        operation: Operation::Jump(match_exit.clone()),
                        source_location,
                    });
                    clause_block.instructions.push_back(terminator_instr);
                    self.current_body.blocks.push(clause_block);
                }

                let mut match_exit_block = FunctionBlock::new_from_symbol(match_exit);
                mem::swap(&mut match_exit_block, &mut self.current_block);

                self.current_body.blocks.push(match_exit_block);
                // self.current_body.blocks.extend(case_blocks);
                None
            }
            NodeStatement::CallProc {
                component_id,
                arguments: call_args,
            } => {
                self.push_source_position(&component_id.start, &component_id.end);

                let mut arguments: Vec<IrIdentifier> = [].to_vec();
                for arg in call_args.iter() {
                    // TODO: xs should be rename .... not clear what this is, but it means function arguments
                    let _ = arg.visit(self)?;
                    let instruction = self.pop_instruction()?;

                    let symbol = self.convert_instruction_to_symbol(instruction);
                    arguments.push(symbol);
                }

                let name = match &component_id.node {
                    NodeComponentId::WithTypeLikeName(_) => unimplemented!(),
                    NodeComponentId::WithRegularId(n) => n,
                };

                let name = IrIdentifier {
                    unresolved: name.to_string(),
                    resolved: None,
                    type_reference: None,
                    kind: IrIndentifierKind::ProcedureName,
                    is_definition: false,
                    source_location: self.current_location(),
                };

                let operation = Operation::CallFunction { name, arguments };
                // TODO: Location from component_id
                let instr = Box::new(Instruction {
                    ssa_name: None,
                    result_type: None,
                    operation,
                    source_location: self.current_location(),
                });

                self.pop_source_position();
                // self.stack.push(StackObject::Instruction(instr));
                Some(instr)
            }
            NodeStatement::Iterate {
                identifier_name: _,
                component_id: _,
            } => {
                unimplemented!()
            }
        };

        match instr {
            Some(instr) => self.current_block.instructions.push_back(instr),
            None => (),
        }
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
                    is_definition: false,
                    source_location: self.current_location(),
                }));
            }
            NodeComponentId::WithTypeLikeName(name) => {
                self.stack.push(StackObject::IrIdentifier(IrIdentifier {
                    unresolved: name.to_string(), // TODO: Travese the tree first and then construct the name
                    resolved: None,
                    type_reference: None,
                    kind: IrIndentifierKind::ComponentName,
                    is_definition: false,
                    source_location: self.current_location(),
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
        _node: &NodeParameterPair,
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
        _node: &NodeStatementBlock,
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

        let mut typename = self.pop_ir_identifier()?;
        assert!(typename.kind == IrIndentifierKind::Unknown);
        typename.kind = IrIndentifierKind::TypeName;

        let s =
            StackObject::VariableDeclaration(VariableDeclaration::new(name.node, false, typename));
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
        _node: &NodeProgram,
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
        _mode: TreeTraversalMode,
        node: &NodeLibraryDefinition,
    ) -> Result<TraversalResult, String> {
        let _ = node.name.visit(self)?;
        let mut ns = self.pop_ir_identifier()?;
        assert!(ns.kind == IrIndentifierKind::Unknown);
        ns.kind = IrIndentifierKind::Namespace;

        self.push_namespace(ns);
        for def in node.definitions.iter() {
            let _ = def.visit(self)?;
        }

        self.pop_namespace();
        Ok(TraversalResult::SkipChildren)
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
                expression,
            } => {
                /*
                let declaration_start = match self.current_function {
                    Some(_) => true,
                    None => false
                };

                if declaration_start {
                    // TODO: self.current_function
                }
                */

                expression.visit(self)?;
                unimplemented!();
            }
            NodeLibrarySingleDefinition::TypeDefinition(name, clauses) => {
                let _ = name.visit(self)?;
                let mut name = self.pop_ir_identifier()?;
                assert!(name.kind == IrIndentifierKind::Unknown);
                name.kind = IrIndentifierKind::TypeName;
                // The name itself is being defined here
                name.is_definition = true;
                let mut user_type = Variant::new();

                if let Some(clauses) = clauses {
                    for clause in clauses.iter() {
                        let _ = clause.visit(self)?;
                        let mut field = self.pop_enum_value()?;

                        // And the field names are being defined as well
                        field.name.is_definition = true;
                        user_type.add_field(field);
                    }
                }

                self.ir.type_definitions.push(ConcreteType::Variant {
                    name,
                    namespace: self.current_namespace.clone(),
                    data_layout: Box::new(user_type),
                });
            }
        }

        Ok(TraversalResult::SkipChildren)
    }

    fn emit_contract_definition(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodeContractDefinition,
    ) -> Result<TraversalResult, String> {
        // TODO: Decide whether the namespace should be distinct
        let _ = node.contract_name.visit(self)?;
        let mut ns = self.pop_ir_identifier()?;
        assert!(ns.kind == IrIndentifierKind::Unknown);
        ns.kind = IrIndentifierKind::Namespace;

        self.push_namespace(ns);

        let _ = node.parameters.visit(self)?;

        if let Some(constraint) = &node.constraint {
            let _ = constraint.visit(self)?;
        }

        for field in node.fields.iter() {
            let _ = field.visit(self)?;
        }

        for component in node.components.iter() {
            let _ = component.visit(self)?;
        }

        self.pop_namespace();
        Ok(TraversalResult::SkipChildren)
    }

    fn emit_contract_field(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodeContractField,
    ) -> Result<TraversalResult, String> {
        let _ = node.typed_identifier.visit(self)?;

        let mut variable = self.pop_variable_declaration()?;
        let _ = node.right_hand_side.visit(self)?;
        let initializer = self.pop_instruction()?;
        variable.name.kind = IrIndentifierKind::State;

        let field = ContractField {
            namespace: self.current_namespace.clone(),
            variable,
            initializer,
        };

        self.ir.fields_definitions.push(field);

        Ok(TraversalResult::SkipChildren)
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
        Ok(TraversalResult::Continue)
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
        for arg in node.parameters.node.parameters.iter() {
            let _ = arg.visit(self)?;
            let ir_arg = self.pop_variable_declaration()?;
            arguments.push(ir_arg);
        }

        // Function body
        let _ = node.body.visit(self)?;

        // Exit
        let mut body = self.pop_function_body()?;

        if let Some(ref mut last_block) = &mut body.blocks.last_mut() {
            if !last_block.terminated {
                // let last_block = last_block.clone();
                // Terminates the block with a void return in the event it is not terminated.
                last_block.instructions.push_back(Box::new(Instruction {
                    ssa_name: None,
                    result_type: None,
                    operation: Operation::Return(None),
                    source_location: self.current_location(),
                }));
                last_block.terminated = true;
            }
        }

        let mut function_name = self.pop_ir_identifier()?;
        assert!(function_name.kind == IrIndentifierKind::ComponentName);
        function_name.kind = IrIndentifierKind::TransitionName;
        function_name.is_definition = true;

        // TODO: Decude return type from body

        let function = ConcreteFunction {
            name: function_name,
            namespace: self.current_namespace.clone(),
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
                let mut enum_name = self.pop_ir_identifier()?;
                assert!(enum_name.kind == IrIndentifierKind::Unknown);
                enum_name.kind = IrIndentifierKind::StaticFunctionName;
                self.stack
                    .push(StackObject::EnumValue(EnumValue::new(enum_name, None)));
            }
            NodeTypeAlternativeClause::ClauseTypeWithArgs(identifier, children) => {
                let _ = identifier.visit(self)?;
                let mut member_name = self.pop_ir_identifier()?;
                assert!(member_name.kind == IrIndentifierKind::Unknown);
                member_name.kind = IrIndentifierKind::StaticFunctionName;

                let mut tuple = Tuple::new();
                for child in children.iter() {
                    let _ = child.visit(self)?;

                    let mut item = self.pop_ir_identifier()?;
                    assert!(item.kind == IrIndentifierKind::Unknown);
                    item.kind = IrIndentifierKind::TypeName;

                    tuple.add_field(item)
                }

                let refid = self
                    .ir
                    .symbol_table
                    .name_generator
                    .generate_anonymous_type_id("Tuple".to_string());

                self.ir.type_definitions.push(ConcreteType::Tuple {
                    name: refid.clone(),
                    namespace: self.current_namespace.clone(),
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
