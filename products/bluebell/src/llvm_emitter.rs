use crate::ast::*;
use crate::code_emitter::{CodeEmitter, TraversalResult, TreeTraversalMode};
use crate::visitor::Visitor;
use inkwell::module::Module;
use inkwell::types::{BasicType, BasicTypeEnum, FunctionType};
use inkwell::values::IntValue;
use inkwell::{
    basic_block::BasicBlock, builder::Builder, context::Context, values::BasicValueEnum,
};

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Variant {
    name: Option<String>,
    fields: Vec<(String, u64, Option<Box<Variant>>)>, // (name, id, data)
}

impl Variant {
    // Constructor method for our struct
    fn new(name: Option<String>) -> Self {
        Self {
            name,
            fields: Vec::new(),
        }
    }
    // Method to determine if the variant is primitive
    fn is_primitive(&self) -> bool {
        for (_, _, data) in self.fields.iter() {
            if let Some(_) = data {
                return false;
            }
        }
        true
    }
    // Method to determine if the variant is anonymous
    fn is_anonymous(&self) -> bool {
        self.name.is_none()
    }
    // Method to add a field into our Variant struct
    fn add_field(&mut self, name: String, value: Option<Box<Variant>>) {
        let id: u64 = match self.fields.last() {
            // if we have at least one field, use the id of the last field + 1
            Some((_, id, _)) => id + 1,
            // else this is the first field, so use 0
            None => 0,
        };
        self.fields.push((name, id, value));
    }
}

#[derive(Debug, Clone)]
enum Identifier {
    ComponentName(String),
    TypeName(String),
    Event(String),
}

enum Literal {
    String(String),
    Bool(bool),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Uint8(u8),
    Uint16(u16),
    Uint32(u32),
    Uint64(u64),
    Float32(f32),
    Float64(f64),
    Char(char),
    Bytes(Vec<u8>),
}

#[derive(Debug, Clone)]
enum StackObject<'ctx> {
    Identifier(Identifier),
    Variant(Variant),
    LlvmValue(BasicValueEnum<'ctx>),
}

struct Scope<'ctx> {
    context: &'ctx Context,
    namespace: String,

    local_values: HashMap<String, BasicValueEnum<'ctx>>,

    builder: Builder<'ctx>,
    basic_block: BasicBlock<'ctx>,
}

impl<'ctx> Scope<'ctx> {
    pub fn new(
        context: &'ctx Context,
        namespace: String,
        builder: Builder<'ctx>,
        basic_block: BasicBlock<'ctx>,
    ) -> Self {
        Self {
            context,
            namespace,

            local_values: HashMap::new(),
            builder,
            basic_block,
        }
    }
}

type TypeAllocatorFunction<'ctx> =
    Box<dyn Fn(&'ctx Context, Vec<StackObject<'ctx>>) -> BasicTypeEnum<'ctx>>;
type ValueAllocatorFunction<'ctx> =
    Box<dyn Fn(&'ctx Context, Vec<StackObject<'ctx>>) -> BasicValueEnum<'ctx>>;

pub struct LlvmEmitter<'ctx> {
    context: &'ctx Context,
    builder: Builder<'ctx>,
    module: Module<'ctx>,

    global_values: HashMap<String, BasicValueEnum<'ctx>>,
    stack: Vec<StackObject<'ctx>>,

    type_allocation_handler: HashMap<String, TypeAllocatorFunction<'ctx>>,
    value_allocation_handler: HashMap<String, ValueAllocatorFunction<'ctx>>,
}

impl<'ctx> LlvmEmitter<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        let builder = context.create_builder();
        let module = context.create_module("main");
        let global_values: HashMap<String, BasicValueEnum<'ctx>> = HashMap::new();
        let stack: Vec<StackObject<'ctx>> = Vec::new();
        let mut type_allocation_handler: HashMap<String, TypeAllocatorFunction<'ctx>> =
            HashMap::new();
        let mut value_allocation_handler: HashMap<String, ValueAllocatorFunction<'ctx>> =
            HashMap::new();

        type_allocation_handler.insert(
            "Int8".to_string(),
            Box::new(
                |ctx: &'ctx Context, _vec: Vec<StackObject<'ctx>>| -> BasicTypeEnum<'ctx> {
                    ctx.i8_type().into()
                },
            ),
        );

        value_allocation_handler.insert(
            "int8".to_string(),
            Box::new(
                |ctx: &'ctx Context, stack: Vec<StackObject<'ctx>>| -> BasicValueEnum<'ctx> {
                    if let StackObject::LlvmValue(BasicValueEnum::IntValue(value)) =
                        stack[0].clone()
                    {
                        return BasicValueEnum::IntValue(value);
                    }
                    panic!("Expecting a literal value to allocate.")
                },
            ),
        );
        // TODO: Repeat similar code for all literals
        LlvmEmitter {
            context,
            builder,
            module,
            global_values,
            stack,
            type_allocation_handler,
            value_allocation_handler,
        }
    }

    pub fn to_string(&self) -> String {
        self.module.print_to_string().to_string()
    }
    pub fn emit(&mut self, node: &mut NodeProgram) -> String {
        let result = node.visit(self);
        match result {
            TraversalResult::Fail(m) => panic!("{}", m),
            _ => (),
        }
        self.to_string()
    }
}

impl<'ctx> CodeEmitter for LlvmEmitter<'ctx> {
    fn emit_byte_str(&mut self, mode: TreeTraversalMode, node: &NodeByteStr) -> TraversalResult {
        unimplemented!();
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
                    self.stack.push(StackObject::Identifier(Identifier::Event(
                        "Event".to_string(),
                    )));
                }
                NodeTypeNameIdentifier::TypeOrEnumLikeIdentifier(n) => {
                    self.stack
                        .push(StackObject::Identifier(Identifier::TypeName(n.to_string())));
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
        unimplemented!();
    }
    fn emit_import_declarations(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeImportDeclarations,
    ) -> TraversalResult {
        unimplemented!();
    }
    fn emit_meta_identifier(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeMetaIdentifier,
    ) -> TraversalResult {
        // TODO:
        TraversalResult::Ok
        //        unimplemented!();
    }
    fn emit_variable_identifier(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeVariableIdentifier,
    ) -> TraversalResult {
        // TODO:
        //        unimplemented!();
        TraversalResult::Ok
    }
    fn emit_builtin_arguments(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeBuiltinArguments,
    ) -> TraversalResult {
        unimplemented!();
    }
    fn emit_type_map_key(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeMapKey,
    ) -> TraversalResult {
        unimplemented!();
    }
    fn emit_type_map_value(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeMapValue,
    ) -> TraversalResult {
        unimplemented!();
    }
    fn emit_type_argument(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeArgument,
    ) -> TraversalResult {
        unimplemented!();
    }
    fn emit_scilla_type(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeScillaType,
    ) -> TraversalResult {
        unimplemented!();
    }
    fn emit_type_map_entry(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeMapEntry,
    ) -> TraversalResult {
        unimplemented!();
    }
    fn emit_address_type_field(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeAddressTypeField,
    ) -> TraversalResult {
        unimplemented!();
    }
    fn emit_address_type(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeAddressType,
    ) -> TraversalResult {
        unimplemented!();
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
                unimplemented!();
            }
            NodeFullExpression::FunctionDeclaration {
                identier_value, // TODO: Missing spelling - global replacement
                type_annotation,
                expression,
            } => {
                unimplemented!();
            }
            NodeFullExpression::FunctionCall {
                function_name,
                argument_list,
            } => {
                unimplemented!();
            }
            NodeFullExpression::ExpressionAtomic(expr) => match &**expr {
                NodeAtomicExpression::AtomicSid(_identifier) => unimplemented!(),
                NodeAtomicExpression::AtomicLit(literal) => {
                    literal.visit(self);
                }
            },
            NodeFullExpression::ExpressionBuiltin { b, targs, xs } => {
                unimplemented!();
            }
            NodeFullExpression::Message(entries) => {
                unimplemented!();
            }
            NodeFullExpression::Match {
                match_expression,
                clauses,
            } => {
                unimplemented!();
            }
            NodeFullExpression::ConstructorCall {
                identifier_name,
                contract_type_arguments,
                argument_list,
            } => {
                println!("{:#?}", node);
                let error_ret = identifier_name.visit(self);
                match error_ret {
                    TraversalResult::Fail(m) => return TraversalResult::Fail(m),
                    _ => (),
                }

                let stack_name = if let Some(s) = self.stack.pop() {
                    s
                } else {
                    return TraversalResult::Fail(
                        "Expected typename, but found nothing.".to_string(),
                    );
                };

                let name = match stack_name {
                    StackObject::Identifier(identifier_name) => match identifier_name {
                        Identifier::TypeName(n) => n,
                        _ => {
                            return TraversalResult::Fail(format!(
                                "Expected typename but found {:?}",
                                identifier_name
                            ))
                        }
                    },
                    _ => {
                        return TraversalResult::Fail(format!(
                            "Expected typename but found {:?}",
                            stack_name
                        ))
                    }
                };

                println!(
                    "{}:\n- {:#?}\n- {:#?}",
                    name, contract_type_arguments, argument_list
                );
                // unimplemented!();
            }
            NodeFullExpression::TemplateFunction {
                identifier_name,
                expression,
            } => {
                unimplemented!();
            }
            NodeFullExpression::TApp {
                identifier_name,
                type_arguments,
            } => {
                unimplemented!();
            }
        }
        TraversalResult::SkipChildren
    }

    fn emit_message_entry(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeMessageEntry,
    ) -> TraversalResult {
        unimplemented!();
    }
    fn emit_pattern_match_expression_clause(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodePatternMatchExpressionClause,
    ) -> TraversalResult {
        unimplemented!();
    }
    fn emit_atomic_expression(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeAtomicExpression,
    ) -> TraversalResult {
        // TODO:
        TraversalResult::Ok
        //        unimplemented!();
    }
    fn emit_contract_type_arguments(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeContractTypeArguments,
    ) -> TraversalResult {
        unimplemented!();
    }
    fn emit_value_literal(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeValueLiteral,
    ) -> TraversalResult {
        unimplemented!();
    }
    fn emit_map_access(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeMapAccess,
    ) -> TraversalResult {
        unimplemented!();
    }
    fn emit_pattern(&mut self, mode: TreeTraversalMode, node: &NodePattern) -> TraversalResult {
        unimplemented!();
    }
    fn emit_argument_pattern(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeArgumentPattern,
    ) -> TraversalResult {
        unimplemented!();
    }
    fn emit_pattern_match_clause(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodePatternMatchClause,
    ) -> TraversalResult {
        unimplemented!();
    }
    fn emit_blockchain_fetch_arguments(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeBlockchainFetchArguments,
    ) -> TraversalResult {
        unimplemented!();
    }
    fn emit_statement(&mut self, mode: TreeTraversalMode, node: &NodeStatement) -> TraversalResult {
        // TODO:
        TraversalResult::Ok
        //        unimplemented!();
    }
    fn emit_remote_fetch_statement(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeRemoteFetchStatement,
    ) -> TraversalResult {
        unimplemented!();
    }
    fn emit_component_id(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeComponentId,
    ) -> TraversalResult {
        match node {
            NodeComponentId::WithRegularId(name) => {
                self.stack
                    .push(StackObject::Identifier(Identifier::ComponentName(
                        name.to_string(),
                    )));
            }
            NodeComponentId::WithTypeLikeName(n) => {
                self.stack
                    .push(StackObject::Identifier(Identifier::ComponentName(
                        n.to_string(),
                    )));
            }
        }

        TraversalResult::SkipChildren
    }
    fn emit_component_parameters(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeComponentParameters,
    ) -> TraversalResult {
        TraversalResult::Ok
        // TODO:        unimplemented!();
    }
    fn emit_parameter_pair(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeParameterPair,
    ) -> TraversalResult {
        unimplemented!();
    }
    fn emit_component_body(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeComponentBody,
    ) -> TraversalResult {
        // TODO:
        TraversalResult::Ok
        //        unimplemented!();
    }
    fn emit_statement_block(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeStatementBlock,
    ) -> TraversalResult {
        // TODO:
        TraversalResult::Ok
        //        unimplemented!();
    }
    fn emit_typed_identifier(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypedIdentifier,
    ) -> TraversalResult {
        unimplemented!();
    }
    fn emit_type_annotation(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeAnnotation,
    ) -> TraversalResult {
        unimplemented!();
    }

    fn emit_program(&mut self, _mode: TreeTraversalMode, node: &NodeProgram) -> TraversalResult {
        match _mode {
            TreeTraversalMode::Enter => {
                // Parse the version string to u64
                let version = match node.version.parse::<u64>() {
                    Ok(v) => v,
                    Err(_) => {
                        eprintln!("Failed to parse version");
                        return TraversalResult::Fail(
                            "Scilla version must be an integer".to_string(),
                        );
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
            }
            TreeTraversalMode::Exit => {
                // Not sure on what's to be done during exit
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
                // TODO: Push namespace: node.name
            }
            TreeTraversalMode::Exit => {
                // TODO: Pop namespace
            }
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

                    // expression.visit(self);

                    // TODO: Type of expression
                    // TODO: compare to type annotation
                    // TODO: Store as variable name
                }
                NodeLibrarySingleDefinition::TypeDefinition(name, clauses) => {
                    // Similarly generate code for TypeDefinition
                    // You will need to switch on type_annotation to get the correct LLVM type.
                    // You may also need to generate LLVM functions, structs, or other types depending on type_annotation.
                }
            },
            _ => {}
        }
        TraversalResult::Ok
    }
    fn emit_contract_definition(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeContractDefinition,
    ) -> TraversalResult {
        match mode {
            TreeTraversalMode::Enter => {
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
            }
            _ => {}
        }
        TraversalResult::Ok
    }

    fn emit_contract_field(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeContractField,
    ) -> TraversalResult {
        unimplemented!();
    }
    fn emit_with_constraint(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeWithConstraint,
    ) -> TraversalResult {
        unimplemented!();
    }
    fn emit_component_definition(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeComponentDefinition,
    ) -> TraversalResult {
        // TODO:
        TraversalResult::Ok
        //        unimplemented!();
    }
    fn emit_procedure_definition(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeProcedureDefinition,
    ) -> TraversalResult {
        unimplemented!();
    }

    fn emit_transition_definition(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTransitionDefinition,
    ) -> TraversalResult {
        match mode {
            TreeTraversalMode::Enter => {
                // Generating component ID
                match node.name.visit(self) {
                    TraversalResult::Fail(m) => {
                        return TraversalResult::Fail(m);
                    }
                    _ => (),
                }

                let identifier = if let Some(identifier) = self.stack.pop() {
                    match identifier {
                        StackObject::Identifier(n) => n,
                        _ => {
                            return TraversalResult::Fail(format!(
                                "Expected idenfier, but found {:?}.",
                                identifier
                            ));
                        }
                    }
                } else {
                    return TraversalResult::Fail(
                        "Expected transition name, but found nothing.".to_string(),
                    );
                };

                let function_name = match identifier {
                    Identifier::ComponentName(s) => s,
                    _ => {
                        return TraversalResult::Fail(format!(
                            "Expected a component name, but {:?}",
                            identifier
                        ));
                    }
                };

                // TODO: Compute the correct function type
                let fn_type: FunctionType = self.context.void_type().fn_type(&[], false);

                let function = self.module.add_function(&function_name, fn_type, None);
                let basic_block = self.context.append_basic_block(function, "entry");
                self.builder.position_at_end(basic_block);
            }
            TreeTraversalMode::Exit => {
                // TODO: Move cursor out of the function
                // Here you might add a return statement or something similar.
            }
        }
        TraversalResult::Ok
    }

    fn emit_type_alternative_clause(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeAlternativeClause,
    ) -> TraversalResult {
        match node {
            NodeTypeAlternativeClause::ClauseType(identifier) => {
                identifier.visit(self);
            }
            NodeTypeAlternativeClause::ClauseTypeWithArgs(_, _) => unimplemented!(),
        }
        TraversalResult::SkipChildren
    }
    fn emit_type_map_value_arguments(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeMapValueArguments,
    ) -> TraversalResult {
        unimplemented!();
    }
    fn emit_type_map_value_allowing_type_arguments(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeMapValueAllowingTypeArguments,
    ) -> TraversalResult {
        unimplemented!();
    }
}
