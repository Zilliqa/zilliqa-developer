use scilla_parser::parser::lexer;

// Struct to represent source errors
// Contains the position of the error in the source code and the error message
pub struct SourceError {
    /// Source position of the error
    pub position: lexer::SourcePosition,

    /// Associated error message
    pub message: String,
}

// Type alias for a list of source errors
pub type ErrorList = Vec<SourceError>;
