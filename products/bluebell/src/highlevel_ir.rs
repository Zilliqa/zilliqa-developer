#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    FunctionName,
    TransitionName,
    ProcedureName,
    ExternalFunctionName,

    TypeName,
    ComponentName,
    Event,
    Namespace,

    Intermediate,
    Block,

    // More info needed to derive kind
    VariableOrSsaName,
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SymbolName {
    pub unresolved: String,
    pub resolved: Option<String>,
    pub kind: SymbolKind,
}

impl SymbolName {
    pub fn qualified_name(&self) -> Result<String, String> {
        // TODO: Change to resolved or throw
        Ok(self.unresolved.clone())
    }
}

#[derive(Debug, Clone)]
pub struct EnumValue {
    pub name: SymbolName,
    pub id: u64,
    pub data: Option<SymbolName>,
}

impl EnumValue {
    pub fn new(name: SymbolName, data: Option<SymbolName>) -> Self {
        Self { name, id: 0, data }
    }
    pub fn set_id(&mut self, v: u64) {
        self.id = v
    }
}

#[derive(Debug, Clone)]
pub struct Tuple {
    pub fields: Vec<SymbolName>,
}

impl Tuple {
    pub fn new() -> Self {
        Self { fields: Vec::new() }
    }

    pub fn add_field(&mut self, value: SymbolName) {
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
    // Method to determine if the variant is primitive
    pub fn is_pure_enum(&self) -> bool {
        for field in self.fields.iter() {
            if let Some(_) = field.data {
                return false;
            }
        }
        true
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

impl SymbolName {
    pub fn new(unresolved: String, kind: SymbolKind) -> Self {
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
    pub typename: SymbolName,
}

impl VariableDeclaration {
    pub fn new(name: String, typename: SymbolName) -> Self {
        Self { name, typename }
    }
}

#[derive(Debug, Clone)]
pub enum Operation {
    Jump(SymbolName),
    ConditionalJump {
        expression: SymbolName,
        on_success: SymbolName,
        on_failure: SymbolName,
    },
    MemLoad,
    MemStore,
    IsEqual {
        left: SymbolName,
        right: SymbolName,
    },
    CallExternalFunction {
        name: SymbolName,
        arguments: Vec<SymbolName>,
    },
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
    Literal {
        data: String,
        typename: SymbolName,
    },
    AcceptTransfer,
    PhiNode(Vec<SymbolName>),
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub ssa_name: Option<SymbolName>,
    pub result_type: Option<SymbolName>,
    pub operation: Operation,
}

#[derive(Debug, Clone)]
pub struct FunctionBlock {
    pub name: SymbolName,
    pub instructions: Vec<Box<Instruction>>,
    pub terminated: bool,
}

impl FunctionBlock {
    pub fn new(name: String) -> Box<Self> {
        Self::new_from_symbol(SymbolName {
            unresolved: name,
            resolved: None,
            kind: SymbolKind::Block,
        })
    }

    pub fn new_from_symbol(name: SymbolName) -> Box<Self> {
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
        name: SymbolName,
        data_layout: Box<Tuple>,
    },
    Variant {
        name: SymbolName,
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
    pub name: SymbolName,
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

/// TODO: possible
pub trait HighLevelIrEmitter {
    fn emit_symbol_kind(&mut self, symbol_kind: &SymbolKind);
    fn emit_symbol_name(&mut self, symbol_name: &SymbolName);
    fn emit_enum_value(&mut self, enum_value: &EnumValue);
    fn emit_tuple(&mut self, tuple: &Tuple);
    fn emit_variant(&mut self, variant: &Variant);
    fn emit_identifier(&mut self, identifier: &Identifier);
    fn emit_variable_declaration(&mut self, var_dec: &VariableDeclaration);
    fn emit_operation(&mut self, operation: &Operation);
    fn emit_instruction(&mut self, instruction: &Instruction);
    fn emit_function_block(&mut self, function_block: &FunctionBlock);
    fn emit_function_body(&mut self, function_body: &FunctionBody);
    fn emit_concrete_type(&mut self, con_type: &ConcreteType);
    fn emit_function_kind(&mut self, function_kind: &FunctionKind);
    fn emit_concrete_function(&mut self, con_function: &ConcreteFunction);
    fn emit(&mut self, highlevel_ir: &HighlevelIr);
}
