// Enums used in various routines
pub enum TreeTraversalMode {
    Enter, // Used when emit_... is invoked before children are visited
    Exit,  // User after children were visited
}

#[derive(PartialEq, Eq)]
pub enum TraversalResult {
    Continue,     // Returned when the visitor should continue tree traversal
    SkipChildren, // Returned when the visitor should skip the children and exit traversal
}

// Symbol constant: These are the constants we use when representing types and instances
// internally.
pub const GLOBAL_VAR_PREFIX: &str = "@"; // prefix for global variables and functions
pub const NAMESPACE_SEPARATOR: &str = "::"; // the separator of namespaces
pub const TEMPLATE_PARAMETERS_START: &str = "<"; // marks the start of template parameters
pub const TEMPLATE_PARAMETERS_END: &str = ">"; // marks the end of template parameters
pub const TEMPLATE_PARAMETERS_SEPARATOR: &str = ","; // separator of template parameters
pub const FUNCTION_ARGUMENTS_START: &str = "("; // marks the start of function arguments
pub const FUNCTION_ARGUMENTS_END: &str = ")"; // marks the end of function arguments
pub const FUNCTION_ARGUMENTS_SEPARATOR: &str = ","; // separator of function arguments
pub const FUNCTION_RETURN_TYPE_SEPARATOR: &str = "->"; // separates function arguments and return type
pub const LOCAL_VAR_PREFIX: &str = "%"; // prefix for local variables
