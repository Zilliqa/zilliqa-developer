use crate::ast::*;
use crate::code_emitter::{CodeEmitter, TraversalResult, TreeTraversalMode};
use crate::visitor::Visitor;
use inkwell::module::Module;
use inkwell::types::AnyTypeEnum;
use inkwell::types::{BasicType, BasicTypeEnum};
use inkwell::{
    basic_block::BasicBlock, builder::Builder, context::Context, values::BasicValueEnum,
};
use std::collections::HashMap;

/*
Things that need renaming:
AtomicSid -> ????
EventType -> AutoType ;; Essentially a JSON dict
*/

#[derive(Debug, Clone, PartialEq)]
enum SymbolKind {
    FunctionName,
    TransitionName,
    ProcedureName,

    TypeName,
    ComponentName,
    Event,
    Namespace,

    // More info needed to derive kind
    VariableOrSsaName,
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
struct SymbolName {
    unresolved: String,
    resolved: Option<String>,
    kind: SymbolKind,
}

impl SymbolName {
    fn qualified_name(&self) -> Result<String, String> {
        // TODO: Change to resolved or throw
        Ok(self.unresolved.clone())
    }
}

#[derive(Debug, Clone)]
struct EnumValue {
    name: SymbolName,
    id: u64,
    data: Option<SymbolName>,
}

impl EnumValue {
    fn new(name: SymbolName, data: Option<SymbolName>) -> Self {
        Self { name, id: 0, data }
    }
    fn set_id(&mut self, v: u64) {
        self.id = v
    }
}

#[derive(Debug, Clone)]
struct Tuple {
    fields: Vec<String>,
}
impl Tuple {
    fn new() -> Self {
        Self { fields: Vec::new() }
    }

    fn add_field(&mut self, value: String) {
        self.fields.push(value);
    }
}

#[derive(Debug, Clone)]
struct Variant {
    fields: Vec<EnumValue>, // (name, id, data)
}

impl Variant {
    // Constructor method for our struct
    fn new() -> Self {
        Self { fields: Vec::new() }
    }
    // Method to determine if the variant is primitive
    fn is_pure_enum(&self) -> bool {
        for field in self.fields.iter() {
            if let Some(_) = field.data {
                return false;
            }
        }
        true
    }

    // Method to add a field into our Variant struct
    fn add_field(&mut self, field: EnumValue) {
        let id: u64 = match self.fields.last() {
            // if we have at least one field, use the id of the last field + 1
            Some(enum_value) => enum_value.id + 1,
            // else this is the first field, so use 0
            None => 0,
        };
        let mut field = field.clone();
        field.set_id(id);
        self.fields.push(field);
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Identifier {
    // TODO: Replace with symbol reference
    ComponentName(String),
    TypeName(String),
    Event(String),
}

impl SymbolName {
    fn new(unresolved: String, kind: SymbolKind) -> Self {
        Self {
            unresolved,
            resolved: None,
            kind,
        }
    }
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
    Char(char),
    Bytes(Vec<u8>),
}

#[derive(Debug, Clone)]
struct VariableDeclaration {
    name: String,
    typename: SymbolName,
}

impl VariableDeclaration {
    fn new_stack_object(name: String, typename: SymbolName) -> StackObject<'static> {
        StackObject::VariableDeclaration(Self { name, typename })
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Jump,
    ConditionalJump,
    MemLoad,
    MemStore,
    CallFunction {
        name: SymbolName,
        arguments: Vec<SymbolName>,
    },
    CallStaticFunction {
        name: SymbolName,
        owner: Option<SymbolName>,
        arguments: Vec<SymbolName>,
    },
    CallMemberFunction {
        name: SymbolName,
        owner: SymbolName,
        arguments: Vec<SymbolName>,
    },
    ResolveSymbol {
        symbol: SymbolName,
    },
}

#[derive(Debug, Clone)]
struct Instruction {
    ssa_name: Option<String>,
    result_type: Option<SymbolName>,
    operation: Operation,
}

#[derive(Debug, Clone)]
struct FunctionBlock {
    name: String,
    instructions: Vec<Box<Instruction>>,
    terminated: bool,
}

impl FunctionBlock {
    fn new(name: String) -> Self {
        Self {
            name,
            instructions: Vec::new(),
            terminated: false,
        }
    }
    fn new_stack_object(name: String) -> StackObject<'static> {
        StackObject::FunctionBlock(Box::new(Self::new(name)))
    }
}

#[derive(Debug, Clone)]
struct FunctionBody {
    blocks: Vec<Box<FunctionBlock>>,
}

impl FunctionBody {
    fn new() -> Self {
        Self { blocks: Vec::new() }
    }
    fn new_stack_object() -> StackObject<'static> {
        StackObject::FunctionBody(Box::new(Self { blocks: Vec::new() }))
    }
}

#[derive(Debug, Clone)]
enum StackObject<'ctx> {
    Identifier(Identifier),
    Variant(Variant),
    EnumValue(EnumValue),

    SymbolName(SymbolName),
    Instruction(Box<Instruction>),
    DataTypeReference(String),

    VariableDeclaration(VariableDeclaration),
    FunctionBody(Box<FunctionBody>),
    FunctionBlock(Box<FunctionBlock>),

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

#[derive(Debug, Clone)]
enum ConcreteType {
    Tuple {
        name: SymbolName,
        data_layout: Box<Tuple>,
    },
    Variant {
        name: SymbolName,
        data_layout: Box<Variant>,
    },
}

#[derive(Debug, Clone)]
enum FunctionKind {
    Procedure,
    Transition,
    Function,
}

#[derive(Debug, Clone)]
struct ConcreteFunction {
    name: SymbolName,
    function_kind: FunctionKind,
    return_type: Option<String>,
    arguments: Vec<VariableDeclaration>,
    body: Box<FunctionBody>,
}

pub struct LlvmEmitter<'ctx> {
    context: &'ctx Context,
    builder: Builder<'ctx>,
    module: Module<'ctx>,

    type_definitions: Vec<ConcreteType>,
    function_definitions: Vec<ConcreteFunction>,
    stack: Vec<StackObject<'ctx>>,

    // TODO: not used atm
    global_values: HashMap<String, BasicValueEnum<'ctx>>,
    type_allocation_handler: HashMap<String, TypeAllocatorFunction<'ctx>>,
    value_allocation_handler: HashMap<String, ValueAllocatorFunction<'ctx>>,

    anonymous_type_number: u64,
}

impl<'ctx> LlvmEmitter<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        let builder = context.create_builder();
        let module = context.create_module("main");
        let global_values: HashMap<String, BasicValueEnum<'ctx>> = HashMap::new();
        let stack: Vec<StackObject<'ctx>> = Vec::new();
        let type_definitions = Vec::new();
        let function_definitions = Vec::new();

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
            type_definitions,
            function_definitions,
            type_allocation_handler,
            value_allocation_handler,
            anonymous_type_number: 0,
        }
    }

    fn get_type_definition(&self, name: &str) -> Result<BasicTypeEnum<'ctx>, String> {
        match name {
            "Uint8" => Ok(self.context.i8_type().into()),
            "Uint16" => Ok(self.context.i16_type().into()),
            "Uint32" => Ok(self.context.i32_type().into()),
            "Uint64" => Ok(self.context.i64_type().into()),
            "Int8" => Ok(self.context.i8_type().into()),
            "Int16" => Ok(self.context.i16_type().into()),
            "Int32" => Ok(self.context.i32_type().into()),
            "Int64" => Ok(self.context.i64_type().into()),
            // "Unit" => Ok(self.context.void_type().into()),
            _ => {
                // Get the struct type from the module if it exists
                let val = self.module.get_struct_type(name);
                match val {
                    Some(val) => Ok(BasicTypeEnum::StructType(val)),
                    None => Err(format!("Type '{}' not defined.", name)),
                }
            }
        }
    }

    pub fn get_constructor_name(&self, name: &String) -> String {
        format!("_construct_{}", name).to_string()
    }

    pub fn get_enum_constructor_name(&self, enum_name: &String, name: &String) -> String {
        format!("{}_construct_{}", enum_name, name).to_string()
    }

    fn build_variant(
        &self,
        data_layout: &Variant,
        name: &str,
    ) -> Result<AnyTypeEnum<'ctx>, String> {
        let mut variant_fields = Vec::new();
        let mut data_size = 0;
        for field in &data_layout.fields {
            if let Some(typename) = field.data.as_ref() {
                let typename = typename.qualified_name()?;
                let typevalue = self.get_type_definition(&typename).unwrap();
                let size = match typevalue.size_of() {
                    Some(s) => s,
                    _ => unimplemented!(), // Add the remaining enums as per your implementation.
                };
                let size = match size.get_sign_extended_constant() {
                    Some(s) => s,
                    None => 100, // TODO: This needs fixing for structs - get the size
                                 /*{
                                     println!("Failed to convert {:?}", size);
                                     println!("- Type: {:?}", typevalue);
                                     unimplemented!()
                                 }*/
                };
                if size > data_size {
                    data_size = size;
                }
                variant_fields.push(typevalue);
            }
        }

        // Creating Tag Type
        let data_size = data_size as u32;
        let max_tag_value = data_layout.fields.len() as u32;
        let required_bits = 32 - max_tag_value.leading_zeros();
        let tag_type = self.context.custom_width_int_type(required_bits);
        // tag_type.set_name("TagType");

        // Type erased container
        let i8_type = self.context.i8_type();
        let variant_struct_type = self.context.opaque_struct_type(name);

        let type_erased_container = i8_type.array_type(data_size);
        if data_size == 0 {
            variant_struct_type.set_body(&[tag_type.into()], false);
        } else {
            variant_struct_type.set_body(&[tag_type.into(), type_erased_container.into()], false);
        }

        for (i, field) in data_layout.fields.iter().enumerate() {
            let constructor_name =
                self.get_enum_constructor_name(&name.to_string(), &field.name.qualified_name()?);

            let (func_val, concrete_field_type) = match &field.data {
                None => {
                    // Creating constructor
                    let func_type = variant_struct_type.fn_type(&[], false);
                    let func_val = self.module.add_function(&constructor_name, func_type, None);
                    let block = self.context.append_basic_block(func_val, "entry");
                    self.builder.position_at_end(block);

                    (func_val, variant_struct_type)
                }
                Some(typename) => {
                    // Enum value with associated data
                    let inner_value_type = self
                        .get_type_definition(&typename.qualified_name()?)
                        .unwrap();

                    // Defining the concrete type
                    let concrete_field_type = self.context.opaque_struct_type(&format!(
                        "{}_{}",
                        name,
                        field.name.qualified_name()?
                    ));
                    concrete_field_type
                        .set_body(&[tag_type.into(), inner_value_type.into()], false);

                    // TODO: Add padding if the type is less then max value

                    // Creating constructor
                    let func_type = variant_struct_type.fn_type(&[inner_value_type.into()], false);
                    let func_val = self.module.add_function(&constructor_name, func_type, None);
                    let block = self.context.append_basic_block(func_val, "entry");
                    self.builder.position_at_end(block);

                    (func_val, concrete_field_type)
                }
            };

            // Allocating the contrete type and populating its fields
            let ret_value = self.builder.build_alloca(
                concrete_field_type,
                &format!("{}_value", field.name.qualified_name()?.to_lowercase()),
            );

            if data_size != 0 {
                if let Some(data_value) = func_val.get_param_iter().last() {
                    let data_ptr = self
                        .builder
                        .build_struct_gep(concrete_field_type, ret_value, 1, "data_ptr")
                        .unwrap();
                    self.builder.build_store(data_ptr, data_value);
                } else {
                    // TODO: Store zeros in concrete field type to avoid uninitalised memory?
                }
            }

            // Getting the pointers to storage
            let id_ptr = self
                .builder
                .build_struct_gep(concrete_field_type, ret_value, 0, "id")
                .unwrap();
            self.builder
                .build_store(id_ptr, self.context.i32_type().const_int(i as u64, false));

            // Converting the concrete value to a type-erased version
            let erased_ret =
                self.builder
                    .build_bitcast(ret_value, variant_struct_type, "erased_ret");
            self.builder.build_return(Some(&erased_ret));
        }
        Ok(variant_struct_type.into())
    }

    pub fn write_type_definitions_to_module(&mut self) -> Result<u32, String> {
        for concrete_type in self.type_definitions.iter() {
            match concrete_type {
                ConcreteType::Tuple { name, data_layout } => {
                    let mut field_types = Vec::new();
                    for field in &data_layout.fields {
                        let field_type = self.get_type_definition(field)?;
                        field_types.push(field_type);
                    }

                    let tuple_struct_type =
                        self.context.opaque_struct_type(&name.qualified_name()?);
                    tuple_struct_type.set_body(&field_types[..], false);
                    // let tuple_struct_type = self.context.struct_type(&field_types[..], false);

                    let func_type = tuple_struct_type.fn_type(
                        &field_types
                            .into_iter()
                            .map(|t| t.into())
                            .collect::<Vec<_>>(),
                        false,
                    );

                    // TODO: Clean up: Streamline internal naming here
                    let constructor_name = self.get_constructor_name(&name.qualified_name()?);
                    let func_val = self.module.add_function(&constructor_name, func_type, None);
                    let block = self.context.append_basic_block(func_val, "entry");
                    self.builder.position_at_end(block);
                    let struct_val = self
                        .builder
                        .build_alloca(tuple_struct_type, "struct_alloca");
                    for (index, param) in func_val.get_param_iter().enumerate() {
                        let ptr = self
                            .builder
                            .build_struct_gep(
                                tuple_struct_type,
                                struct_val,
                                index as u32,
                                &format!("field{}", index),
                            )
                            .unwrap();
                        self.builder.build_store(ptr, param);
                    }

                    self.builder.build_return(Some(&struct_val));
                }
                ConcreteType::Variant { name, data_layout } => {
                    self.build_variant(data_layout, &name.qualified_name()?);
                }
            }
        }

        Ok(0)
    }

    fn write_function_definitions_to_module(&mut self) -> Result<u32, String> {
        println!("Functions: {:#?}", self.function_definitions);

        Ok(0)
    }

    fn pop_block(&mut self) -> Result<Box<FunctionBlock>, String> {
        let ret = if let Some(candidate) = self.stack.pop() {
            match candidate {
                StackObject::FunctionBlock(n) => n,
                _ => {
                    return Err(format!("Expected instruction, but found {:?}.", candidate));
                }
            }
        } else {
            return Err("Expected instruction, but found nothing.".to_string());
        };

        Ok(ret)
    }

    fn pop_symbol_name(&mut self) -> Result<SymbolName, String> {
        let ret = if let Some(candidate) = self.stack.pop() {
            match candidate {
                StackObject::SymbolName(n) => n,
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

    fn pop_identifier_expect(&mut self, expected: &Identifier) -> Result<String, String> {
        let identifier = if let Some(identifier) = self.stack.pop() {
            match identifier {
                StackObject::Identifier(n) => n,
                _ => {
                    return Err(format!("Expected identifier, but found {:?}.", identifier));
                }
            }
        } else {
            return Err("Expected identifier, but found nothing.".to_string());
        };
        match (identifier, expected) {
            (Identifier::ComponentName(s), Identifier::ComponentName(_)) => Ok(s),
            (Identifier::TypeName(s), Identifier::TypeName(_)) => Ok(s),
            (Identifier::Event(s), Identifier::Event(_)) => Ok(s),
            _ => Err(format!(
                "Expected a variant of {:?}, but found different variant",
                expected
            )),
        }
    }

    fn pop_datatype_reference(&mut self) -> Result<String, String> {
        let ret = if let Some(candidate) = self.stack.pop() {
            match candidate {
                StackObject::DataTypeReference(n) => n,
                _ => {
                    return Err(format!("Expected data type, but found {:?}.", candidate));
                }
            }
        } else {
            return Err("Expected data type, but found nothing.".to_string());
        };

        Ok(ret)
    }

    fn generate_anonymous_type_id(&mut self, prefix: String) -> SymbolName {
        let n = self.anonymous_type_number;
        self.anonymous_type_number += 1;

        SymbolName {
            unresolved: format!("{}{}", prefix, n).to_string(),
            resolved: None,
            kind: SymbolKind::TypeName,
        }
    }

    pub fn to_string(&self) -> String {
        self.module.print_to_string().to_string()
    }

    pub fn emit(&mut self, node: &mut NodeProgram) -> Result<String, String> {
        let result = node.visit(self);
        match result {
            Err(m) => panic!("{}", m),
            _ => (),
        }

        println!("\n\nDefined types:{:#?}\n\n", self.type_definitions);
        self.write_type_definitions_to_module()?;
        self.write_function_definitions_to_module()?;

        Ok(self.to_string())
    }
}

impl<'ctx> CodeEmitter for LlvmEmitter<'ctx> {
    fn emit_byte_str(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeByteStr,
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
                    self.stack.push(StackObject::Identifier(Identifier::Event(
                        "Event".to_string(),
                    )));
                }
                NodeTypeNameIdentifier::TypeOrEnumLikeIdentifier(name) => {
                    let symbol = SymbolName::new(name.to_string(), SymbolKind::Unknown);

                    self.stack.push(StackObject::SymbolName(symbol));
                }
            },
            TreeTraversalMode::Exit => (),
        }
        Ok(TraversalResult::Continue)
    }
    fn emit_imported_name(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeImportedName,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_import_declarations(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeImportDeclarations,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_meta_identifier(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeMetaIdentifier,
    ) -> Result<TraversalResult, String> {
        // TODO:
        Ok(TraversalResult::Continue)
        //        unimplemented!();
    }
    fn emit_variable_identifier(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeVariableIdentifier,
    ) -> Result<TraversalResult, String> {
        match node {
            NodeVariableIdentifier::VariableName(name) => {
                let operation = Operation::ResolveSymbol {
                    symbol: SymbolName::new(name.to_string(), SymbolKind::VariableOrSsaName),
                };
                let instr = Box::new(Instruction {
                    ssa_name: None,
                    result_type: None,
                    operation,
                });
                self.stack.push(StackObject::Instruction(instr));
            }
            NodeVariableIdentifier::SpecialIdentifier(String) => unimplemented!(),
            NodeVariableIdentifier::VariableInNamespace(NodeTypeNameIdentifier, String) => {
                unimplemented!()
            }
        }
        Ok(TraversalResult::SkipChildren)
    }
    fn emit_builtin_arguments(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeBuiltinArguments,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_type_map_key(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeMapKey,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_type_map_value(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeMapValue,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_type_argument(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeArgument,
    ) -> Result<TraversalResult, String> {
        match node {
            NodeTypeArgument::EnclosedTypeArgument(n) => {
                unimplemented!();
            }
            NodeTypeArgument::GenericTypeArgument(n) => {
                n.visit(self);
                let type_name =
                    self.pop_identifier_expect(&Identifier::TypeName("".to_string()))?;
                self.stack.push(StackObject::DataTypeReference(type_name))
            }
            NodeTypeArgument::TemplateTypeArgument(n) => {
                unimplemented!();
            }
            NodeTypeArgument::AddressTypeArgument(n) => {
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
        mode: TreeTraversalMode,
        node: &NodeScillaType,
    ) -> Result<TraversalResult, String> {
        match node {
            NodeScillaType::GenericTypeWithArgs(lead, args) => {
                lead.visit(self)?;
                if args.len() > 0 {
                    // TODO: Deal with arguments
                    unimplemented!()
                }
            }
            NodeScillaType::MapType(key, value) => {
                key.visit(self);
                value.visit(self);
                // TODO: Pop the two and create type Map<X,Y>
                unimplemented!()
            }
            NodeScillaType::FunctionType(a, b) => {
                (*a).visit(self);
                (*b).visit(self);
                // TODO: Implement the function type
                unimplemented!()
            }

            NodeScillaType::PolyFunctionType(name, a) => {
                // TODO: What to do with name
                (*a).visit(self);
                unimplemented!()
            }
            NodeScillaType::EnclosedType(a) => {
                (*a).visit(self);
            }
            NodeScillaType::ScillaAddresseType(a) => {
                (*a).visit(self);
            }
            NodeScillaType::TypeVarType(name) => {
                self.stack
                    .push(StackObject::Identifier(Identifier::TypeName(
                        name.to_string(),
                    )));
                unimplemented!()
            }
        };
        Ok(TraversalResult::SkipChildren)
    }

    fn emit_type_map_entry(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeMapEntry,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_address_type_field(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeAddressTypeField,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_address_type(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeAddressType,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }

    fn emit_full_expression(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeFullExpression,
    ) -> Result<TraversalResult, String> {
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
                NodeAtomicExpression::AtomicSid(identifier) => {
                    identifier.visit(self)?;
                }
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
                identifier_name.visit(self)?;

                // Expecting function name symbol
                let mut name = self.pop_symbol_name()?;
                assert!(name.kind == SymbolKind::Unknown);
                name.kind = SymbolKind::FunctionName;

                let arguments: Vec<SymbolName> = [].to_vec();

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
        Ok(TraversalResult::SkipChildren)
    }

    fn emit_message_entry(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeMessageEntry,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_pattern_match_expression_clause(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodePatternMatchExpressionClause,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_atomic_expression(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeAtomicExpression,
    ) -> Result<TraversalResult, String> {
        // TODO:
        Ok(TraversalResult::Continue)
        //        unimplemented!();
    }
    fn emit_contract_type_arguments(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeContractTypeArguments,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_value_literal(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeValueLiteral,
    ) -> Result<TraversalResult, String> {
        match node {
            NodeValueLiteral::LiteralInt(typename, value) => {
                typename.visit(self)?;
                println!("Stack: {:#?}", self.stack.last());
                println!("Typename: {:#?}", typename);
                println!("Value: {:#?}", value);
                unimplemented!();
            }
            NodeValueLiteral::LiteralHex(value) => {
                unimplemented!();
            }
            NodeValueLiteral::LiteralString(value) => {
                unimplemented!();
            }
            NodeValueLiteral::LiteralEmptyMap(key, value) => {
                unimplemented!();
            }
        }
        println!("Value: {:#?}", node);
        Ok(TraversalResult::SkipChildren)
    }
    fn emit_map_access(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeMapAccess,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_pattern(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodePattern,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_argument_pattern(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeArgumentPattern,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_pattern_match_clause(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodePatternMatchClause,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_blockchain_fetch_arguments(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeBlockchainFetchArguments,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }

    fn emit_statement(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeStatement,
    ) -> Result<TraversalResult, String> {
        match node {
            NodeStatement::Load {
                left_hand_side,
                right_hand_side,
            } => {
                unimplemented!()
            }
            NodeStatement::RemoteFetch(remote_stmt) => {
                unimplemented!()
            }
            NodeStatement::Store {
                left_hand_side,
                right_hand_side,
            } => {
                unimplemented!()
            }
            NodeStatement::Bind {
                left_hand_side,
                right_hand_side,
            } => {
                // Generating instruction and setting its name
                right_hand_side.visit(self)?;
                println!("RHS: {:#?}", right_hand_side);
                println!("Stack: {:#?}", self.stack);
                let mut right_hand_side = self.pop_instruction()?;
                (*right_hand_side).ssa_name = Some(left_hand_side.to_string());

                // Adding the instruction to the current block we are building
                let mut block = self.pop_block()?;
                block.instructions.push(right_hand_side);
                self.stack.push(StackObject::FunctionBlock(block));
            }
            NodeStatement::ReadFromBC {
                left_hand_side,
                type_name,
                arguments,
            } => {
                unimplemented!()
            }
            NodeStatement::MapGet {
                left_hand_side,
                keys,
                right_hand_side,
            } => {
                unimplemented!()
            }
            NodeStatement::MapGetExists {
                left_hand_side,
                keys,
                right_hand_side,
            } => {
                unimplemented!()
            }
            NodeStatement::MapUpdate {
                left_hand_side,
                keys,
                right_hand_side,
            } => {
                unimplemented!()
            }
            NodeStatement::MapUpdateDelete {
                left_hand_side,
                keys,
            } => {
                unimplemented!()
            }
            NodeStatement::Accept => {
                unimplemented!()
            }
            NodeStatement::Send { identifier_name } => {
                unimplemented!()
            }
            NodeStatement::CreateEvnt { identifier_name } => {
                unimplemented!()
            }
            NodeStatement::Throw { error_variable } => {
                unimplemented!()
            }
            NodeStatement::MatchStmt { variable, clauses } => {
                unimplemented!()
            }
            NodeStatement::CallProc {
                component_id,
                arguments,
            } => {
                unimplemented!()
            }
            NodeStatement::Iterate {
                identifier_name,
                component_id,
            } => {
                unimplemented!()
            }
        }
        Ok(TraversalResult::SkipChildren)
    }

    fn emit_remote_fetch_statement(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeRemoteFetchStatement,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_component_id(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeComponentId,
    ) -> Result<TraversalResult, String> {
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

        Ok(TraversalResult::SkipChildren)
    }

    fn emit_component_parameters(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeComponentParameters,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
        // TODO:        unimplemented!();
    }

    fn emit_parameter_pair(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeParameterPair,
    ) -> Result<TraversalResult, String> {
        // Delibarate pass through
        Ok(TraversalResult::Continue)
    }

    fn emit_component_body(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeComponentBody,
    ) -> Result<TraversalResult, String> {
        self.stack.push(FunctionBody::new_stack_object());

        if let Some(block) = &node.statement_block {
            block.visit(self)?;
        }

        let last_block = self.pop_function_block()?;
        let mut body = self.pop_function_body()?;
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
                self.stack
                    .push(FunctionBlock::new_stack_object("entry".to_string()));
            }
            _ => (),
        }

        Ok(TraversalResult::Continue)
    }
    fn emit_typed_identifier(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypedIdentifier,
    ) -> Result<TraversalResult, String> {
        let name = node.identifier_name.clone();
        node.annotation.visit(self)?;

        let mut typename = self.pop_symbol_name()?;
        assert!(typename.kind == SymbolKind::Unknown);
        typename.kind = SymbolKind::TypeName;

        self.stack
            .push(VariableDeclaration::new_stack_object(name, typename));

        Ok(TraversalResult::SkipChildren)
    }
    fn emit_type_annotation(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeAnnotation,
    ) -> Result<TraversalResult, String> {
        // Pass through
        Ok(TraversalResult::Continue)
        //        unimplemented!();
    }

    fn emit_program(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodeProgram,
    ) -> Result<TraversalResult, String> {
        match _mode {
            TreeTraversalMode::Enter => {
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
        mode: TreeTraversalMode,
        node: &NodeLibrarySingleDefinition,
    ) -> Result<TraversalResult, String> {
        match node {
            NodeLibrarySingleDefinition::LetDefinition {
                variable_name,
                type_annotation,
                expression,
            } => {
                unimplemented!();
            }
            NodeLibrarySingleDefinition::TypeDefinition(name, clauses) => {
                name.visit(self);
                let mut name = self.pop_symbol_name()?;
                assert!(name.kind == SymbolKind::Unknown);
                name.kind = SymbolKind::TypeName;

                let mut user_type = Variant::new();

                if let Some(clauses) = clauses {
                    for clause in clauses.iter() {
                        clause.visit(self);
                        let field = self.pop_enum_value()?;
                        user_type.add_field(field);
                    }
                }

                self.type_definitions.push(ConcreteType::Variant {
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
        Ok(TraversalResult::Continue)
    }

    fn emit_contract_field(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeContractField,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_with_constraint(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeWithConstraint,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_component_definition(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeComponentDefinition,
    ) -> Result<TraversalResult, String> {
        // TODO:
        Ok(TraversalResult::Continue)
        //        unimplemented!();
    }
    fn emit_procedure_definition(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeProcedureDefinition,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }

    fn emit_transition_definition(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTransitionDefinition,
    ) -> Result<TraversalResult, String> {
        // Enter
        node.name.visit(self)?;

        let mut arguments: Vec<VariableDeclaration> = [].to_vec();
        for arg in node.parameters.parameters.iter() {
            arg.visit(self)?;
            let ir_arg = self.pop_variable_declaration()?;
            arguments.push(ir_arg);
        }

        // Function body
        node.body.visit(self)?;

        // Exit
        let mut body = self.pop_function_body()?;
        let mut function_name = self.pop_symbol_name()?;
        assert!(function_name.kind == SymbolKind::Unknown);
        function_name.kind = SymbolKind::TransitionName;

        // TODO: Decude return type from body

        let function = ConcreteFunction {
            name: function_name,
            function_kind: FunctionKind::Transition,
            return_type: None, // TODO: Pop of the stack
            arguments,
            body,
        };

        self.function_definitions.push(function);

        Ok(TraversalResult::SkipChildren)
    }

    fn emit_type_alternative_clause(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeAlternativeClause,
    ) -> Result<TraversalResult, String> {
        match node {
            NodeTypeAlternativeClause::ClauseType(identifier) => {
                identifier.visit(self);
                let mut enum_name = self.pop_symbol_name()?;
                assert!(enum_name.kind == SymbolKind::Unknown);
                enum_name.kind = SymbolKind::TypeName;

                self.stack
                    .push(StackObject::EnumValue(EnumValue::new(enum_name, None)));
            }
            NodeTypeAlternativeClause::ClauseTypeWithArgs(identifier, children) => {
                identifier.visit(self);
                let mut member_name = self.pop_symbol_name()?;
                assert!(member_name.kind == SymbolKind::Unknown);
                member_name.kind = SymbolKind::TypeName;

                let mut tuple = Tuple::new();
                for child in children.iter() {
                    let x = child.visit(self)?;
                    let item = self.pop_datatype_reference()?;
                    tuple.add_field(item)
                }

                let refid = self.generate_anonymous_type_id("Tuple".to_string());

                self.type_definitions.push(ConcreteType::Tuple {
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
        mode: TreeTraversalMode,
        node: &NodeTypeMapValueArguments,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_type_map_value_allowing_type_arguments(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeMapValueAllowingTypeArguments,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
}
