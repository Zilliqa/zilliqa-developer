use std::fmt::Debug;

#[derive(Clone, Debug)]
pub enum NodeByteStr {
    Constant(String),
    Type(String),
}

#[derive(Clone, Debug)]
pub enum NodeTypeNameIdentifier {
    ByteStringType(NodeByteStr),
    EventType,
    CustomType(String),
}
#[derive(Clone, Debug)]
pub enum NodeImportedName {
    RegularImport(NodeTypeNameIdentifier),
    AliasedImport(NodeTypeNameIdentifier, NodeTypeNameIdentifier),
}

#[derive(Clone, Debug)]
pub struct NodeImportDeclarations {
    pub import_list: Vec<NodeImportedName>,
}

#[derive(Clone, Debug)]
pub enum NodeMetaIdentifier {
    MetaName(NodeTypeNameIdentifier),
    MetaNameInNamespace(NodeTypeNameIdentifier, NodeTypeNameIdentifier),
    MetaNameInHexspace(String, NodeTypeNameIdentifier),
    ByteString,
}

#[derive(Clone, Debug)]
pub enum NodeVariableIdentifier {
    VariableName(String),
    SpecialIdentifier(String),
    VariableInNamespace(NodeTypeNameIdentifier, String),
}

#[derive(Clone, Debug)]
pub struct NodeBuiltinArguments {
    pub arguments: Vec<NodeVariableIdentifier>,
}

#[derive(Clone, Debug)]
pub enum NodeTypeMapKey {
    GenericMapKey(NodeMetaIdentifier),
    EnclosedGenericId(NodeMetaIdentifier),
    EnclosedAddressMapKeyType(NodeAddressType),
    AddressMapKeyType(NodeAddressType),
}

#[derive(Clone, Debug)]
pub enum NodeTypeMapValue {
    MapValueCustomType(NodeMetaIdentifier),
    MapKeyValue(Box<NodeTypeMapEntry>),
    MapValueParanthesizedType(Box<NodeTypeMapValueAllowingTypeArguments>),
    MapValueAddressType(Box<NodeAddressType>),
}

#[derive(Clone, Debug)]
pub enum NodeTypeArgument {
    EnclosedTypeArgument(Box<NodeScillaType>),
    GenericTypeArgument(NodeMetaIdentifier),
    TemplateTypeArgument(String),
    AddressTypeArgument(NodeAddressType),
    MapTypeArgument(NodeTypeMapKey, NodeTypeMapValue),
}

#[derive(Clone, Debug)]
pub enum NodeScillaType {
    GenericTypeWithArgs(NodeMetaIdentifier, Vec<NodeTypeArgument>),
    MapType(NodeTypeMapKey, NodeTypeMapValue),
    FunctionType(Box<NodeScillaType>, Box<NodeScillaType>),
    EnclosedType(Box<NodeScillaType>),
    ScillaAddresseType(Box<NodeAddressType>),
    PolyFunctionType(String, Box<NodeScillaType>),
    TypeVarType(String),
}

#[derive(Clone, Debug)]
pub struct NodeTypeMapEntry {
    pub key: NodeTypeMapKey,
    pub value: NodeTypeMapValue,
}

#[derive(Clone, Debug)]
pub struct NodeAddressTypeField {
    pub identifier: NodeVariableIdentifier,
    pub type_name: NodeScillaType,
}

#[derive(Clone, Debug)]
pub struct NodeAddressType {
    pub identifier: NodeTypeNameIdentifier,
    pub type_name: String,
    pub address_fields: Vec<NodeAddressTypeField>,
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub enum NodeMessageEntry {
    MessageLiteral(NodeVariableIdentifier, NodeValueLiteral),
    MessageVariable(NodeVariableIdentifier, NodeVariableIdentifier),
}

#[derive(Clone, Debug)]
pub struct NodePatternMatchExpressionClause {
    pub pattern: NodePattern,
    pub expression: NodeFullExpression,
}

#[derive(Clone, Debug)]
pub enum NodeAtomicExpression {
    AtomicSid(NodeVariableIdentifier),
    AtomicLit(NodeValueLiteral),
}

#[derive(Clone, Debug)]
pub struct NodeContractTypeArguments {
    pub type_arguments: Vec<NodeTypeArgument>,
}

#[derive(Clone, Debug)]
pub enum NodeValueLiteral {
    LiteralInt(NodeTypeNameIdentifier, String),
    LiteralHex(String),
    LiteralString(String),
    LiteralEmptyMap(NodeTypeMapKey, NodeTypeMapValue),
}

#[derive(Clone, Debug)]
pub struct NodeMapAccess {
    pub identifier_name: NodeVariableIdentifier,
}

#[derive(Clone, Debug)]
pub enum NodePattern {
    Wildcard,
    Binder(String),
    Constructor(NodeMetaIdentifier, Vec<NodeArgumentPattern>),
}

#[derive(Clone, Debug)]
pub enum NodeArgumentPattern {
    WildcardArgument,
    BinderArgument(String),
    ConstructorArgument(NodeMetaIdentifier),
    PatternArgument(Box<NodePattern>),
}

#[derive(Clone, Debug)]
pub struct PatternMatchExpressionClause {
    pub pattern_expression: Box<NodePattern>,
    pub expression: Box<NodeFullExpression>,
}

#[derive(Clone, Debug)]
pub struct NodePatternMatchClause {
    pub pattern_expression: Box<NodePattern>,
    pub statement_block: Option<NodeStatementBlock>,
}

#[derive(Clone, Debug)]
pub struct NodeBlockchainFetchArguments {
    pub arguments: Vec<NodeVariableIdentifier>,
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub enum NodeRemoteFetchStatement {
    ReadStateMutable(String, String, NodeVariableIdentifier),
    ReadStateMutableSpecialId(String, String, String),
    ReadStateMutableMapAccess(String, String, String, Vec<NodeMapAccess>),
    ReadStateMutableMapAccessExists(String, String, String, Vec<NodeMapAccess>),
    ReadStateMutableCastAddress(String, NodeVariableIdentifier, NodeAddressType),
}

#[derive(Clone, Debug)]
pub enum NodeComponentId {
    WithTypeLikeName(NodeTypeNameIdentifier),
    WithRegularId(String),
}

#[derive(Clone, Debug)]
pub struct NodeComponentParameters {
    pub parameters: Vec<NodeParameterPair>,
}

#[derive(Clone, Debug)]
pub struct NodeParameterPair {
    pub identifier_with_type: NodeTypedIdentifier,
}

#[derive(Clone, Debug)]
pub struct NodeComponentBody {
    pub statement_block: Option<NodeStatementBlock>,
}

#[derive(Clone, Debug)]
pub struct NodeStatementBlock {
    pub statements: Vec<NodeStatement>,
}

#[derive(Clone, Debug)]
pub struct NodeTypedIdentifier {
    pub identifier_name: String,
    pub type_annotation: NodeTypeAnnotation,
}

#[derive(Clone, Debug)]
pub struct NodeTypeAnnotation {
    pub type_name: NodeScillaType,
}

#[derive(Clone, Debug)]
pub struct NodeProgram {
    pub version: String,
    pub import_declarations: Option<NodeImportDeclarations>,
    pub library_definition: Option<NodeLibraryDefinition>,
    pub contract_definition: NodeContractDefinition,
}

#[derive(Clone, Debug)]
pub struct NodeLibraryDefinition {
    pub name: NodeTypeNameIdentifier,
    pub definitions: Vec<NodeLibrarySingleDefinition>,
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct NodeContractDefinition {
    pub contract_name: NodeTypeNameIdentifier,
    pub parameters: NodeComponentParameters,
    pub constraint: Option<NodeWithConstraint>,
    pub fields: Vec<NodeContractField>,
    pub components: Vec<NodeComponentDefinition>,
}

#[derive(Clone, Debug)]
pub struct NodeContractField {
    pub typed_identifier: NodeTypedIdentifier,
    pub right_hand_side: NodeFullExpression,
}

#[derive(Clone, Debug)]
pub struct NodeWithConstraint {
    pub expression: Box<NodeFullExpression>,
}

#[derive(Clone, Debug)]
pub enum NodeComponentDefinition {
    TransitionComponent(Box<NodeTransitionDefinition>),
    ProcedureComponent(Box<NodeProcedureDefinition>),
}

#[derive(Clone, Debug)]
pub struct NodeProcedureDefinition {
    pub name: NodeComponentId,
    pub parameters: NodeComponentParameters,
    pub body: NodeComponentBody,
}

#[derive(Clone, Debug)]
pub struct NodeTransitionDefinition {
    pub name: NodeComponentId,
    pub parameters: NodeComponentParameters,
    pub body: NodeComponentBody,
}

#[derive(Clone, Debug)]
pub enum NodeTypeAlternativeClause {
    ClauseType(NodeTypeNameIdentifier),
    ClauseTypeWithArgs(NodeTypeNameIdentifier, Vec<NodeTypeArgument>),
}

#[derive(Clone, Debug)]
pub enum NodeTypeMapValueArguments {
    EnclosedTypeMapValue(Box<NodeTypeMapValueAllowingTypeArguments>),
    GenericMapValueArgument(NodeMetaIdentifier),
    MapKeyValueType(NodeTypeMapKey, NodeTypeMapValue),
}

#[derive(Clone, Debug)]
pub enum NodeTypeMapValueAllowingTypeArguments {
    TypeMapValueNoArgs(NodeTypeMapValue),
    TypeMapValueWithArgs(NodeMetaIdentifier, Vec<NodeTypeMapValueArguments>),
}
