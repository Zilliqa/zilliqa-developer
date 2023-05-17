use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeAnnotation {
    pub type_name: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeByteStr {
    Constant(String),
    Type(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeTypeNameIdentifier {
    ByteStringType(NodeByteStr),
    EventType,
    CustomType(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeImportedName {
    RegularImport(NodeTypeNameIdentifier),
    AliasedImport(NodeTypeNameIdentifier, NodeTypeNameIdentifier),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeImportDeclarations {
    pub import_list: Vec<NodeImportedName>,
    pub type_annotation: Option<TypeAnnotation>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeMetaIdentifier {
    MetaName(NodeTypeNameIdentifier),
    MetaNameInNamespace(NodeTypeNameIdentifier, NodeTypeNameIdentifier),
    MetaNameInHexspace(String, NodeTypeNameIdentifier),
    ByteString,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeVariableIdentifier {
    VariableName(String),
    SpecialIdentifier(String),
    VariableInNamespace(NodeTypeNameIdentifier, String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeBuiltinArguments {
    pub arguments: Vec<NodeVariableIdentifier>,
    pub type_annotation: Option<TypeAnnotation>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeTypeMapKey {
    GenericMapKey(NodeMetaIdentifier),
    EnclosedGenericId(NodeMetaIdentifier),
    EnclosedAddressMapKeyType(NodeAddressType),
    AddressMapKeyType(NodeAddressType),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeTypeMapValue {
    MapValueCustomType(NodeMetaIdentifier),
    MapKeyValue(Box<NodeTypeMapEntry>),
    MapValueParanthesizedType(Box<NodeTypeMapValueAllowingTypeArguments>),
    MapValueAddressType(Box<NodeAddressType>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeTypeArgument {
    EnclosedTypeArgument(Box<NodeScillaType>),
    GenericTypeArgument(NodeMetaIdentifier),
    TemplateTypeArgument(String),
    AddressTypeArgument(NodeAddressType),
    MapTypeArgument(NodeTypeMapKey, NodeTypeMapValue),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeScillaType {
    GenericTypeWithArgs(NodeMetaIdentifier, Vec<NodeTypeArgument>),
    MapType(NodeTypeMapKey, NodeTypeMapValue),
    FunctionType(Box<NodeScillaType>, Box<NodeScillaType>),
    EnclosedType(Box<NodeScillaType>),
    ScillaAddresseType(Box<NodeAddressType>),
    PolyFunctionType(String, Box<NodeScillaType>),
    TypeVarType(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeTypeMapEntry {
    pub key: NodeTypeMapKey,
    pub value: NodeTypeMapValue,
    pub type_annotation: Option<TypeAnnotation>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeAddressTypeField {
    pub identifier: NodeVariableIdentifier,
    pub type_name: NodeScillaType,
    pub type_annotation: Option<TypeAnnotation>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeAddressType {
    pub identifier: NodeTypeNameIdentifier,
    pub type_name: String,
    pub address_fields: Vec<NodeAddressTypeField>,
    pub type_annotation: Option<TypeAnnotation>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeMessageEntry {
    MessageLiteral(NodeVariableIdentifier, NodeValueLiteral),
    MessageVariable(NodeVariableIdentifier, NodeVariableIdentifier),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodePatternMatchExpressionClause {
    pub pattern: NodePattern,
    pub expression: NodeFullExpression,
    pub type_annotation: Option<TypeAnnotation>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeAtomicExpression {
    AtomicSid(NodeVariableIdentifier),
    AtomicLit(NodeValueLiteral),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeContractTypeArguments {
    pub type_arguments: Vec<NodeTypeArgument>,
    pub type_annotation: Option<TypeAnnotation>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeValueLiteral {
    LiteralInt(NodeTypeNameIdentifier, String),
    LiteralHex(String),
    LiteralString(String),
    LiteralEmptyMap(NodeTypeMapKey, NodeTypeMapValue),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeMapAccess {
    pub identifier_name: NodeVariableIdentifier,
    pub type_annotation: Option<TypeAnnotation>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodePattern {
    Wildcard,
    Binder(String),
    Constructor(NodeMetaIdentifier, Vec<NodeArgumentPattern>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeArgumentPattern {
    WildcardArgument,
    BinderArgument(String),
    ConstructorArgument(NodeMetaIdentifier),
    PatternArgument(Box<NodePattern>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PatternMatchExpressionClause {
    pub pattern_expression: Box<NodePattern>,
    pub expression: Box<NodeFullExpression>,
    pub type_annotation: Option<TypeAnnotation>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodePatternMatchClause {
    pub pattern_expression: Box<NodePattern>,
    pub statement_block: Option<NodeStatementBlock>,
    pub type_annotation: Option<TypeAnnotation>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeBlockchainFetchArguments {
    pub arguments: Vec<NodeVariableIdentifier>,
    pub type_annotation: Option<TypeAnnotation>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeRemoteFetchStatement {
    ReadStateMutable(String, String, NodeVariableIdentifier),
    ReadStateMutableSpecialId(String, String, String),
    ReadStateMutableMapAccess(String, String, String, Vec<NodeMapAccess>),
    ReadStateMutableMapAccessExists(String, String, String, Vec<NodeMapAccess>),
    ReadStateMutableCastAddress(String, NodeVariableIdentifier, NodeAddressType),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeComponentId {
    WithTypeLikeName(NodeTypeNameIdentifier),
    WithRegularId(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeComponentParameters {
    pub parameters: Vec<NodeParameterPair>,
    pub type_annotation: Option<TypeAnnotation>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeParameterPair {
    pub identifier_with_type: NodeTypedIdentifier,
    pub type_annotation: Option<TypeAnnotation>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeComponentBody {
    pub statement_block: Option<NodeStatementBlock>,
    pub type_annotation: Option<TypeAnnotation>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeStatementBlock {
    pub statements: Vec<NodeStatement>,
    pub type_annotation: Option<TypeAnnotation>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeTypedIdentifier {
    pub identifier_name: String,
    pub annotation: NodeTypeAnnotation,
    pub type_annotation: Option<TypeAnnotation>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeTypeAnnotation {
    pub type_name: NodeScillaType,
    pub type_annotation: Option<TypeAnnotation>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeProgram {
    pub version: String,
    pub import_declarations: Option<NodeImportDeclarations>,
    pub library_definition: Option<NodeLibraryDefinition>,
    pub contract_definition: NodeContractDefinition,
    pub type_annotation: Option<TypeAnnotation>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeLibraryDefinition {
    pub name: NodeTypeNameIdentifier,
    pub definitions: Vec<NodeLibrarySingleDefinition>,
    pub type_annotation: Option<TypeAnnotation>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeContractDefinition {
    pub contract_name: NodeTypeNameIdentifier,
    pub parameters: NodeComponentParameters,
    pub constraint: Option<NodeWithConstraint>,
    pub fields: Vec<NodeContractField>,
    pub components: Vec<NodeComponentDefinition>,
    pub type_annotation: Option<TypeAnnotation>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeContractField {
    pub typed_identifier: NodeTypedIdentifier,
    pub right_hand_side: NodeFullExpression,
    pub type_annotation: Option<TypeAnnotation>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeWithConstraint {
    pub expression: Box<NodeFullExpression>,
    pub type_annotation: Option<TypeAnnotation>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeComponentDefinition {
    TransitionComponent(Box<NodeTransitionDefinition>),
    ProcedureComponent(Box<NodeProcedureDefinition>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeProcedureDefinition {
    pub name: NodeComponentId,
    pub parameters: NodeComponentParameters,
    pub body: NodeComponentBody,
    pub type_annotation: Option<TypeAnnotation>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeTransitionDefinition {
    pub name: NodeComponentId,
    pub parameters: NodeComponentParameters,
    pub body: NodeComponentBody,
    pub type_annotation: Option<TypeAnnotation>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeTypeAlternativeClause {
    ClauseType(NodeTypeNameIdentifier),
    ClauseTypeWithArgs(NodeTypeNameIdentifier, Vec<NodeTypeArgument>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeTypeMapValueArguments {
    EnclosedTypeMapValue(Box<NodeTypeMapValueAllowingTypeArguments>),
    GenericMapValueArgument(NodeMetaIdentifier),
    MapKeyValueType(NodeTypeMapKey, NodeTypeMapValue),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeTypeMapValueAllowingTypeArguments {
    TypeMapValueNoArgs(NodeTypeMapValue),
    TypeMapValueWithArgs(NodeMetaIdentifier, Vec<NodeTypeMapValueArguments>),
}
