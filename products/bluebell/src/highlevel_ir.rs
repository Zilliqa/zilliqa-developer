use inkwell::module::Module;
use inkwell::types::AnyTypeEnum;
use inkwell::types::{BasicType, BasicTypeEnum};
use inkwell::{
    basic_block::BasicBlock, builder::Builder, context::Context, values::BasicValueEnum,
};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum IrIndentifierKind {
    FunctionName,
    TransitionName,
    ProcedureName,
    ExternalFunctionName,

    TypeName,
    ComponentName,
    Event,
    Namespace,

    Block,
    VirtualRegister,
    VirtualRegisterIntermediate,

    // More info needed to derive kind
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IrIdentifier {
    pub unresolved: String,
    pub resolved: Option<String>,
    pub kind: IrIndentifierKind,
}

impl IrIdentifier {
    pub fn qualified_name(&self) -> Result<String, String> {
        // TODO: Change to resolved or throw
        Ok(self.unresolved.clone())
    }
}

#[derive(Debug, Clone)]
pub struct EnumValue {
    pub name: IrIdentifier,
    pub id: u64,
    pub data: Option<IrIdentifier>,
}

impl EnumValue {
    pub fn new(name: IrIdentifier, data: Option<IrIdentifier>) -> Self {
        Self { name, id: 0, data }
    }
    pub fn set_id(&mut self, v: u64) {
        self.id = v
    }
}

#[derive(Debug, Clone)]
pub struct Tuple {
    pub fields: Vec<IrIdentifier>,
}

impl Tuple {
    pub fn new() -> Self {
        Self { fields: Vec::new() }
    }

    pub fn add_field(&mut self, value: IrIdentifier) {
        self.fields.push(value);
    }
}

#[derive(Debug, Clone)]
pub struct Variant {
    pub fields: Vec<EnumValue>, // (name, id, data)
}

impl Variant {
    // Constructor method for our struct
    pub fn new() -> Self {
        Self { fields: Vec::new() }
    }

    // Method to add a field into our Variant struct
    pub fn add_field(&mut self, field: EnumValue) {
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
pub enum Identifier {
    // TODO: Replace with symbol reference
    ComponentName(String),
    TypeName(String),
    Event(String),
}

impl IrIdentifier {
    pub fn new(unresolved: String, kind: IrIndentifierKind) -> Self {
        Self {
            unresolved,
            resolved: None,
            kind,
        }
    }
}

#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub name: String,
    pub typename: IrIdentifier,
}

impl VariableDeclaration {
    pub fn new(name: String, typename: IrIdentifier) -> Self {
        Self { name, typename }
    }
}

#[derive(Debug, Clone)]
pub enum Operation {
    Jump(IrIdentifier),
    ConditionalJump {
        expression: IrIdentifier,
        on_success: IrIdentifier,
        on_failure: IrIdentifier,
    },
    MemLoad,
    MemStore,
    IsEqual {
        left: IrIdentifier,
        right: IrIdentifier,
    },
    CallExternalFunction {
        name: IrIdentifier,
        arguments: Vec<IrIdentifier>,
    },
    CallFunction {
        name: IrIdentifier,
        arguments: Vec<IrIdentifier>,
    },
    CallStaticFunction {
        name: IrIdentifier,
        owner: Option<IrIdentifier>,
        arguments: Vec<IrIdentifier>,
    },
    CallMemberFunction {
        name: IrIdentifier,
        owner: IrIdentifier,
        arguments: Vec<IrIdentifier>,
    },
    ResolveSymbol {
        symbol: IrIdentifier,
    },
    Literal {
        data: String,
        typename: IrIdentifier,
    },
    AcceptTransfer,
    PhiNode(Vec<IrIdentifier>),
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub ssa_name: Option<IrIdentifier>,
    pub result_type: Option<IrIdentifier>,
    pub operation: Operation,
}

#[derive(Debug, Clone)]
pub struct FunctionBlock {
    pub name: IrIdentifier,
    pub instructions: Vec<Box<Instruction>>,
    pub terminated: bool,
}

impl FunctionBlock {
    pub fn new(name: String) -> Box<Self> {
        Self::new_from_symbol(IrIdentifier {
            unresolved: name,
            resolved: None,
            kind: IrIndentifierKind::Block,
        })
    }

    pub fn new_from_symbol(name: IrIdentifier) -> Box<Self> {
        Box::new(Self {
            name,
            instructions: Vec::new(),
            terminated: false,
        })
    }
}

#[derive(Debug, Clone)]
pub struct FunctionBody {
    pub blocks: Vec<Box<FunctionBlock>>,
}

impl FunctionBody {
    pub fn new() -> Box<Self> {
        Box::new(Self { blocks: Vec::new() })
    }
}

#[derive(Debug, Clone)]
pub enum ConcreteType {
    Tuple {
        name: IrIdentifier,
        data_layout: Box<Tuple>,
    },
    Variant {
        name: IrIdentifier,
        data_layout: Box<Variant>,
    },
}

#[derive(Debug, Clone)]
pub enum FunctionKind {
    Procedure,
    Transition,
    Function,
}

#[derive(Debug, Clone)]
pub struct ConcreteFunction {
    pub name: IrIdentifier,
    pub function_kind: FunctionKind,
    pub return_type: Option<String>,
    pub arguments: Vec<VariableDeclaration>,
    pub body: Box<FunctionBody>,
}

pub struct HighlevelIr {
    pub type_definitions: Vec<ConcreteType>,
    pub function_definitions: Vec<ConcreteFunction>,
}

impl HighlevelIr {
    pub fn new() -> Self {
        HighlevelIr {
            type_definitions: Vec::new(),
            function_definitions: Vec::new(),
        }
    }
}

pub trait IrLowering {
    fn lower_concrete_type(&mut self, con_type: &ConcreteType);
    fn lower_concrete_function(&mut self, con_function: &ConcreteFunction);
    fn lower(&mut self, highlevel_ir: &HighlevelIr);
}

struct LlvmIrGenerator<'ctx> {
    context: &'ctx Context,
    builder: Builder<'ctx>,
    module: Module<'ctx>,
    ir: Box<HighlevelIr>, // TODO: Add any members needed for the generation here
}

impl<'ctx> LlvmIrGenerator<'ctx> {
    pub fn new(context: &'ctx Context, ir: Box<HighlevelIr>) -> Self {
        let builder = context.create_builder();
        let module = context.create_module("main");

        LlvmIrGenerator {
            context,
            builder,
            module,
            ir,
        }
    }
}

impl<'ctx> IrLowering for LlvmIrGenerator<'ctx> {
    // Lower a single concrete type from HighlevelIr to LLVM IR.
    fn lower_concrete_type(&mut self, con_type: &ConcreteType) {
        match con_type {
            ConcreteType::Tuple { name, data_layout } => {
                // provide functionality to handle tuple type
                unimplemented!()
            }
            ConcreteType::Variant { name, data_layout } => {
                // provide functionality to handle variant type
                unimplemented!()
            }
        }
    }
    // Lower a single concrete function from HighlevelIr to LLVM IR.
    fn lower_concrete_function(&mut self, con_function: &ConcreteFunction) {
        let func_name = &con_function
            .name
            .resolved
            .as_ref()
            .unwrap_or(&con_function.name.unresolved);
        match con_function.function_kind {
            FunctionKind::Procedure => {
                // provide functionality for procedure kind
                unimplemented!()
            }
            FunctionKind::Transition => {
                // provide functionality for Transition kind
                unimplemented!()
            }
            FunctionKind::Function => {
                // provide functionality for function kind
                unimplemented!()
            }
        }
    }
    // Lower the entire HighLevelIr to LLVM IR.
    fn lower(&mut self, highlevel_ir: &HighlevelIr) {
        for con_type in &highlevel_ir.type_definitions {
            self.lower_concrete_type(con_type);
        }
        for con_function in &highlevel_ir.function_definitions {
            self.lower_concrete_function(con_function);
        }
        // After lowering all elements, perform a final step.
    }
}
