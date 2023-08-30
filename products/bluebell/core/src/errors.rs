use crate::parser::lexer;

pub struct SourceError {
    pub position: lexer::SourcePosition,
    pub message: String,
}
pub type ErrorList = Vec<SourceError>;
