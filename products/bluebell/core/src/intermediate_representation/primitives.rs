use crate::symbol_table::SymbolTable;

#[derive(Debug, Clone, PartialEq)]
pub enum IrIndentifierKind {
    FunctionName,
    StaticFunctionName,
    TransitionName,
    ProcedureName,
    TemplateFunctionName,
    ExternalFunctionName,

    TypeName,
    ComponentName,
    Event,
    Namespace,
    BlockLabel,

    // Storage and reference
    VirtualRegister,
    VirtualRegisterIntermediate,
    Memory,

    // More info needed to derive kind
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IrIdentifier {
    pub unresolved: String,
    pub resolved: Option<String>,
    pub type_reference: Option<String>,
    pub kind: IrIndentifierKind,
    pub is_definition: bool,
}

impl IrIdentifier {
    pub fn new(unresolved: String, kind: IrIndentifierKind) -> Self {
        Self {
            unresolved,
            resolved: None,
            type_reference: None,
            kind,
            is_definition: false,
        }
    }

    pub fn qualified_name(&self) -> Result<String, String> {
        // TODO: Change to resolved or throw
        if let Some(resolved) = &self.resolved {
            Ok(resolved.clone())
        } else {
            Ok(format!("[{}]", self.unresolved).to_string())
        }
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

/*
#[derive(Debug, Clone, PartialEq)]
pub enum Identifier {
    // TODO: Replace with symbol reference
    ComponentName(String),
    TypeName(String),
    Event(String),
}
*/

#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub name: IrIdentifier,
    pub typename: IrIdentifier,
    pub mutable: bool,
}

impl VariableDeclaration {
    pub fn new(name: String, mutable: bool, typename: IrIdentifier) -> Self {
        Self {
            name: IrIdentifier {
                unresolved: name,
                resolved: None,
                type_reference: None,
                kind: if mutable {
                    IrIndentifierKind::Memory
                } else {
                    IrIndentifierKind::VirtualRegister
                },
                is_definition: true,
            },
            typename,
            mutable,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Operation {
    Noop,
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
        owner: Option<IrIdentifier>,
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

    Return(Option<IrIdentifier>),
    Revert(Option<IrIdentifier>),
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
        Self::new_from_symbol(Self::new_label(name))
    }

    pub fn new_from_symbol(name: IrIdentifier) -> Box<Self> {
        Box::new(Self {
            name,
            instructions: Vec::new(),
            terminated: false,
        })
    }

    pub fn new_label(label: String) -> IrIdentifier {
        IrIdentifier {
            unresolved: label.clone(),
            resolved: Some(label), // Label is immediately resolved as it is unrelated to globals and garantueed to be non-conflicting
            type_reference: None,
            kind: IrIndentifierKind::BlockLabel,
            is_definition: true,
        }
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
        namespace: IrIdentifier,
        data_layout: Box<Tuple>,
    },
    Variant {
        name: IrIdentifier,
        namespace: IrIdentifier,
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
    pub namespace: IrIdentifier,
    pub function_kind: FunctionKind,
    pub return_type: Option<String>, // TODO: Should be Identifier
    pub arguments: Vec<VariableDeclaration>,
    pub body: Box<FunctionBody>,
}

#[derive(Debug, Clone)]
pub struct LambdaFunctionSingleArgument {
    pub name: IrIdentifier,
    pub capture: Box<Tuple>,
    pub argument: VariableDeclaration,
    pub return_type: Option<String>,
    pub block: FunctionBlock,
}

/*
impl ComputableState {
    pub fn get_concrete_name(&self, types: Vec<String>) -> Result<String, String> {
        let mut basename = if let Some(n) = &self.base.name.resolved {
            n.clone()
        } else {
            return Err("Internal error: Base function does not have a resolved name".to_string());
        };

        if types.len() != self.template_arguments.len() {
            return Err(format!(
                "Template function expected {} arguments, but found {} arguments",
                self.template_arguments.len(),
                types.len()
            ));
        }

        basename.push_str("<");
        for (i, arg) in types.iter().enumerate() {
            if i > 0 {
                basename.push_str(", ");
            }
            basename.push_str(arg);
        }
        basename.push_str(">");

        Ok(basename)
    }
}
*/

#[derive(Debug)]
pub struct HighlevelIr {
    pub version: String,
    pub type_definitions: Vec<ConcreteType>,
    pub function_definitions: Vec<ConcreteFunction>,
    pub lambda_functions: Vec<LambdaFunctionSingleArgument>,
    pub symbol_table: SymbolTable,
}

impl HighlevelIr {
    pub fn new() -> Self {
        HighlevelIr {
            version: "".to_string(),
            type_definitions: Vec::new(),
            function_definitions: Vec::new(),
            lambda_functions: Vec::new(),
            symbol_table: SymbolTable::new(),
        }
    }
}

pub trait IrLowering {
    fn lower_concrete_type(&mut self, con_type: &ConcreteType);
    fn lower_concrete_function(&mut self, con_function: &ConcreteFunction);
    fn lower(&mut self, primitives: &HighlevelIr);
}
