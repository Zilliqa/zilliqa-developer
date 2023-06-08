use bluebell::{
    ast::*, formatter::ScillaFormatter, lexer::Lexer, lexer::*, parser::*, type_inference::*, *,
};

#[cfg(test)]
mod tests {
    use super::*;
    use bluebell::ast::*;
    use bluebell::type_classes::*;
    use std::collections::HashMap;

    #[test]
    fn test_type_of_variable_identifier() {
        // Initialize the environment with a variable and its type
        let env: HashMap<String, Box<dyn BaseType>> = {
            let mut env: HashMap<String, Box<dyn BaseType>> = HashMap::new();
            let variable_type = BuiltinType {
                name: String::from("Int32"),
                symbol: String::from("Int32"),
            };
            env.insert(String::from("my_variable"), Box::new(variable_type));
            env
        };

        // Create a variable identifier
        let identifier = NodeVariableIdentifier::VariableName(String::from("my_variable"));

        // Call the function to get the type annotation
        let result = type_of_variable_identifier(&identifier, &env);
        assert!(result.is_ok());
        match result.unwrap() {
            TypeAnnotation::BuiltinType(t) => assert_eq!(t.to_string(), "Int32"),
            _ => panic!("Unexpected TypeAnnotation variant"),
        };

        let result = type_of(&identifier, &env);
        assert!(result.is_ok());
        match result.unwrap() {
            TypeAnnotation::BuiltinType(t) => assert_eq!(t.to_string(), "Int32"),
            _ => panic!("Unexpected TypeAnnotation variant"),
        };
    }

    /*
    #[test]
    fn test_type_of_node_message_entry() {
        // Initialize the environment with a variable and its type
        let env = {
            let mut env: HashMap<String, Box<dyn BaseType>> = HashMap::new();
            let variable_type = BuiltinType {
                name: String::from("String"),
                symbol: String::from("String"),
            };
            env.insert(String::from("my_variable"), Box::new(variable_type));
            env
        };

        // Create a variable identifier for the left-hand side
        let left_identifier = NodeVariableIdentifier::VariableName(String::from("my_variable"));

        // Create a variable identifier for the right-hand side
        let right_identifier = NodeVariableIdentifier::VariableName(String::from("my_variable"));

        // Create a message entry node
        let entry_node =
            NodeMessageEntry::MessageVariable(left_identifier.clone(), right_identifier.clone());

        // Call the function to get the type annotation
        let result = type_of_node_message_entry(&entry_node, &env);

        // Check the result against the expected type annotation
        // TODO: Work out the correct type of MesageEntry assert!(result.is_ok());
        match result.unwrap() {
            TypeAnnotation::BuiltinType(t) => assert_eq!(t.to_string(), "String"),
            _ => panic!("Unexpected TypeAnnotation variant"),
        };
    }
    */

    /*
    #[test]
    fn test_type_check_func() {
        // Create a simple function that accepts an Int32 and returns an Int32
        let parameter_name = String::from("input");
        let input_type = TypeAnnotation::BuiltinType(BuiltinType {
            name: String::from("Int32"),
            symbol: String::from("Int32"),
        });

        let output_type = TypeAnnotation::BuiltinType(BuiltinType {
            name: String::from("Int32"),
            symbol: String::from("Int32"),
        });

        let return_expression = NodeExpression::VariableIdentifier(NodeVariableIdentifier::VariableName(parameter_name.clone()));

        let func = NodeProcedureDefinition {
            parameters: vec![(parameter_name.clone(), input_type.clone())],
            // return_type: output_type.clone(),
            body: vec![return_expression],
        };

        // Create a HashMap for the function's environment
        let mut env = HashMap::new();
        env.insert(parameter_name, Box::new(input_type.to_base_type().unwrap()) as Box<dyn BaseType>);

        // Call the type_check_func function to check the types of the function
        let result = type_check_func(&func, &env);

        assert!(result.is_ok());

        // Check the result against the expected return type
        let result_type = result.unwrap();
        assert_eq!(result_type, output_type);
    }
    */
}
