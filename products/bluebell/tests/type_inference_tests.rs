use bluebell::{
    ast::*, formatter::ScillaFormatter, lexer::Lexer, lexer::*, parser::*, type_inference::*, *,
};

#[cfg(test)]
mod tests {
    use super::*;
    use bluebell::ast::*;
    use bluebell::constants::*;
    use bluebell::lexer;
    use bluebell::parser;
    use bluebell::type_classes::*;
    use bluebell::type_inference::*;
    use std::collections::HashMap;
    macro_rules! get_ast {
        ($parser:ty, $result:expr) => {{
            let mut errors = vec![];
            let ast = <$parser>::new().parse(&mut errors, lexer::Lexer::new($result));
            match ast {
                Ok(parsed_ast) => parsed_ast,
                Err(err) => panic!("Parsing error: {:?}", err),
            }
        }};
    }

    fn setup_workspace() -> Workspace {
        let mut workspace = Workspace {
            env: HashMap::new(),
            namespace: "".to_string(),
        };
        workspace.env.insert(
            "String".to_string(),
            Box::new(BuiltinType {
                name: "String".to_string(),
                symbol: "String".to_string(),
            }),
        );
        workspace.env.insert(
            "MyCustomType".to_string(),
            Box::new(TemplateType {
                name: "MyCustomType".to_string(),
                symbol: "MyCustomType".to_string(),
            }),
        );
        workspace.env.insert(
            "MyNamespace".to_string(),
            Box::new(UnionType {
                // TODO: Come up with namespace type
                name: "myCustomType".to_string(),
                types: [].to_vec(),
                symbol: "myCustomType".to_string(),
            }),
        );

        workspace.env.insert(
            "MyNamespace::MyCustomType".to_string(),
            Box::new(TemplateType {
                name: "MyCustomType".to_string(),
                symbol: "MyNamespace::MyCustomType".to_string(),
            }),
        );
        workspace.env.insert(
            "ByStr1".to_string(),
            Box::new(BuiltinType {
                name: "ByStr1".to_string(),
                symbol: "ByStr1".to_string(),
            }),
        );
        workspace.env.insert(
            "Event".to_string(),
            Box::new(BuiltinType {
                name: "Event".to_string(),
                symbol: "Event".to_string(),
            }),
        );
        workspace
    }

    #[test]
    fn type_inference_variable_identifier_test() {
        let mut workspace = setup_workspace();

        // Non-existant type
        let ident = get_ast!(parser::VariableIdentifierParser, "nonExistent");
        let result = ident.get_type(&mut workspace);
        assert!(
            result.is_err(),
            "Expected an error when parsing non-existent type"
        );

        // Base type resolution
        let ident = get_ast!(parser::VariableIdentifierParser, "myCustomType");
        assert_eq!(
            ident.get_type(&mut workspace).unwrap(),
            TypeAnnotation::UnionType(UnionType {
                name: "myCustomType".to_string(),
                types: [].to_vec(),
                symbol: "myCustomType".to_string(),
            }),
        );

        // Base type resolution
        let ident = get_ast!(parser::VariableIdentifierParser, "MyNamespace.myCustomType");
        assert_eq!(
            ident.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "myCustomType".to_string(),
                symbol: "MyNamespace::myCustomType".to_string()
            }),
        );

        // Type in namespace
        workspace.namespace = "MyNamespace".to_string();

        let ident = get_ast!(parser::VariableIdentifierParser, "myCustomType");
        assert_eq!(
            ident.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "myCustomType".to_string(),
                symbol: "MyNamespace::myCustomType".to_string()
            })
        )

        // TODO: add more cases to cover the different scenarios
    }

    #[test]
    fn type_inference_variable_in_namespace_test() {
        let mut workspace = Workspace {
            env: HashMap::new(),
            namespace: "".to_string(),
        };
        workspace.env.insert(
            "MyModule".to_string(),
            Box::new(BuiltinType {
                name: "MyModule".to_string(),
                symbol: "MyModule".to_string(),
            }),
        );
        workspace.env.insert(
            "MyModule::myType".to_string(),
            Box::new(TemplateType {
                name: "myType".to_string(),
                symbol: "MyModule::myType".to_string(),
            }),
        );
        // Type resolution within module
        let ident = get_ast!(parser::VariableIdentifierParser, "MyModule.myType");
        assert_eq!(
            ident.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "myType".to_string(),
                symbol: "MyModule::myType".to_string(),
            }),
        );
        // Non-existent type within module
        let ident = get_ast!(parser::VariableIdentifierParser, "NonexistentModule.myType");
        assert!(ident.get_type(&mut workspace).is_err());
    }

    #[test]
    fn type_inference_node_type_name_identifier_test() {
        let mut workspace = setup_workspace();
        // ByteStr type resolution
        let ident = get_ast!(parser::TypeNameIdentifierParser, "ByStr1");
        assert_eq!(
            ident.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "ByStr".to_string(),
                symbol: "ByStr".to_string()
            })
        );
        // Event type resolution
        let ident = get_ast!(parser::TypeNameIdentifierParser, "Event");
        assert_eq!(
            ident.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "Event".to_string(),
                symbol: "Event".to_string()
            })
        );
        // Custom type resolution
        let ident = get_ast!(parser::TypeNameIdentifierParser, "MyCustomType");
        assert_eq!(
            ident.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "MyCustomType".to_string(),
                symbol: "MyCustomType".to_string()
            })
        );
        // Type in namespace
        workspace.namespace = "MyNamespace".to_string();
        let ident = get_ast!(parser::TypeNameIdentifierParser, "MyCustomType");
        assert_eq!(
            ident.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "myCustomType".to_string(),
                symbol: "MyNamespace::MyCustomType".to_string()
            })
        );
        // Nonexistent type
        let ident = get_ast!(parser::TypeNameIdentifierParser, "NonexistentType");
        assert!(ident.get_type(&mut workspace).is_err());
    }

    #[test]
    fn type_inference_node_byte_str_test() {
        let mut workspace = Workspace {
            env: HashMap::new(),
            namespace: "".to_string(),
        };
        workspace.env.insert(
            "Int32".to_string(),
            Box::new(BuiltinType {
                name: "Int32".to_string(),
                symbol: "Int32".to_string(),
            }),
        );
        workspace.env.insert(
            "Uint128".to_string(),
            Box::new(BuiltinType {
                name: "Uint128".to_string(),
                symbol: "Uint128".to_string(),
            }),
        );
        workspace.env.insert(
            "Uint32".to_string(),
            Box::new(BuiltinType {
                name: "Uint32".to_string(),
                symbol: "Uint32".to_string(),
            }),
        );
        // Check for valid ByteStr type, ByStr20
        let byte_str = NodeByteStr::Type("ByStr20".to_string());
        assert_eq!(
            byte_str.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "ByStr20".to_string(),
                symbol: "ByStr20".to_string()
            })
        );
        // Check for valid integer type, Int32
        let byte_str = NodeByteStr::Type("Int32".to_string());
        assert_eq!(
            byte_str.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "Int32".to_string(),
                symbol: "Int32".to_string()
            })
        );
        // Check for valid Unsigned type, Uint128
        let byte_str = NodeByteStr::Type("Uint128".to_string());
        assert_eq!(
            byte_str.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "Uint128".to_string(),
                symbol: "Uint128".to_string()
            })
        );
        // Check for valid unsigned type, Uint32
        let byte_str = NodeByteStr::Type("Uint32".to_string());
        assert_eq!(
            byte_str.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "Uint32".to_string(),
                symbol: "Uint32".to_string()
            })
        );
        // Check for non-existent type
        let byte_str = NodeByteStr::Type("NonexistentType".to_string());
        assert!(byte_str.get_type(&mut workspace).is_err());
    }

    #[test]
    fn type_inference_transition_definition_test() {
        // Some workspace initialization for the test. Please adjust accordingly.
        let mut workspace = Workspace {
            env: HashMap::new(),
            namespace: "".to_string(),
        };
        workspace.env.insert(
            "Int32".to_string(),
            Box::new(BuiltinType {
                name: "Int32".to_string(),
                symbol: "Int32".to_string(),
            }),
        );
        workspace.env.insert(
            "Uint32".to_string(),
            Box::new(BuiltinType {
                name: "Uint32".to_string(),
                symbol: "Uint32".to_string(),
            }),
        );
        workspace.env.insert(
            "Bool".to_string(),
            Box::new(BuiltinType {
                name: "Bool".to_string(),
                symbol: "Bool".to_string(),
            }),
        );
        // Define a NodeTransitionDefinition object for the test. The choice of transitions is
        // arbitrary as long as it's a valid syntax.
        let transition_definition = get_ast!(
            parser::TransitionDefinitionParser,
            "transition foo(x: Int32, y: Uint32) x := 12; y := 13 end"
        );
        // Check the transition name
        if let NodeComponentId::WithRegularId(ref identifier_value) = transition_definition.name {
            assert_eq!(identifier_value, "foo");
        } else {
            panic!("Expected regular identifier for transition name");
        }
        // Check the transition parameters
        assert_eq!(transition_definition.parameters.parameters.len(), 2);
        let param_x = transition_definition.parameters.parameters.get(0).unwrap();
        let param_y = transition_definition.parameters.parameters.get(1).unwrap();
        // TODO: Check the types of the parameters
        assert_eq!(param_x.identifier_with_type.identifier_name, "x");
        assert_eq!(param_y.identifier_with_type.identifier_name, "y");
        // Check the transition body
        let statements = transition_definition
            .body
            .statement_block
            .as_ref() // Create a reference to the Option, instead of consuming it
            .unwrap()
            .statements
            .clone(); // Add clone here if `statements` type implements `Clone`
                      // TODO: Check the types of the statements
        assert_eq!(statements.len(), 2);
        // You need to write assertion for each statement in the transition body to
        // confirm they are correct. For simplicity, I'm just checking the count here.
        // Get the type of the transition_definition. In Scilla, transitions have Void type.
        let type_annotation = transition_definition.get_type(&mut workspace).unwrap();
        // Test the inferred type.
        assert_eq!(type_annotation, TypeAnnotation::Void,);
    }

    #[test]
    fn type_inference_alternative_clause_test_extended_with_get_ast() {
        let mut workspace = setup_workspace();
        // Valid type String
        let clause_simple = get_ast!(parser::TypeAlternativeClauseParser, "| String");
        assert_eq!(
            clause_simple.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "String".to_string(),
                symbol: "String".to_string(),
            })
        );
        // Valid type ByStr20
        let clause_bystr20 = get_ast!(parser::TypeAlternativeClauseParser, "| ByStr20");
        assert_eq!(
            clause_bystr20.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "ByStr20".to_string(),
                symbol: "ByStr20".to_string(),
            })
        );
        // Invalid type, the function returns an error
        let clause_invalid = get_ast!(parser::TypeAlternativeClauseParser, "| InvalidType");
        assert!(clause_invalid.get_type(&mut workspace).is_err());
    }

    #[test]
    fn type_inference_map_value_arguments_test() {
        let mut workspace = setup_workspace();
        let mut map_key_value_type;
        // Parse and evaluate an Int32 ByteStringType
        let key = get_ast!(parser::TypeMapValueArgumentsParser, "Int32");
        assert_eq!(
            key.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "Int32".to_string(),
                symbol: "Int32".to_string()
            }),
        );
        // Parse and evaluate a String ByteStringType
        let value = get_ast!(parser::TypeMapValueArgumentsParser, "String");
        assert_eq!(
            value.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "String".to_string(),
                symbol: "String".to_string(),
            }),
        );
        // Parse and evaluate a Map with key-value pairs
        map_key_value_type = get_ast!(parser::TypeMapValueArgumentsParser, "Map Int32 String");
        assert_eq!(
            map_key_value_type.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "Map[Int32, String]".to_string(),
                symbol: "Map".to_string(),
            }),
        );
        // Parse and evaluate a Map with Nested key-value pairs
        map_key_value_type = get_ast!(
            parser::TypeMapValueArgumentsParser,
            "Map Int32 Map String String"
        );
        assert_eq!(
            map_key_value_type.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "Map[Int32, Map[String, String]]".to_string(),
                symbol: "Map".to_string(),
            }),
        );
    }

    /*
        #[test]
        fn type_inference_map_value_allow_type_args_test() {
            // Set up workspace.environment with required types
            let mut workspace = Workspace {
                env: HashMap::new(),
                namespace: "".to_string()
            };
            workspace.env.insert("String".to_string(), Box::new(BuiltinType { name: "String".to_string(), symbol: "String".to_string() }));
            // Implement test logic for both NodeTypeMapValueAllowingTypeArguments variants
        }
    */

    #[test]
    fn type_inference_imported_name_test() {
        let mut workspace = setup_workspace();
        // Test imported name for custom type
        let ident = get_ast!(parser::ImportedNameParser, "MyCustomType");
        assert_eq!(
            ident.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "MyCustomType".to_string(),
                symbol: "MyCustomType".to_string(),
            })
        );
        // Test imported name for event type
        let ident = get_ast!(parser::ImportedNameParser, "Event");
        assert_eq!(
            ident.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "Event".to_string(),
                symbol: "Event".to_string(),
            })
        );
        // Test imported aliasing. Aliases can be used to rename imported types.
        let ident = get_ast!(parser::ImportedNameParser, "Event as MyEventType");
        assert_eq!(
            ident.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "Event".to_string(),
                symbol: "MyEventType".to_string(),
            })
        );
        // Test imported name for a namespace type, as a namespace is also valid for importing.
        let ident = get_ast!(parser::ImportedNameParser, "MyNamespace::MyCustomType");
        assert_eq!(
            ident.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "MyCustomType".to_string(),
                symbol: "MyNamespace::MyCustomType".to_string(),
            })
        );
    }

    #[test]
    fn type_inference_import_declarations_test() {
        // we reference our workspace setup function from Attachment #1
        let mut workspace = setup_workspace();
        // scenario: we have valid import declarations in our code
        let node_import_declaration_str = r#"
    import MyCustomType
    import MyNamespace.MyCustomType
    "#;
        // getting the relevant AST Node => NodeImportDeclarations
        let node_import_declarations = get_ast!(
            parser::ImportDeclarationsParser,
            node_import_declaration_str
        );
        // this should be successful as we imported valid types
        assert!(node_import_declarations.get_type(&mut workspace).is_ok());
        // Check the imported types
        assert!(workspace.env.contains_key("MyCustomType"));
        assert!(workspace.env.contains_key("MyNamespace::MyCustomType"));
        // scenario: we have invalid import declaration in our code
        let node_import_declaration_str = r#"
    import NonExistentType
    "#;
        // getting the relevant AST Node => NodeImportDeclarations
        let node_import_declarations = get_ast!(
            parser::ImportDeclarationsParser,
            node_import_declaration_str
        );
        // this should fail because we are trying to import a non-existent type
        assert!(node_import_declarations.get_type(&mut workspace).is_err());
        // Check that the invalid import didn't get added to the symbol table
        assert!(!workspace.env.contains_key("NonExistentType"));
    }

    #[test]
    fn type_inference_meta_identifier_test() {
        // Set up the workspace
        let mut workspace = setup_workspace();
        // Parsing and inferring type for a base type
        let ident = get_ast!(parser::MetaIdentifierParser, "String");
        assert_eq!(
            ident.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "String".to_string(),
                symbol: "String".to_string()
            })
        );
        // Parsing and inferring type in a namespace
        let ident = get_ast!(parser::MetaIdentifierParser, "MyNamespace::MyCustomType");
        assert_eq!(
            ident.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "MyCustomType".to_string(),
                symbol: "MyNamespace::MyCustomType".to_string()
            })
        );
        // Parsing and inferring type in a Hexspace
        workspace.namespace = "MyNamespace".to_string(); // Ensuring we are in the right namespace
        let ident = get_ast!(parser::MetaIdentifierParser, "0x1234.MyCustomType");
        assert_eq!(
            ident.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "MyCustomType".to_string(),
                symbol: "0x1234.MyCustomType".to_string()
            })
        );
        // Parsing and inferring ByteString type
        let ident = get_ast!(parser::MetaIdentifierParser, "ByStr1");
        assert_eq!(
            ident.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "ByStr1".to_string(),
                symbol: "ByStr1".to_string()
            })
        );
    }

    #[test]
    fn type_inference_builtin_arguments_test() {
        let mut workspace = setup_workspace();
        // Test BuiltinType argument
        let node_builtin_arguments = NodeBuiltinArguments {
            arguments: vec![NodeVariableIdentifier::VariableName("String".to_string())],
            type_annotation: None,
        };
        assert_eq!(
            node_builtin_arguments.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "String".to_string(),
                symbol: "String".to_string(),
            })
        );
        // Test TemplateType argument
        let node_builtin_arguments = NodeBuiltinArguments {
            arguments: vec![NodeVariableIdentifier::VariableName(
                "MyCustomType".to_string(),
            )],
            type_annotation: None,
        };
        assert_eq!(
            node_builtin_arguments.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "MyCustomType".to_string(),
                symbol: "MyCustomType".to_string(),
            })
        );
        // Test for invalid/empty arguments
        let empty_builtin_arguments = NodeBuiltinArguments {
            arguments: vec![],
            type_annotation: None,
        };
        assert!(empty_builtin_arguments.get_type(&mut workspace).is_err());
        // Test for non-existant type
        let ident = get_ast!(parser::VariableIdentifierParser, "nonExistentType");
        let result = ident.get_type(&mut workspace);
        assert!(
            result.is_err(),
            "Expected an error when parsing non-existent type"
        );
        // Test for the type ByStr1
        let ident = get_ast!(parser::VariableIdentifierParser, "ByStr1");
        assert_eq!(
            ident.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "ByStr1".to_string(),
                symbol: "ByStr1".to_string(),
            })
        );
        // Test for Event type
        let ident = get_ast!(parser::VariableIdentifierParser, "Event");
        assert_eq!(
            ident.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "Event".to_string(),
                symbol: "Event".to_string(),
            })
        );
    }

    #[test]
    fn type_inference_node_type_map_key_test() {
        let mut workspace = setup_workspace();
        // NodeTypeNameIdentifier test
        let ident = get_ast!(parser::TypeMapKeyParser, "Foo");
        assert_eq!(
            ident.get_type(&mut workspace).unwrap(),
            TypeAnnotation::EnumType(EnumType {
                name: "Foo".to_string(),
                values: [].to_vec(),
                symbol: "Foo".to_string(),
            }),
        );
        // NodeMetaIdentifier test
        let ident = get_ast!(parser::TypeMapKeyParser, "(MyNamespace::MyCustomType)");
        assert_eq!(
            ident.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "MyCustomType".to_string(),
                symbol: "MyNamespace::MyCustomType".to_string(),
            }),
        );
        // NodeAddressType test
        let ident = get_ast!(parser::TypeMapKeyParser, "(ByStr1 with end)");
        assert_eq!(
            ident.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "ByStr1".to_string(),
                symbol: "ByStr1".to_string(),
            }),
        );
        // Non-existant type
        let ident = get_ast!(parser::TypeMapKeyParser, "nonExistent");
        let result = ident.get_type(&mut workspace);
        assert!(
            result.is_err(),
            "Expected an error when parsing non-existent type"
        );
    }

    #[test]
    fn type_inference_node_type_map_value_test() {
        let mut workspace = setup_workspace();
        // Parse a NodeTypeNameIdentifier
        let typenameident = get_ast!(parser::TypeNameIdentifierParser, "Uint32");
        let result = typenameident.get_type(&mut workspace);
        assert_eq!(
            result.unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "Uint32".to_string(),
                symbol: "Uint32".to_string(),
            }),
            "Expected BuiltinType with name and symbol 'Uint32'"
        );
        // Parse a NodeTypeMapKey
        let mapkey = get_ast!(parser::TypeMapKeyParser, "GenericMapKey Uint32");
        let result = mapkey.get_type(&mut workspace);
        assert_eq!(
            result.unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "GenericMapKey".to_string(),
                symbol: "Uint32".to_string(),
            }),
            "Expected BuiltinType definition for a generic map key of type 'GenericMapKey'"
        );
        // Parse a NodeAddressType
        let addresstype = get_ast!(parser::AddressTypeParser, "Address with end");
        let result = addresstype.get_type(&mut workspace);
        assert_eq!(
            result.unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "Address".to_string(),
                symbol: "Address".to_string(),
            }),
            "Expected BuiltinType 'Address'"
        );
        // Parse a NodeTypeMapValue
        let mapvalue = get_ast!(parser::TypeMapValueParser, "MapValueCustomType Uint32");
        let result = mapvalue.get_type(&mut workspace);
        assert_eq!(
            result.unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "MapValueCustomType".to_string(),
                symbol: "Uint32".to_string(),
            }),
            "Expected 'MapValueCustomType' for a map value of custom type 'Uint32'"
        );
        // TODO: add more cases to cover the different scenarios
    }

    #[test]
    fn type_inference_node_type_argument_test() {
        let mut workspace = setup_workspace();
        // Test generic type argument
        let node = get_ast!(parser::TypeArgumentParser, "Uint32");
        let result = node.get_type(&mut workspace).ok().expect("Expected a type");
        assert_eq!(
            result,
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "Uint32".to_string(),
                symbol: "Uint32".to_string(),
            }),
        );
        // Test generic custom type argument
        let node = get_ast!(parser::TypeArgumentParser, "MyCustomType");
        let result = node.get_type(&mut workspace).ok().expect("Expected a type");
        assert_eq!(
            result,
            TypeAnnotation::TemplateType(TemplateType {
                name: "MyCustomType".to_string(),
                symbol: "MyCustomType".to_string(),
            }),
        );
        // Test Map type argument
        let node = get_ast!(parser::TypeArgumentParser, "Map Uint32 Bool");
        let result = node.get_type(&mut workspace).ok().expect("Expected a type");
        assert_eq!(
            result,
            TypeAnnotation::MapType(MapType {
                name: "Map".to_string(),
                // Note: This is a simplified representation and in a real context,
                // your application should correctly match key and value types.
                symbol: "Map<Uint32, Bool>".to_string(),
            }),
        );
        // Test address type argument
        let node = get_ast!(parser::TypeArgumentParser, "'A");
        let result = node
            .get_type(&mut workspace)
            .expect_err("Expected an error");
        // TODO: Not working assert!(matches!(result, ParsingError::UnknownTypeError(_)), "Expected unknown type error");
        // Test variable argument
        let node = get_ast!(parser::TypeArgumentParser, "'A");
        let _ = workspace.env.insert(
            "'A".to_string(),
            Box::new(TypeVar {
                instance: None,
                symbol: "'A".to_string(),
            }),
        );
        let result = node.get_type(&mut workspace).ok().expect("Expected a type");
        assert_eq!(
            result,
            TypeAnnotation::TypeVar(TypeVar {
                instance: None,
                symbol: "'A".to_string(),
            }),
        );
        // Add more test cases according to syntax of Scilla
    }

    #[test]
    fn type_inference_node_scilla_type_test() {
        let mut workspace = setup_workspace();
        // Case1: A simple type, type should be inferred as itself.
        let ident = get_ast!(parser::ScillaTypeParser, "String"); // replace with a valid expression
        assert_eq!(
            ident.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "String".to_string(),
                symbol: "String".to_string(),
            }),
        );
        // Case2: Test Custom type Type inference
        let ident = get_ast!(parser::ScillaTypeParser, "MyCustomType"); // replace with a valid expression
        assert_eq!(
            ident.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "MyCustomType".to_string(),
                symbol: "MyCustomType".to_string(),
            }),
        );
        // Case3: A variable not present in the workspace, type inference should fail.
        let ident = get_ast!(parser::ScillaTypeParser, "nonExistent"); // replace with an expression resulting to undefined type
        assert!(
            ident.get_type(&mut workspace).is_err(),
            "Expected an error when parsing non-existent type"
        );
        // Case4: Test namespace Type inference
        let ident = get_ast!(parser::ScillaTypeParser, "MyNamespace.myCustomType"); // replace with a valid expression
        let result = ident.get_type(&mut workspace);
        assert_eq!(
            result.unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "myCustomType".to_string(),
                symbol: "MyNamespace::MyCustomType".to_string()
            }),
        );
        // TODO: check that all cases are cover and add if not
    }

    #[test]
    fn type_inference_address_type_field_test() {
        // TODO: Implement
    }

    #[test]
    fn type_inference_address_type_test() {
        // Set up workspace
        let mut workspace = setup_workspace();
        // Test ByStr20 address type.
        let address_type = get_ast!(parser::AddressTypeParser, "ByStr20 with end");
        assert_eq!(
            address_type.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "ByStr20".to_string(),
                symbol: "ByStr20".to_string(),
            })
        );
        // Test ByStr33 address type with library end.
        let address_type = get_ast!(parser::AddressTypeParser, "ByStr33 with library end");
        assert_eq!(
            address_type.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "ByStr33".to_string(),
                symbol: "ByStr33".to_string(),
            })
        );
        // Test name in namespace e.g ByStr20 with contract end.
        workspace.namespace = "MyNamespace".to_string();
        let address_type = get_ast!(parser::AddressTypeParser, "ByStr20 with contract end");
        assert_eq!(
            address_type.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "ByStr20".to_string(),
                symbol: "MyNamespace::ByStr20".to_string(),
            })
        );
        // TODO: add more cases to cover the different scenarios
    }

    #[test]
    fn type_inference_message_entry_test() {
        let mut workspace = setup_workspace();
        // get mut reference to workspace
        let w = &mut workspace;
        // Parse NodeMessageEntry with a LiteraInt and check its type
        let msg_entry = get_ast!(parser::MessageEntryParser, "tag: Integer 42");
        let result = msg_entry.get_type(w);
        assert_eq!(
            result.unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "Int32".to_string(),
                symbol: "Int32".to_string(),
            }),
        );
        // Parse NodeMessageEntry with a LiteralString and check its type
        let msg_entry = get_ast!(parser::MessageEntryParser, "label: String \"String Test\"");
        let result = msg_entry.get_type(w);
        assert_eq!(
            result.unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "String".to_string(),
                symbol: "String".to_string(),
            }),
        );
        // Parse NodeMessageEntry with a LiteralHex and check its type
        let msg_entry = get_ast!(parser::MessageEntryParser, "hash: 0xABC123");
        let result = msg_entry.get_type(w);
        assert_eq!(
            result.unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "ByStr20".to_string(),
                symbol: "ByStr20".to_string(),
            }),
        );
        // Parse NodeMessageEntry with a NodeVariableIdentifier and associated type
        w.env.insert(
            "testVar".to_string(),
            Box::new(BuiltinType {
                name: "ByStr20".to_string(),
                symbol: "ByStr20".to_string(),
            }),
        );
        let msg_entry = get_ast!(parser::MessageEntryParser, "id: testVar");
        let result = msg_entry.get_type(w);
        assert_eq!(
            result.unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "ByStr20".to_string(),
                symbol: "ByStr20".to_string(),
            }),
        );
    }

    #[test]
    fn type_inference_pattern_match_expression_clause_test() {
        /*
        TODO:
            let mut workspace = Workspace {
                env: HashMap::new(),
                namespace: "".to_string()
            };
            workspace.env.insert("Int32".to_string(), Box::new(BuiltinType { name: "Int32".to_string(), symbol: "Int32".to_string() }));
            let pattern = NodePattern::WildCardNone;
            let expression = NodeLiteral::IntLiteral(42, None);
            let pattern_match_expression_clause = NodePatternMatchExpressionClause {
                pattern,
                expression,
                type_annotation: None,
            };
            assert_eq!(
                pattern_match_expression_clause.get_type(&mut workspace).unwrap(),
                TypeAnnotation::BuiltinType(BuiltinType { name: "Int32".to_string(), symbol: "Int32".to_string() })
            );
        */
    }

    fn type_inference_contract_type_arguments_test() {
        // TODO:
        /*
          let mut workspace = Workspace {
            env: HashMap::new(),
            namespace: "".to_string()
        };
            workspace.env.insert("String".to_string(), Box::new(BuiltinType { name: "String".to_string(), symbol: "String".to_string() }));
            workspace.env.insert("MyCustomType".to_string(), Box::new(TemplateType { name: "MyCustomType".to_string(), symbol: "MyCustomType".to_string() }));
            let node_contract_type_arguments = NodeContractTypeArguments {
                type_arguments: vec![
                    NodeTypeArgument::new(TypeAnnotation::LookupType(Box::new(NodePrimitiveTypeLoopup::by((1, 1),  "String".to_string())))),
                    NodeTypeArgument::new(TypeAnnotation::LookupType(Box::new(NodePrimitiveTypeLoopup::by((1, 3),  "MyCustomType".to_string())))),
                ],
                type_annotation: None
            };
            assert_eq!(
                node_contract_type_arguments.get_type(&mut workspace).unwrap(),
                TypeAnnotation::BuiltinType(BuiltinType { name: "String".to_string(), symbol: "String".to_string() })
            );
            let empty_contract_type_arguments = NodeContractTypeArguments { type_arguments: vec![], type_annotation: None };
            assert!(empty_contract_type_arguments.get_type(&mut workspace).is_err());
        */
    }

    #[test]
    fn type_inference_value_literal_test() {
        /*
        let mut workspace = Workspace {
            env: HashMap::new(),
            namespace: "".to_string()
        };
        workspace.env.insert("Int32".to_string(), Box::new(BuiltinType { name: "Int32".to_string(), symbol: "Int32".to_string() }));
        let literal = NodeValueLiteral::LiteralInt(Box::new(NodeTypeIdentifier::TypeIdentifier("Int32".to_string())), 42);
        assert_eq!(
            literal.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType { name: "Int32".to_string(), symbol: "Int32".to_string() })
        );
        let literal = NodeValueLiteral::LiteralHex("DEADBEEF".to_string());
        assert_eq!(
            literal.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType { name: "Uint128".to_string(), symbol: "Uint128".to_string() })
        );
        let literal = NodeValueLiteral::LiteralString("Hello, world!".to_string());
        assert_eq!(
            literal.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType { name: "String".to_string(), symbol: "String".to_string() })
        );
        let literal = NodeValueLiteral::LiteralEmptyMap(
            Box::new(NodeTypeIdentifier::TypeIdentifier("String".to_string())),
            Box::new(NodeTypeIdentifier::TypeIdentifier("Int32".to_string())),
        );
        assert_eq!(
            literal.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType { name: "Map (String : Int32)".to_string(), symbol: "Map (String : Int32)".to_string() })
        );
        */
    }

    // attachment: /Users/tfr/Documents/Projects/dirac.up/zilliqa-developer/products/bluebell/tests/type_inference_tests.rs
    #[test]
    fn type_inference_node_map_access_test() {
        let mut workspace = Workspace {
            env: HashMap::new(),
            namespace: "".to_string(),
        };
        workspace.env.insert(
            "MyKeyType".to_string(),
            Box::new(TemplateType {
                name: "MyKeyType".to_string(),
                symbol: "MyKeyType".to_string(),
            }),
        );
        workspace.env.insert(
            "MyValueType".to_string(),
            Box::new(TemplateType {
                name: "MyValueType".to_string(),
                symbol: "MyValueType".to_string(),
            }),
        );
        workspace.env.insert(
            "Map".to_string(),
            Box::new(BuiltinType {
                name: "MyKeyType".to_string(),
                symbol: "Map".to_string(),
            }),
        );
        let map_access = NodeMapAccess {
            identifier_name: NodeVariableIdentifier::VariableName("Map".to_string()),
            type_annotation: None,
        };
        assert_eq!(
            map_access.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "MyValueType".to_string(),
                symbol: "MyValueType".to_string(),
            })
        );
        let non_map_type_access = NodeMapAccess {
            identifier_name: NodeVariableIdentifier::VariableName("NonexistentType".to_string()),
            type_annotation: None,
        };
        assert!(non_map_type_access.get_type(&mut workspace).is_err());
    }

    #[test]
    fn type_inference_node_pattern_test() {
        let mut workspace = Workspace {
            env: HashMap::new(),
            namespace: "".to_string(),
        };
        workspace.env.insert(
            "String".to_string(),
            Box::new(BuiltinType {
                name: "String".to_string(),
                symbol: "String".to_string(),
            }),
        );
        workspace.env.insert(
            "MyCustomType".to_string(),
            Box::new(TemplateType {
                name: "MyCustomType".to_string(),
                symbol: "MyCustomType".to_string(),
            }),
        );
        let pattern = NodePattern::Binder("MyCustomType".to_string());
        assert_eq!(
            pattern.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "MyCustomType".to_string(),
                symbol: "MyCustomType".to_string()
            })
        );
        let pattern = NodePattern::Binder("NonexistentType".to_string());
        assert!(pattern.get_type(&mut workspace).is_err());
        let pattern = NodePattern::Wildcard;
        assert!(pattern.get_type(&mut workspace).is_err());
    }

    #[test]
    fn type_inference_argument_pattern_test() {
        /*
        let mut workspace = Workspace {
            env: HashMap::new(),
            namespace: "".to_string()
        };
          workspace.env.insert(
              "Int".to_string(),
              Box::new(BuiltinType {
                  name: "Int".to_string(),
                  symbol: "Int".to_string(),
              }),
          );
          workspace.env.insert(
              "MyUnionType".to_string(),
              Box::new(UnionType {
                  types: vec![
                      TypeAnnotation::BuiltinType(BuiltinType {
                          name: "Int".to_string(),
                          symbol: "Int".to_string(),
                      }),
                      TypeAnnotation::BuiltinType(BuiltinType {
                          name: "String".to_string(),
                          symbol: "String".to_string(),
                      }),
                  ],
                  symbol: "MyUnionType".to_string(),
              }),
          );
          let arg_pattern = NodeArgumentPattern::BinderArgument("Int".to_string());
          assert_eq!(
              arg_pattern.get_type(&mut workspace).unwrap(),
              TypeAnnotation::BuiltinType(BuiltinType {
                  name: "Int".to_string(),
                  symbol: "Int".to_string(),
              })
          );
          let arg_pattern = NodeArgumentPattern::BinderArgument("NonexistentType".to_string());
          assert!(arg_pattern.get_type(&mut workspace).is_err());
          let constructor = NodeConstructor {
              constructor: "MyUnionType".to_string(),
              patterns: vec![],
          };
          let arg_pattern = NodeArgumentPattern::ConstructorArgument(constructor.clone());
          assert_eq!(
              arg_pattern.get_type(&mut workspace).unwrap(),
              TypeAnnotation::UnionType(UnionType {
                  types: vec![
                      TypeAnnotation::BuiltinType(BuiltinType {
                          name: "Int".to_string(),
                          symbol: "Int".to_string(),
                      }),
                      TypeAnnotation::BuiltinType(BuiltinType {
                          name: "String".to_string(),
                          symbol: "String".to_string(),
                      }),
                  ],
                  symbol: "MyUnionType".to_string(),
              })
          );
          let pattern = NodePattern {
              constructor: constructor,
              patterns: vec![],
          };
          let arg_pattern = NodeArgumentPattern::PatternArgument(pattern);
          assert!(!arg_pattern.get_type(&mut workspace).is_err());
          */
    }

    // Assuming you have added the type_inference_tests.rs file in the same directory
    use crate::type_classes::{BuiltinType, TypeAnnotation};
    #[test]
    fn type_inference_pattern_match_clause_test() {

        /*
        let mut workspace = Workspace {
            env: HashMap::new(),
            namespace: "".to_string()
        };
        workspace.env.insert(
            "String".to_string(),
            Box::new(BuiltinType {
                name: "String".to_string(),
                symbol: "String".to_string(),
            }),
        );
        let pattern_expression = NodePattern::Variable("x".to_string());
        let statement_block = NodeBlockStatement {
            statement_list: vec![NodeStatement::Emp],
        };
        let pattern_match_clause = NodePatternMatchClause {
            pattern_expression,
            statement_block: Some(Box::new(statement_block)),
        };
        assert_eq!(
            pattern_match_clause.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "Unit".to_string(),
                symbol: "Unit".to_string(),
            })
        );
        */
    }

    #[test]
    fn type_inference_blockchain_fetch_arguments_test() {
        let mut workspace = Workspace {
            env: HashMap::new(),
            namespace: "".to_string(),
        };
        workspace.env.insert(
            "String".to_string(),
            Box::new(BuiltinType {
                name: "String".to_string(),
                symbol: "String".to_string(),
            }),
        );
        workspace.env.insert(
            "MyCustomType".to_string(),
            Box::new(TemplateType {
                name: "MyCustomType".to_string(),
                symbol: "MyCustomType".to_string(),
            }),
        );

        let args = NodeBlockchainFetchArguments {
            arguments: vec![
                NodeVariableIdentifier::VariableName("String".to_string()),
                NodeVariableIdentifier::VariableName("MyCustomType".to_string()),
            ],
            type_annotation: None,
        };

        assert_eq!(
            args.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "NodeBlockchainFetchArguments".to_string(),
                symbol: "NodeBlockchainFetchArguments".to_string()
            })
        );
    }

    #[test]
    fn type_inference_node_statement_test() {
        /*
            let mut workspace = Workspace {
                env: HashMap::new(),
                namespace: "".to_string()
            };
            // ...populate the workspace.environment with types...
            // Create NodeStatement instances and test their type inference
            let statement = NodeStatement::Bind {
                left_hand_side: "test".to_string(),
                right_hand_side: Box::new(/* NodeFullExpression instance */),
            };

          // TODO:
        */
    }
    #[test]
    fn type_inference_remote_fetch_statement_test() {
        let mut workspace = Workspace {
            env: HashMap::new(),
            namespace: "".to_string(),
        };
        workspace.env.insert(
            "String".to_string(),
            Box::new(BuiltinType {
                name: "String".to_string(),
                symbol: "String".to_string(),
            }),
        );
        workspace.env.insert(
            "MyCustomType".to_string(),
            Box::new(TemplateType {
                name: "MyCustomType".to_string(),
                symbol: "MyCustomType".to_string(),
            }),
        );
        let ident = NodeVariableIdentifier::VariableName("MyCustomType".to_string());
        let fetch_stmt = NodeRemoteFetchStatement::ReadStateMutable(
            "ContractA".to_string(),
            "someVariable".to_string(),
            ident.clone(),
        );
        assert_eq!(
            fetch_stmt.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "MyCustomType".to_string(),
                symbol: "MyCustomType".to_string(),
            }),
        );
        let ident = NodeVariableIdentifier::VariableName("NonexistentType".to_string());
        let fetch_stmt = NodeRemoteFetchStatement::ReadStateMutable(
            "ContractA".to_string(),
            "someVariable".to_string(),
            ident,
        );
        assert!(fetch_stmt.get_type(&mut workspace).is_err());
    }

    #[test]
    fn type_inference_node_component_id_test() {
        /*
        let mut workspace = Workspace {
            env: HashMap::new(),
            namespace: "".to_string()
        };
        workspace.env.insert("String".to_string(), Box::new(BuiltinType { name: "String".to_string(), symbol: "String".to_string() }));
        workspace.env.insert("MyCustomType".to_string(), Box::new(TemplateType { name: "MyCustomType".to_string(), symbol: "MyCustomType".to_string() }));
        let cid = NodeComponentId::WithTypeLikeName(NodeTypeNameIdentifier::new("MyCustomType".to_string()));
        assert_eq!(
            cid.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TemplateType(TemplateType { name: "MyCustomType".to_string(), symbol: "MyCustomType".to_string() })
        );
        let cid = NodeComponentId::WithRegularId("String".to_string());
        assert_eq!(
            cid.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType { name: "String".to_string(), symbol: "String".to_string() })
        );
        let cid = NodeComponentId::WithRegularId("NonexistentType".to_string());
        assert!(cid.get_type(&mut workspace).is_err());
        */
    }

    #[test]
    fn type_inference_node_component_parameters_test() {
        /*
        let mut workspace = Workspace {
            env: HashMap::new(),
            namespace: "".to_string()
        };
          workspace.env.insert("Int64".to_string(), Box::new(BuiltinType { name: "Int64".to_string(), symbol: "Int64".to_string() }));
          let parameters = vec![NodeParameterPair {
              key: "b".to_string(),
              value: NodeLiteral::IntegerLiteral(123),
          }];
          let type_annotation = Some(TypeAnnotation::BuiltinType(BuiltinType { name: "Int64".to_string(), symbol: "Int64".to_string() }));
          let component_parameters = NodeComponentParameters { parameters, type_annotation };
          assert_eq!(
              component_parameters.get_type(&mut workspace).unwrap(),
              TypeAnnotation::BuiltinType(BuiltinType { name: "Int64".to_string(), symbol: "Int64".to_string() }),
          );
          let component_parameters_without_type_annotation = NodeComponentParameters { parameters: parameters.clone(), type_annotation: None };
          assert!(component_parameters_without_type_annotation.get_type(&mut workspace).is_err());
        */
    }

    #[test]
    fn type_inference_parameter_pair_test() {

        /*
        let mut workspace = Workspace {
            env: HashMap::new(),
            namespace: "".to_string()
        };
        workspace.env.insert(
            "String".to_string(),
            Box::new(BuiltinType {
                name: "String".to_string(),
                symbol: "String".to_string(),
            }),
        );
        workspace.env.insert(
            "MyCustomType".to_string(),
            Box::new(TemplateType {
                name: "MyCustomType".to_string(),
                symbol: "MyCustomType".to_string(),
            }),
        );
        let typed_identifier = NodeTypedIdentifier::new("testParam".to_string(), TypeAnnotation::TemplateType(TemplateType { name: "MyCustomType".to_string(), symbol: "MyCustomType".to_string() }));
        let parameter_pair = NodeParameterPair {
            identifier_with_type: typed_identifier,
            type_annotation: None,
        };
        assert_eq!(
            parameter_pair.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TemplateType(TemplateType { name: "MyCustomType".to_string(), symbol: "MyCustomType".to_string() })
        );
        */
    }

    #[test]
    fn type_inference_component_body_test() {
        /*
            let mut workspace = Workspace {
                env: HashMap::new(),
                namespace: "".to_string()
            };
            workspace.env.insert(
                "Bool".to_string(),
                Box::new(BuiltinType {
                    name: "Bool".to_string(),
                    symbol: "Bool".to_string(),
                }),
            );
            let statement_block = NodeStatementBlock { statements: vec![] };
            let component_body = NodeComponentBody {
                statement_block: Some(statement_block),
                type_annotation: Some(TypeAnnotation::BuiltinType(BuiltinType {
                    name: "Bool".to_string(),
                    symbol: "Bool".to_string(),
                })),
            };
            assert_eq!(
                component_body.get_type(&mut workspace).unwrap(),
                TypeAnnotation::BuiltinType(BuiltinType {
                    name: "Bool".to_string(),
                    symbol: "Bool".to_string(),
                })
            );
            let component_body_no_type_annotation = NodeComponentBody {
                statement_block: None,
                type_annotation: None,
            };
            assert!(component_body_no_type_annotation.get_type(&mut workspace).is_err());
        */
    }

    #[test]
    fn type_inference_statement_block_test() {
        /*
        let mut workspace = Workspace {
            env: HashMap::new(),
            namespace: "".to_string()
        };
        workspace.env.insert(
            "Int32".to_string(),
            Box::new(BuiltinType {
                name: "Int32".to_string(),
                symbol: "Int32".to_string(),
            }),
        );
        workspace.env.insert(
            "String".to_string(),
            Box::new(BuiltinType {
                name: "String".to_string(),
                symbol: "String".to_string(),
            }),
        );
        let statement_block = NodeStatementBlock {
            statements: vec![
                NodeStatement::Assignment(Box::new(AssignPattern::Ident(
                    "a".to_string(),
                )), Box::new(NodeLiteral::Int32Literal(42))),
                NodeStatement::Expression(Box::new(
                    NodeLiteral::StringLiteral("Hello, world!".to_string()),
                )),
            ],
            type_annotation: None,
        };
        assert_eq!(
            statement_block.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "String".to_string(),
                symbol: "String".to_string(),
            }),


        */
    }

    #[test]
    fn type_inference_typed_identifier_test() {
        /*
        let mut workspace = Workspace {
            env: HashMap::new(),
            namespace: "".to_string()
        };
        let t1_annotation = TypeAnnotation::BuiltinType(BuiltinType { name: "Int".to_string(), symbol: "Int".to_string() });
        let typed_ident = NodeTypedIdentifier {
            identifier_name: "x".to_string(),
            annotation: NodeTypeAnnotation::ScillaType(ScillaType::Int),
            type_annotation: Some(t1_annotation.clone()),
        };
        assert_eq!(typed_ident.get_type(&mut workspace).unwrap(), t1_annotation);
        let typed_ident_no_annotation = NodeTypedIdentifier {
            identifier_name: "y".to_string(),
            annotation: NodeTypeAnnotation::ScillaType(ScillaType::Int),
            type_annotation: None,
        };
        assert!(typed_ident_no_annotation.get_type(&mut workspace).is_err());
        */
    }

    #[test]
    fn type_inference_type_annotation_test() {
        /*
          let mut workspace = Workspace {
            env: HashMap::new(),
            namespace: "".to_string()
        };
            workspace.env.insert("String".to_string(), Box::new(BuiltinType { name: "String".to_string(), symbol: "String".to_string() }));
            workspace.env.insert("MyCustomType".to_string(), Box::new(TemplateType { name: "MyCustomType".to_string(), symbol: "MyCustomType".to_string() }));
            let type_annotation = NodeTypeAnnotation {
                type_name: NodeScillaType::TypeName("MyCustomType".to_string()),
                type_annotation: None,
            };
            assert_eq!(
                type_annotation.get_type(&mut workspace).unwrap(),
                TypeAnnotation::TemplateType(TemplateType { name: "MyCustomType".to_string(), symbol: "MyCustomType".to_string() })
            );
            let type_annotation = NodeTypeAnnotation {
                type_name: NodeScillaType::TypeName("NonexistentType".to_string()),
                type_annotation: None,
            };
            assert!(type_annotation.get_type(&mut workspace).is_err());
        */
    }

    // Attachment #9 - `/tests/type_inference_tests.rs`
    #[test]
    fn type_inference_node_program_test() {
        /*
            let mut workspace = Workspace {
                env: HashMap::new(),
                namespace: "".to_string()
            };
            workspace.env.insert(
                "String".to_string(),
                Box::new(BuiltinType {
                    name: "String".to_string(),
                    symbol: "String".to_string(),
                }),
            );
            workspace.env.insert(
                "MyCustomType".to_string(),
                Box::new(TemplateType {
                    name: "MyCustomType".to_string(),
                    symbol: "MyCustomType".to_string(),
                }),
            );
            let version = "0".to_string();
            let import_declarations = None;
            let library_definition = None;
            let contract_definition = create_custom_contract_definition(); // You need to replace this with a valid NodeContractDefinition instance
            let program = NodeProgram {
                version,
                import_declarations,
                library_definition,
                contract_definition,
                type_annotation: None,
            };
            let inferred_type = program.get_type(&mut workspace);
            // Replace the expected_type with the correct type based on the contract_definition.
            let expected_type = TypeAnnotation::TemplateType(TemplateType {
                name: "MyCustomType".to_string(),
                symbol: "MyCustomType".to_string(),
            });
            assert_eq!(inferred_type.unwrap(), expected_type);
        */
    }

    #[test]
    fn type_inference_library_definition_test() {
        /*
        let mut workspace = Workspace {
            env: HashMap::new(),
            namespace: "".to_string()
        };
          workspace.env.insert("LibraryType".to_string(), Box::new(TemplateType { name: "LibraryType".to_string(), symbol: "LibraryType".to_string() }));
          // Case where the type annotation is defined
          let lib_def = NodeLibraryDefinition {
              name: NodeTypeNameIdentifier(vec![TokenIdent("MyLibrary".to_string())]),
              definitions: vec![],
              type_annotation: Some(TypeAnnotation::TemplateType(TemplateType { name: "LibraryType".to_string(), symbol: "LibraryType".to_string() })),
          };
          assert_eq!(
              lib_def.get_type(&mut workspace).unwrap(),
              TypeAnnotation::TemplateType(TemplateType { name: "LibraryType".to_string(), symbol: "LibraryType".to_string() })
          );
          // Case where the type annotation is not defined
          let lib_def = NodeLibraryDefinition {
              name: NodeTypeNameIdentifier(vec![TokenIdent("MyLibrary".to_string())]),
              definitions: vec![],
              type_annotation: None,
          };
          assert!(lib_def.get_type(&mut workspace).is_err());
          */
    }

    #[test]
    fn type_inference_library_single_definition_test() {
        /*
        let mut workspace = Workspace {
            env: HashMap::new(),
            namespace: "".to_string()
        };
        workspace.env.insert("MyCustomType".to_string(), Box::new(TemplateType { name: "MyCustomType".to_string(), symbol: "MyCustomType".to_string() }));
        workspace.env.insert("MyCustomTypeA".to_string(), Box::new(TemplateType { name: "MyCustomTypeA".to_string(), symbol: "MyCustomTypeA".to_string() }));
        workspace.env.insert("MyCustomTypeB".to_string(), Box::new(TemplateType { name: "MyCustomTypeB".to_string(), symbol: "MyCustomTypeB".to_string() }));
        let type_definition = NodeLibrarySingleDefinition::TypeDefinition(
            NodeTypeNameIdentifier::TypeName("MyCustomType".to_string()),
            Some(vec![
                NodeTypeAlternativeClause::TypeAlternativeClause("MyCustomTypeA".to_string()),
                NodeTypeAlternativeClause::TypeAlternativeClause("MyCustomTypeB".to_string())
            ]),
        );
        assert_eq!(
            type_definition.get_type(&mut workspace).unwrap(),
            TypeAnnotation::UnionType(UnionType {
                types: vec![
                    TypeAnnotation::TemplateType(TemplateType { name: "MyCustomTypeA".to_string(), symbol: "MyCustomTypeA".to_string() }),
                    TypeAnnotation::TemplateType(TemplateType { name: "MyCustomTypeB".to_string(), symbol: "MyCustomTypeB".to_string() }),
                ],
                symbol: "MyCustomType".to_string()
            })
        );
        */
    }

    #[test]
    fn type_inference_contract_definition_test() {
        /*
            let mut workspace = Workspace {
                env: HashMap::new(),
                namespace: "".to_string()
            };
            workspace.env.insert(
                "ContractType".to_string(),
                Box::new(TemplateType {
                    name: "ContractType".to_string(),
                    symbol: "ContractType".to_string(),
                }),
            );
            let contract_def_no_type_annotation = create_test_contract_definition();
            // Assuming `create_test_contract_definition` is a function that returns a NodeContractDefinition without type_annotation.
            assert!(contract_def_no_type_annotation.get_type(&mut workspace).is_err());
            let contract_def_with_type_annotation = create_test_contract_definition_with_type_annotation();
            // Assuming `create_test_contract_definition_with_type_annotation` is a function that returns a NodeContractDefinition with type_annotation.
            assert_eq!(
                contract_def_with_type_annotation.get_type(&mut workspace).unwrap(),
                TypeAnnotation::TemplateType(TemplateType {
                    name: "ContractType".to_string(),
                    symbol: "ContractType".to_string(),
                })
            );
        */
    }

    #[test]
    fn type_inference_contract_field_test() {
        /*
        let mut workspace = Workspace {
            env: HashMap::new(),
            namespace: "".to_string()
        };
          workspace.env.insert("Int32".to_string(), Box::new(BuiltinType { name: "Int32".to_string(), symbol: "Int32".to_string() }));
          let typed_identifier = NodeTypedIdentifier {
              name: "x".to_string(),
              ty: NodeTypeName::UserDefinedTypeName("Int32".to_string()),
          };
          let right_hand_side = NodeFullExpression::Leaf(NodeLiteralInt32(42));
          let contract_field = NodeContractField {
              typed_identifier,
              right_hand_side,
              type_annotation: None,
          };
          assert_eq!(
              contract_field.get_type(&mut workspace).unwrap(),
              TypeAnnotation::BuiltinType(BuiltinType { name: "Int32".to_string(), symbol: "Int32".to_string() })
          );
          */
    }

    // Add to your `type_inference_tests.rs` file
    #[test]
    fn type_inference_with_constraint_test() {
        /*
        let mut workspace = Workspace {
            env: HashMap::new(),
            namespace: "".to_string()
        };
        workspace.env.insert(
            "Int".to_string(),
            Box::new(BuiltinType {
                name: "Int".to_string(),
                symbol: "Int".to_string(),
            }),
        );
        let expression = NodeFullExpression::LiteralInt(NodeLiteralInt { value: 1234 });
        let with_constraint = NodeWithConstraint {
            expression: Box::new(expression),
            type_annotation: None,
        };
        assert_eq!(
            with_constraint.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "Int".to_string(),
                symbol: "Int".to_string(),
            })
        );
        */
    }

    #[test]
    fn type_inference_component_definition_test() {
        /*
          let mut workspace = Workspace {
            env: HashMap::new(),
            namespace: "".to_string()
        };
            workspace.env.insert(
                "String".to_string(),
                Box::new(BuiltinType {
                    name: "String".to_string(),
                    symbol: "String".to_string(),
                }),
            );
            let tran_def = NodeTransitionDefinition {...}; // create a NodeTransitionDefinition instance
            let proc_def = NodeProcedureDefinition {...}; // create a NodeProcedureDefinition instance
            let comp_def_tran = NodeComponentDefinition::TransitionComponent(Box::new(tran_def));
            assert!(
                comp_def_tran.get_type(&mut workspace).is_ok(),
                "Expected type inference for NodeComponentDefinition::TransitionComponent to be successful"
            );
            let comp_def_procedure = NodeComponentDefinition::ProcedureComponent(Box::new(proc_def));
            assert!(
                comp_def_procedure.get_type(&mut workspace).is_ok(),
                "Expected type inference for NodeComponentDefinition::ProcedureComponent to be successful"
            );
        */
    }
}
