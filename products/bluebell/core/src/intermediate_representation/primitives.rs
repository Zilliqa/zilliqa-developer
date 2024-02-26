use std::collections::{BTreeSet, HashMap, VecDeque};

use scilla_parser::parser::lexer::SourcePosition;

use crate::intermediate_representation::symbol_table::SymbolTable;

/// Enum representing the different kinds of identifiers in the intermediate representation.
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

    ContextResource,

    // Storage and reference
    VirtualRegister,
    VirtualRegisterIntermediate,
    Memory,
    State,

    // More info needed to derive kind
    Unknown,
}

/// Struct representing an identifier in the intermediate representation.
#[derive(Debug, Clone, PartialEq)]
pub struct IrIdentifier {
    pub unresolved: String,
    pub resolved: Option<String>,
    pub type_reference: Option<String>,
    pub kind: IrIndentifierKind,
    pub is_definition: bool,
    pub source_location: (SourcePosition, SourcePosition),
}

impl IrIdentifier {
    /// Constructor for the IrIdentifier struct.
    pub fn new(
        unresolved: String,
        kind: IrIndentifierKind,
        source_location: (SourcePosition, SourcePosition),
    ) -> Self {
        Self {
            unresolved,
            resolved: None,
            type_reference: None,
            kind,
            is_definition: false,
            source_location,
        }
    }

    /// Method to get the qualified name of the identifier.
    pub fn qualified_name(&self) -> Result<String, String> {
        // TODO: Change to resolved or throw
        if let Some(resolved) = &self.resolved {
            Ok(resolved.clone())
        } else {
            Ok(format!("[{}]", self.unresolved).to_string())
        }
    }
}

/// Struct representing an enum value in the intermediate representation.
#[derive(Debug, Clone)]
pub struct EnumValue {
    pub name: IrIdentifier,
    pub id: u64,
    pub data: Option<IrIdentifier>,
    // TODO:     pub source_location: (SourcePosition,SourcePosition)
}

impl EnumValue {
    /// Constructor for the EnumValue struct.
    pub fn new(name: IrIdentifier, data: Option<IrIdentifier>) -> Self {
        Self { name, id: 0, data }
    }
    /// Method to set the id of the enum value.
    pub fn set_id(&mut self, v: u64) {
        self.id = v
    }
}

/// Struct representing a tuple in the intermediate representation.
#[derive(Debug, Clone)]
pub struct Tuple {
    pub fields: Vec<IrIdentifier>,
    // TODO:     pub source_location: (SourcePosition,SourcePosition)
}

impl Tuple {
    /// Constructor for the Tuple struct.
    pub fn new() -> Self {
        Self { fields: Vec::new() }
    }

    /// Method to add a field to the tuple.
    pub fn add_field(&mut self, value: IrIdentifier) {
        self.fields.push(value);
    }
}

/// Struct representing a variant in the intermediate representation.
#[derive(Debug, Clone)]
pub struct Variant {
    pub fields: Vec<EnumValue>, // (name, id, data)
                                // TODO:     pub source_location: (SourcePosition,SourcePosition)
}

impl Variant {
    /// Constructor for the Variant struct.
    pub fn new() -> Self {
        Self { fields: Vec::new() }
    }

    /// Method to add a field to the variant.
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

/// Struct representing a variable declaration in the intermediate representation.
#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub name: IrIdentifier,
    pub typename: IrIdentifier,
    pub mutable: bool,
    // TODO:     pub source_location: (SourcePosition,SourcePosition)
}

impl VariableDeclaration {
    /// Constructor for the VariableDeclaration struct.
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
                source_location: (
                    SourcePosition::invalid_position(),
                    SourcePosition::invalid_position(),
                ),
            },
            typename,
            mutable,
        }
    }
}

/// Struct representing a field address in the intermediate representation.
#[derive(Debug, Clone)]
pub struct FieldAddress {
    pub name: IrIdentifier,
    pub value: Option<Vec<u8>>, // TODO: Consider dropping this one
                                // TODO:     pub source_location: (SourcePosition,SourcePosition)
}

/// Struct representing a case clause in the intermediate representation.
#[derive(Debug, Clone)]
pub struct CaseClause {
    pub expression: IrIdentifier,
    pub label: IrIdentifier,
    // TODO:     pub source_location: (SourcePosition,SourcePosition)
}

/// Enum representing the different kinds of operations in the intermediate representation.
#[derive(Debug, Clone)]
pub enum Operation {
    Noop,
    TerminatingRef(IrIdentifier), // Noop operation introduced to balance block arguments for conditional blocks. It can be assumed that the referenced variable is not used after this instruction
    Jump(IrIdentifier),
    ConditionalJump {
        expression: IrIdentifier,
        on_success: IrIdentifier,
        on_failure: IrIdentifier,
    },
    MemLoad,
    MemStore,
    StateLoad {
        address: FieldAddress,
    },
    StateStore {
        address: FieldAddress,
        value: IrIdentifier,
    },
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
    ResolveContextResource {
        symbol: IrIdentifier,
    },
    Literal {
        data: String,
        typename: IrIdentifier,
    },
    PhiNode(Vec<IrIdentifier>),

    Return(Option<IrIdentifier>),
    Revert(Option<IrIdentifier>),
}

/// Struct representing an instruction in the intermediate representation.
#[derive(Debug, Clone)]
pub struct Instruction {
    pub ssa_name: Option<IrIdentifier>,
    pub result_type: Option<IrIdentifier>,
    pub operation: Operation,
    pub source_location: (SourcePosition, SourcePosition),
}

/// Struct representing a function block in the intermediate representation.
#[derive(Debug, Clone)]
pub struct FunctionBlock {
    pub name: IrIdentifier,
    pub block_arguments: BTreeSet<String>,
    pub enters_from: BTreeSet<String>,
    pub exits_to: BTreeSet<String>,
    pub defined_ssas: BTreeSet<String>,
    pub jump_required_arguments: HashMap<String, BTreeSet<String>>,
    pub instructions: VecDeque<Box<Instruction>>,
    pub terminated: bool,
    // TODO:     pub source_location: (SourcePosition,SourcePosition)
}

impl FunctionBlock {
    /// Constructor for the FunctionBlock struct.
    pub fn new(name: String) -> Box<Self> {
        Self::new_from_symbol(Self::new_label(name))
    }

    /// Method to create a new FunctionBlock from a symbol.
    pub fn new_from_symbol(name: IrIdentifier) -> Box<Self> {
        Box::new(Self {
            name,
            block_arguments: BTreeSet::new(),
            enters_from: BTreeSet::new(),
            exits_to: BTreeSet::new(),
            defined_ssas: BTreeSet::new(),
            jump_required_arguments: HashMap::new(),
            instructions: VecDeque::new(),
            terminated: false,
        })
    }

    /// Method to create a new label for a FunctionBlock.
    pub fn new_label(label: String) -> IrIdentifier {
        IrIdentifier {
            unresolved: label.clone(),
            resolved: Some(label), // Label is immediately resolved as it is unrelated to globals and garantueed to be non-conflicting
            type_reference: None,
            kind: IrIndentifierKind::BlockLabel,
            is_definition: true,
            source_location: (
                SourcePosition::invalid_position(),
                SourcePosition::invalid_position(),
            ),
        }
    }
}

/// Struct representing a function body in the intermediate representation.
#[derive(Debug, Clone)]
pub struct FunctionBody {
    pub blocks: Vec<Box<FunctionBlock>>,
    // TODO:     pub source_location: (SourcePosition,SourcePosition)
}

impl FunctionBody {
    /// Constructor for the FunctionBody struct.
    pub fn new() -> Box<Self> {
        Box::new(Self { blocks: Vec::new() })
    }
}

/// Enum representing the different kinds of concrete types in the intermediate representation.
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

/// Enum representing the different kinds of functions in the intermediate representation.
#[derive(Debug, Clone)]
pub enum FunctionKind {
    Procedure,
    Transition,
    Function,
}

/// Struct representing a concrete function in the intermediate representation.
#[derive(Debug, Clone)]
pub struct ConcreteFunction {
    pub name: IrIdentifier,
    pub namespace: IrIdentifier,
    pub function_kind: FunctionKind,
    pub return_type: Option<String>, // TODO: Should be Identifier
    pub arguments: Vec<VariableDeclaration>,
    pub body: Box<FunctionBody>,
}

/// Struct representing a lambda function with a single argument in the intermediate representation.
#[derive(Debug, Clone)]
pub struct LambdaFunctionSingleArgument {
    pub name: IrIdentifier,
    pub capture: Box<Tuple>,
    pub argument: VariableDeclaration,
    pub return_type: Option<String>,
    pub block: FunctionBlock,
}

/// Struct representing a contract field in the intermediate representation.
#[derive(Debug)]
pub struct ContractField {
    pub namespace: IrIdentifier,
    pub variable: VariableDeclaration,
    pub initializer: Box<Instruction>,
}

/// Struct representing the intermediate representation of a program.
#[derive(Debug)]
pub struct IntermediateRepresentation {
    // Program IR
    pub version: String,
    pub type_definitions: Vec<ConcreteType>,
    pub function_definitions: Vec<ConcreteFunction>,
    pub fields_definitions: Vec<ContractField>,
    pub lambda_functions: Vec<LambdaFunctionSingleArgument>,

    // Symbols, storage and memory layout
    pub symbol_table: SymbolTable,
}

impl IntermediateRepresentation {
    /// Constructor for the IntermediateRepresentation struct.
    pub fn new(symbol_table: SymbolTable) -> Self {
        IntermediateRepresentation {
            version: "".to_string(),
            type_definitions: Vec::new(),
            function_definitions: Vec::new(),
            fields_definitions: Vec::new(),
            lambda_functions: Vec::new(),
            symbol_table,
        }
    }
}
