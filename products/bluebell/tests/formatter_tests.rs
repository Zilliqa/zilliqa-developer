#[cfg(test)]
mod tests {
    use bluebell::formatter::ScillaFormatter;
    use bluebell::lexer;
    use bluebell::parser;
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

    macro_rules! check_ast_formatting_ok {
        ($ast:expr, $expected:expr) => {
            let formatted = $ast.to_string();
            assert_eq!(
                formatted, $expected,
                "\nExpected:\n\n`{}`\n\nFormatted code:\n\n`{}`\n\n",
                $expected, formatted
            );
        };
    }

    macro_rules! check_formatting_ok {
        ($parser:ty, $input:expr, $expected:expr) => {
            let ast = get_ast!($parser, $input);
            check_ast_formatting_ok!(ast, $expected);
        };
    }

    #[test]
    fn bytestring_formatter() {
        // Reviewed and corrected

        check_formatting_ok!(parser::ByteStringParser, "  ByStr1234 ", "ByStr1234");
        check_formatting_ok!(parser::ByteStringParser, " ByStr11", "ByStr11");
        check_formatting_ok!(parser::ByteStringParser, "ByStr0    ", "ByStr0");
        // TODO: Constants
    }

    #[test]
    fn type_name_identifier_formatter() {
        // Reviewed and corrected
        check_formatting_ok!(parser::TypeNameIdentifierParser, "  Event  ", "Event");
        check_formatting_ok!(parser::TypeNameIdentifierParser, " Foo", "Foo");
        check_formatting_ok!(parser::TypeNameIdentifierParser, "ByStr42  ", "ByStr42");
        check_formatting_ok!(parser::TypeNameIdentifierParser, "  Event  ", "Event");
        check_formatting_ok!(parser::TypeNameIdentifierParser, "\n\tEvent\n", "Event");
        check_formatting_ok!(parser::TypeNameIdentifierParser, "Event  ", "Event");
        check_formatting_ok!(parser::TypeNameIdentifierParser, "  Foo  ", "Foo");
        check_formatting_ok!(parser::TypeNameIdentifierParser, "\n\tBar\n", "Bar");
        check_formatting_ok!(parser::TypeNameIdentifierParser, "Baz  ", "Baz");
    }

    #[test]
    fn imported_name_formatter() {
        // Reviewed and corrected

        check_formatting_ok!(parser::ImportedNameParser, "  Event  ", "Event");
        check_formatting_ok!(parser::ImportedNameParser, "Foo", "Foo");
        check_formatting_ok!(parser::ImportedNameParser, "ByStr42 ", "ByStr42");
        check_formatting_ok!(
            parser::ImportedNameParser,
            " Event     as   \n Alias ",
            "Event as Alias"
        );
        check_formatting_ok!(parser::ImportedNameParser, "Foo  as Alias", "Foo as Alias");
        check_formatting_ok!(
            parser::ImportedNameParser,
            "ByStr42 as Alias",
            "ByStr42 as Alias"
        );
    }

    #[test]
    fn import_declarations_formatter() {
        // Reviewed and corrected

        check_formatting_ok!(
            parser::ImportDeclarationsParser,
            " import \n \nFoo\n\n  \n",
            "import Foo"
        );
        check_formatting_ok!(
            parser::ImportDeclarationsParser,
            "\n import Event import     ByStr42",
            "import Event import ByStr42"
        );
        check_formatting_ok!(
            parser::ImportDeclarationsParser,
            "import Foo as    Bar\t",
            "import Foo as Bar"
        );
    }

    #[test]
    fn meta_identifiers_formatter() {
        // Reviewed and corrected

        check_formatting_ok!(parser::MetaIdentifierParser, "  Event  ", "Event");
        check_formatting_ok!(parser::MetaIdentifierParser, "   Foo.Bar   ", "Foo.Bar");
        check_formatting_ok!(parser::MetaIdentifierParser, " ABCD.Event  ", "ABCD.Event");
        check_formatting_ok!(
            parser::MetaIdentifierParser,
            " 0x1234.Event",
            "0x1234.Event"
        );
        check_formatting_ok!(parser::MetaIdentifierParser, "  ByStr ", "ByStr");
        check_formatting_ok!(parser::MetaIdentifierParser, "  ByStr10 ", "ByStr10");
    }

    #[test]
    fn variable_identifier_formatter() {
        // Reviewed and corrected
        check_formatting_ok!(parser::VariableIdentifierParser, "  foo  ", "foo");
        check_formatting_ok!(parser::VariableIdentifierParser, " _bar ", "_bar");
        check_formatting_ok!(parser::VariableIdentifierParser, " Foo.bar ", "Foo.bar");
        check_formatting_ok!(
            parser::VariableIdentifierParser,
            " ByStr42.baz ",
            "ByStr42.baz"
        );
        check_formatting_ok!(parser::VariableIdentifierParser, " Event.qux ", "Event.qux");
    }

    #[test]
    fn builtin_arguments_formatter() {
        // Reviewed and corrected
        check_formatting_ok!(parser::BuiltinArgumentsParser, "   ( )   ", "( )");
        check_formatting_ok!(parser::BuiltinArgumentsParser, " foo ", "foo");
        check_formatting_ok!(
            parser::BuiltinArgumentsParser,
            "  foo bar baz  ",
            "foo bar baz"
        );
        check_formatting_ok!(
            parser::BuiltinArgumentsParser,
            "   Event.qux   ",
            "Event.qux"
        );
    }

    #[test]
    fn scilla_types_formatter() {
        // Reviewed and corrected
        check_formatting_ok!(parser::ScillaTypeParser, "  Uint32  ", "Uint32");
        check_formatting_ok!(parser::ScillaTypeParser, "Foo( Bar )", "Foo (Bar)");
        check_formatting_ok!(
            parser::ScillaTypeParser,
            "Map  Uint32  String",
            "Map Uint32 String"
        );
        check_formatting_ok!(
            parser::ScillaTypeParser,
            "Uint32  ->  Bool",
            "Uint32 -> Bool"
        );
        check_formatting_ok!(parser::ScillaTypeParser, " (Uint32)", "(Uint32)");
        check_formatting_ok!(
            parser::ScillaTypeParser,
            "  Address  with  end",
            "Address with end"
        );
        check_formatting_ok!(
            parser::ScillaTypeParser,
            "forall  'A.  forall  'B.  (  'B  ->  'A  ->  'B)  ->  'B  ->  List  'A  ->  'B",
            "forall 'A . forall 'B . ('B -> 'A -> 'B) -> 'B -> List 'A -> 'B"
        );
        check_formatting_ok!(parser::ScillaTypeParser, " T ", "T");

        // New test cases
        check_formatting_ok!(
            parser::ScillaTypeParser,
            "Result ( List Uint32 ) String",
            "Result (List Uint32) String"
        );

        check_formatting_ok!(
            parser::ScillaTypeParser,
            " 'E ->  'F ->  'E",
            "'E -> 'F -> 'E"
        );
    }

    #[test]
    fn address_type_formatter() {
        // Reviewed, TODO: FAILING
        check_formatting_ok!(
            parser::AddressTypeParser,
            "  Foo with end  ",
            "Foo with end"
        );
        check_formatting_ok!(
            parser::AddressTypeParser,
            "  ByStr42 with end  ",
            "ByStr42 with end"
        );
        check_formatting_ok!(
            parser::AddressTypeParser,
            "  Event with end  ",
            "Event with end"
        );
        check_formatting_ok!(
            parser::AddressTypeParser,
            "  Foo with contract field field1 : Uint32, field field2 : Uint32 end   ",
            "Foo with contract field field1 : Uint32, field field2 : Uint32 end"
        );
        check_formatting_ok!(
            parser::AddressTypeParser,
            "  ByStr42 with contract field field1 : Uint32 end  ",
            "ByStr42 with contract field field1 : Uint32 end"
        );
        check_formatting_ok!(
            parser::AddressTypeParser,
            "  Event with contract end ",
            "Event with contract end"
        );
        check_formatting_ok!(
            parser::AddressTypeParser,
            "  Foo with library end  ",
            "Foo with library end"
        );
        check_formatting_ok!(
            parser::AddressTypeParser,
            "  ByStr42 with library end  ",
            "ByStr42 with library end"
        );
        check_formatting_ok!(
            parser::AddressTypeParser,
            "  Event with library end  ",
            "Event with library end"
        );
        check_formatting_ok!(
            parser::AddressTypeParser,
            "  Foo with _foo end ",
            "Foo with _foo end"
        );
        check_formatting_ok!(
            parser::AddressTypeParser,
            "  ByStr42 with _foo end  ",
            "ByStr42 with _foo end"
        );
        check_formatting_ok!(
            parser::AddressTypeParser,
            "  Event with _foo end  ",
            "Event with _foo end"
        );
        check_formatting_ok!(
            parser::AddressTypeParser,
            "  Foo with contract field field1: Uint32 end  ",
            "Foo with contract field field1 : Uint32 end"
        );
        check_formatting_ok!(
            parser::AddressTypeParser,
            "  ByStr42 with contract field field1 : Uint32, field field2: Uint32, field field3: Uint32 end  ",
            "ByStr42 with contract field field1 : Uint32, field field2 : Uint32, field field3 : Uint32 end"
        );
        check_formatting_ok!(
            parser::AddressTypeParser,
            "  Event with contract field field1: Uint32 end  ",
            "Event with contract field field1 : Uint32 end"
        );
    }

    #[test]
    fn type_map_key_formatter() {
        // BOOK:
        check_formatting_ok!(parser::TypeMapKeyParser, "  Foo  ", "Foo");
        check_formatting_ok!(parser::TypeMapKeyParser, "(Foo)   ", "(Foo)");
        //        check_formatting_ok!(parser::TypeMapKeyParser, "  ByStr20 with end  ", "ByStr20 with end");
        check_formatting_ok!(
            parser::TypeMapKeyParser,
            " (ByStr42 with contract end) ",
            "(ByStr42 with contract end)"
        );
        check_formatting_ok!(parser::TypeMapKeyParser, "  ByStr  ", "ByStr");
        check_formatting_ok!(parser::TypeMapKeyParser, " (ByStr)   ", "(ByStr)");
    }

    #[test]
    fn type_map_value_formatter() {
        check_formatting_ok!(parser::TypeMapValueParser, "  Uint32  ", "Uint32");
        check_formatting_ok!(parser::TypeMapValueParser, " Map Foo Bar ", "Map Foo Bar");
        check_formatting_ok!(parser::TypeMapValueParser, " (Uint32) ", "(Uint32)");
        check_formatting_ok!(
            parser::TypeMapValueParser,
            " Address with end ",
            "Address with end"
        );
        check_formatting_ok!(
            parser::TypeMapValueParser,
            " Foo with contract field  \n field1: \nUint32 end ",
            "Foo with contract field field1 : Uint32 end"
        );
    }
    #[test]
    fn type_map_value_arguments_formatter() {
        check_formatting_ok!(parser::TypeMapValueArgumentsParser, " (Uint32) ", "Uint32");
        check_formatting_ok!(parser::TypeMapValueArgumentsParser, " Foo ", "Foo");
        check_formatting_ok!(
            parser::TypeMapValueArgumentsParser,
            " Map Foo Bar ",
            "Map Foo Bar"
        );
    }

    #[test]
    fn type_argument_formatter() {
        check_formatting_ok!(parser::TypeArgumentParser, " Foo ", "Foo");
        check_formatting_ok!(parser::TypeArgumentParser, " ( Bar ) ", "(Bar)");
        check_formatting_ok!(parser::TypeArgumentParser, "  Uint32  ", "Uint32");
        check_formatting_ok!(parser::TypeArgumentParser, " 'A ", "'A");
        check_formatting_ok!(parser::TypeArgumentParser, " ( Uint32 ) ", "(Uint32)");
        check_formatting_ok!(
            parser::TypeArgumentParser,
            "Address  with contract   field  field1 : Uint32 ,  field field2 : Uint32 end",
            "Address with contract field field1 : Uint32, field field2 : Uint32 end"
        );
        check_formatting_ok!(
            parser::TypeArgumentParser,
            "  Map Uint32  Bool",
            "Map Uint32 Bool"
        );
    }

    /*
    TODO:
    #[test]
    fn full_expressions_formatter() {
        check_formatting_ok!(
            parser::FullExpressionParser,
            "let x = Int32  42 in  X",
            "let x = Int32 42 in X"
        );
        check_formatting_ok!(
            parser::FullExpressionParser,
            "let   x:  Int32 =  Int32   42 in   X",
            "let x: Int32 = Int32 42 in X"
        );
        check_formatting_ok!(
            parser::FullExpressionParser,
            "    let x = Uint128 42 in builtin  lt x x   ",
            "let x = Uint128 42 in builtin lt x x"
        );
        check_formatting_ok!(
            parser::FullExpressionParser,
            "fun (   x:    Int32) =>   builtin lt x x",
            "fun (x: Int32) => builtin lt x x"
        );
        check_formatting_ok!(
            parser::FullExpressionParser,
            "fun (x:   Int32) =>   foo",
            "fun (x: Int32) => foo"
        );
        check_formatting_ok!(parser::FullExpressionParser, "  foo  ", "foo");
        check_formatting_ok!(
            parser::FullExpressionParser,
            "  foo   bar  baz ",
            "foo bar baz"
        );
        check_formatting_ok!(parser::FullExpressionParser, " UInt32    42", "UInt32 42");
        check_formatting_ok!(parser::FullExpressionParser, "  true  ", "true");
        check_formatting_ok!(
            parser::FullExpressionParser,
            "builtin blabla a   b  ",
            "builtin blabla a b"
        );
        check_formatting_ok!(
            parser::FullExpressionParser,
            "{ foo:  UInt32 42;   bar: Int64   23 }",
            "{ foo: UInt32 42; bar: Int64 23 }"
        );
        check_formatting_ok!(
            parser::FullExpressionParser,
            "match foo  with  \n| False  => True \n| _ =>   False \nend",
            "match foo with \n| False => True \n| _ => False \nend"
        );
        check_formatting_ok!(parser::FullExpressionParser, " Foo ", "Foo");
        check_formatting_ok!(parser::FullExpressionParser, " Foo  m   n", "Foo m n");
        check_formatting_ok!(
            parser::FullExpressionParser,
            "tfun 'T =>  fun (x:   'T)  => x",
            "tfun 'T => fun (x: 'T) => x"
        );
        check_formatting_ok!(parser::FullExpressionParser, "   @foo  Type  ", "@foo Type");
    }
    */

    #[test]
    fn atomic_expression_formatter() {
        check_formatting_ok!(parser::AtomicExpressionParser, "  foo    ", "foo");
        check_formatting_ok!(parser::AtomicExpressionParser, " Uint32 42", "Uint32 42");
        check_formatting_ok!(parser::AtomicExpressionParser, "0x123abc  ", "0x123abc");
        check_formatting_ok!(
            parser::AtomicExpressionParser,
            r#" "string" "#,
            r#""string""#
        );
    }

    #[test]
    fn value_literal_formatter() {
        // Reviewed and correct
        check_formatting_ok!(parser::AtomicExpressionParser, "  foo ", "foo");
        check_formatting_ok!(
            parser::AtomicExpressionParser,
            "  Uint32   42  ",
            "Uint32 42"
        );
        check_formatting_ok!(
            parser::AtomicExpressionParser,
            "     0x123abc    ",
            "0x123abc"
        );
        check_formatting_ok!(
            parser::AtomicExpressionParser,
            r#"  "string"  "#,
            r#""string""#
        );
        check_formatting_ok!(
            parser::AtomicExpressionParser,
            r#" Emp Uint32 Uint32 "#,
            r#"Emp Uint32{Uint32}"#
        );
    }
    #[test]
    fn map_access_formatter() {
        check_formatting_ok!(parser::MapAccessParser, " [foo] ", "[foo]");
        check_formatting_ok!(parser::MapAccessParser, "[bar123]  ", "[bar123]");
        check_formatting_ok!(parser::MapAccessParser, "    [_result]   ", "[_result]");
    }

    #[test]
    fn pattern_formatter() {
        check_formatting_ok!(parser::PatternParser, "  _", "_");
        check_formatting_ok!(parser::PatternParser, "  foo  ", "foo");
        check_formatting_ok!(parser::PatternParser, "Bar42   ", "Bar42");
        check_formatting_ok!(parser::PatternParser, "Bar42   \n_", "Bar42 _");
        check_formatting_ok!(parser::PatternParser, "Bar42 hello", "Bar42 hello");
        check_formatting_ok!(parser::PatternParser, "  Bar42(  _)", "Bar42 (_)");
        check_formatting_ok!(parser::PatternParser, "Bar42(Foo )", "Bar42 (Foo)");
        check_formatting_ok!(parser::PatternParser, "Bar42(Foo Bar)", "Bar42 (Foo Bar)");
        check_formatting_ok!(
            parser::PatternParser,
            "Bar42    (\nByStr42   Int32)",
            "Bar42 (ByStr42 Int32)"
        );
        check_formatting_ok!(
            parser::PatternParser,
            "Bar42(Foo.Bar \nBar.Baz)  ",
            "Bar42 (Foo.Bar Bar.Baz)"
        );
    }

    #[test]
    fn argument_pattern_formatter() {
        check_formatting_ok!(parser::ArgumentPatternParser, "_", "_");
        check_formatting_ok!(parser::ArgumentPatternParser, " foo", "foo");
        check_formatting_ok!(parser::ArgumentPatternParser, "MyType   ", "MyType");
        check_formatting_ok!(parser::ArgumentPatternParser, "  (baz) ", "(baz)");
        check_formatting_ok!(parser::ArgumentPatternParser, " my_type  ", "my_type");
        check_formatting_ok!(parser::ArgumentPatternParser, " (Bar42 _) ", "(Bar42 _)");
    }

    #[test]
    fn pattern_match_expression_clause_formatter() {
        check_formatting_ok!(
            parser::PatternMatchExpressionClauseParser,
            " | _               => Uint32 42 ",
            "| _ => Uint32 42"
        );
        check_formatting_ok!(
            parser::PatternMatchExpressionClauseParser,
            "|Foo =>         \n\n\n\nUint32 42",
            "| Foo => Uint32 42"
        );
        check_formatting_ok!(
            parser::PatternMatchExpressionClauseParser,
            " | Foo \n\n_\n\n\t\t\t       Bar => Uint32 42",
            "| Foo _ Bar => Uint32 42"
        );
        check_formatting_ok!(
            parser::PatternMatchExpressionClauseParser,
            "|  Bar _ => Uint32 42  ",
            "| Bar _ => Uint32 42"
        );
        check_formatting_ok!(
            parser::PatternMatchExpressionClauseParser,
            "| Bar    _ => Int32 -1",
            "| Bar _ => Int32 -1"
        );
        check_formatting_ok!(
            parser::PatternMatchExpressionClauseParser,
            "|  Foo _ \n\n\t =>    let x = Uint32 1 \n\n\nin x         ",
            "| Foo _ => let x = Uint32 1 in x"
        );
    }

    #[test]
    fn message_entry_formatter() {
        check_formatting_ok!(
            parser::MessageEntryParser,
            " foo: Uint32 42 ",
            "foo : Uint32 42"
        );
        check_formatting_ok!(parser::MessageEntryParser, " foo: bar ", "foo : bar");
        check_formatting_ok!(parser::MessageEntryParser, " foo: 0x1337 ", "foo : 0x1337");
    }

    #[test]
    fn test_type_annotation_formatter() {
        check_formatting_ok!(parser::TypeAnnotationParser, ": Int", ": Int");
        check_formatting_ok!(
            parser::TypeAnnotationParser,
            " : MyTypeOrEnumLikeIdentifier",
            ": MyTypeOrEnumLikeIdentifier"
        );
        check_formatting_ok!(parser::TypeAnnotationParser, ": ByStr32 ", ": ByStr32");
        check_formatting_ok!(
            parser::TypeAnnotationParser,
            "  :    \n (Map       ByStr32 Uint32)  ",
            ": (Map ByStr32 Uint32)"
        );
        check_formatting_ok!(
            parser::TypeAnnotationParser,
            ": (Map  \n\n\t ByStr32 (\t\tMap \nByStr32 Uint32) )",
            ": (Map ByStr32 (Map ByStr32 Uint32))"
        );
        check_formatting_ok!(
            parser::TypeAnnotationParser,
            " : (List MyTypeOrEnumLikeIdentifier)",
            ": (List MyTypeOrEnumLikeIdentifier)"
        );
        check_formatting_ok!(
            parser::TypeAnnotationParser,
            ": (Pair MyTypeOrEnumLikeIdentifier1 MyTypeOrEnumLikeIdentifier2 ) ",
            ": (Pair MyTypeOrEnumLikeIdentifier1 MyTypeOrEnumLikeIdentifier2)"
        );
        check_formatting_ok!(
            parser::TypeAnnotationParser,
            " : (Pair (Pair Int Bool) (List MyTypeOrEnumLikeIdentifier))",
            ": (Pair (Pair Int Bool) (List MyTypeOrEnumLikeIdentifier))"
        );
    }

    #[test]
    fn typed_identifier_formatter() {
        // Reviewed and corrected
        check_formatting_ok!(parser::TypedIdentifierParser, "  foo: Int  ", "foo : Int");
        check_formatting_ok!(
            parser::TypedIdentifierParser,
            " bar: ByStr20",
            "bar : ByStr20"
        );
        check_formatting_ok!(
            parser::TypedIdentifierParser,
            "baz: (Int Bool)    ",
            "baz : (Int Bool)"
        );
    }

    #[test]
    fn test_statement_formatter() {
        check_formatting_ok!(parser::StatementParser, "foo <- bar", "foo <- bar");
        check_formatting_ok!(
            parser::StatementParser,
            " remoteFetchStatement ",
            "remoteFetchStatement"
        );
        check_formatting_ok!(parser::StatementParser, "foo := bar\t", "foo := bar");
        check_formatting_ok!(
            parser::StatementParser,
            " foo = Uint32 42 ",
            "foo = Uint32 42"
        );
        check_formatting_ok!(
            parser::StatementParser,
            " foo <- &Event   ",
            "foo <- &Event"
        );
        check_formatting_ok!(
            parser::StatementParser,
            "foo <-\n qux[baz]  ",
            "foo <- qux[baz]"
        );
        check_formatting_ok!(
            parser::StatementParser,
            " foo <-        \n\t exists baz[bar] ",
            "foo <- exists baz[bar]"
        );
        check_formatting_ok!(
            parser::StatementParser,
            "foo[bar] := qux ",
            "foo[bar] := qux"
        );
        check_formatting_ok!(
            parser::StatementParser,
            "delete foo[bar]\t",
            "delete foo[bar]"
        );
        check_formatting_ok!(parser::StatementParser, " accept \t", "accept");
        check_formatting_ok!(parser::StatementParser, "send foo    ", "send foo");
        check_formatting_ok!(parser::StatementParser, "  event foo  ", "event foo");
        check_formatting_ok!(parser::StatementParser, "throw   ", "throw");
        check_formatting_ok!(
            parser::StatementParser,
            " match foo with | False => True | _ => False end ",
            "match foo with\n  | False => True\n  | _ => False\nend"
        );
        check_formatting_ok!(
            parser::StatementParser,
            "  match foo with | _ => value end  ",
            "match foo with\n  | _ => value\nend"
        );
        check_formatting_ok!(parser::StatementParser, " Foo\tbar\tbaz    ", "Foo bar baz");
        check_formatting_ok!(
            parser::StatementParser,
            " forall foo    Event ",
            "forall foo Event"
        );
    }

    #[test]
    fn blockchain_fetch_arguments_formatter() {
        check_formatting_ok!(
            parser::BlockchainFetchArgumentsParser,
            " (foo bar) ",
            "(foo bar)"
        );
        check_formatting_ok!(parser::BlockchainFetchArgumentsParser, " (x)   ", "(x)");
        check_formatting_ok!(
            parser::BlockchainFetchArgumentsParser,
            "(y z a)  ",
            "(y z a)"
        );
    }

    #[test]
    fn statement_block_formatter() {
        check_formatting_ok!(
            parser::StatementBlockParser,
            " x<-y;     z:= a",
            "x <- y;\nz := a"
        );
        check_formatting_ok!(parser::StatementBlockParser, "  accept   ", "accept");
        check_formatting_ok!(parser::StatementBlockParser, "  send x  ", "send x");
        check_formatting_ok!(
            parser::StatementBlockParser,
            "  event myEvent  ",
            "event myEvent"
        );
        check_formatting_ok!(
            parser::StatementBlockParser,
            "  match x with | _ => accept end  ",
            "match x with\n  | _ => accept\nend"
        );
        check_formatting_ok!(
            parser::StatementBlockParser,
            "  match x with | _ => y <- z end ",
            "match x with\n  | _ => y <- z\nend"
        );
        check_formatting_ok!(
            parser::StatementBlockParser,
            " MyComponent y  ",
            "MyComponent y"
        );
        check_formatting_ok!(
            parser::StatementBlockParser,
            "MyComponent y; forall foo Event ; match x with | _ => y <- z end",
            "MyComponent y;\nforall foo Event;\nmatch x with\n  | _ => y <- z\nend"
        );
    }

    #[test]
    fn parameter_pair_formatter() {
        check_formatting_ok!(parser::ParameterPairParser, "foo: Uint32", "foo : Uint32");
        check_formatting_ok!(parser::ParameterPairParser, "bar :Bool", "bar : Bool");
        check_formatting_ok!(
            parser::ParameterPairParser,
            "    baz:   Address",
            "baz : Address"
        );
        check_formatting_ok!(
            parser::ParameterPairParser,
            "qux : Map Uint32 Bool",
            "qux : Map Uint32 Bool"
        );
    }

    /*
      #[test]
      fn component_definition_formatter() {
          check_formatting_ok!(
              parser::ComponentDefinitionParser,
              "transition myTransition(param1: Uint32,   param2: Uint32) end",
              "transition myTransition\n    (param1 : Uint32, param2 : Uint32)\nend"
          );
          check_formatting_ok!(
              parser::ComponentDefinitionParser,
              " procedure    myProcedure(param: Uint32) end",
              "procedure myProcedure\n    (param : Uint32)\nend"
          );
          check_formatting_ok!(
              parser::ComponentDefinitionParser,
              "procedure    myProcedure(param: Map ByStr32   ByStr32)   param end",
              "procedure myProcedure\n    (param : Map ByStr32 ByStr32)\n    param\nend"
          );
          check_formatting_ok!(
          parser::ComponentDefinitionParser,
          "transition myTransition(param: Bool) match param with | False => True | _ => False end end",
          "transition myTransition\n    (param : Bool)\n    match param with\n    | False => True\n    | _ => False\n    end\nend"
      );
      }
    */

    /*
    TODO:
    #[test]
    fn procedure_definition_formatter() {
        check_formatting_ok!(
            parser::ProcedureDefinitionParser,
            "  procedure   foo()  end ",
            "procedure foo() end"
        );
        check_formatting_ok!(
            parser::ProcedureDefinitionParser,
            "procedure  bar( x: Int32 , y: Uint32 )  baz x y end",
            "procedure bar(x: Int32, y: Uint32) baz x y end"
        );
    }
    */

    /*
    TODO:
    #[test]
    fn transition_definition_formatter() {
        check_formatting_ok!(
            parser::TransitionDefinitionParser,
            "   transition    foo()   bar end  ",
            "transition foo() bar end"
        );
        check_formatting_ok!(
            parser::TransitionDefinitionParser,
            "transition bar( x: Int32 ,y: Uint32 ) foo end",
            "transition bar(x: Int32, y: Uint32) foo end"
        );
        check_formatting_ok!(
            parser::TransitionDefinitionParser,
            "transition qux(   bar: Bool ) bar end ",
            "transition qux(bar: Bool) bar end"
        );
        check_formatting_ok!(
            parser::TransitionDefinitionParser,
            " transition  empty()   end  ",
            "transition empty() end"
        );
    }
    */

    #[test]
    fn component_id_formatter() {
        check_formatting_ok!(parser::ComponentIdParser, "  MyType  ", "MyType");
        check_formatting_ok!(parser::ComponentIdParser, " Event42 ", "Event42");
        check_formatting_ok!(parser::ComponentIdParser, "   Foo_Bar   ", "Foo_Bar");
        check_formatting_ok!(parser::ComponentIdParser, "ByStr42  ", "ByStr42");
        check_formatting_ok!(parser::ComponentIdParser, "  regular_id", "regular_id");
    }
    #[test]
    fn component_parameters_formatter() {
        check_formatting_ok!(parser::ComponentParametersParser, "  ()  ", "()");
        check_formatting_ok!(
            parser::ComponentParametersParser,
            " (a: Int32) ",
            "(a : Int32)"
        );
        check_formatting_ok!(
            parser::ComponentParametersParser,
            "(a: Int32, b: Bool)",
            "(a : Int32, b : Bool)"
        );
        check_formatting_ok!(
            parser::ComponentParametersParser,
            " (a: Int32, b: Bool, c: String) ",
            "(a : Int32, b : Bool, c : String)"
        );
        check_formatting_ok!(
            parser::ComponentParametersParser,
            " (a: ByStr20, b: Map ByStr20 (Uint256)) ",
            "(a : ByStr20, b : Map ByStr20 (Uint256))"
        );
    }

    #[test]
    fn component_body_formatter() {
        check_formatting_ok!(
            parser::ComponentBodyParser,
            "
    RequireNotPaused;
    RequireContractOwner;
    is_paused := true;
    e = {
      _eventname: \"Pause\";
      is_paused: true
    };
    event e
  end
  ",
            "RequireNotPaused;
RequireContractOwner;
is_paused := true;
e = {
  _eventname : \"Pause\";
  is_paused : true
};
event e
end"
        );
        check_formatting_ok!(
            parser::ComponentBodyParser,
            "
    RequirePaused;
    RequireContractOwner;
    is_paused := false;
    e = {
      _eventname: \"Unpause\";
      is_paused: false
    };
    event e
  end
  ",
            "RequirePaused;
RequireContractOwner;
is_paused := false;
e = {
  _eventname : \"Unpause\";
  is_paused : false
};
event e
end"
        );
        check_formatting_ok!(
        parser::ComponentBodyParser,
        "
      current_init <-& init.dApp;
      xPointsDApp = \"xpoints\"; get_addr <-& current_init.dns[xPointsDApp]; addr = option_bystr20_value get_addr;
      is_xPoints = builtin eq _sender addr; match is_xPoints with
      | True => | False => e = { _exception : \"donate.tyron-WrongCaller\" }; throw e end;
      get_xPoints <- xpoints[_origin]; x_points = option_uint128_value get_xPoints; IsSufficient x_points amount;
      new_bal = builtin sub x_points amount; xpoints[_origin] := new_bal end
  ",
        "current_init <-& init.dApp
;
xPointsDApp = \"xpoints\";
get_addr <-& current_init.dns[  xPointsDApp]
;
addr = option_bystr20_value get_addr;
is_xPoints = builtin eq _sender addr;
match is_xPoints with
  | True
  | False => e = {
  _exception : \"donate.tyron-WrongCaller\"
};
throw e
end;
get_xPoints <- xpoints[_origin];
x_points = option_uint128_value get_xPoints;
IsSufficient x_points amount;
new_bal = builtin sub x_points amount;
xpoints[_origin] := new_bal
end"
    );
    }

    /*
    TODO:
    #[test]
    fn contract_fields_formatter() {
        check_formatting_ok!(
            parser::ContractFieldParser,
            "field foo: Int32 = Int32 42",
            "field foo : Int32 = Int32 42" // Add a space before the colon
        );
        check_formatting_ok!(
            parser::ContractFieldParser,
            "field bar: Map ByStr32 (List Uint32) = Emp ByStr32 (List Uint32)",
            "field bar : Map ByStr32 (List Uint32) = Emp ByStr32 (List Uint32)" // Add a space before the colon
        );
        check_formatting_ok!(
            parser::ContractFieldParser,
            "field baz: Event = Event",
            "field baz : Event = Event" // Add a space before the colon
        );
        // Additional test cases
        check_formatting_ok!(
            parser::ContractFieldParser,
            "field nested_map: Map ByStr20 (Map Uint32 Bool) = Emp ByStr20 (Map Uint32 Bool)",
            "field nested_map : Map ByStr20 (Map Uint32 Bool) = Emp ByStr20 (Map Uint32 Bool)" // Add a space before the colon
        );
        check_formatting_ok!(
            parser::ContractFieldParser,
            "field nested_list: List (List Uint32) = Cons (List Uint32) [(Uint32 1)] Nil (List (List Uint32))",
            "field nested_list : List (List Uint32) = Cons (List Uint32) [(Uint32 1)] Nil (List (List Uint32))" // Add a space before the colon
        );
    }
    */

    /*
    TODO:
    #[test]
    fn test_with_constraint_formatter() {
        // BOOK:
        check_formatting_ok!(
            parser::WithConstraintParser,
            "  with   builtin   blt   end_of_life   =>  ",
            "with builtin blt end_of_life =>"
        );
        check_formatting_ok!(
            parser::WithConstraintParser,
            "with builtin add {UInt32} one  =>",
            "with builtin add {UInt32} one =>"
        );
        check_formatting_ok!(
            parser::WithConstraintParser,
            "  with true =>",
            "with true =>"
        );
        check_formatting_ok!(
            parser::WithConstraintParser,
            "  with false =>",
            "with false =>"
        );
        check_formatting_ok!(
            parser::WithConstraintParser,
            "with variableIdentifier =>",
            "with variableIdentifier =>"
        );
    }
    */

    /*
    TODO:
    #[test]
    fn contract_definition_formatter() {
        check_formatting_ok!(
            parser::ContractDefinitionParser,
            " contract  MyContract(   ) ",
            "contract MyContract()"
        );
        check_formatting_ok!(
            parser::ContractDefinitionParser,
            " contract MyContract( address : ByStr20 )",
            "contract MyContract(address: ByStr20)"
        );
        check_formatting_ok!(
            parser::ContractDefinitionParser,
            "contract MyContract(address: ByStr20     )   with true  =>",
            "contract MyContract(address: ByStr20) with true =>"
        );
        check_formatting_ok!(
            parser::ContractDefinitionParser,
            " contract MyContract(address: ByStr20) with true => field field1: Uint32  = Uint32 1 ",
            "contract MyContract(address: ByStr20) with true => field field1: Uint32 = Uint32 1"
        );
    }
    */

    #[test]
    fn type_alternative_clause_formatter() {
        check_formatting_ok!(
            parser::TypeAlternativeClauseParser,
            "  | MyType  ",
            "| MyType"
        );
        check_formatting_ok!(
            parser::TypeAlternativeClauseParser,
            "| MyType of   Int",
            "| MyType of Int"
        );
        check_formatting_ok!(
            parser::TypeAlternativeClauseParser,
            "| ByStr123   of Map MyType    Int",
            "| ByStr123 of Map MyType Int"
        );
    }

    /*
    TODO:
    #[test]
    fn library_single_definition_formatter() {
        check_formatting_ok!(
            parser::LibrarySingleDefinitionParser,
            "  let   foo =  Int32    42   ",
            "let foo = Int32 42"
        );
        check_formatting_ok!(
            parser::LibrarySingleDefinitionParser,
            "  let foo:  Int32 = Int32   42  ",
            "let foo: Int32 = Int32 42"
        );
        check_formatting_ok!(
            parser::LibrarySingleDefinitionParser,
            "type Foo  ",
            "type Foo"
        );
        check_formatting_ok!(
            parser::LibrarySingleDefinitionParser,
            "  type Foo =   | Bar  | Baz ",
            "type Foo = | Bar | Baz"
        );
    }
    */

    /*
    TODO:
    #[test]
    fn library_definition_formatter() {
        check_formatting_ok!(
            parser::LibraryDefinitionParser,
            "  library Foo",
            "library Foo"
        );
        check_formatting_ok!(
            parser::LibraryDefinitionParser,
            "library Bar let x = Int32 10",
            "library Bar let x = Int32 10"
        );
        check_formatting_ok!(
            parser::LibraryDefinitionParser,
            "library Baz type Quux",
            "library Baz type Quux"
        );
        check_formatting_ok!(
            parser::LibraryDefinitionParser,
            "library Qux type Quux = | Event",
            "library Qux type Quux = | Event"
        );
        check_formatting_ok!(
            parser::LibraryDefinitionParser,
            "library Quux type Quux = | Event of Uint256",
            "library Quux type Quux = | Event of Uint256"
        );
        check_formatting_ok!(
            parser::LibraryDefinitionParser,
            "library Quuz type Quux = | Event of Uint256 | AnotherEvent of ByStr20",
            "library Quuz type Quux = | Event of Uint256 | AnotherEvent of ByStr20"
        );
        check_formatting_ok!(
            parser::LibraryDefinitionParser,
            "library Qoorx let x: Int32 = Int32 42",
            "library Qoorx let x: Int32 = Int32 42"
        );
    }
    */
}
