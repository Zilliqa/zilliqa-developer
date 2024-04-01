use scilla_parser::ast::nodes::NodeProgram;

/// Trait for a queue that lists the next Ast to be compiled.
pub trait AstQueue {
    /// Add a library to the queue.
    fn enqueue(&mut self, library_name: &str) -> Result<(), String>;

    /// Add a library to the queue.
    fn enqueue_with_alias(&mut self, library_name: &str, alias_name: &str) -> Result<(), String>;
    /// Get the next Ast to be converted to the IR.
    fn pop_front(&mut self) -> Option<NodeProgram>;
}
