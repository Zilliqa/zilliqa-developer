use bluebell::{
    ast::*, formatter::ScillaFormatter, lexer::Lexer, lexer::*, parser::*, type_inference::*, *,
};

#[cfg(test)]
mod tests {
    use super::*;
    use bluebell::ast::*;
    use bluebell::type_classes::*;
    use bluebell::type_inference::*;
    use std::collections::HashMap;

    #[test]
    fn type_inference_variable_identifier_test() {
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
        // Insert VariableInNamespace type
        workspace.env.insert(
            "MyNamespace::MyCustomType".to_string(),
            Box::new(TemplateType {
                name: "MyCustomType".to_string(),
                symbol: "MyNamespace::MyCustomType".to_string(),
            }),
        );
        let ident = NodeVariableIdentifier::VariableName("MyCustomType".to_string());
        assert_eq!(
            ident.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "MyCustomType".to_string(),
                symbol: "MyCustomType".to_string()
            })
        );
        let ident = NodeVariableIdentifier::VariableName("NonexistentType".to_string());
        assert!(ident.get_type(&mut workspace).is_err());
        // Test VariableInNamespace
        let ident = NodeVariableIdentifier::VariableInNamespace(
            NodeTypeNameIdentifier::CustomType(String::from("MyNamespace")),
            "MyCustomType".to_string(),
        );
        /*
        TODO: Fix this
        assert_eq!(
            ident.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "MyCustomType".to_string(),
                symbol: "MyNamespace::MyCustomType".to_string()
            })
        );
        */
    }

    /*
    TODO: Replace
    #[test]
    fn type_inference_variable_in_namespace_test() {
        let mut workspace = Workspace {
            env: HashMap::new(),
            namespace: "".to_string()
        };
        workspace.env.insert("Module".to_string(), Box::new(CustomType { name: "Module".to_string(), symbol: "Module".to_string() }));
        workspace.env.insert("Module.Type".to_string(), Box::new(CustomType { name: "Module.Type".to_string(), symbol: "Module.Type".to_string() }));
        let ident = NodeVariableIdentifier::VariableInNamespace(
            Box::new(NodeVariableIdentifier::VariableName("Module".to_string())),
            "Type".to_string()
        );
        assert_eq!(
            ident.get_type(&mut workspace).unwrap(),
            TypeAnnotation::CustomType(CustomType { name: "Module.Type".to_string(), symbol: "Module.Type".to_string() })
        );
        let ident = NodeVariableIdentifier::VariableInNamespace(
            Box::new(NodeVariableIdentifier::VariableName("NonexistentModule".to_string())),
            "Type".to_string()
        );
        assert!(ident.get_type(&mut workspace).is_err());
    }
    */

    #[test]
    fn type_inference_node_type_name_identifier_test() {
        /*
          let mut workspace = Workspace {
            env: HashMap::new(),
            namespace: "".to_string()
        };
            workspace.env.insert("String".to_string(), Box::new(BuiltinType { name: "String".to_string(), symbol: "String".to_string() }));
            workspace.env.insert("MyCustomType".to_string(), Box::new(TemplateType { name: "MyCustomType".to_string(), symbol: "MyCustomType".to_string() }));
            let ident = NodeTypeNameIdentifier::ByteStringType(NodeByteStr { byte_size: 1 });
            assert_eq!(
                ident.get_type(&mut workspace).unwrap(),
                TypeAnnotation::BuiltinType(BuiltinType { name: "ByStr".to_string(), symbol: "ByStr".to_string() })
            );
            let ident = NodeTypeNameIdentifier::EventType;
            assert_eq!(
                ident.get_type(&mut workspace).unwrap(),
                TypeAnnotation::BuiltinType(BuiltinType { name: "Event".to_string(), symbol: "Event".to_string() })
            );
            let ident = NodeTypeNameIdentifier::CustomType("MyCustomType".to_string());
            assert_eq!(
                ident.get_type(&mut workspace).unwrap(),
                TypeAnnotation::TemplateType(TemplateType { name: "MyCustomType".to_string(), symbol: "MyCustomType".to_string() })
            );
            let ident = NodeTypeNameIdentifier::CustomType("NonexistentType".to_string());
            assert!(ident.get_type(&mut workspace).is_err());
        */
    }

    #[test]
    fn type_inference_node_byte_str_test() {
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
            "ByStr20".to_string(),
            Box::new(BuiltinType {
                name: "ByStr20".to_string(),
                symbol: "ByStr20".to_string(),
            }),
        );
        let byte_str = NodeByteStr::Type("ByStr20".to_string());
        assert_eq!(
            byte_str.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "ByStr20".to_string(),
                symbol: "ByStr20".to_string()
            })
        );
        let byte_str = NodeByteStr::Type("NonexistentType".to_string());
        assert!(byte_str.get_type(&mut workspace).is_err());
        let byte_str = NodeByteStr::Constant("0x1234".to_string());
        assert!(byte_str.get_type(&mut workspace).is_err());
    }

    /*
      TODO:
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::type_classes::{BuiltinType, TemplateType};
        #[test]
        fn type_inference_transition_definition_test() {
            // Some workspace.environment initialization for the test. Please adjust accordingly.
            let mut workspace = Workspace {
                env: HashMap::new(),
                namespace: "".to_string()
            };
            workspace.env.insert("String".to_string(), Box::new(BuiltinType { name: "String".to_string(), symbol: "String".to_string() }));
            workspace.env.insert("MyCustomType".to_string(), Box::new(TemplateType { name: "MyCustomType".to_string(), symbol: "MyCustomType".to_string() }));
            // Define a NodeTransitionDefinition object for the test.
            // You need to create/parse this object using your AST representation.
            let transition_definition: NodeTransitionDefinition = ...
            // Get the type of the transition_definition
            let type_annotation = transition_definition.get_type(&mut workspace).unwrap();
            // Test the inferred type.
            // This assertion depends on the correct types to check against based on the transition_definition.
            assert_eq!(
                type_annotation,
                TypeAnnotation::...
            );
        }
    }
    */

    /*
    #[test]
    fn type_inference_alternative_clause_test() {
        let mut workspace = Workspace {
            env: HashMap::new(),
            namespace: "".to_string()
        };
        workspace.env.insert("String".to_string(), Box::new(BuiltinType { name: "String".to_string(), symbol: "String".to_string() }));
        workspace.env.insert("Int".to_string(), Box::new(BuiltinType { name: "Int".to_string(), symbol: "Int".to_string() }));
        let clause_simple = NodeTypeAlternativeClause::ClauseType("String".to_string());
        assert_eq!(
            clause_simple.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType { name: "String".to_string(), symbol: "String".to_string() })
        );
        let clause_with_args = NodeTypeAlternativeClause::ClauseTypeWithArgs(
            "String".to_string(),
            vec![Box::new(NodeTypeNameIdentifier::EventType)]
        );
        assert!(clause_with_args.get_type(&mut workspace).is_err());
    }
    */

    /*
    // In type_inference_tests.rs
    #[test]
    fn type_inference_map_value_arguments_test() {
        let mut workspace = Workspace {
            env: HashMap::new(),
            namespace: "".to_string()
        };
        workspace.env.insert("String".to_string(), Box::new(BuiltinType { name: "String".to_string(), symbol: "String".to_string() }));
        workspace.env.insert("Int32".to_string(), Box::new(BuiltinType { name: "Int32".to_string(), symbol: "Int32".to_string() }));
        let key = NodeTypeMapValueArguments::GenericMapValueArgument(NodeTypeNameIdentifier::ByteStringType("String".to_string()));
        let value = NodeTypeMapValueArguments::GenericMapValueArgument(NodeTypeNameIdentifier::ByteStringType("Int32".to_string()));
        let map_key_value_type = NodeTypeMapValueArguments::MapKeyValueType(Box::new(key), Box::new(value));
        assert_eq!(
            map_key_value_type.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "Map[String, Int32]".to_string(),
                symbol: "Map".to_string(),
            }),
        );
    }
    */
    /*
    // type_inference_tests.rs
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

    // Add this to `tests/type_inference_tests.rs`
    #[test]
    fn type_inference_imported_name_test() {
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
        let ident = NodeImportedName::RegularImport(NodeTypeNameIdentifier::CustomType(
            "MyCustomType".to_string(),
        ));
        assert_eq!(
            ident.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "MyCustomType".to_string(),
                symbol: "MyCustomType".to_string(),
            })
        );
        /*
        TODO:
        let ident = NodeImportedName::AliasedImport(
            NodeTypeNameIdentifier::CustomType("String".to_string()),
            "StringAlias".to_string(),
        );
        assert_eq!(
            ident.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "String".to_string(),
                symbol: "String".to_string(),
            })
        );
        */
    }

    #[test]
    fn type_inference_import_declarations_error_test() {
        let mut workspace = Workspace {
            env: HashMap::new(),
            namespace: "".to_string(),
        };
        let node_import_declarations = NodeImportDeclarations {
            import_list: Vec::new(),
            type_annotation: None,
        };
        assert!(node_import_declarations.get_type(&mut workspace).is_err());
    }

    // In `tests/type_inference_tests.rs`:
    #[test]
    fn type_inference_meta_identifier_test() {
        /*
        TODO:
          let mut workspace = Workspace {
            env: HashMap::new(),
            namespace: "".to_string()
        };
            workspace.env.insert("String".to_string(), Box::new(BuiltinType { name: "String".to_string(), symbol: "String".to_string() }));
            workspace.env.insert("Namespace".to_string(), Box::new(TemplateType { name: "Namespace".to_string(), symbol: "Namespace".to_string() }));
            let ident = NodeMetaIdentifier::MetaName(NodeVariableIdentifier::VariableName("String".to_string()));
            assert_eq!(
                ident.get_type(&mut workspace).unwrap(),
                TypeAnnotation::BuiltinType(BuiltinType { name: "String".to_string(), symbol: "String".to_string() })
            );
            let ident = NodeMetaIdentifier::MetaNameInNamespace(
                Box::new(NodeVariableIdentifier::VariableName("Namespace".to_string())),
                Box::new(NodeVariableIdentifier::VariableName("String".to_string()))
            );
            assert_eq!(
                ident.get_type(&mut workspace).unwrap(),
                TypeAnnotation::TemplateType(TemplateType { name: "Namespace.String".to_string(), symbol: "Namespace.String".to_string() })
            );
            let ident = NodeMetaIdentifier::MetaNameInHexspace("0x".to_string(), Box::new(NodeVariableIdentifier::VariableName("String".to_string())));
            assert_eq!(
                ident.get_type(&mut workspace).unwrap(),
                TypeAnnotation::BuiltinType(BuiltinType { name: "String".to_string(), symbol: "String".to_string() })
            );
            let ident = NodeMetaIdentifier::ByteString;
            assert_eq!(
                ident.get_type(&mut workspace).unwrap(),
                TypeAnnotation::BuiltinType(BuiltinType { name: "ByStr".to_string(), symbol: "ByStr".to_string() })
            );
            */
    }

    #[test]
    fn type_inference_builtin_arguments_test() {
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
        let node_builtin_arguments = NodeBuiltinArguments {
            arguments: vec![
                NodeVariableIdentifier::VariableName("String".to_string()),
                NodeVariableIdentifier::VariableName("MyCustomType".to_string()),
            ],
            type_annotation: None,
        };
        assert_eq!(
            node_builtin_arguments.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "String".to_string(),
                symbol: "String".to_string()
            })
        );
        let empty_builtin_arguments = NodeBuiltinArguments {
            arguments: vec![],
            type_annotation: None,
        };
        assert!(empty_builtin_arguments.get_type(&mut workspace).is_err());
    }

    #[test]
    fn type_inference_node_type_map_key_test() {
        /*
        TODO:
        let mut workspace = Workspace {
            env: HashMap::new(),
            namespace: "".to_string()
        };
          workspace.env.insert("NodeMetaIdentifier".to_string(), Box::new(SomeTypeForNodeMetaIdentifier { /* fields */ }));
          workspace.env.insert("NodeAddressType".to_string(), Box::new(SomeTypeForNodeAddressType { /* fields */ }));
          // Prepare examples of NodeTypeMapKey here, for instance:
          let node_type_map_key_generic = NodeTypeMapKey::GenericMapKey( /* instantiate NodeMetaIdentifier */ );
          let node_type_map_key_enclosed_generic = NodeTypeMapKey::EnclosedGenericId( /* instantiate NodeMetaIdentifier */ );
          let node_type_map_key_enclosed_address = NodeTypeMapKey::EnclosedAddressMapKeyType( /* instantiate NodeAddressType */ );
          let node_type_map_key_address = NodeTypeMapKey::AddressMapKeyType( /* instantiate NodeAddressType */ );
          // Assert the expected types for each example
          assert_eq!(
              node_type_map_key_generic.get_type(&mut workspace).unwrap(),
              TypeAnnotation::SomeAnnotationForNodeMetaIdentifier
          );
          assert_eq!(
              node_type_map_key_enclosed_generic.get_type(&mut workspace).unwrap(),
              TypeAnnotation::SomeAnnotationForNodeMetaIdentifier
          );
          assert_eq!(
              node_type_map_key_enclosed_address.get_type(&mut workspace).unwrap(),
              TypeAnnotation::SomeAnnotationForNodeAddressType
          );
          assert_eq!(
              node_type_map_key_address.get_type(&mut workspace).unwrap(),
              TypeAnnotation::SomeAnnotationForNodeAddressType
          );
        */
    }

    #[test]
    fn type_inference_node_type_map_value_test() {
        // TODO: Implement
    }

    #[test]
    fn type_inference_node_type_argument_test() {
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
        let node_type_argument = NodeTypeArgument::TemplateTypeArgument("String".to_string());
        assert_eq!(
            node_type_argument.get_type(&mut workspace).unwrap(),
            TypeAnnotation::BuiltinType(BuiltinType {
                name: "String".to_string(),
                symbol: "String".to_string()
            })
        );
        let node_type_argument = NodeTypeArgument::TemplateTypeArgument("MyCustomType".to_string());
        assert_eq!(
            node_type_argument.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "MyCustomType".to_string(),
                symbol: "MyCustomType".to_string()
            })
        );
        let node_type_argument = NodeTypeArgument::TemplateTypeArgument("UnknownType".to_string());
        assert!(node_type_argument.get_type(&mut workspace).is_err());
    }

    #[test]
    fn type_inference_node_scilla_type_test() {
        // TODO: Implement
    }

    #[test]
    fn type_inference_address_type_field_test() {
        /*
        TODO:
            let mut workspace = Workspace {
                env: HashMap::new(),
                namespace: "".to_string()
            };
            workspace.env.insert("String".to_string(), Box::new(BuiltinType { name: "String".to_string(), symbol: "String".to_string() }));
            workspace.env.insert("MyCustomType".to_string(), Box::new(TemplateType { name: "MyCustomType".to_string(), symbol: "MyCustomType".to_string() }));
            let node_address_type_field = NodeAddressTypeField {
                identifier: NodeVariableIdentifier::VariableName("field_name".to_string()),
                type_name: NodeVariableIdentifier::VariableName("String".to_string()),
                type_annotation: None
            };
            assert_eq!(
                node_address_type_field.get_type(&mut workspace).unwrap(),
                TypeAnnotation::BuiltinType(BuiltinType { name: "String".to_string(), symbol: "String".to_string() })
            );
            */
    }

    #[test]
    fn type_inference_address_type_test() {
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
        let node_address_type = NodeAddressType {
            identifier: NodeTypeNameIdentifier::CustomType("addr".to_string()),
            type_name: "MyCustomType".to_string(),
            address_fields: vec![],
            type_annotation: None,
        };
        assert_eq!(
            node_address_type.get_type(&mut workspace).unwrap(),
            TypeAnnotation::TemplateType(TemplateType {
                name: "MyCustomType".to_string(),
                symbol: "MyCustomType".to_string()
            })
        );
        let empty_type_name = NodeAddressType {
            type_name: "".to_string(),
            ..node_address_type
        };
        assert!(empty_type_name.get_type(&mut workspace).is_err());
    }

    #[test]
    fn type_inference_message_entry_test() {
        /*
        TODO:
          let mut workspace = Workspace {
            env: HashMap::new(),
            namespace: "".to_string()
        };
          workspace.env.insert("Int32".to_string(), Box::new(BuiltinType { name: "Int32".to_string(), symbol: "Int32".to_string() }));
          workspace.env.insert("MyCustomType".to_string(), Box::new(TemplateType { name: "MyCustomType".to_string(), symbol: "MyCustomType".to_string() }));
          let msg_literal = NodeMessageEntry::MessageLiteral(
              NodeVariableIdentifier::VariableName("msg_id".to_string()),
              NodeLiteral::IntegerLiteral(42),
          );
          assert_eq!(
              msg_literal.get_type(&mut workspace).unwrap(),
              TypeAnnotation::BuiltinType(BuiltinType { name: "Int32".to_string(), symbol: "Int32".to_string() })
          );
          let msg_variable = NodeMessageEntry::MessageVariable(
              NodeVariableIdentifier::VariableName("msg_id".to_string()),
              NodeVariableIdentifier::VariableName("MyCustomType".to_string()),
          );
          assert_eq!(
              msg_variable.get_type(&mut workspace).unwrap(),
              TypeAnnotation::TemplateType(TemplateType { name: "MyCustomType".to_string(), symbol: "MyCustomType".to_string() })
          );
        */
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
