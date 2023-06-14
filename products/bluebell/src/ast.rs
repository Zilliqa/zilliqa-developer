use crate::type_classes::TypeAnnotation;
use std::fmt;

pub enum NodeAny<'a> {
    NodeByteStr(&'a NodeByteStr),
    NodeTypeNameIdentifier(&'a NodeTypeNameIdentifier),
    NodeImportedName(&'a NodeImportedName),
    NodeImportDeclarations(&'a NodeImportDeclarations),
    NodeMetaIdentifier(&'a NodeMetaIdentifier),
    NodeVariableIdentifier(&'a NodeVariableIdentifier),
    NodeBuiltinArguments(&'a NodeBuiltinArguments),
    NodeTypeMapKey(&'a NodeTypeMapKey),
    NodeTypeMapValue(&'a NodeTypeMapValue),
    NodeTypeArgument(&'a NodeTypeArgument),
    NodeScillaType(&'a NodeScillaType),
    NodeTypeMapEntry(&'a NodeTypeMapEntry),
    NodeAddressTypeField(&'a NodeAddressTypeField),
    NodeAddressType(&'a NodeAddressType),
    NodeFullExpression(&'a NodeFullExpression),
    NodeMessageEntry(&'a NodeMessageEntry),
    NodePatternMatchExpressionClause(&'a NodePatternMatchExpressionClause),
    NodeAtomicExpression(&'a NodeAtomicExpression),
    NodeContractTypeArguments(&'a NodeContractTypeArguments),
    NodeValueLiteral(&'a NodeValueLiteral),
    NodeMapAccess(&'a NodeMapAccess),
    NodePattern(&'a NodePattern),
    NodeArgumentPattern(&'a NodeArgumentPattern),
    NodePatternMatchClause(&'a NodePatternMatchClause),
    NodeBlockchainFetchArguments(&'a NodeBlockchainFetchArguments),
    NodeStatement(&'a NodeStatement),
    NodeRemoteFetchStatement(&'a NodeRemoteFetchStatement),
    NodeComponentId(&'a NodeComponentId),
    NodeComponentParameters(&'a NodeComponentParameters),
    NodeParameterPair(&'a NodeParameterPair),
    NodeComponentBody(&'a NodeComponentBody),
    NodeStatementBlock(&'a NodeStatementBlock),
    NodeTypedIdentifier(&'a NodeTypedIdentifier),
    NodeTypeAnnotation(&'a NodeTypeAnnotation),
    NodeProgram(&'a NodeProgram),
    NodeLibraryDefinition(&'a NodeLibraryDefinition),
    NodeLibrarySingleDefinition(&'a NodeLibrarySingleDefinition),
    NodeContractDefinition(&'a NodeContractDefinition),
    NodeContractField(&'a NodeContractField),
    NodeWithConstraint(&'a NodeWithConstraint),
    NodeComponentDefinition(&'a NodeComponentDefinition),
    NodeProcedureDefinition(&'a NodeProcedureDefinition),
    NodeTransitionDefinition(&'a NodeTransitionDefinition),
    NodeTypeAlternativeClause(&'a NodeTypeAlternativeClause),
    NodeTypeMapValueArguments(&'a NodeTypeMapValueArguments),
    NodeTypeMapValueAllowingTypeArguments(&'a NodeTypeMapValueAllowingTypeArguments),
}

pub trait AnyKind {
    fn to_any<'a>(&'a self) -> NodeAny<'a>;
}

macro_rules! impl_any_kind {
    ($type_name:ident) => {
        impl AnyKind for $type_name {
            fn to_any<'a>(&'a self) -> NodeAny<'a> {
                NodeAny::$type_name(self)
            }
        }
    };
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeByteStr {
    Constant(String),
    Type(String),
}

impl_any_kind!(NodeByteStr);

impl NodeByteStr {
    pub fn to_string(&self) -> String {
        match self {
            NodeByteStr::Constant(s) => s.clone(),
            NodeByteStr::Type(t) => t.clone(),
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
    ByteStringType(NodeByteStr),
    EventType,
    CustomType(String),
}

impl_any_kind!(NodeTypeNameIdentifier);

impl NodeTypeNameIdentifier {
    pub fn to_string(&self) -> String {
        match self {
            NodeTypeNameIdentifier::ByteStringType(byte_str) => {
                format!("{}", byte_str.to_string())
            }
            NodeTypeNameIdentifier::EventType => format!("Event"),
            NodeTypeNameIdentifier::CustomType(custom_type) => {
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
    RegularImport(NodeTypeNameIdentifier),
    AliasedImport(NodeTypeNameIdentifier, NodeTypeNameIdentifier),
}
impl_any_kind!(NodeImportedName);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeImportDeclarations {
    pub import_list: Vec<NodeImportedName>,
    pub type_annotation: Option<TypeAnnotation>,
}
impl_any_kind!(NodeImportDeclarations);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeMetaIdentifier {
    MetaName(NodeTypeNameIdentifier),
    MetaNameInNamespace(NodeTypeNameIdentifier, NodeTypeNameIdentifier),
    MetaNameInHexspace(String, NodeTypeNameIdentifier),
    ByteString,
}
impl_any_kind!(NodeMetaIdentifier);

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
    VariableName(String),
    SpecialIdentifier(String),
    VariableInNamespace(NodeTypeNameIdentifier, String),
}
impl_any_kind!(NodeVariableIdentifier);

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
    pub arguments: Vec<NodeVariableIdentifier>,
    pub type_annotation: Option<TypeAnnotation>,
}
impl_any_kind!(NodeBuiltinArguments);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeTypeMapKey {
    GenericMapKey(NodeMetaIdentifier),
    EnclosedGenericId(NodeMetaIdentifier),
    EnclosedAddressMapKeyType(NodeAddressType),
    AddressMapKeyType(NodeAddressType),
}
impl_any_kind!(NodeTypeMapKey);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeTypeMapValue {
    MapValueCustomType(NodeMetaIdentifier),
    MapKeyValue(Box<NodeTypeMapEntry>),
    MapValueParanthesizedType(Box<NodeTypeMapValueAllowingTypeArguments>),
    MapValueAddressType(Box<NodeAddressType>),
}
impl_any_kind!(NodeTypeMapValue);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeTypeArgument {
    EnclosedTypeArgument(Box<NodeScillaType>),
    GenericTypeArgument(NodeMetaIdentifier),
    TemplateTypeArgument(String),
    AddressTypeArgument(NodeAddressType),
    MapTypeArgument(NodeTypeMapKey, NodeTypeMapValue),
}
impl_any_kind!(NodeTypeArgument);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeScillaType {
    GenericTypeWithArgs(NodeMetaIdentifier, Vec<NodeTypeArgument>),
    MapType(NodeTypeMapKey, NodeTypeMapValue),
    FunctionType(Box<NodeScillaType>, Box<NodeScillaType>),
    EnclosedType(Box<NodeScillaType>),
    ScillaAddresseType(Box<NodeAddressType>),
    PolyFunctionType(String, Box<NodeScillaType>),
    TypeVarType(String),
}
impl_any_kind!(NodeScillaType);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeTypeMapEntry {
    pub key: NodeTypeMapKey,
    pub value: NodeTypeMapValue,
    pub type_annotation: Option<TypeAnnotation>,
}
impl_any_kind!(NodeTypeMapEntry);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeAddressTypeField {
    pub identifier: NodeVariableIdentifier,
    pub type_name: NodeScillaType,
    pub type_annotation: Option<TypeAnnotation>,
}
impl_any_kind!(NodeAddressTypeField);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeAddressType {
    pub identifier: NodeTypeNameIdentifier,
    pub type_name: String,
    pub address_fields: Vec<NodeAddressTypeField>,
    pub type_annotation: Option<TypeAnnotation>,
}
impl_any_kind!(NodeAddressType);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeFullExpression {
    LocalVariableDeclaration {
        identifier_name: String,
        expression: Box<NodeFullExpression>,
        type_annotation: Option<NodeTypeAnnotation>,
        containing_expression: Box<NodeFullExpression>,
    },
    FunctionDeclaration {
        identier_value: String,
        type_annotation: NodeTypeAnnotation,
        expression: Box<NodeFullExpression>,
    },
    FunctionCall {
        function_name: NodeVariableIdentifier,
        argument_list: Vec<NodeVariableIdentifier>,
    },
    ExpressionAtomic(Box<NodeAtomicExpression>),
    ExpressionBuiltin {
        b: String,
        targs: Option<NodeContractTypeArguments>,
        xs: NodeBuiltinArguments,
    },
    Message(Vec<NodeMessageEntry>),
    Match {
        match_expression: NodeVariableIdentifier,
        clauses: Vec<NodePatternMatchExpressionClause>,
    },
    ConstructorCall {
        identifier_name: NodeMetaIdentifier,
        contract_type_arguments: Option<NodeContractTypeArguments>,
        argument_list: Vec<NodeVariableIdentifier>,
    },
    TemplateFunction {
        identifier_name: String,
        expression: Box<NodeFullExpression>,
    },
    TApp {
        identifier_name: NodeVariableIdentifier,
        type_arguments: Vec<NodeTypeArgument>,
    },
}
impl_any_kind!(NodeFullExpression);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeMessageEntry {
    MessageLiteral(NodeVariableIdentifier, NodeValueLiteral),
    MessageVariable(NodeVariableIdentifier, NodeVariableIdentifier),
}
impl_any_kind!(NodeMessageEntry);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodePatternMatchExpressionClause {
    pub pattern: NodePattern,
    pub expression: NodeFullExpression,
    pub type_annotation: Option<TypeAnnotation>,
}
impl_any_kind!(NodePatternMatchExpressionClause);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeAtomicExpression {
    AtomicSid(NodeVariableIdentifier),
    AtomicLit(NodeValueLiteral),
}
impl_any_kind!(NodeAtomicExpression);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeContractTypeArguments {
    pub type_arguments: Vec<NodeTypeArgument>,
    pub type_annotation: Option<TypeAnnotation>,
}
impl_any_kind!(NodeContractTypeArguments);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeValueLiteral {
    LiteralInt(NodeTypeNameIdentifier, String),
    LiteralHex(String),
    LiteralString(String),
    LiteralEmptyMap(NodeTypeMapKey, NodeTypeMapValue),
}
impl_any_kind!(NodeValueLiteral);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeMapAccess {
    pub identifier_name: NodeVariableIdentifier,
    pub type_annotation: Option<TypeAnnotation>,
}
impl_any_kind!(NodeMapAccess);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodePattern {
    Wildcard,
    Binder(String),
    Constructor(NodeMetaIdentifier, Vec<NodeArgumentPattern>),
}
impl_any_kind!(NodePattern);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeArgumentPattern {
    WildcardArgument,
    BinderArgument(String),
    ConstructorArgument(NodeMetaIdentifier),
    PatternArgument(Box<NodePattern>),
}
impl_any_kind!(NodeArgumentPattern);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodePatternMatchClause {
    pub pattern_expression: Box<NodePattern>,
    pub statement_block: Option<NodeStatementBlock>,
    pub type_annotation: Option<TypeAnnotation>,
}
impl_any_kind!(NodePatternMatchClause);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeBlockchainFetchArguments {
    pub arguments: Vec<NodeVariableIdentifier>,
    pub type_annotation: Option<TypeAnnotation>,
}
impl_any_kind!(NodeBlockchainFetchArguments);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeStatement {
    Load {
        left_hand_side: String,
        right_hand_side: NodeVariableIdentifier,
    },
    RemoteFetch(Box<NodeRemoteFetchStatement>),
    Store {
        left_hand_side: String,
        right_hand_side: NodeVariableIdentifier,
    },
    Bind {
        left_hand_side: String,
        right_hand_side: Box<NodeFullExpression>,
    },
    ReadFromBC {
        left_hand_side: String,
        type_name: NodeTypeNameIdentifier,
        arguments: Option<NodeBlockchainFetchArguments>,
    },
    MapGet {
        left_hand_side: String,
        keys: Vec<NodeMapAccess>,
        right_hand_side: String,
    },
    MapGetExists {
        left_hand_side: String,
        keys: Vec<NodeMapAccess>,
        right_hand_side: String,
    },
    MapUpdate {
        left_hand_side: String,
        keys: Vec<NodeMapAccess>,
        right_hand_side: NodeVariableIdentifier,
    },
    MapUpdateDelete {
        left_hand_side: String,
        keys: Vec<NodeMapAccess>,
    },
    Accept,
    Send {
        identifier_name: NodeVariableIdentifier,
    },
    CreateEvnt {
        identifier_name: NodeVariableIdentifier,
    },
    Throw {
        error_variable: Option<NodeVariableIdentifier>,
    },
    MatchStmt {
        variable: NodeVariableIdentifier,
        clauses: Vec<NodePatternMatchClause>,
    },
    CallProc {
        component_id: NodeComponentId,
        arguments: Vec<NodeVariableIdentifier>,
    },
    Iterate {
        identifier_name: NodeVariableIdentifier,
        component_id: NodeComponentId,
    },
}
impl_any_kind!(NodeStatement);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeRemoteFetchStatement {
    ReadStateMutable(String, String, NodeVariableIdentifier),
    ReadStateMutableSpecialId(String, String, String),
    ReadStateMutableMapAccess(String, String, String, Vec<NodeMapAccess>),
    ReadStateMutableMapAccessExists(String, String, String, Vec<NodeMapAccess>),
    ReadStateMutableCastAddress(String, NodeVariableIdentifier, NodeAddressType),
}
impl_any_kind!(NodeRemoteFetchStatement);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeComponentId {
    WithTypeLikeName(NodeTypeNameIdentifier),
    WithRegularId(String),
}
impl_any_kind!(NodeComponentId);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeComponentParameters {
    pub parameters: Vec<NodeParameterPair>,
    pub type_annotation: Option<TypeAnnotation>,
}
impl_any_kind!(NodeComponentParameters);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeParameterPair {
    pub identifier_with_type: NodeTypedIdentifier,
    pub type_annotation: Option<TypeAnnotation>,
}
impl_any_kind!(NodeParameterPair);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeComponentBody {
    pub statement_block: Option<NodeStatementBlock>,
    pub type_annotation: Option<TypeAnnotation>,
}
impl_any_kind!(NodeComponentBody);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeStatementBlock {
    pub statements: Vec<NodeStatement>,
    pub type_annotation: Option<TypeAnnotation>,
}
impl_any_kind!(NodeStatementBlock);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeTypedIdentifier {
    pub identifier_name: String,
    pub annotation: NodeTypeAnnotation,
    pub type_annotation: Option<TypeAnnotation>,
}
impl_any_kind!(NodeTypedIdentifier);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeTypeAnnotation {
    pub type_name: NodeScillaType,
    pub type_annotation: Option<TypeAnnotation>,
}
impl_any_kind!(NodeTypeAnnotation);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeProgram {
    pub version: String,
    pub import_declarations: Option<NodeImportDeclarations>,
    pub library_definition: Option<NodeLibraryDefinition>,
    pub contract_definition: NodeContractDefinition,
    pub type_annotation: Option<TypeAnnotation>,
}
impl_any_kind!(NodeProgram);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeLibraryDefinition {
    pub name: NodeTypeNameIdentifier,
    pub definitions: Vec<NodeLibrarySingleDefinition>,
    pub type_annotation: Option<TypeAnnotation>,
}
impl_any_kind!(NodeLibraryDefinition);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeLibrarySingleDefinition {
    LetDefinition {
        variable_name: String,
        type_annotation: Option<NodeTypeAnnotation>,
        expression: NodeFullExpression,
    },
    TypeDefinition(
        NodeTypeNameIdentifier,
        Option<Vec<NodeTypeAlternativeClause>>,
    ),
}
impl_any_kind!(NodeLibrarySingleDefinition);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeContractDefinition {
    pub contract_name: NodeTypeNameIdentifier,
    pub parameters: NodeComponentParameters,
    pub constraint: Option<NodeWithConstraint>,
    pub fields: Vec<NodeContractField>,
    pub components: Vec<NodeComponentDefinition>,
    pub type_annotation: Option<TypeAnnotation>,
}
impl_any_kind!(NodeContractDefinition);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeContractField {
    pub typed_identifier: NodeTypedIdentifier,
    pub right_hand_side: NodeFullExpression,
    pub type_annotation: Option<TypeAnnotation>,
}
impl_any_kind!(NodeContractField);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeWithConstraint {
    pub expression: Box<NodeFullExpression>,
    pub type_annotation: Option<TypeAnnotation>,
}
impl_any_kind!(NodeWithConstraint);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeComponentDefinition {
    TransitionComponent(Box<NodeTransitionDefinition>),
    ProcedureComponent(Box<NodeProcedureDefinition>),
}
impl_any_kind!(NodeComponentDefinition);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeProcedureDefinition {
    pub name: NodeComponentId,
    pub parameters: NodeComponentParameters,
    pub body: NodeComponentBody,
    pub type_annotation: Option<TypeAnnotation>,
}
impl_any_kind!(NodeProcedureDefinition);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NodeTransitionDefinition {
    pub name: NodeComponentId,
    pub parameters: NodeComponentParameters,
    pub body: NodeComponentBody,
    pub type_annotation: Option<TypeAnnotation>,
}
impl_any_kind!(NodeTransitionDefinition);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeTypeAlternativeClause {
    ClauseType(NodeTypeNameIdentifier),
    ClauseTypeWithArgs(NodeTypeNameIdentifier, Vec<NodeTypeArgument>),
}
impl_any_kind!(NodeTypeAlternativeClause);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeTypeMapValueArguments {
    EnclosedTypeMapValue(Box<NodeTypeMapValueAllowingTypeArguments>),
    GenericMapValueArgument(NodeMetaIdentifier),
    MapKeyValueType(NodeTypeMapKey, NodeTypeMapValue),
}
impl_any_kind!(NodeTypeMapValueArguments);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum NodeTypeMapValueAllowingTypeArguments {
    TypeMapValueNoArgs(NodeTypeMapValue),
    TypeMapValueWithArgs(NodeMetaIdentifier, Vec<NodeTypeMapValueArguments>),
}
impl_any_kind!(NodeTypeMapValueAllowingTypeArguments);
