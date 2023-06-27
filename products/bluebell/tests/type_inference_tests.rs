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
            "keyValueStore".to_string(),
            Box::new(MapType {
                name: "keyValueStore".to_string(),
                symbol: "Map<String, Map<String, Int32>>".to_string(),
                key_type: Some(Box::new(TypeAnnotation::BuiltinType(BuiltinType {
                    name: "String".to_string(),
                    symbol: "String".to_string(),
                }))),
                value_type: Some(Box::new(TypeAnnotation::MapType(MapType {
                    name: "".to_string(),
                    symbol: "Map<String, Int32>".to_string(),
                    key_type: Some(Box::new(TypeAnnotation::BuiltinType(BuiltinType {
                        name: "String".to_string(),
                        symbol: "String".to_string(),
                    }))),
                    value_type: Some(Box::new(TypeAnnotation::BuiltinType(BuiltinType {
                        name: "Int32".to_string(),
                        symbol: "Int32".to_string(),
                    }))),
                }))),
            }),
        );

        workspace.env.insert(
            "MyTypeOrEnumLikeIdentifier".to_string(),
            Box::new(TemplateType {
                name: "MyTypeOrEnumLikeIdentifier".to_string(),
                symbol: "MyTypeOrEnumLikeIdentifier".to_string(),
            }),
        );
        workspace.env.insert(
            "MyNamespace".to_string(),
            Box::new(UnionType {
                // TODO: Come up with namespace type
                name: "MyTypeOrEnumLikeIdentifier".to_string(),
                types: [].to_vec(),
                symbol: "MyTypeOrEnumLikeIdentifier".to_string(),
            }),
        );

        workspace.env.insert(
            "MyNamespace::MyTypeOrEnumLikeIdentifier".to_string(),
            Box::new(TemplateType {
                name: "MyTypeOrEnumLikeIdentifier".to_string(),
                symbol: "MyNamespace::MyTypeOrEnumLikeIdentifier".to_string(),
            }),
        );

        workspace.env.insert(
            "myVariable".to_string(),
            Box::new(TypeVar {
                instance: None,
                symbol: "Map<String, String>".to_string(),
            }),
        );
        workspace.env.insert(
            "MyNamespace".to_string(),
            Box::new(NamespaceType {
                // TODO: Come up with namespace type
                name: "MyTypeOrEnumLikeIdentifier".to_string(),
                symbol: "MyTypeOrEnumLikeIdentifier".to_string(),
            }),
        );

        workspace.env.insert(
            "MyNamespace::myVariable".to_string(),
            Box::new(TypeVar {
                instance: None,
                symbol: "Int32".to_string(),
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
        let ident = get_ast!(parser::VariableIdentifierParser, "myVariable");
        assert_eq!(
            ident.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TypeVar(TypeVar {
                instance: None,
                symbol: "Map<String, String>".to_string()
            }),
        );

        // Base type resolution
        let ident = get_ast!(parser::VariableIdentifierParser, "MyNamespace.myVariable");
        assert_eq!(
            ident.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TypeVar(TypeVar {
                instance: None,
                symbol: "Int32".to_string()
            }),
        );

        // Type in namespace
        workspace.namespace = "MyNamespace".to_string();

        let ident = get_ast!(parser::VariableIdentifierParser, "myVariable");
        assert_eq!(
            ident.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TypeVar(TypeVar {
                instance: None,
                symbol: "Int32".to_string()
            }),
        );

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
        let ident = get_ast!(
            parser::TypeNameIdentifierParser,
            "MyTypeOrEnumLikeIdentifier"
        );
        assert_eq!(
            ident.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "MyTypeOrEnumLikeIdentifier".to_string(),
                symbol: "MyTypeOrEnumLikeIdentifier".to_string()
            })
        );
        // Type in namespace
        workspace.namespace = "MyNamespace".to_string();
        let ident = get_ast!(
            parser::TypeNameIdentifierParser,
            "MyTypeOrEnumLikeIdentifier"
        );
        assert_eq!(
            ident.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "MyTypeOrEnumLikeIdentifier".to_string(),
                symbol: "MyNamespace::MyTypeOrEnumLikeIdentifier".to_string()
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
        let ident = get_ast!(parser::ImportedNameParser, "MyTypeOrEnumLikeIdentifier");
        assert_eq!(
            ident.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "MyTypeOrEnumLikeIdentifier".to_string(),
                symbol: "MyTypeOrEnumLikeIdentifier".to_string(),
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
        let ident = get_ast!(
            parser::ImportedNameParser,
            "MyNamespace::MyTypeOrEnumLikeIdentifier"
        );
        assert_eq!(
            ident.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "MyTypeOrEnumLikeIdentifier".to_string(),
                symbol: "MyNamespace::MyTypeOrEnumLikeIdentifier".to_string(),
            })
        );
    }

    #[test]
    fn type_inference_import_declarations_test() {
        // we reference our workspace setup function from Attachment #1
        let mut workspace = setup_workspace();
        // scenario: we have valid import declarations in our code
        let node_import_declaration_str = r#"
    import MyTypeOrEnumLikeIdentifier
    import MyNamespace.MyTypeOrEnumLikeIdentifier
    "#;
        // getting the relevant AST Node => NodeImportDeclarations
        let node_import_declarations = get_ast!(
            parser::ImportDeclarationsParser,
            node_import_declaration_str
        );
        // this should be successful as we imported valid types
        assert!(node_import_declarations.get_type(&mut workspace).is_ok());
        // Check the imported types
        assert!(workspace.env.contains_key("MyTypeOrEnumLikeIdentifier"));
        assert!(workspace
            .env
            .contains_key("MyNamespace::MyTypeOrEnumLikeIdentifier"));
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
        let ident = get_ast!(
            parser::MetaIdentifierParser,
            "MyNamespace::MyTypeOrEnumLikeIdentifier"
        );
        assert_eq!(
            ident.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "MyTypeOrEnumLikeIdentifier".to_string(),
                symbol: "MyNamespace::MyTypeOrEnumLikeIdentifier".to_string()
            })
        );
        // Parsing and inferring type in a Hexspace
        workspace.namespace = "MyNamespace".to_string(); // Ensuring we are in the right namespace
        let ident = get_ast!(
            parser::MetaIdentifierParser,
            "0x1234.MyTypeOrEnumLikeIdentifier"
        );
        assert_eq!(
            ident.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "MyTypeOrEnumLikeIdentifier".to_string(),
                symbol: "0x1234.MyTypeOrEnumLikeIdentifier".to_string()
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
                "MyTypeOrEnumLikeIdentifier".to_string(),
            )],
            type_annotation: None,
        };
        assert_eq!(
            node_builtin_arguments.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "MyTypeOrEnumLikeIdentifier".to_string(),
                symbol: "MyTypeOrEnumLikeIdentifier".to_string(),
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
        let ident = get_ast!(
            parser::TypeMapKeyParser,
            "(MyNamespace::MyTypeOrEnumLikeIdentifier)"
        );
        assert_eq!(
            ident.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "MyTypeOrEnumLikeIdentifier".to_string(),
                symbol: "MyNamespace::MyTypeOrEnumLikeIdentifier".to_string(),
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
        let mapvalue = get_ast!(
            parser::TypeMapValueParser,
            "MapValueTypeOrEnumLikeIdentifier Uint32"
        );
        let result = mapvalue.get_type(&mut workspace);
        assert_eq!(
            result.unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "MapValueTypeOrEnumLikeIdentifier".to_string(),
                symbol: "Uint32".to_string(),
            }),
            "Expected 'MapValueTypeOrEnumLikeIdentifier' for a map value of custom type 'Uint32'"
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
        let node = get_ast!(parser::TypeArgumentParser, "MyTypeOrEnumLikeIdentifier");
        let result = node.get_type(&mut workspace).ok().expect("Expected a type");
        assert_eq!(
            result,
            TypeAnnotation::TemplateType(TemplateType {
                name: "MyTypeOrEnumLikeIdentifier".to_string(),
                symbol: "MyTypeOrEnumLikeIdentifier".to_string(),
            }),
        );
        // Test Map type argument
        let node = get_ast!(parser::TypeArgumentParser, "Map Uint32 Bool");
        let result = node.get_type(&mut workspace).ok().expect("Expected a type");
        assert_eq!(
            result,
            TypeAnnotation::MapType(MapType {
                name: "Map".to_string(),
                key_type: None,
                value_type: None,
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
        let ident = get_ast!(parser::ScillaTypeParser, "MyTypeOrEnumLikeIdentifier"); // replace with a valid expression
        assert_eq!(
            ident.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "MyTypeOrEnumLikeIdentifier".to_string(),
                symbol: "MyTypeOrEnumLikeIdentifier".to_string(),
            }),
        );
        // Case3: A variable not present in the workspace, type inference should fail.
        let ident = get_ast!(parser::ScillaTypeParser, "nonExistent"); // replace with an expression resulting to undefined type
        assert!(
            ident.get_type(&mut workspace).is_err(),
            "Expected an error when parsing non-existent type"
        );
        // Case4: Test namespace Type inference
        let ident = get_ast!(
            parser::ScillaTypeParser,
            "MyNamespace.MyTypeOrEnumLikeIdentifier"
        ); // replace with a valid expression
        let result = ident.get_type(&mut workspace);
        assert_eq!(
            result.unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "MyTypeOrEnumLikeIdentifier".to_string(),
                symbol: "MyNamespace::MyTypeOrEnumLikeIdentifier".to_string()
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
        let mut workspace = setup_workspace();
        // Testing pattern match on variable of union type
        let ident = get_ast!(
            parser::PatternMatchExpressionClauseParser,
            "| MyTypeOrEnumLikeIdentifier Foo => Uint32 42"
        );
        let result = ident.get_type(&mut workspace);
        assert_eq!(
            result.unwrap(),
            TypeAnnotation::UnionType(UnionType {
                name: "MyTypeOrEnumLikeIdentifier".to_string(),
                types: [].to_vec(),
                symbol: "MyTypeOrEnumLikeIdentifier".to_string(),
            })
        );
        // Testing pattern match on variable of builtin type
        let ident = get_ast!(
            parser::PatternMatchExpressionClauseParser,
            "| _ => Uint32 42"
        );
        let result = ident.get_type(&mut workspace);
        assert_eq!(
            result.unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "Uint32".to_string(),
                symbol: "Uint32".to_string(),
            })
        );
        // Testing pattern match on variable of custom type (user-defined)
        workspace.namespace = "MyNamespace".to_string();
        let ident = get_ast!(
            parser::PatternMatchExpressionClauseParser,
            "| MyNamespace::MyTypeOrEnumLikeIdentifier _ => MyNamespace::MyTypeOrEnumLikeIdentifier _"
        );
        let result = ident.get_type(&mut workspace);
        assert_eq!(
            result.unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "MyTypeOrEnumLikeIdentifier".to_string(),
                symbol: "MyNamespace::MyTypeOrEnumLikeIdentifier".to_string(),
            })
        );
        // Testing pattern match on variable of builtin type in a namespace
        let ident = get_ast!(
            parser::PatternMatchExpressionClauseParser,
            "| String _ => String _"
        );
        let result = ident.get_type(&mut workspace);
        assert_eq!(
            result.unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "String".to_string(),
                symbol: "String".to_string(),
            })
        );
    }

    fn type_inference_contract_type_arguments_test() {
        // TODO: Implement
    }

    #[test]
    fn type_inference_value_literal_test() {
        let mut workspace = setup_workspace();
        // Literal Int Test
        let literal_int = get_ast!(parser::ValueLiteralParser, "Uint32 42");
        assert_eq!(
            literal_int.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "Uint32".to_string(),
                symbol: "Uint32".to_string()
            }),
        );
        // Literal Hex Test
        let literal_hex = get_ast!(parser::ValueLiteralParser, "0x123abc");
        assert_eq!(
            literal_hex.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "ByStr".to_string(),
                symbol: "ByStr".to_string()
            }),
        );
        // Literal String Test
        let literal_string = get_ast!(parser::ValueLiteralParser, r#""string""#);
        assert_eq!(
            literal_string.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "String".to_string(),
                symbol: "String".to_string()
            }),
        );
        // Empty Map Test: In Scilla, the empty map is represented by LiteraEmptyMap(NodeTypeMapKey, NodeTypeMapValue)
        // Assuming we have a map of String -> Uint32
        let empty_map = get_ast!(parser::ValueLiteralParser, "Emp String Uint32");
        assert_eq!(
            empty_map.get_type(&mut workspace).unwrap(),
            TypeAnnotation::MapType(MapType {
                name: "Emp".to_string(),
                symbol: "Emp::String::Uint32".to_string(),
                key_type: None,
                value_type: None,
            }),
        );
        // TODO: Add more cases to cover other Scilla literal types
    }

    #[test]
    fn type_inference_node_map_access_test() {
        let mut workspace = setup_workspace();
        let map_ident = get_ast!(parser::MapAccessParser, "[myVariable]");
        let result = map_ident.get_type(&mut workspace);
        assert!(
            result.is_err(),
            "Expected an error when since MapAccess does not have a type in its own right."
        );
    }

    #[test]
    fn type_inference_node_pattern_test() {
        let mut workspace = setup_workspace();
        // Pattern: Wildcard
        let pattern = get_ast!(parser::PatternParser, "_");
        assert_eq!(
            pattern.get_type(&mut workspace).unwrap(),
            TypeAnnotation::Void
        );
        // Pattern: Binder
        let pattern = get_ast!(parser::PatternParser, "foo");
        let result = pattern.get_type(&mut workspace);
        assert!(
            result.is_err(),
            "Expected an error when parsing a Binder with unbounded name"
        );
        // Pattern: Constructor
        workspace.namespace = "MyNamespace".to_string();
        let pattern = get_ast!(parser::PatternParser, "MyTypeOrEnumLikeIdentifier()");
        assert_eq!(
            pattern.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "MyTypeOrEnumLikeIdentifier".to_string(),
                symbol: "MyNamespace::MyTypeOrEnumLikeIdentifier".to_string(),
            }),
        );
        // ArgumentPattern: WildcardArgument
        let arg_pattern = get_ast!(parser::ArgumentPatternParser, "_");
        assert_eq!(
            arg_pattern.get_type(&mut workspace).unwrap(),
            TypeAnnotation::Void
        );
        // ArgumentPattern: BinderArgument
        let arg_pattern = get_ast!(parser::ArgumentPatternParser, "foo");
        let result = arg_pattern.get_type(&mut workspace);
        assert!(
            result.is_err(),
            "Expected an error when parsing a BinderArgument with unbounded name"
        );
        // ArgumentPattern: ConstructorArgument with namespace
        let arg_pattern = get_ast!(parser::ArgumentPatternParser, "MyTypeOrEnumLikeIdentifier");
        assert_eq!(
            arg_pattern.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "MyTypeOrEnumLikeIdentifier".to_string(),
                symbol: "MyNamespace::MyTypeOrEnumLikeIdentifier".to_string(),
            }),
        );
        // ArgumentPattern: PatternArgument
        let arg_pattern = get_ast!(
            parser::ArgumentPatternParser,
            "MyTypeOrEnumLikeIdentifier()"
        );
        assert_eq!(
            arg_pattern.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "MyTypeOrEnumLikeIdentifier".to_string(),
                symbol: "MyNamespace::MyTypeOrEnumLikeIdentifier".to_string(),
            }),
        );
    }

    #[test]
    fn type_inference_argument_pattern_test() {
        let mut workspace = setup_workspace();
        // Test for variable identifier
        let argument_pattern = get_ast!(parser::ArgumentPatternParser, "_foobar");
        let result = argument_pattern.get_type(&mut workspace);
        assert_eq!(
            result.unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "_foobar".to_string(),
                symbol: "_foobar".to_string()
            }),
            "Expected correct type from argument pattern."
        );

        // Test for constructor pattern
        let argument_pattern = get_ast!(parser::ArgumentPatternParser, "(Pair _foobar _barfoo)");
        let result = argument_pattern.get_type(&mut workspace);
        assert_eq!(
            result.unwrap(),
            TypeAnnotation::UnionType(UnionType {
                name: "Pair".to_string(),
                types: vec![
                    TypeAnnotation::TemplateType(TemplateType {
                        name: "_foobar".to_string(),
                        symbol: "_foobar".to_string()
                    }),
                    TypeAnnotation::TemplateType(TemplateType {
                        name: "_barfoo".to_string(),
                        symbol: "_barfoo".to_string()
                    })
                ],
                symbol: "Pair<_foobar, _barfoo>".to_string()
            }),
            "Expected correct type from argument pattern."
        );
        // Test for invalid argument pattern
        let argument_pattern = get_ast!(parser::ArgumentPatternParser, "(Pair _foo2bar _bar2foo)");
        let result = argument_pattern.get_type(&mut workspace);
        assert!(
            result.is_err(),
            "Expected an error when parsing invalid argument pattern"
        );
    }

    #[test]
    fn type_inference_pattern_match_clause_test() {
        // prepare workspace
        let mut workspace = setup_workspace();
        // valid Scilla pattern match expression: "| _ => Uint32 42"
        let pattern_match = get_ast!(
            parser::PatternMatchExpressionClauseParser,
            "| _ => Uint32 42"
        );
        let result = pattern_match.get_type(&mut workspace);
        assert!(
            result.is_ok(),
            "Expected an ok when parsing valid pattern match expression"
        );
        // valid Scilla pattern match expression: "| Foo => Uint32 42"
        let pattern_match = get_ast!(
            parser::PatternMatchExpressionClauseParser,
            "| Foo => Uint32 42"
        );
        let result = pattern_match.get_type(&mut workspace);
        assert!(
            result.is_ok(),
            "Expected an ok when parsing valid pattern match expression"
        );
        // invalid Scilla pattern match expression: "| Foo => "
        let pattern_match = get_ast!(parser::PatternMatchExpressionClauseParser, "| Foo => ");
        let result = pattern_match.get_type(&mut workspace);
        assert!(
            result.is_err(),
            "Expected an error when parsing invalid pattern match expression"
        );
        // TODO: add more cases to cover the different scenarios
    }

    #[test]
    fn type_inference_blockchain_fetch_arguments_test() {
        let mut workspace = setup_workspace();
        // Parsing simple arguments
        let node = get_ast!(parser::BlockchainFetchArgumentsParser, "(a b c)");
        let args = match node {
            NodeBlockchainFetchArguments { arguments, .. } => arguments,
        };
        assert_eq!(args.len(), 3);
        assert_eq!(args[0].to_string(), "a");
        assert_eq!(args[1].to_string(), "b");
        assert_eq!(args[2].to_string(), "c");
        // Parsing argument with namespace
        let node = get_ast!(
            parser::BlockchainFetchArgumentsParser,
            "(MyNamespace::a MyNamespace::b MyNamespace::c)"
        );
        let args = match node {
            NodeBlockchainFetchArguments { arguments, .. } => arguments,
        };
        assert_eq!(args.len(), 3);
        assert_eq!(args[0].to_string(), "MyNamespace::a");
        assert_eq!(args[1].to_string(), "MyNamespace::b");
        assert_eq!(args[2].to_string(), "MyNamespace::c");
        // Note: This test only checks whether the parsing logic of BlockchainFetchArgumentsParser works correctly.
        // It does not verify the type inference logic.
    }

    #[test]
    fn type_inference_node_statement_map_get_test() {
        let mut workspace = setup_workspace();
        assert!(!workspace.env.contains_key("value1"));
        assert!(!workspace.env.contains_key("value2"));

        let access_map = get_ast!(parser::StatementParser, "value1 <- keyValueStore[key]");

        assert_eq!(
            access_map.get_type(&mut workspace).unwrap(),
            TypeAnnotation::MapType(MapType {
                name: "".to_string(),
                symbol: "Map<String, Int32>".to_string(),
                key_type: Some(Box::new(TypeAnnotation::BuiltinType(BuiltinType {
                    name: "String".to_string(),
                    symbol: "String".to_string(),
                }))),
                value_type: Some(Box::new(TypeAnnotation::BuiltinType(BuiltinType {
                    name: "Int32".to_string(),
                    symbol: "Int32".to_string(),
                }))),
            })
        );
        assert!(workspace.env.contains_key("value1"));
        assert!(!workspace.env.contains_key("value2"));
        assert_eq!(
            workspace.env.get("value1").unwrap().get_instance(),
            TypeAnnotation::MapType(MapType {
                name: "".to_string(),
                symbol: "Map<String, Int32>".to_string(),
                key_type: Some(Box::new(TypeAnnotation::BuiltinType(BuiltinType {
                    name: "String".to_string(),
                    symbol: "String".to_string(),
                }))),
                value_type: Some(Box::new(TypeAnnotation::BuiltinType(BuiltinType {
                    name: "Int32".to_string(),
                    symbol: "Int32".to_string(),
                }))),
            })
        );

        let access_map = get_ast!(
            parser::StatementParser,
            "value2 <- keyValueStore[key][key2]"
        );
        assert_eq!(
            access_map.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "Int32".to_string(),
                symbol: "Int32".to_string(),
            })
        );
        assert!(workspace.env.contains_key("value1"));
        assert!(workspace.env.contains_key("value2"));
        assert_eq!(
            workspace.env.get("value2").unwrap().get_instance(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "Int32".to_string(),
                symbol: "Int32".to_string(),
            })
        );

        return;
    }

    #[test]
    fn type_inference_node_statement_accept_test() {
        let mut workspace = setup_workspace();
        let accept_stmt = get_ast!(parser::StatementParser, "accept");
        assert_eq!(
            accept_stmt.get_type(&mut workspace).unwrap(),
            TypeAnnotation::Void
        );
    }
    #[test]
    fn type_inference_node_statement_send_test() {
        let mut workspace = setup_workspace();
        let send_stmt = get_ast!(parser::StatementParser, "send msgs");
        assert_eq!(
            send_stmt.get_type(&mut workspace).unwrap(),
            TypeAnnotation::Void
        );
    }
    #[test]
    fn type_inference_node_statement_create_evnt_test() {
        let mut workspace = setup_workspace();
        let create_evnt_stmt = get_ast!(parser::StatementParser, "event e");
        assert_eq!(
            create_evnt_stmt.get_type(&mut workspace).unwrap(),
            TypeAnnotation::Void
        );
    }
    #[test]
    fn type_inference_node_statement_throw_test() {
        let mut workspace = setup_workspace();
        let throw_stmt = get_ast!(parser::StatementParser, "throw error");
        assert_eq!(
            throw_stmt.get_type(&mut workspace).unwrap(),
            TypeAnnotation::Void
        );
    }

    #[test]
    fn type_inference_node_statement_load_test() {
        let mut workspace = setup_workspace();
        // Define a Scilla Load statement to test
        let load_statement = get_ast!(parser::StatementParser, "y <- x");
        // Assert that x and y are initially not in the workspace
        assert!(!workspace.env.contains_key("x"));
        assert!(!workspace.env.contains_key("y"));
        // Add x to the workspace as an Int32
        workspace.env.insert(
            "x".to_string(),
            Box::new(BuiltinType {
                name: "Int32".to_string(),
                symbol: "Int32".to_string(),
            }),
        );
        // Assert that the load statement works as expected
        assert_eq!(
            load_statement.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "Int32".to_string(),
                symbol: "Int32".to_string(),
            })
        );
        // Assert that y has been added to the workspace as a result of the load statement
        assert!(workspace.env.contains_key("y"));
        assert_eq!(
            workspace.env.get("y").unwrap().get_instance(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "Int32".to_string(),
                symbol: "Int32".to_string(),
            })
        );
    }
    #[test]
    fn type_inference_node_statement_store_test() {
        let mut workspace = setup_workspace();
        // Define a Scilla Store statement to test
        let store_statement = get_ast!(parser::StatementParser, "x := y");
        // Assert that x is initially not in the workspace
        assert!(!workspace.env.contains_key("x"));
        assert!(!workspace.env.contains_key("y"));

        // Assert that the store statement works as expected
        workspace.env.insert(
            "y".to_string(),
            Box::new(BuiltinType {
                name: "Int32".to_string(),
                symbol: "Int32".to_string(),
            }),
        );
        // Assert that the load statement works as expected
        assert_eq!(
            store_statement.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "Int32".to_string(),
                symbol: "Int32".to_string(),
            })
        );
        // Assert that y has been added to the workspace as a result of the load statement
        assert!(workspace.env.contains_key("x"));
        assert_eq!(
            workspace.env.get("x").unwrap().get_instance(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "Int32".to_string(),
                symbol: "Int32".to_string(),
            })
        );
    }

    #[test]
    fn type_inference_remote_fetch_statement_test() {
        // TODO: Implement
    }

    #[test]
    fn type_inference_node_component_id_test() {
        let mut workspace = setup_workspace();
        // Test for a valid function creation.
        let fun_def = get_ast!(parser::ComponentIdParser, "ComponentId.HelloWorld");
        let result = fun_def.get_type(&mut workspace);
        assert!(
            result.is_ok(),
            "Expected a success for a valid function creation"
        );
        // Test for an invalid function name.
        let invalid_fun_def = get_ast!(parser::ComponentIdParser, "Invalid-Function-Name");
        let invalid_result = invalid_fun_def.get_type(&mut workspace);
        assert!(
            invalid_result.is_err(),
            "Expected an error for an invalid function name"
        );
        // Test for a function name with namespace.
        let namespace_fun_def = get_ast!(parser::ComponentIdParser, "Namespace::ValidFunctionName");
        let namespace_result = namespace_fun_def.get_type(&mut workspace);
        assert!(
            namespace_result.is_ok(),
            "Expected a success for a function name with namespace"
        );
        // Test for an invalid function name with namespace.
        let invalid_namespace_fun_def =
            get_ast!(parser::ComponentIdParser, "Invalid-Namespace::FunctionName");
        let invalid_namespace_result = invalid_namespace_fun_def.get_type(&mut workspace);
        assert!(
            invalid_namespace_result.is_err(),
            "Expected an error for an invalid function name with namespace"
        );
    }

    #[test]
    // Test to ensure type inference correctly recognizes parameter types in function components
    fn type_inference_node_component_parameters_test() {
        let mut workspace = setup_workspace();
        // Testing function call parameters (a: Int32, b: Bool)
        let params = get_ast!(parser::ComponentParametersParser, "(a: Int32, b: Bool)");
        for param in params.parameters.iter() {
            let param_type = &param.identifier_with_type.annotation.type_name;
            match param_type {
                NodeScillaType::GenericTypeWithArgs(type_ident, _) => {
                    let t = type_ident.get_type(&mut workspace).unwrap();
                    assert!(
                        matches!(t, TypeAnnotation::BuiltinType(_)),
                        "Expected Int or Bool type annotation for Component Parameters"
                    );
                }
                _ => panic!("Unexpected Component Parameter type"),
            }
        }
        // Testing function call parameters involving custom types (x: MyTypeOrEnumLikeIdentifier)
        let params = get_ast!(
            parser::ComponentParametersParser,
            "(x: MyTypeOrEnumLikeIdentifier)"
        );
        let param = &params.parameters[0];
        let param_type = &param.identifier_with_type.annotation.type_name;
        match param_type {
            NodeScillaType::GenericTypeWithArgs(type_ident, _) => {
                let t = type_ident.get_type(&mut workspace).unwrap();
                assert!(
                    matches!(t, TypeAnnotation::TemplateType(_)),
                    "Expected MyTypeOrEnumLikeIdentifier annotation for Component Parameters"
                );
            }
            _ => panic!("Unexpected Component Parameter type"),
        }
        //Testing a function call with non-existent types
        let params = get_ast!(parser::ComponentParametersParser, "(a: NonExistentType)");
        let param = &params.parameters[0];
        let param_type = &param.identifier_with_type.annotation.type_name;
        match param_type {
            NodeScillaType::GenericTypeWithArgs(type_ident, _) => {
                let result = type_ident.get_type(&mut workspace);
                assert!(
                    result.is_err(),
                    "Expected an error when parsing a non-existent type"
                );
            }
            _ => panic!("Unexpected Component Parameter type"),
        }
        // TODO:  Implement more test cases when Map and Address types implemented
        // E.g. (a: Map ByStr20 (Uint256))
    }

    #[test]
    fn type_inference_parameter_pair_test() {
        let mut workspace = setup_workspace();
        // Valid parameter pair case
        let ident = get_ast!(parser::ParameterPairParser, "parameter: Uint32");
        let result = ident.get_type(&mut workspace);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "Uint32".to_string(),
                symbol: "Uint32".to_string(),
            }),
        );
        // Namespace type resolution
        let ident = get_ast!(
            parser::ParameterPairParser,
            "parameter: MyNamespace::MyTypeOrEnumLikeIdentifier"
        );
        assert_eq!(
            ident.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "MyTypeOrEnumLikeIdentifier".to_string(),
                symbol: "MyNamespace::MyTypeOrEnumLikeIdentifier".to_string()
            }),
        );
        // Non-existant type
        let ident = get_ast!(parser::ParameterPairParser, "parameter: nonExistent");
        let result = ident.get_type(&mut workspace);
        assert!(
            result.is_err(),
            "Expected an error when parsing non-existent type"
        );
        // More test cases involving different types, namespaces and nested types should be included for completeness
    }

    #[test]
    fn type_inference_component_body_test() {
        let mut workspace = setup_workspace();
        // Parsing an Immutable variable declaration
        let component_body = get_ast!(
            parser::ComponentBodyParser,
            "
        x = Uint128 10;
    "
        );
        assert_eq!(
            component_body.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "Uint128".to_string(),
                symbol: "Uint128".to_string()
            })
        );
        // Parsing a Map declaration
        let component_body = get_ast!(
            parser::ComponentBodyParser,
            "
        map_x = Emp Uint128 ByStr20;
    "
        );
        assert_eq!(
            component_body.get_type(&mut workspace).unwrap(),
            TypeAnnotation::MapType(MapType {
                name: "MapType".to_string(),
                symbol: "MapType".to_string(),
                key_type: None,
                value_type: None
            })
        );
        // Parsing a Message constructor
        let component_body = get_ast!(
            parser::ComponentBodyParser,
            "
        msg = {_tag : \"\"; _recipient : _sender; _amount : _amount; param1 : x};
    "
        );
        assert_eq!(
            component_body.get_type(&mut workspace).unwrap(),
            TypeAnnotation::StructType(StructType {
                name: "Message".to_string(),
                fields: vec![
                    (
                        "_tag".to_string(),
                        TypeAnnotation::BuiltinType(BuiltinType {
                            name: "String".to_string(),
                            symbol: "String".to_string()
                        })
                    ),
                    (
                        "_recipient".to_string(),
                        TypeAnnotation::BuiltinType(BuiltinType {
                            name: "ByStr20".to_string(),
                            symbol: "ByStr20".to_string()
                        })
                    ),
                    (
                        "_amount".to_string(),
                        TypeAnnotation::BuiltinType(BuiltinType {
                            name: "Uint128".to_string(),
                            symbol: "Uint128".to_string()
                        })
                    ),
                    (
                        "param1".to_string(),
                        TypeAnnotation::BuiltinType(BuiltinType {
                            name: "Uint128".to_string(),
                            symbol: "Uint128".to_string()
                        })
                    ),
                ],
                symbol: "Message".to_string(),
            })
        );
    }

    #[test]
    fn type_inference_statement_block_test() {
        let mut workspace = setup_workspace();
        let fragment = r#"
        import Bool as B;
        procedure divide(x: Int32, y: Int32)
          if (y != 0)    
            then Int32.div x y
            else error "Division by zero"
        end"#;
        let stmt_block = get_ast!(parser::StatementBlockParser, fragment);
        // Check if procedural logic `divide` is correctly inferred and present in workspace
        let proc_name = "divide";
        assert!(
            workspace.env.contains_key(proc_name),
            "The workspace should contain the 'divide' procedural logic"
        );
        // Check if the procedural logic is of `FunType`
        let proc_type_sig = stmt_block.get_type(&mut workspace);
        assert!(
            matches!(proc_type_sig.unwrap(), TypeAnnotation::FunType(_)),
            "'divide' should parse into a function type"
        );
    }

    #[test]
    fn type_inference_typed_identifier_test() {
        let mut workspace = setup_workspace();
        // Basic type resolution
        let t_ident = get_ast!(parser::TypedIdentifierParser, "basicIdentifier: String");
        assert_eq!(
            t_ident.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "String".to_string(),
                symbol: "String".to_string(),
            }),
        );
        // Template type resolution
        let t_ident = get_ast!(
            parser::TypedIdentifierParser,
            "templateIdentifier: MyTypeOrEnumLikeIdentifier"
        );
        assert_eq!(
            t_ident.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "MyTypeOrEnumLikeIdentifier".to_string(),
                symbol: "MyTypeOrEnumLikeIdentifier".to_string(),
            }),
        );
        // Type resolution with namespace
        let t_ident = get_ast!(
            parser::TypedIdentifierParser,
            "identifierNamespace: MyNamespace::MyTypeOrEnumLikeIdentifier"
        );
        assert_eq!(
            t_ident.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "MyTypeOrEnumLikeIdentifier".to_string(),
                symbol: "MyNamespace::MyTypeOrEnumLikeIdentifier".to_string(),
            })
        );
        // Error case - non-existent type
        let t_ident = get_ast!(
            parser::TypedIdentifierParser,
            "nonExistentIdentifier: NonExistent"
        );
        let result = t_ident.get_type(&mut workspace);
        assert!(
            result.is_err(),
            "Expected an error when parsing non-existent type"
        );
    }

    #[test]
    fn type_inference_type_annotation_test() {
        let mut workspace = setup_workspace();
        // Base type annotation
        let base_type = get_ast!(parser::TypeAnnotationParser, ": Int");
        assert_eq!(
            base_type.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "Int".to_string(),
                symbol: "Int".to_string(),
            }),
        );
        // Custom type annotation
        let custom_type = get_ast!(parser::TypeAnnotationParser, ": MyTypeOrEnumLikeIdentifier");
        assert_eq!(
            custom_type.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "MyTypeOrEnumLikeIdentifier".to_string(),
                symbol: "MyTypeOrEnumLikeIdentifier".to_string(),
            }),
        );
        // Complex type annotation - List
        let list_type = get_ast!(
            parser::TypeAnnotationParser,
            ": (List MyTypeOrEnumLikeIdentifier)"
        );
        assert_eq!(
            list_type.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TypeVar(TypeVar {
                instance: Some(Box::new(TypeAnnotation::TemplateType(TemplateType {
                    name: "MyTypeOrEnumLikeIdentifier".to_string(),
                    symbol: "MyTypeOrEnumLikeIdentifier".to_string(),
                }))),
                symbol: "(List MyTypeOrEnumLikeIdentifier)".to_string(),
            }),
        );
        // Complex type annotation - Map
        let map_type = get_ast!(parser::TypeAnnotationParser, ": (Map ByStr32 Uint32)");
        assert_eq!(
            map_type.get_type(&mut workspace).unwrap(),
            TypeAnnotation::MapType(MapType {
                name: "(Map ByStr32 Uint32)".to_string(),
                symbol: "(Map ByStr32 Uint32)".to_string(),
                key_type: None,
                value_type: None
            }),
        );
        // TODO: Add more types to cover all possible type annotations
    }

    #[test]
    fn type_inference_node_program_test() {
        // TODO: Implement
    }

    #[test]
    fn type_inference_library_definition_test() {
        // 1. Standard library definition with predefined types
        let mut workspace = setup_workspace();
        let library_def = get_ast!(parser::LibraryDefinitionParser, "library MyLib");
        // TODO: assert_eq!(library_def.get_type(&mut workspace).unwrap(), NodeLibraryDefinition { name: "MyLib".to_string(), ...});
        // 2. Library definition with custom, namespace-qualified types
        let library_def = get_ast!(
            parser::LibraryDefinitionParser,
            "library MyNamespace::MyLib"
        );
        // TODO: assert_eq!(library_def.get_type(&mut workspace).unwrap(), LibraryDefinition { name: "MyNamespace::MyLib".to_string(), ...});
        // 3. Library defining a custom type
        let library_def = get_ast!(
            parser::LibraryDefinitionParser,
            "library TypeLib type MyTypeOrEnumLikeIdentifier"
        );
        // TODO: assert_eq!(library_def.get_type(&mut workspace).unwrap(), LibraryDefinition {..., definitions: [TypeDefinition(..)]});
        // 4. Library defining a value
        let library_def = get_ast!(
            parser::LibraryDefinitionParser,
            "library ValLib let x = Int32 10"
        );
        // TODO: assert_eq!(library_def.get_type(&mut_workspace).unwrap(), LibraryDefinition { ..., definitions: [LetDefinition(..)]});
        // 5. Library defining a type with a union
        let library_def = get_ast!(
            parser::LibraryDefinitionParser,
            "library UnionLib type UnionType = | Type1 | Type2"
        );
        // TODO: assert_eq!(library_def.get_type(&mut workspace).unwrap(), LibraryDefinition {..., definitions: [TypeDefinition(UnionType { ...})]});
    }

    #[test]
    fn type_inference_library_single_definition_test() {
        let mut workspace = setup_workspace();
        // Test with 'let' definition
        let single_definition = get_ast!(
            parser::LibrarySingleDefinitionParser,
            "let foo: MyTypeOrEnumLikeIdentifier = MyTypeOrEnumLikeIdentifier 42"
        );
        let result = single_definition.get_type(&mut workspace);
        assert!(
            result.is_ok(),
            "Expected successful parsing of a let definition."
        );
        assert_eq!(
            result.unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "MyTypeOrEnumLikeIdentifier".to_string(),
                symbol: "MyNamespace::MyTypeOrEnumLikeIdentifier".to_string(),
            }),
        );
        // Test with 'type' definition without '=' operator
        let single_definition = get_ast!(parser::LibrarySingleDefinitionParser, "type Foo");
        let result = single_definition.get_type(&mut workspace);
        assert!(
            result.is_ok(),
            "Expected successful parsing of a type definition without equals."
        );
        assert_eq!(result.unwrap(), TypeAnnotation::Void,);
        // Test with 'type' definition with '=' operator
        let single_definition = get_ast!(
            parser::LibrarySingleDefinitionParser,
            "type Foo = | Bar | Baz"
        );
        let result = single_definition.get_type(&mut workspace);
        assert!(
            result.is_ok(),
            "Expected successful parsing of a type definition with equals."
        );
        assert_eq!(
            result.unwrap(),
            TypeAnnotation::UnionType(UnionType {
                name: "Foo".to_string(),
                types: vec![
                    TypeAnnotation::TemplateType(TemplateType {
                        name: "Bar".to_string(),
                        symbol: "MyNamespace::Bar".to_string(),
                    }),
                    TypeAnnotation::TemplateType(TemplateType {
                        name: "Baz".to_string(),
                        symbol: "MyNamespace::Baz".to_string(),
                    }),
                ],
                symbol: "MyNamespace::Foo".to_string(),
            }),
        );
        // Invalid 'let' definition without a type
        let single_definition = get_ast!(
            parser::LibrarySingleDefinitionParser,
            "let foo = MyTypeOrEnumLikeIdentifier 42"
        );
        let result = single_definition.get_type(&mut workspace);
        assert!(
            result.is_err(),
            "Expected an error when parsing let definition without a type."
        );
        // Invalid 'type' definition without a name
        let single_definition =
            get_ast!(parser::LibrarySingleDefinitionParser, "type = | Bar | Baz");
        let result = single_definition.get_type(&mut workspace);
        assert!(
            result.is_err(),
            "Expected an error when parsing type definition without a name."
        );
    }

    #[test]
    fn type_inference_contract_definition_test() {
        let mut workspace = setup_workspace();
        let contract_definition_str = "contract MyContract(init_balance : Uint128) with true => field balance : Uint128 = init_balance transition transfer() end";
        // Parsing the contract definition.
        let contract_definition_ast =
            get_ast!(parser::ContractDefinitionParser, contract_definition_str);
        // Type checking the contract definition.
        let result = contract_definition_ast.get_type(&mut workspace);
        // Check if the type is as expected.
        assert_eq!(
            result.unwrap(),
            TypeAnnotation::TypeVar(TypeVar {
                instance: None,
                symbol: "".to_string(),
                //            meta_data: HashMap::new(),
            }),
        );
        // Try with non-existing type
        let invalid_contract_definition_str =
            "contract InvalidContract(invalid_type : NonExistantType) => end";
        let invalid_contract_definition_ast = get_ast!(
            parser::ContractDefinitionParser,
            invalid_contract_definition_str
        );
        let result = invalid_contract_definition_ast.get_type(&mut workspace);
        assert!(
            result.is_err(),
            "Expected an error when parsing non-existent type"
        );
        // Try contract with function
        let contract_with_function_str = "contract MyContract(init_balance : Uint128) with true => field balance : Uint128 = init_balance transition transfer(to: ByStr20, amount : Uint128) => b = builtin sub balance amount; match b with | Some v => balance := v | None => error = InsufficientFunds; Throw error end end";
        let contract_with_function_ast =
            get_ast!(parser::ContractDefinitionParser, contract_with_function_str);
        let result = contract_with_function_ast.get_type(&mut workspace);
        // Check if type is as expected.
        assert_eq!(
            result.unwrap(),
            TypeAnnotation::TypeVar(TypeVar {
                instance: None,
                symbol: "".to_string(),
                //            meta_data: HashMap::new(),
            }),
        );
        // TODO: add more cases to cover the different scenarios
    }

    #[test]
    fn type_inference_contract_field_test() {
        let mut workspace = setup_workspace();
        // existing type
        let field_ast = get_ast!(
            parser::ContractFieldParser,
            "field MyTypeOrEnumLikeIdentifier: MyNamespace::MyTypeOrEnumLikeIdentifier = MyNamespace::MyTypeOrEnumLikeIdentifier"
        );
        // Now we can get the type of this AST node
        let result = field_ast.get_type(&mut workspace);
        // Check the type, assuming it should be of MyNamespace's MyTypeOrEnumLikeIdentifier.
        assert_eq!(
            result.unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "MyTypeOrEnumLikeIdentifier".to_string(),
                symbol: "MyNamespace::MyTypeOrEnumLikeIdentifier".to_string(),
            }),
        );
        // Non-existant type
        let field_ast = get_ast!(
            parser::ContractFieldParser,
            "field nonExistentType: nonExistentType = nonExistentType"
        );
        // Now we can get the type of this AST node
        let result = field_ast.get_type(&mut workspace);
        // Check if the result is error as nonExistentType is not defined.
        assert!(
            result.is_err(),
            "Expected an error when parsing non-existent type"
        );
        // Test with a more complex type like Map
        // Assuming that you have implemented and added Map to the environment
        let field_ast = get_ast!(
            parser::ContractFieldParser,
            "field myMap: Map ByStr20 Uint128 = Emp ByStr20 Uint128"
        );
        let result = field_ast.get_type(&mut workspace);
        assert_eq!(
            result.unwrap(),
            TypeAnnotation::MapType(MapType {
                name: "".to_string(),
                symbol: "Map<ByStr20, Uint128>".to_string(), // Provide your expected result
                key_type: None,
                value_type: None
            }),
        );
    }

    #[test]
    fn type_inference_with_constraint_test() {
        let mut workspace = setup_workspace();
        // Non-existant type
        let test_str = "with builtin blt end_of_life =>";
        let with_constraint = get_ast!(parser::WithConstraintParser, test_str);
        let result = with_constraint.get_type(&mut workspace);
        // TODO: assert!(result.is_err(), "Expected an error when parsing non-existent type");
        // Base type resolution
        let test_str = "with builtin add {UInt32} one  =>";
        let with_constraint = get_ast!(parser::WithConstraintParser, test_str);
        let result = with_constraint.get_type(&mut workspace).unwrap();
        // TODO: assert!(result.is_ok(), "Expected an OK when parsing existent type");
        // Use of with true
        let test_str = "with true =>";
        let with_constraint = get_ast!(parser::WithConstraintParser, test_str);
        let result = with_constraint.get_type(&mut workspace).unwrap();
        assert_eq!(
            result,
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "Bool".to_string(),
                symbol: "Bool".to_string()
            }),
            "Expected a boolean type"
        );
        // Use of with variableIdentifier
        let test_str = "with MyTypeOrEnumLikeIdentifier =>";
        let with_constraint = get_ast!(parser::WithConstraintParser, test_str);
        let result = with_constraint.get_type(&mut workspace).unwrap();
        assert_eq!(
            result,
            TypeAnnotation::TemplateType(TemplateType {
                name: "MyTypeOrEnumLikeIdentifier".to_string(),
                symbol: "MyNamespace::MyTypeOrEnumLikeIdentifier".to_string()
            }),
            "Expected a template type"
        );
        // Empty with constraint
        let test_str = "with =>";
        let with_constraint = get_ast!(parser::WithConstraintParser, test_str);
        let result = with_constraint.get_type(&mut workspace);
        // TODO: assert!(result.is_err(), "Expected an error when parsing empty with constraint");
        // TODO: add more cases to cover the different scenarios
    }

    #[test]
    fn type_inference_component_definition_test() {
        let mut workspace = setup_workspace();
        let procedure_definition = get_ast!(
            parser::ComponentDefinitionParser,
            "procedure myProcedure(name: ByStr20, number: Int32)
               begin 
                  (* ..... *)
               end"
        );
        /*
        assert_eq!(
            procedure_definition.get_type(&mut workspace),
            TypeAnnotation::ProcedureComponent(NodeProcedureDefinition {
                name: NodeComponentId::WithRegularId("myProcedure".to_string()),
                parameters: NodeComponentParameters {
                    parameters: vec![
                        NodeParameterPair {
                            identifier_with_type: NodeTypedIdentifier {
                                identifier_name: "name".to_string(),
                                annotation: NodeTypeAnnotation {
                                    type_name: NodeScillaType::ScillaAddresseType(Box::new(NodeAddressType {
                                        identifier: NodeTypeNameIdentifier::ByteStringType(NodeByteStr::Constant("ByStr20".to_string())),
                                        type_name: "ByStr20".to_string(),
                                        address_fields: vec![],
                                        type_annotation: None,
                                    })),
                                    type_annotation: None,
                                },
                                type_annotation: None,
                            },
                            type_annotation: None
                        },
                        NodeParameterPair {
                            identifier_with_type: NodeTypedIdentifier {
                                identifier_name: "number".to_string(),
                                annotation: NodeTypeAnnotation {
                                    type_name: NodeScillaType::GenericTypeWithArgs(NodeMetaIdentifier::MetaName(
                                        NodeTypeNameIdentifier::TypeOrEnumLikeIdentifier("Int32".to_string())
                                    ), vec![]),
                                    type_annotation: None,
                                },
                                type_annotation: None,
                            },
                            type_annotation: None
                        }
                    ],
                    type_annotation: None
                },
                body: NodeComponentBody {
                   statement_block: None,
                   type_annotation: None
                },
                type_annotation: None
            })
        );
        */
    }
}
