use std::fmt;

use crate::parser::lexer::SourcePosition;

/// A wrapper struct that adds source position to an AST node.
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct WithMetaData<T> {
    /// The AST node
    pub node: T,
    /// The starting position of the AST node in the source code
    pub start: SourcePosition,
    /// The ending position of the AST node in the source code
    pub end: SourcePosition,
}

/// Implementing Display trait for WithMetaData struct
impl<T: fmt::Display> fmt::Display for WithMetaData<T> {
    /// Formats the WithMetaData instance into a string
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.node.to_string())
    }
}

/// NodeByteStr represents a byte string node in the AST
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeByteStr {
    /// Represents a constant byte string
    /// Example: `let x = "constant";`
    Constant(WithMetaData<String>), // TODO: Apparently not used anywhere
    /// Represents a byte string type
    /// Example: `let x: ByStr = "type";`
    Type(WithMetaData<String>),
}

impl NodeByteStr {
    /// Converts the NodeByteStr to a string
    pub fn to_string(&self) -> String {
        match self {
            NodeByteStr::Constant(s) => s.node.clone(),
            NodeByteStr::Type(t) => t.node.clone(),
        }
    }
}
impl fmt::Display for NodeByteStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/// NodeTypeNameIdentifier represents a type name identifier node in the AST
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeTypeNameIdentifier {
    /// Represents a byte string type
    /// Example: `let x: ByStr = "type";`
    ByteStringType(WithMetaData<NodeByteStr>),
    /// Represents an event type
    /// Example: `event e;`
    EventType,
    /// Represents a type or enum-like identifier
    /// Example: `let x: CustomType = "type";`
    TypeOrEnumLikeIdentifier(WithMetaData<String>),
}

impl NodeTypeNameIdentifier {
    /// Converts the NodeTypeNameIdentifier to a string
    pub fn to_string(&self) -> String {
        match self {
            NodeTypeNameIdentifier::ByteStringType(byte_str) => {
                format!("{}", byte_str.node.to_string())
            }
            NodeTypeNameIdentifier::EventType => format!("Event"),
            NodeTypeNameIdentifier::TypeOrEnumLikeIdentifier(custom_type) => {
                format!("{}", custom_type.clone())
            }
        }
    }
}
impl fmt::Display for NodeTypeNameIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/// NodeImportedName represents an imported name node in the AST
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeImportedName {
    /// Represents a regular import
    /// Example: `import CustomType;`
    RegularImport(WithMetaData<NodeTypeNameIdentifier>),
    /// Represents an aliased import
    /// Example: `import CustomType as Alias;`
    AliasedImport(
        WithMetaData<NodeTypeNameIdentifier>,
        WithMetaData<NodeTypeNameIdentifier>,
    ),
}

/// NodeImportDeclarations represents a list of import declarations in the AST
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeImportDeclarations {
    pub import_list: Vec<WithMetaData<NodeImportedName>>,
}

/// NodeMetaIdentifier represents a meta identifier node in the AST
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeMetaIdentifier {
    /// Represents a meta name
    /// Example: `let x: MetaName = "type";`
    MetaName(WithMetaData<NodeTypeNameIdentifier>),
    /// Represents a meta name in a namespace
    /// Example: `let x: Namespace.MetaName = "type";`
    MetaNameInNamespace(
        WithMetaData<NodeTypeNameIdentifier>,
        WithMetaData<NodeTypeNameIdentifier>,
    ),
    /// Represents a meta name in a hexspace
    /// Example: `let x: 0x123.MetaName = "type";`
    MetaNameInHexspace(WithMetaData<String>, WithMetaData<NodeTypeNameIdentifier>),
    /// Represents a byte string
    /// Example: `let x: ByStr = "type";`
    ByteString,
}

impl NodeMetaIdentifier {
    /// Converts the NodeMetaIdentifier to a string
    pub fn to_string(&self) -> String {
        match self {
            NodeMetaIdentifier::MetaName(name) => {
                format!("{}", name)
            }
            NodeMetaIdentifier::MetaNameInNamespace(namespace, name) => {
                format!("{}.{}", namespace, name)
            }
            NodeMetaIdentifier::MetaNameInHexspace(hexspace, name) => {
                format!("{}.{}", hexspace, name)
            }
            NodeMetaIdentifier::ByteString => format!("ByStr"),
        }
    }
}

impl fmt::Display for NodeMetaIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/// NodeVariableIdentifier represents a variable identifier node in the AST
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeVariableIdentifier {
    /// Represents a variable name
    /// Example: `let x = "variable";`
    VariableName(WithMetaData<String>),
    /// Represents a special identifier
    /// Example: `let _ = "special";`
    SpecialIdentifier(WithMetaData<String>),
    /// Represents a variable in a namespace
    /// Example: `let x: Namespace.Variable = "variable";`
    VariableInNamespace(WithMetaData<NodeTypeNameIdentifier>, WithMetaData<String>),
}

impl NodeVariableIdentifier {
    /// Converts the NodeVariableIdentifier to a string
    pub fn to_string(&self) -> String {
        match self {
            NodeVariableIdentifier::VariableName(name) => format!("{}", name),
            NodeVariableIdentifier::SpecialIdentifier(id) => format!("{}", id),
            NodeVariableIdentifier::VariableInNamespace(namespace, var_name) => {
                format!("{}.{}", namespace.to_string(), var_name)
            }
        }
    }
}

impl fmt::Display for NodeVariableIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/// NodeBuiltinArguments represents a list of arguments for a built-in function in the AST
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeBuiltinArguments {
    pub arguments: Vec<WithMetaData<NodeVariableIdentifier>>,
}

/// NodeTypeMapKey represents a type map key node in the AST
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeTypeMapKey {
    /// Represents a generic map key
    /// Example: `let x: Map (KeyType, ValueType) = Emp;`
    GenericMapKey(WithMetaData<NodeMetaIdentifier>),
    /// Represents an enclosed generic id
    /// Example: `let x: Map ((KeyType), ValueType) = Emp;`
    EnclosedGenericId(WithMetaData<NodeMetaIdentifier>),
    /// Represents an enclosed address map key type
    /// Example: `let x: Map ((ByStr20), ValueType) = Emp;`
    EnclosedAddressMapKeyType(WithMetaData<NodeAddressType>),
    /// Represents an address map key type
    /// Example: `let x: Map (ByStr20, ValueType) = Emp;`
    AddressMapKeyType(WithMetaData<NodeAddressType>),
}

/// NodeTypeMapValue represents a type map value node in the AST
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeTypeMapValue {
    /// Represents a map value type or enum-like identifier
    /// Example: `let x: Map (KeyType, ValueType) = Emp;`
    MapValueTypeOrEnumLikeIdentifier(WithMetaData<NodeMetaIdentifier>),
    /// Represents a map key value type
    /// Example: `let x: Map (KeyType, (KeyType, ValueType)) = Emp;`
    MapKeyValue(Box<WithMetaData<NodeTypeMapEntry>>),
    /// Represents a map value paranthesized type
    /// Example: `let x: Map (KeyType, (ValueType)) = Emp;`
    MapValueParanthesizedType(Box<WithMetaData<NodeTypeMapValueAllowingTypeArguments>>),
    /// Represents a map value address type
    /// Example: `let x: Map (KeyType, ByStr20) = Emp;`
    MapValueAddressType(Box<WithMetaData<NodeAddressType>>),
}

/// NodeTypeArgument represents a type argument node in the AST
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeTypeArgument {
    /// Represents an enclosed type argument
    /// Example: `let x: CustomType (ArgType) = "type";`
    EnclosedTypeArgument(Box<WithMetaData<NodeScillaType>>),
    /// Represents a generic type argument
    /// Example: `let x: CustomType ArgType = "type";`
    GenericTypeArgument(WithMetaData<NodeMetaIdentifier>),
    /// Represents a template type argument
    /// Example: `let x: CustomType "ArgType" = "type";`
    TemplateTypeArgument(WithMetaData<String>),
    /// Represents an address type argument
    /// Example: `let x: CustomType ByStr20 = "type";`
    AddressTypeArgument(WithMetaData<NodeAddressType>),
    /// Represents a map type argument
    /// Example: `let x: CustomType (KeyType, ValueType) = "type";`
    MapTypeArgument(WithMetaData<NodeTypeMapKey>, WithMetaData<NodeTypeMapValue>),
}

/// NodeScillaType represents a Scilla type node in the AST
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeScillaType {
    /// Represents a generic type with arguments
    /// Example: `let x: CustomType ArgType = "type";`
    GenericTypeWithArgs(
        WithMetaData<NodeMetaIdentifier>,
        Vec<WithMetaData<NodeTypeArgument>>,
    ),
    /// Represents a map type
    /// Example: `let x: Map (KeyType, ValueType) = Emp;`
    MapType(WithMetaData<NodeTypeMapKey>, WithMetaData<NodeTypeMapValue>),
    /// Represents a function type
    /// Example: `let x: Fun (ArgType) ReturnType = fun (arg : ArgType) => arg;`
    FunctionType(
        Box<WithMetaData<NodeScillaType>>,
        Box<WithMetaData<NodeScillaType>>,
    ),
    /// Represents an enclosed type
    /// Example: `let x: (CustomType) = "type";`
    EnclosedType(Box<WithMetaData<NodeScillaType>>),
    /// Represents a Scilla address type
    /// Example: `let x: ByStr20 = "0x123";`
    ScillaAddresseType(Box<WithMetaData<NodeAddressType>>),
    /// Represents a poly function type
    /// Example: `let x: forall 'A. ('A -> 'A) = fun (arg : 'A) => arg;`
    PolyFunctionType(WithMetaData<String>, Box<WithMetaData<NodeScillaType>>),
    /// Represents a type var type
    /// Example: `let x: 'A = "type";`
    TypeVarType(WithMetaData<String>),
}

/// NodeTypeMapEntry represents a type map entry node in the AST
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeTypeMapEntry {
    pub key: WithMetaData<NodeTypeMapKey>,
    pub value: WithMetaData<NodeTypeMapValue>,
}

/// NodeAddressTypeField represents an address type field node in the AST
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeAddressTypeField {
    pub identifier: WithMetaData<NodeVariableIdentifier>,
    pub type_name: WithMetaData<NodeScillaType>,
}

/// NodeAddressType represents an address type node in the AST
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeAddressType {
    pub identifier: WithMetaData<NodeTypeNameIdentifier>,
    pub type_name: WithMetaData<String>,
    pub address_fields: Vec<WithMetaData<NodeAddressTypeField>>,
}

/// NodeFullExpression represents a full expression node in the AST
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeFullExpression {
    /// Represents a local variable declaration
    /// Example: `let x = "variable";`
    LocalVariableDeclaration {
        identifier_name: WithMetaData<String>,
        expression: Box<WithMetaData<NodeFullExpression>>,
        type_annotation: Option<WithMetaData<NodeTypeAnnotation>>,
        containing_expression: Box<WithMetaData<NodeFullExpression>>,
    },
    /// Represents a function declaration
    /// Example: `let f = fun (arg : ArgType) => arg;`
    FunctionDeclaration {
        identier_value: WithMetaData<String>,
        type_annotation: WithMetaData<NodeTypeAnnotation>,
        expression: Box<WithMetaData<NodeFullExpression>>,
    },
    /// Represents a function call
    /// Example: `f(arg);`
    FunctionCall {
        function_name: WithMetaData<NodeVariableIdentifier>,
        argument_list: Vec<WithMetaData<NodeVariableIdentifier>>,
    },
    /// Represents an atomic expression
    /// Example: `let x = "atomic";`
    ExpressionAtomic(Box<WithMetaData<NodeAtomicExpression>>),
    /// Represents a built-in expression
    /// Example: `let x = builtin f arg;`
    ExpressionBuiltin {
        b: WithMetaData<String>,
        targs: Option<WithMetaData<NodeContractTypeArguments>>,
        xs: WithMetaData<NodeBuiltinArguments>,
    },
    /// Represents a message
    /// Example: `msg = { _tag : "tag", _recipient : "0x123", _amount : "0", param : "value" };`
    Message(Vec<WithMetaData<NodeMessageEntry>>),
    /// Represents a match expression
    /// Example: `match x with | Nil => "nil" | Cons a b => "cons" end`
    Match {
        match_expression: WithMetaData<NodeVariableIdentifier>,
        clauses: Vec<WithMetaData<NodePatternMatchExpressionClause>>,
    },
    /// Represents a constructor call
    /// Example: `let x = CustomType arg;`
    ConstructorCall {
        identifier_name: WithMetaData<NodeMetaIdentifier>,
        contract_type_arguments: Option<WithMetaData<NodeContractTypeArguments>>,
        argument_list: Vec<WithMetaData<NodeVariableIdentifier>>,
    },
    /// Represents a template function
    /// Example: `let x = tfun 'A => fun (arg : 'A) => arg;`
    TemplateFunction {
        identifier_name: WithMetaData<String>,
        expression: Box<WithMetaData<NodeFullExpression>>,
    },
    /// Represents a type application
    /// Example: `let x = @CustomType arg;`
    TApp {
        identifier_name: WithMetaData<NodeVariableIdentifier>,
        type_arguments: Vec<WithMetaData<NodeTypeArgument>>,
    },
}

/// NodeMessageEntry represents a message entry node in the AST
/// It can either be a MessageLiteral or a MessageVariable
/// Example: `msg = { _tag : "tag", _recipient : "0x123", _amount : "0", param : "value" };`
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeMessageEntry {
    /// Represents a message literal
    /// Example: `msg = { _tag : "tag", _recipient : "0x123", _amount : "0", param : "value" };`
    MessageLiteral(
        WithMetaData<NodeVariableIdentifier>,
        WithMetaData<NodeValueLiteral>,
    ),
    /// Represents a message variable
    /// Example: `msg = { _tag : "tag", _recipient : "0x123", _amount : "0", param : variable };`
    MessageVariable(
        WithMetaData<NodeVariableIdentifier>,
        WithMetaData<NodeVariableIdentifier>,
    ),
}

/// NodePatternMatchExpressionClause represents a pattern match expression clause node in the AST
/// It contains a pattern and an expression
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodePatternMatchExpressionClause {
    /// The pattern of the clause
    pub pattern: WithMetaData<NodePattern>,
    /// The expression of the clause
    pub expression: WithMetaData<NodeFullExpression>,
}

/// NodeAtomicExpression represents an atomic expression node in the AST
/// It can either be an AtomicSid or an AtomicLit
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeAtomicExpression {
    /// Represents an atomic sid
    /// Example: `let x = sid;`
    AtomicSid(WithMetaData<NodeVariableIdentifier>),
    /// Represents an atomic literal
    /// Example: `let x = "literal";`
    AtomicLit(WithMetaData<NodeValueLiteral>),
}

/// NodeContractTypeArguments represents a contract type arguments node in the AST
/// It contains a vector of type arguments
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeContractTypeArguments {
    /// The type arguments of the contract
    pub type_arguments: Vec<WithMetaData<NodeTypeArgument>>,
}

/// NodeValueLiteral represents a value literal node in the AST
/// It can either be a LiteralInt, LiteralHex, LiteralString or LiteralEmptyMap
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeValueLiteral {
    /// Represents a literal integer
    /// Example: `let x = 10;`
    LiteralInt(WithMetaData<NodeTypeNameIdentifier>, WithMetaData<String>),
    /// Represents a literal hexadecimal
    /// Example: `let x = 0x123;`
    LiteralHex(WithMetaData<String>),
    /// Represents a literal string
    /// Example: `let x = "string";`
    LiteralString(WithMetaData<String>),
    /// Represents a literal empty map
    /// Example: `let x: Map (KeyType, ValueType) = Emp;`
    LiteralEmptyMap(WithMetaData<NodeTypeMapKey>, WithMetaData<NodeTypeMapValue>),
}

/// NodeMapAccess represents a map access node in the AST
/// It contains an identifier name
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeMapAccess {
    /// The identifier name of the map access
    pub identifier_name: WithMetaData<NodeVariableIdentifier>,
}

/// NodePattern represents a pattern node in the AST
/// It can either be a Wildcard, Binder or Constructor
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodePattern {
    /// Represents a wildcard pattern
    /// Example: `match x with | _ => "wildcard" end`
    Wildcard,
    /// Represents a binder pattern
    /// Example: `match x with | a => "binder" end`
    Binder(WithMetaData<String>),
    /// Represents a constructor pattern
    /// Example: `match x with | Cons a b => "constructor" end`
    Constructor(
        WithMetaData<NodeMetaIdentifier>,
        Vec<WithMetaData<NodeArgumentPattern>>,
    ),
}

/// NodeArgumentPattern represents an argument pattern node in the AST
/// It can either be a WildcardArgument, BinderArgument, ConstructorArgument or PatternArgument
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeArgumentPattern {
    /// Represents a wildcard argument
    /// Example: `match x with | Cons _ _ => "wildcard argument" end`
    WildcardArgument,
    /// Represents a binder argument
    /// Example: `match x with | Cons a _ => "binder argument" end`
    BinderArgument(WithMetaData<String>),
    /// Represents a constructor argument
    /// Example: `match x with | Cons (Cons a b) _ => "constructor argument" end`
    ConstructorArgument(WithMetaData<NodeMetaIdentifier>),
    /// Represents a pattern argument
    /// Example: `match x with | Cons (Cons a _) _ => "pattern argument" end`
    PatternArgument(Box<WithMetaData<NodePattern>>),
}

/// NodePatternMatchClause represents a pattern match clause node in the AST
/// It contains a pattern expression and an optional statement block
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodePatternMatchClause {
    /// The pattern expression of the clause
    pub pattern_expression: Box<WithMetaData<NodePattern>>,
    /// The statement block of the clause
    pub statement_block: Option<WithMetaData<NodeStatementBlock>>,
}

/// NodeBlockchainFetchArguments represents a blockchain fetch arguments node in the AST
/// It contains a vector of arguments
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeBlockchainFetchArguments {
    /// The arguments of the blockchain fetch
    pub arguments: Vec<WithMetaData<NodeVariableIdentifier>>,
}

/// NodeStatement represents a statement node in the AST
/// It can be one of many different types of statements
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeStatement {
    /// Represents a load statement
    /// Example: `load x;`
    Load {
        left_hand_side: WithMetaData<String>,
        right_hand_side: WithMetaData<NodeVariableIdentifier>,
    },
    /// Represents a remote fetch statement
    /// Example: `fetch x from remote;`
    RemoteFetch(Box<NodeRemoteFetchStatement>),
    /// Represents a store statement
    /// Example: `store x;`
    Store {
        left_hand_side: WithMetaData<String>,
        right_hand_side: WithMetaData<NodeVariableIdentifier>,
    },
    /// Represents a bind statement
    /// Example: `bind x = y;`
    Bind {
        left_hand_side: WithMetaData<String>,
        right_hand_side: Box<WithMetaData<NodeFullExpression>>,
    },
    /// Represents a read from blockchain statement
    /// Example: `read x from bc;`
    ReadFromBC {
        left_hand_side: WithMetaData<String>,
        type_name: WithMetaData<NodeTypeNameIdentifier>,
        arguments: Option<NodeBlockchainFetchArguments>,
    },
    /// Represents a map get statement
    /// Example: `get x from map;`
    MapGet {
        left_hand_side: WithMetaData<String>,
        keys: Vec<WithMetaData<NodeMapAccess>>,
        right_hand_side: WithMetaData<String>,
    },
    /// Represents a map get exists statement
    /// Example: `get x from map if exists;`
    MapGetExists {
        left_hand_side: WithMetaData<String>,
        keys: Vec<WithMetaData<NodeMapAccess>>,
        right_hand_side: WithMetaData<String>,
    },
    /// Represents a map update statement
    /// Example: `update x in map;`
    MapUpdate {
        left_hand_side: WithMetaData<String>,
        keys: Vec<WithMetaData<NodeMapAccess>>,
        right_hand_side: WithMetaData<NodeVariableIdentifier>,
    },
    /// Represents a map update delete statement
    /// Example: `delete x from map;`
    MapUpdateDelete {
        left_hand_side: WithMetaData<String>,
        keys: Vec<WithMetaData<NodeMapAccess>>,
    },
    /// Represents an accept statement
    /// Example: `accept;`
    Accept,
    /// Represents a send statement
    /// Example: `send x;`
    Send {
        identifier_name: WithMetaData<NodeVariableIdentifier>,
    },
    /// Represents a create event statement
    /// Example: `create event x;`
    CreateEvnt {
        identifier_name: WithMetaData<NodeVariableIdentifier>,
    },
    /// Represents a throw statement
    /// Example: `throw x;`
    Throw {
        error_variable: Option<WithMetaData<NodeVariableIdentifier>>,
    },
    /// Represents a match statement
    /// Example: `match x with | Nil => "nil" | Cons a b => "cons" end`
    MatchStmt {
        variable: WithMetaData<NodeVariableIdentifier>,
        clauses: Vec<WithMetaData<NodePatternMatchClause>>,
    },
    /// Represents a call procedure statement
    /// Example: `call proc x;`
    CallProc {
        component_id: WithMetaData<NodeComponentId>,
        arguments: Vec<WithMetaData<NodeVariableIdentifier>>,
    },
    /// Represents an iterate statement
    /// Example: `iterate x over y;`
    Iterate {
        identifier_name: WithMetaData<NodeVariableIdentifier>,
        component_id: WithMetaData<NodeComponentId>,
    },
}

/// NodeRemoteFetchStatement represents a remote fetch statement node in the AST
/// It can be one of many different types of remote fetch statements
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeRemoteFetchStatement {
    /// Represents a read state mutable statement
    /// Example: `read x from state;`
    ReadStateMutable(
        WithMetaData<String>,
        WithMetaData<String>,
        WithMetaData<NodeVariableIdentifier>,
    ),
    /// Represents a read state mutable special id statement
    /// Example: `read x from state with id;`
    ReadStateMutableSpecialId(
        WithMetaData<String>,
        WithMetaData<String>,
        WithMetaData<String>,
    ),
    /// Represents a read state mutable map access statement
    /// Example: `read x from state with map access;`
    ReadStateMutableMapAccess(
        WithMetaData<String>,
        WithMetaData<String>,
        WithMetaData<String>,
        Vec<WithMetaData<NodeMapAccess>>,
    ),
    /// Represents a read state mutable map access exists statement
    /// Example: `read x from state with map access if exists;`
    ReadStateMutableMapAccessExists(
        WithMetaData<String>,
        WithMetaData<String>,
        WithMetaData<String>,
        Vec<WithMetaData<NodeMapAccess>>,
    ),
    /// Represents a read state mutable cast address statement
    /// Example: `read x from state with cast address;`
    ReadStateMutableCastAddress(
        WithMetaData<String>,
        WithMetaData<NodeVariableIdentifier>,
        WithMetaData<NodeAddressType>,
    ),
}

/// NodeComponentId represents a component id node in the AST
/// It can either be a WithTypeLikeName or a WithRegularId
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeComponentId {
    /// Represents a component id with a type like name
    /// Example: `component WithTypeLikeName;`
    WithTypeLikeName(WithMetaData<NodeTypeNameIdentifier>),
    /// Represents a component id with a regular id
    /// Example: `component WithRegularId;`
    WithRegularId(WithMetaData<String>),
}

/// NodeComponentParameters represents a component parameters node in the AST
/// It contains a vector of parameters
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeComponentParameters {
    /// The parameters of the component
    pub parameters: Vec<WithMetaData<NodeParameterPair>>,
}

/// NodeParameterPair represents a parameter pair node in the AST
/// It contains an identifier with type
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeParameterPair {
    /// The identifier with type of the parameter pair
    pub identifier_with_type: WithMetaData<NodeTypedIdentifier>,
}

/// NodeComponentBody represents a component body node in the AST
/// It contains an optional statement block
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeComponentBody {
    /// The statement block of the component body
    pub statement_block: Option<WithMetaData<NodeStatementBlock>>,
}

/// NodeStatementBlock represents a statement block node in the AST
/// It contains a vector of statements
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeStatementBlock {
    /// The statements of the statement block
    pub statements: Vec<NodeStatement>,
}

/// NodeTypedIdentifier represents a typed identifier node in the AST
/// It contains an identifier name and an annotation
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeTypedIdentifier {
    /// The identifier name of the typed identifier
    pub identifier_name: WithMetaData<String>,
    /// The annotation of the typed identifier
    pub annotation: WithMetaData<NodeTypeAnnotation>,
}

/// NodeTypeAnnotation represents a type annotation node in the AST
/// It contains a type name
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeTypeAnnotation {
    /// The type name of the type annotation
    pub type_name: WithMetaData<NodeScillaType>,
}

/// NodeProgram represents a program node in the AST
/// It contains a version, optional import declarations, optional library definition and a contract definition
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeProgram {
    /// The version of the program
    pub version: WithMetaData<String>,
    /// The import declarations of the program
    pub import_declarations: Option<WithMetaData<NodeImportDeclarations>>,
    /// The library definition of the program
    pub library_definition: Option<WithMetaData<NodeLibraryDefinition>>,
    /// The contract definition of the program
    pub contract_definition: WithMetaData<NodeContractDefinition>,
}

/// NodeLibraryDefinition represents a library definition node in the AST
/// It contains a name and a vector of definitions
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeLibraryDefinition {
    /// The name of the library definition
    pub name: WithMetaData<NodeTypeNameIdentifier>,
    /// The definitions of the library definition
    pub definitions: Vec<WithMetaData<NodeLibrarySingleDefinition>>,
}

/// NodeLibrarySingleDefinition represents a library single definition node in the AST
/// It can either be a LetDefinition or a TypeDefinition
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeLibrarySingleDefinition {
    /// Represents a let definition
    /// Example: `let x = y;`
    LetDefinition {
        variable_name: WithMetaData<String>,
        type_annotation: Option<WithMetaData<NodeTypeAnnotation>>,
        expression: WithMetaData<NodeFullExpression>,
    },
    /// Represents a type definition
    /// Example: `type x = y;`
    TypeDefinition(
        // TODO: Enum definition
        WithMetaData<NodeTypeNameIdentifier>,
        Option<Vec<WithMetaData<NodeTypeAlternativeClause>>>,
    ),
}

/// NodeContractDefinition represents a contract definition node in the AST
/// It contains a contract name, parameters, optional constraint, fields and components
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeContractDefinition {
    /// The contract name of the contract definition
    pub contract_name: WithMetaData<NodeTypeNameIdentifier>,
    /// The parameters of the contract definition
    pub parameters: WithMetaData<NodeComponentParameters>,
    /// The constraint of the contract definition
    pub constraint: Option<WithMetaData<NodeWithConstraint>>,
    /// The fields of the contract definition
    pub fields: Vec<WithMetaData<NodeContractField>>,
    /// The components of the contract definition
    pub components: Vec<WithMetaData<NodeComponentDefinition>>,
}

/// NodeContractField represents a contract field node in the AST
/// It contains a typed identifier and a right hand side
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeContractField {
    /// The typed identifier of the contract field
    pub typed_identifier: WithMetaData<NodeTypedIdentifier>,
    /// The right hand side of the contract field
    pub right_hand_side: WithMetaData<NodeFullExpression>,
}

/// NodeWithConstraint represents a with constraint node in the AST
/// It contains an expression
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeWithConstraint {
    /// The expression of the with constraint
    pub expression: Box<WithMetaData<NodeFullExpression>>,
}

/// NodeComponentDefinition represents a component definition node in the AST
/// It can either be a TransitionComponent or a ProcedureComponent
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeComponentDefinition {
    /// Represents a transition component
    /// Example: `transition x;`
    TransitionComponent(Box<WithMetaData<NodeTransitionDefinition>>),
    /// Represents a procedure component
    /// Example: `procedure x;`
    ProcedureComponent(Box<WithMetaData<NodeProcedureDefinition>>),
}

/// NodeProcedureDefinition represents a procedure definition node in the AST
/// It contains a name, parameters and a body
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeProcedureDefinition {
    /// The name of the procedure definition
    pub name: WithMetaData<NodeComponentId>,
    /// The parameters of the procedure definition
    pub parameters: WithMetaData<NodeComponentParameters>,
    /// The body of the procedure definition
    pub body: WithMetaData<NodeComponentBody>,
}

/// NodeTransitionDefinition represents a transition definition node in the AST
/// It contains a name, parameters and a body
/// Example: `transition Transfer (from: ByStr20, to: ByStr20, amount: Uint128) = ...`
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeTransitionDefinition {
    /// The name of the transition definition
    pub name: WithMetaData<NodeComponentId>,
    /// The parameters of the transition definition
    pub parameters: WithMetaData<NodeComponentParameters>,
    /// The body of the transition definition
    pub body: WithMetaData<NodeComponentBody>,
}

/// NodeTypeAlternativeClause represents an alternative clause node in the AST
/// It can either be a ClauseType or a ClauseTypeWithArgs
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeTypeAlternativeClause {
    /// Represents a clause type
    /// Example: `match x with | ClauseType => ...`
    ClauseType(WithMetaData<NodeTypeNameIdentifier>),
    /// Represents a clause type with arguments
    /// Example: `match x with | ClauseType arg1 arg2 => ...`
    ClauseTypeWithArgs(
        WithMetaData<NodeTypeNameIdentifier>,
        Vec<WithMetaData<NodeTypeArgument>>,
    ),
}

/// NodeTypeMapValueArguments represents map value arguments node in the AST
/// It can either be an EnclosedTypeMapValue, a GenericMapValueArgument or a MapKeyValueType
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeTypeMapValueArguments {
    /// Represents an enclosed type map value
    /// Example: `let x: Map ((KeyType), ValueType) = Emp;`
    EnclosedTypeMapValue(Box<WithMetaData<NodeTypeMapValueAllowingTypeArguments>>),
    /// Represents a generic map value argument
    /// Example: `let x: Map (KeyType, ValueType) = Emp;`
    GenericMapValueArgument(WithMetaData<NodeMetaIdentifier>),
    /// Represents a map key value type
    /// Example: `let x: Map ((ByStr20), ValueType) = Emp;`
    MapKeyValueType(WithMetaData<NodeTypeMapKey>, WithMetaData<NodeTypeMapValue>),
}

/// NodeTypeMapValueAllowingTypeArguments represents a map value allowing type arguments node in the AST
/// It can either be a TypeMapValueNoArgs or a TypeMapValueWithArgs
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeTypeMapValueAllowingTypeArguments {
    /// Represents a type map value with no arguments
    /// Example: `let x: Map (KeyType, ValueType) = Emp;`
    TypeMapValueNoArgs(WithMetaData<NodeTypeMapValue>),
    /// Represents a type map value with arguments
    /// Example: `let x: Map ((KeyType), ValueType) = Emp;`
    TypeMapValueWithArgs(
        WithMetaData<NodeMetaIdentifier>,
        Vec<WithMetaData<NodeTypeMapValueArguments>>,
    ),
}
