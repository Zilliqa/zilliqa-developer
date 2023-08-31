use crate::parser::lexer::SourcePosition;
use std::fmt;

/*
Things that need renaming:
AtomicSid -> ????
EventType -> AutoType ;; Essentially a JSON dict
*/

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct WithMetaData<T> {
    pub node: T,
    pub start: SourcePosition,
    pub end: SourcePosition,
}

impl<T: fmt::Display> fmt::Display for WithMetaData<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.node.to_string())
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeByteStr {
    Constant(WithMetaData<String>), // TODO: Aparently not used anywhere
    Type(WithMetaData<String>),
}

impl NodeByteStr {
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

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeTypeNameIdentifier {
    ByteStringType(WithMetaData<NodeByteStr>),
    EventType,
    TypeOrEnumLikeIdentifier(WithMetaData<String>),
}

impl NodeTypeNameIdentifier {
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

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeImportedName {
    RegularImport(WithMetaData<NodeTypeNameIdentifier>),
    AliasedImport(
        WithMetaData<NodeTypeNameIdentifier>,
        WithMetaData<NodeTypeNameIdentifier>,
    ),
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeImportDeclarations {
    pub import_list: Vec<WithMetaData<NodeImportedName>>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeMetaIdentifier {
    MetaName(WithMetaData<NodeTypeNameIdentifier>),
    MetaNameInNamespace(
        WithMetaData<NodeTypeNameIdentifier>,
        WithMetaData<NodeTypeNameIdentifier>,
    ),
    MetaNameInHexspace(WithMetaData<String>, WithMetaData<NodeTypeNameIdentifier>),
    ByteString,
}

impl NodeMetaIdentifier {
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

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeVariableIdentifier {
    VariableName(WithMetaData<String>),
    SpecialIdentifier(WithMetaData<String>),
    VariableInNamespace(WithMetaData<NodeTypeNameIdentifier>, WithMetaData<String>),
}

impl NodeVariableIdentifier {
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

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeBuiltinArguments {
    pub arguments: Vec<WithMetaData<NodeVariableIdentifier>>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeTypeMapKey {
    GenericMapKey(WithMetaData<NodeMetaIdentifier>),
    EnclosedGenericId(WithMetaData<NodeMetaIdentifier>),
    EnclosedAddressMapKeyType(WithMetaData<NodeAddressType>),
    AddressMapKeyType(WithMetaData<NodeAddressType>),
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeTypeMapValue {
    MapValueTypeOrEnumLikeIdentifier(WithMetaData<NodeMetaIdentifier>),
    MapKeyValue(Box<NodeTypeMapEntry>),
    MapValueParanthesizedType(Box<NodeTypeMapValueAllowingTypeArguments>),
    MapValueAddressType(Box<WithMetaData<NodeAddressType>>),
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeTypeArgument {
    EnclosedTypeArgument(Box<WithMetaData<NodeScillaType>>),
    GenericTypeArgument(WithMetaData<NodeMetaIdentifier>),
    TemplateTypeArgument(WithMetaData<String>),
    AddressTypeArgument(WithMetaData<NodeAddressType>),
    MapTypeArgument(WithMetaData<NodeTypeMapKey>, WithMetaData<NodeTypeMapValue>),
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeScillaType {
    GenericTypeWithArgs(WithMetaData<NodeMetaIdentifier>, Vec<NodeTypeArgument>),
    MapType(WithMetaData<NodeTypeMapKey>, WithMetaData<NodeTypeMapValue>),
    FunctionType(
        Box<WithMetaData<NodeScillaType>>,
        Box<WithMetaData<NodeScillaType>>,
    ),
    EnclosedType(Box<WithMetaData<NodeScillaType>>),
    ScillaAddresseType(Box<WithMetaData<NodeAddressType>>),
    PolyFunctionType(WithMetaData<String>, Box<WithMetaData<NodeScillaType>>),
    TypeVarType(WithMetaData<String>),
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeTypeMapEntry {
    pub key: WithMetaData<NodeTypeMapKey>,
    pub value: WithMetaData<NodeTypeMapValue>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeAddressTypeField {
    pub identifier: WithMetaData<NodeVariableIdentifier>,
    pub type_name: WithMetaData<NodeScillaType>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeAddressType {
    pub identifier: WithMetaData<NodeTypeNameIdentifier>,
    pub type_name: WithMetaData<String>,
    pub address_fields: Vec<WithMetaData<NodeAddressTypeField>>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeFullExpression {
    LocalVariableDeclaration {
        identifier_name: WithMetaData<String>,
        expression: Box<WithMetaData<NodeFullExpression>>,
        type_annotation: Option<NodeTypeAnnotation>,
        containing_expression: Box<WithMetaData<NodeFullExpression>>,
    },
    FunctionDeclaration {
        identier_value: WithMetaData<String>,
        type_annotation: NodeTypeAnnotation,
        expression: Box<WithMetaData<NodeFullExpression>>,
    },
    FunctionCall {
        function_name: WithMetaData<NodeVariableIdentifier>,
        argument_list: Vec<WithMetaData<NodeVariableIdentifier>>,
    },
    ExpressionAtomic(Box<NodeAtomicExpression>),
    ExpressionBuiltin {
        b: WithMetaData<String>,
        targs: Option<NodeContractTypeArguments>,
        xs: NodeBuiltinArguments,
    },
    Message(Vec<NodeMessageEntry>),
    Match {
        match_expression: WithMetaData<NodeVariableIdentifier>,
        clauses: Vec<NodePatternMatchExpressionClause>,
    },
    ConstructorCall {
        identifier_name: WithMetaData<NodeMetaIdentifier>,
        contract_type_arguments: Option<NodeContractTypeArguments>,
        argument_list: Vec<WithMetaData<NodeVariableIdentifier>>,
    },
    TemplateFunction {
        identifier_name: WithMetaData<String>,
        expression: Box<WithMetaData<NodeFullExpression>>,
    },
    TApp {
        identifier_name: WithMetaData<NodeVariableIdentifier>,
        type_arguments: Vec<NodeTypeArgument>,
    },
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeMessageEntry {
    MessageLiteral(WithMetaData<NodeVariableIdentifier>, NodeValueLiteral),
    MessageVariable(
        WithMetaData<NodeVariableIdentifier>,
        WithMetaData<NodeVariableIdentifier>,
    ),
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodePatternMatchExpressionClause {
    pub pattern: NodePattern,
    pub expression: WithMetaData<NodeFullExpression>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeAtomicExpression {
    AtomicSid(WithMetaData<NodeVariableIdentifier>),
    AtomicLit(NodeValueLiteral),
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeContractTypeArguments {
    pub type_arguments: Vec<NodeTypeArgument>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeValueLiteral {
    LiteralInt(WithMetaData<NodeTypeNameIdentifier>, WithMetaData<String>),
    LiteralHex(WithMetaData<String>),
    LiteralString(WithMetaData<String>),
    LiteralEmptyMap(WithMetaData<NodeTypeMapKey>, WithMetaData<NodeTypeMapValue>),
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeMapAccess {
    pub identifier_name: WithMetaData<NodeVariableIdentifier>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodePattern {
    Wildcard,
    Binder(WithMetaData<String>),
    Constructor(WithMetaData<NodeMetaIdentifier>, Vec<NodeArgumentPattern>),
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeArgumentPattern {
    WildcardArgument,
    BinderArgument(WithMetaData<String>),
    ConstructorArgument(WithMetaData<NodeMetaIdentifier>),
    PatternArgument(Box<NodePattern>),
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodePatternMatchClause {
    pub pattern_expression: Box<NodePattern>,
    pub statement_block: Option<NodeStatementBlock>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeBlockchainFetchArguments {
    pub arguments: Vec<WithMetaData<NodeVariableIdentifier>>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeStatement {
    Load {
        left_hand_side: WithMetaData<String>,
        right_hand_side: WithMetaData<NodeVariableIdentifier>,
    },
    RemoteFetch(Box<NodeRemoteFetchStatement>),
    Store {
        left_hand_side: WithMetaData<String>,
        right_hand_side: WithMetaData<NodeVariableIdentifier>,
    },
    Bind {
        left_hand_side: WithMetaData<String>,
        right_hand_side: Box<WithMetaData<NodeFullExpression>>,
    },
    ReadFromBC {
        left_hand_side: WithMetaData<String>,
        type_name: WithMetaData<NodeTypeNameIdentifier>,
        arguments: Option<NodeBlockchainFetchArguments>,
    },
    MapGet {
        left_hand_side: WithMetaData<String>,
        keys: Vec<NodeMapAccess>,
        right_hand_side: WithMetaData<String>,
    },
    MapGetExists {
        left_hand_side: WithMetaData<String>,
        keys: Vec<NodeMapAccess>,
        right_hand_side: WithMetaData<String>,
    },
    MapUpdate {
        left_hand_side: WithMetaData<String>,
        keys: Vec<NodeMapAccess>,
        right_hand_side: WithMetaData<NodeVariableIdentifier>,
    },
    MapUpdateDelete {
        left_hand_side: WithMetaData<String>,
        keys: Vec<NodeMapAccess>,
    },
    Accept,
    Send {
        identifier_name: WithMetaData<NodeVariableIdentifier>,
    },
    CreateEvnt {
        identifier_name: WithMetaData<NodeVariableIdentifier>,
    },
    Throw {
        error_variable: Option<WithMetaData<NodeVariableIdentifier>>,
    },
    MatchStmt {
        variable: WithMetaData<NodeVariableIdentifier>,
        clauses: Vec<NodePatternMatchClause>,
    },
    CallProc {
        component_id: NodeComponentId,
        arguments: Vec<WithMetaData<NodeVariableIdentifier>>,
    },
    Iterate {
        identifier_name: WithMetaData<NodeVariableIdentifier>,
        component_id: NodeComponentId,
    },
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeRemoteFetchStatement {
    ReadStateMutable(
        WithMetaData<String>,
        WithMetaData<String>,
        WithMetaData<NodeVariableIdentifier>,
    ),
    ReadStateMutableSpecialId(
        WithMetaData<String>,
        WithMetaData<String>,
        WithMetaData<String>,
    ),
    ReadStateMutableMapAccess(
        WithMetaData<String>,
        WithMetaData<String>,
        WithMetaData<String>,
        Vec<NodeMapAccess>,
    ),
    ReadStateMutableMapAccessExists(
        WithMetaData<String>,
        WithMetaData<String>,
        WithMetaData<String>,
        Vec<NodeMapAccess>,
    ),
    ReadStateMutableCastAddress(
        WithMetaData<String>,
        WithMetaData<NodeVariableIdentifier>,
        WithMetaData<NodeAddressType>,
    ),
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeComponentId {
    WithTypeLikeName(WithMetaData<NodeTypeNameIdentifier>),
    WithRegularId(WithMetaData<String>),
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeComponentParameters {
    pub parameters: Vec<NodeParameterPair>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeParameterPair {
    pub identifier_with_type: NodeTypedIdentifier,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeComponentBody {
    pub statement_block: Option<NodeStatementBlock>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeStatementBlock {
    pub statements: Vec<NodeStatement>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeTypedIdentifier {
    pub identifier_name: WithMetaData<String>,
    pub annotation: NodeTypeAnnotation,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeTypeAnnotation {
    pub type_name: WithMetaData<NodeScillaType>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeProgram {
    pub version: WithMetaData<String>,
    pub import_declarations: Option<NodeImportDeclarations>,
    pub library_definition: Option<NodeLibraryDefinition>,
    pub contract_definition: NodeContractDefinition,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeLibraryDefinition {
    pub name: WithMetaData<NodeTypeNameIdentifier>,
    pub definitions: Vec<NodeLibrarySingleDefinition>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeLibrarySingleDefinition {
    LetDefinition {
        variable_name: WithMetaData<String>,
        type_annotation: Option<NodeTypeAnnotation>,
        expression: WithMetaData<NodeFullExpression>,
    },
    TypeDefinition(
        // TODO: Enum definition
        WithMetaData<NodeTypeNameIdentifier>,
        Option<Vec<NodeTypeAlternativeClause>>,
    ),
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeContractDefinition {
    pub contract_name: WithMetaData<NodeTypeNameIdentifier>,
    pub parameters: NodeComponentParameters,
    pub constraint: Option<NodeWithConstraint>,
    pub fields: Vec<NodeContractField>,
    pub components: Vec<NodeComponentDefinition>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeContractField {
    pub typed_identifier: NodeTypedIdentifier,
    pub right_hand_side: WithMetaData<NodeFullExpression>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeWithConstraint {
    pub expression: Box<WithMetaData<NodeFullExpression>>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeComponentDefinition {
    TransitionComponent(Box<NodeTransitionDefinition>),
    ProcedureComponent(Box<NodeProcedureDefinition>),
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeProcedureDefinition {
    pub name: NodeComponentId,
    pub parameters: NodeComponentParameters,
    pub body: NodeComponentBody,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeTransitionDefinition {
    pub name: NodeComponentId,
    pub parameters: NodeComponentParameters,
    pub body: NodeComponentBody,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeTypeAlternativeClause {
    ClauseType(WithMetaData<NodeTypeNameIdentifier>),
    ClauseTypeWithArgs(WithMetaData<NodeTypeNameIdentifier>, Vec<NodeTypeArgument>),
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeTypeMapValueArguments {
    EnclosedTypeMapValue(Box<NodeTypeMapValueAllowingTypeArguments>),
    GenericMapValueArgument(WithMetaData<NodeMetaIdentifier>),
    MapKeyValueType(WithMetaData<NodeTypeMapKey>, WithMetaData<NodeTypeMapValue>),
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeTypeMapValueAllowingTypeArguments {
    TypeMapValueNoArgs(WithMetaData<NodeTypeMapValue>),
    TypeMapValueWithArgs(
        WithMetaData<NodeMetaIdentifier>,
        Vec<NodeTypeMapValueArguments>,
    ),
}
