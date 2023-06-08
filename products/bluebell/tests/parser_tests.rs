#[cfg(test)]
mod tests {
    use bluebell::formatter::ScillaFormatter;
    use bluebell::lexer;
    use bluebell::lexer::Lexer;
    use bluebell::parser;

    use std::collections::HashMap;
    use std::env;
    use std::fs::File;
    use std::io::Read;
    use std::process;

    macro_rules! check_ok {
        ($parser:ty, $result:expr) => {
            let mut errors = vec![];
            assert!(<$parser>::new()
                .parse(&mut errors, bluebell::lexer::Lexer::new($result))
                .is_ok());
        };
    }

    macro_rules! check_err {
        ($parser:ty, $result:expr) => {
            let mut errors = vec![];
            assert!(<$parser>::new()
                .parse(&mut errors, bluebell::lexer::Lexer::new($result))
                .is_err());
        };
    }

    #[test]
    fn bytestring_parser() {
        check_ok!(parser::ByteStringParser, "ByStr1234");
        check_ok!(parser::ByteStringParser, "ByStr11");
        check_ok!(parser::ByteStringParser, "ByStr0");

        check_err!(parser::ByteStringParser, "ByStr 3");
        check_err!(parser::ByteStringParser, "ByStr 11");
        check_err!(parser::ByteStringParser, "ByStr 0");
        check_err!(parser::ByteStringParser, "ByStr");
    }

    #[test]
    fn type_name_identifier_parser() {
        check_ok!(parser::TypeNameIdentifierParser, "Event");
        check_ok!(parser::TypeNameIdentifierParser, "Foo");
        check_ok!(parser::TypeNameIdentifierParser, "ByStr42");

        check_err!(parser::TypeNameIdentifierParser, "bystr");
        check_err!(parser::TypeNameIdentifierParser, "_foo");
        check_err!(parser::TypeNameIdentifierParser, "ByStr 0");
        check_err!(parser::TypeNameIdentifierParser, "Type With Spaces");
    }

    #[test]
    fn imported_name_parser() {
        check_ok!(parser::ImportedNameParser, "Event");
        check_ok!(parser::ImportedNameParser, "Foo");
        check_ok!(parser::ImportedNameParser, "ByStr42");
        check_ok!(parser::ImportedNameParser, "Event as Alias");
        check_ok!(parser::ImportedNameParser, "Foo as Alias");
        check_ok!(parser::ImportedNameParser, "ByStr42 as Alias");

        check_err!(parser::ImportedNameParser, "foo");
        check_err!(parser::ImportedNameParser, "Type With Spaces");
        check_err!(parser::ImportedNameParser, "Foo As Alias");
        check_err!(parser::ImportedNameParser, "Event as");
    }

    #[test]
    fn import_declarations() {
        check_ok!(parser::ImportDeclarationsParser, "import Foo");
        check_ok!(
            parser::ImportDeclarationsParser,
            "import Event import ByStr42"
        );
        check_ok!(parser::ImportDeclarationsParser, "import Foo as Bar");

        check_err!(parser::ImportDeclarationsParser, "foo");
        check_err!(parser::ImportDeclarationsParser, "import");
        check_err!(parser::ImportDeclarationsParser, "import Foo import");
    }

    #[test]
    fn meta_identifiers() {
        check_ok!(parser::MetaIdentifierParser, "Event");
        check_ok!(parser::MetaIdentifierParser, "Foo.Bar");
        check_ok!(parser::MetaIdentifierParser, "ABCD.Event");
        check_ok!(parser::MetaIdentifierParser, "0x1234.Event");
        check_ok!(parser::MetaIdentifierParser, "ByStr");

        check_err!(parser::MetaIdentifierParser, "_foo");
        check_err!(parser::MetaIdentifierParser, "Type With Spaces");
        check_err!(parser::MetaIdentifierParser, "Foo .. Bar");
        check_err!(parser::MetaIdentifierParser, "0x1234 - Bar");
        check_err!(parser::MetaIdentifierParser, "Foo.");
        check_err!(parser::MetaIdentifierParser, "0x1234.42");
    }

    #[test]
    fn variable_identifier() {
        check_ok!(parser::VariableIdentifierParser, "foo");
        check_ok!(parser::VariableIdentifierParser, "_bar");
        check_ok!(parser::VariableIdentifierParser, "Foo.bar");
        check_ok!(parser::VariableIdentifierParser, "ByStr42.baz");
        check_ok!(parser::VariableIdentifierParser, "Event.qux");

        check_err!(parser::VariableIdentifierParser, "42");
        check_err!(parser::VariableIdentifierParser, "_");
        check_err!(parser::VariableIdentifierParser, "Type With Spaces.baz");
        // check_err!(parser::VariableIdentifierParser, "Bystr.qux");
        check_err!(parser::VariableIdentifierParser, "Event42.Bar");
        check_err!(parser::VariableIdentifierParser, "Foo.");
    }

    #[test]
    fn builtin_arguments() {
        check_ok!(parser::BuiltinArgumentsParser, "( )");
        check_ok!(parser::BuiltinArgumentsParser, "foo");
        check_ok!(parser::BuiltinArgumentsParser, "foo bar baz");
        check_ok!(parser::BuiltinArgumentsParser, "Event.qux");

        check_err!(parser::BuiltinArgumentsParser, "42");
        check_err!(parser::BuiltinArgumentsParser, "_");
        check_err!(parser::BuiltinArgumentsParser, "Type With Spaces.baz");
        check_err!(parser::BuiltinArgumentsParser, "Event42.Bar");
        check_err!(parser::BuiltinArgumentsParser, "Foo.");
    }

    #[test]
    fn scilla_types() {
        check_ok!(parser::ScillaTypeParser, "Uint32");
        check_ok!(parser::ScillaTypeParser, "Foo(Bar)");
        check_ok!(parser::ScillaTypeParser, "Map Uint32 String");
        check_ok!(parser::ScillaTypeParser, "Uint32 -> Bool");
        check_ok!(parser::ScillaTypeParser, "(Uint32)");
        check_ok!(parser::ScillaTypeParser, "Address with end");
        check_ok!(
            parser::ScillaTypeParser,
            "forall 'A. forall 'B. ( 'B -> 'A -> 'B) -> 'B -> List 'A -> 'B"
        );
        check_ok!(parser::ScillaTypeParser, "T");

        check_err!(parser::ScillaTypeParser, "Map");
        check_err!(parser::ScillaTypeParser, "Uint32 ->");
        check_err!(parser::ScillaTypeParser, "-> Bool");
        check_err!(parser::ScillaTypeParser, "address with");
        check_err!(parser::ScillaTypeParser, "address with Foo end");
        check_err!(parser::ScillaTypeParser, "forall T. Map(T, Uint32)");
        check_err!(parser::ScillaTypeParser, "Foo(Bar");
    }

    #[test]
    fn test_address_type() {
        check_ok!(parser::AddressTypeParser, "Foo with end");
        check_ok!(parser::AddressTypeParser, "ByStr42 with end");
        check_ok!(parser::AddressTypeParser, "Event with end");
        check_ok!(
            parser::AddressTypeParser,
            "Foo with contract field field1: Uint32, field field2: Uint32 end"
        );
        check_ok!(
            parser::AddressTypeParser,
            "ByStr42 with contract field field1: Uint32 end"
        );
        check_ok!(parser::AddressTypeParser, "Event with contract end");
        check_ok!(parser::AddressTypeParser, "Foo with library end");
        check_ok!(parser::AddressTypeParser, "ByStr42 with library end");
        check_ok!(parser::AddressTypeParser, "Event with library end");
        check_ok!(parser::AddressTypeParser, "Foo with _foo end");
        check_ok!(parser::AddressTypeParser, "ByStr42 with _foo end");
        check_ok!(parser::AddressTypeParser, "Event with _foo end");
        check_ok!(
            parser::AddressTypeParser,
            "Foo with contract field field1: Uint32 end"
        );
        check_ok!(parser::AddressTypeParser, "ByStr42 with contract field field1: Uint32, field field2: Uint32, field field3: Uint32 end");
        check_ok!(
            parser::AddressTypeParser,
            "Event with contract field field1: Uint32 end"
        );

        check_err!(parser::AddressTypeParser, "foo with end");
        check_err!(parser::AddressTypeParser, "Foo with ");
        check_err!(parser::AddressTypeParser, "Foo with foo bar: Uint32 end");
        check_err!(
            parser::AddressTypeParser,
            "Foo with contract field1: Uint32, field2: Uint32"
        );
        check_err!(parser::AddressTypeParser, "Foo with contract, end");
    }

    #[test]
    fn test_type_map_key() {
        check_ok!(parser::TypeMapKeyParser, "Foo");
        check_ok!(parser::TypeMapKeyParser, "(Foo)");
        check_ok!(parser::TypeMapKeyParser, "Foo with end");
        check_ok!(parser::TypeMapKeyParser, "(Foo with end)");
        check_ok!(parser::TypeMapKeyParser, "(ByStr42 with contract end)");
        check_ok!(parser::TypeMapKeyParser, "Foo with library end");

        check_err!(parser::TypeMapKeyParser, "foo");
        check_err!(parser::TypeMapKeyParser, "Foo()");
        check_err!(parser::TypeMapKeyParser, "(Foo with bar end)");
        check_err!(parser::TypeMapKeyParser, "(42)");
    }

    #[test]
    fn type_map_value() {
        check_ok!(parser::TypeMapValueParser, "Uint32");
        check_ok!(parser::TypeMapValueParser, "Map Foo Bar");
        check_ok!(parser::TypeMapValueParser, "(Uint32)");
        check_ok!(parser::TypeMapValueParser, "Address with end");
        check_ok!(
            parser::TypeMapValueParser,
            "Foo with contract field field1: Uint32 end"
        );

        check_err!(parser::TypeMapValueParser, "foo");
        check_err!(parser::TypeMapValueParser, "bystr1");
        check_err!(parser::TypeMapValueParser, "event");
        check_err!(parser::TypeMapValueParser, "map(foo, bar)");
        check_err!(parser::TypeMapValueParser, "(42)");
        check_err!(parser::TypeMapValueParser, "address with");
        check_err!(parser::TypeMapValueParser, "foo with foo bar");
    }

    #[test]
    fn type_map_value_arguments() {
        check_ok!(parser::TypeMapValueArgumentsParser, "(Uint32)");
        check_ok!(parser::TypeMapValueArgumentsParser, "Foo");
        check_ok!(parser::TypeMapValueArgumentsParser, "Map Foo Bar");

        check_err!(parser::TypeMapValueArgumentsParser, "Foo Bar");
        check_err!(parser::TypeMapValueArgumentsParser, "map(foo, bar)");
        check_err!(parser::TypeMapValueArgumentsParser, "(42)()");
        check_err!(parser::TypeMapValueArgumentsParser, "(Uint32");
        check_err!(parser::TypeMapValueArgumentsParser, "Map(Foo)");
        check_err!(parser::TypeMapValueArgumentsParser, "Map(Foo, Bar)");
    }

    #[test]
    fn type_argument() {
        check_ok!(parser::TypeArgumentParser, "Foo");
        check_ok!(parser::TypeArgumentParser, "(Bar)");
        check_ok!(parser::TypeArgumentParser, "Uint32");
        check_ok!(parser::TypeArgumentParser, "'A");
        check_ok!(parser::TypeArgumentParser, "(Uint32)");
        check_ok!(
            parser::TypeArgumentParser,
            "Address with contract field field1: Uint32, field field2: Uint32 end"
        );
        check_ok!(parser::TypeArgumentParser, "Map Uint32 Bool");

        check_err!(parser::TypeArgumentParser, "foo bar");
        check_err!(parser::TypeArgumentParser, "123");
        check_err!(parser::TypeArgumentParser, "foo.bar");
        check_err!(parser::TypeArgumentParser, "mapUint32Uint32");
        check_err!(parser::TypeArgumentParser, "'_A");
        check_err!(parser::TypeArgumentParser, "(map(Int32, String))");
        check_err!(parser::TypeArgumentParser, "Foo.bar");
        check_err!(parser::TypeArgumentParser, "Map(Int32, String, Bool)");
    }

    #[test]
    fn full_expressions() {
        check_ok!(parser::FullExpressionParser, "let x = Int32 42 in X");
        check_ok!(parser::FullExpressionParser, "let x: Int32 = Int32 42 in X");
        check_ok!(
            parser::FullExpressionParser,
            "let x = Uint128 42 in builtin lt x x"
        );
        check_ok!(
            parser::FullExpressionParser,
            "fun (x: Int32) => builtin lt x x"
        );
        check_ok!(parser::FullExpressionParser, "fun (x: Int32) => foo");
        check_ok!(parser::FullExpressionParser, "foo");
        check_ok!(parser::FullExpressionParser, "foo bar baz");
        check_ok!(parser::FullExpressionParser, "UInt32 42");
        check_ok!(parser::FullExpressionParser, "true");
        check_ok!(parser::FullExpressionParser, "builtin blabla a b");
        check_ok!(
            parser::FullExpressionParser,
            "{ foo: UInt32 42; bar: Int64 23 }"
        );
        check_ok!(
            parser::FullExpressionParser,
            "match foo with \n| False => True \n| _ => False \nend"
        );
        check_ok!(parser::FullExpressionParser, "Foo");
        check_ok!(parser::FullExpressionParser, "Foo  m  n");
        check_ok!(parser::FullExpressionParser, "tfun 'T => fun (x: 'T) => x");
        check_ok!(parser::FullExpressionParser, "@foo Type");

        check_err!(parser::FullExpressionParser, "let 42 = x in x");
        check_err!(parser::FullExpressionParser, "let x: = 42 in x");
        check_err!(parser::FullExpressionParser, "fun x => x + 1");
        check_err!(parser::FullExpressionParser, "42foo");
        check_err!(parser::FullExpressionParser, "42.foo");
        check_err!(parser::FullExpressionParser, "42.23");
        check_err!(parser::FullExpressionParser, "builtin noop");
        check_err!(parser::FullExpressionParser, "{ foo = 42; bar: 23 }");
        check_err!(parser::FullExpressionParser, "{ foo: 42, bar = 23 }");
        check_err!(parser::FullExpressionParser, "{ foo = 42, }");
        check_err!(
            parser::FullExpressionParser,
            "match foo with | 42 => true end"
        );
        check_err!(parser::FullExpressionParser, "Foo()");
        check_err!(parser::FullExpressionParser, "tfun T => Foo");
        check_err!(parser::FullExpressionParser, "@foo()");
    }

    #[test]
    fn atomic_expression() {
        check_ok!(parser::AtomicExpressionParser, "foo");
        check_ok!(parser::AtomicExpressionParser, "Uint32 42");
        check_ok!(parser::AtomicExpressionParser, "0x123abc");
        check_ok!(parser::AtomicExpressionParser, r#""string""#);

        check_err!(parser::AtomicExpressionParser, "(foo)");
        check_err!(parser::AtomicExpressionParser, "42.0");
        check_err!(parser::AtomicExpressionParser, "True");
    }

    #[test]
    fn value_literal() {
        check_ok!(parser::AtomicExpressionParser, "foo");
        check_ok!(parser::AtomicExpressionParser, "Uint32 42");
        check_ok!(parser::AtomicExpressionParser, "0x123abc");
        check_ok!(parser::AtomicExpressionParser, r#""string""#);

        check_err!(parser::AtomicExpressionParser, "(foo)");
        check_err!(parser::AtomicExpressionParser, "42.0");
        check_err!(parser::AtomicExpressionParser, "True");
    }

    #[test]
    fn map_access() {
        check_ok!(parser::MapAccessParser, "[foo]");
        check_ok!(parser::MapAccessParser, "[bar123]");
        check_ok!(parser::MapAccessParser, "[_result]");

        check_err!(parser::MapAccessParser, "[0x0232]");
        check_err!(parser::MapAccessParser, r#"["xx"]"#);
        check_err!(parser::MapAccessParser, "[Foo]");
        check_err!(parser::MapAccessParser, "[]");
        check_err!(parser::MapAccessParser, "[foo.bar]");
    }

    #[test]
    fn pattern() {
        check_ok!(parser::PatternParser, "_");
        check_ok!(parser::PatternParser, "foo");
        check_ok!(parser::PatternParser, "Bar42");
        check_ok!(parser::PatternParser, "Bar42 _");
        check_ok!(parser::PatternParser, "Bar42 hello");
        check_ok!(parser::PatternParser, "Bar42(_)");
        check_ok!(parser::PatternParser, "Bar42(Foo)");
        check_ok!(parser::PatternParser, "Bar42(Foo Bar)");
        check_ok!(parser::PatternParser, "Bar42(ByStr42 Int32)");
        check_ok!(parser::PatternParser, "Bar42(Foo.Bar Bar.Baz)");

        check_err!(parser::PatternParser, "_ _");
        check_err!(parser::PatternParser, "42Bar");
        check_err!(parser::PatternParser, "foo bar");
        check_err!(parser::PatternParser, "Foo42(, Bar)");
        check_err!(parser::PatternParser, "Foo42(Map, Bar)");
        check_err!(parser::PatternParser, "Bar42(Map ByStr42 Int32)");
    }

    #[test]
    fn argument_pattern() {
        check_ok!(parser::ArgumentPatternParser, "_");
        check_ok!(parser::ArgumentPatternParser, "foo");
        check_ok!(parser::ArgumentPatternParser, "MyType");
        check_ok!(parser::ArgumentPatternParser, "(baz)");
        check_ok!(parser::ArgumentPatternParser, "(Bar42 _)");
        check_ok!(parser::ArgumentPatternParser, "my_type");

        check_err!(parser::ArgumentPatternParser, "2bar");
        check_err!(parser::ArgumentPatternParser, "MyType()");
        check_err!(parser::ArgumentPatternParser, "(2bar)");
    }

    #[test]
    fn pattern_match_expression_clause() {
        check_ok!(
            parser::PatternMatchExpressionClauseParser,
            "| _ => Uint32 42"
        );
        check_ok!(
            parser::PatternMatchExpressionClauseParser,
            "| Foo => Uint32 42"
        );
        check_ok!(
            parser::PatternMatchExpressionClauseParser,
            "| Foo _ Bar => Uint32 42"
        );
        check_ok!(
            parser::PatternMatchExpressionClauseParser,
            "| Bar _ => Uint32 42"
        );
        check_ok!(
            parser::PatternMatchExpressionClauseParser,
            "| Bar _ => Int32 -1"
        );
        check_ok!(
            parser::PatternMatchExpressionClauseParser,
            "| Foo _ => let x = Uint32 1 in x"
        );

        check_err!(parser::PatternMatchExpressionClauseParser, "| Foo =>");
        check_err!(parser::PatternMatchExpressionClauseParser, "| => 42");
        check_err!(parser::PatternMatchExpressionClauseParser, "| _ => ");
        check_err!(parser::PatternMatchExpressionClauseParser, "| () => 42");
        check_err!(
            parser::PatternMatchExpressionClauseParser,
            "| Foo + Bar => 42"
        );
    }

    #[test]
    fn message_entry() {
        check_ok!(parser::MessageEntryParser, "foo: Uint32 42");
        check_ok!(parser::MessageEntryParser, "foo: bar");
        check_ok!(parser::MessageEntryParser, "foo: 0x1337");

        check_err!(parser::MessageEntryParser, "foo");
        check_err!(parser::MessageEntryParser, "foo: bar: baz");
        check_err!(parser::MessageEntryParser, ": 42");
    }

    #[test]
    fn test_type_annotation() {
        check_ok!(parser::TypeAnnotationParser, ": Int");
        check_ok!(parser::TypeAnnotationParser, ": MyCustomType");
        check_ok!(parser::TypeAnnotationParser, ": ByStr32");
        check_ok!(parser::TypeAnnotationParser, ": (Map ByStr32 Uint32)");
        check_ok!(
            parser::TypeAnnotationParser,
            ": (Map ByStr32 (Map ByStr32 Uint32))"
        );
        check_ok!(parser::TypeAnnotationParser, ": (List MyCustomType)");
        check_ok!(
            parser::TypeAnnotationParser,
            ": (Pair MyCustomType1 MyCustomType2)"
        );
        check_ok!(
            parser::TypeAnnotationParser,
            ": (Pair (Pair Int Bool) (List MyCustomType))"
        );

        check_err!(parser::TypeAnnotationParser, ": MyCustomType (");
        check_err!(parser::TypeAnnotationParser, ": (Map MyCustomType)");
        check_err!(
            parser::TypeAnnotationParser,
            ": (Pair MyCustomType1 MyCustomType2"
        );
        check_err!(parser::TypeAnnotationParser, "Int");
        check_err!(parser::TypeAnnotationParser, ": 42");
    }

    #[test]
    fn typed_identifier() {
        check_ok!(parser::TypedIdentifierParser, "foo: Int");
        check_ok!(parser::TypedIdentifierParser, "bar: ByStr20");
        check_ok!(parser::TypedIdentifierParser, "baz: (Int Bool)");

        check_err!(parser::TypedIdentifierParser, "1foo: Int");
        check_err!(parser::TypedIdentifierParser, "foo: int");
        check_err!(parser::TypedIdentifierParser, "foo: (,)");
    }

    #[test]
    fn test_statement() {
        check_ok!(parser::StatementParser, "foo <- bar");
        check_ok!(parser::StatementParser, "remoteFetchStatement");
        check_ok!(parser::StatementParser, "foo := bar");
        check_ok!(parser::StatementParser, "foo = Uint32 42");
        check_ok!(parser::StatementParser, "foo <- &Event");
        check_ok!(parser::StatementParser, "foo <- qux[baz]");
        check_ok!(parser::StatementParser, "foo <- exists baz[bar]");
        check_ok!(parser::StatementParser, "foo[bar] := qux");
        check_ok!(parser::StatementParser, "delete foo[bar]");
        check_ok!(parser::StatementParser, "accept");
        check_ok!(parser::StatementParser, "send foo");
        check_ok!(parser::StatementParser, "event foo");
        check_ok!(parser::StatementParser, "throw");
        check_ok!(
            parser::StatementParser,
            "match foo with | False => True | _ => False end"
        );
        check_ok!(parser::StatementParser, "match foo with | _ => value end");
        check_ok!(parser::StatementParser, "Foo bar baz");
        check_ok!(parser::StatementParser, "forall foo Event");

        check_err!(parser::StatementParser, "foo < bar");
        check_err!(parser::StatementParser, "42 = foo");
        check_err!(parser::StatementParser, "&Event");
        check_err!(parser::StatementParser, "foo[] <- bar");
        check_err!(parser::StatementParser, "foo <- exists");
        check_err!(parser::StatementParser, "foo[] := bar");
        check_err!(parser::StatementParser, "foo := qux[bar][baz]");
        check_err!(parser::StatementParser, "foo.delete[bar]");
        check_err!(parser::StatementParser, "send");
        check_err!(parser::StatementParser, "event");
        check_err!(parser::StatementParser, "match with _ => 42 end");
        check_err!(parser::StatementParser, "forall");
    }

    #[test]
    fn blockchain_fetch_arguments() {
        check_ok!(parser::BlockchainFetchArgumentsParser, "(foo bar)");
        check_ok!(parser::BlockchainFetchArgumentsParser, "(x)");
        check_ok!(parser::BlockchainFetchArgumentsParser, "(y z a)");

        check_err!(parser::BlockchainFetchArgumentsParser, "()");
        check_err!(parser::BlockchainFetchArgumentsParser, "(123)");
        check_err!(parser::BlockchainFetchArgumentsParser, "(foo, 123)");
        check_err!(parser::BlockchainFetchArgumentsParser, "foo, bar");
        check_err!(parser::BlockchainFetchArgumentsParser, "(foo; bar)");
        check_err!(parser::BlockchainFetchArgumentsParser, "foo");
    }

    #[test]
    fn statement_block() {
        check_ok!(parser::StatementBlockParser, "x <- y; z := a");
        check_ok!(parser::StatementBlockParser, "accept");
        check_ok!(parser::StatementBlockParser, "send x");
        check_ok!(parser::StatementBlockParser, "event myEvent");
        check_ok!(
            parser::StatementBlockParser,
            "match x with | _ => accept end"
        );
        check_ok!(
            parser::StatementBlockParser,
            "match x with | _ => y <- z end"
        );
        check_ok!(parser::StatementBlockParser, "MyComponent y");
        check_ok!(
            parser::StatementBlockParser,
            "MyComponent y; forall foo Event ; match x with | _ => y <- z end"
        );

        check_err!(parser::StatementBlockParser, "x < y");
        check_err!(parser::StatementBlockParser, "x <-");
        check_err!(parser::StatementBlockParser, "accept event");
        check_err!(parser::StatementBlockParser, "send");
        check_err!(parser::StatementBlockParser, "match x with _ => accept end");
        check_err!(parser::StatementBlockParser, "MyComponent X");
        check_err!(parser::StatementBlockParser, "MyComponent y forall x");
    }

    #[test]
    fn parameter_pair() {
        check_ok!(parser::ParameterPairParser, "foo: Uint32");
        check_ok!(parser::ParameterPairParser, "bar: Bool");
        check_ok!(parser::ParameterPairParser, "baz: Address");
        check_ok!(parser::ParameterPairParser, "qux: Map Uint32 Bool");

        check_err!(parser::ParameterPairParser, "foo Uint32");
        check_err!(parser::ParameterPairParser, "foo");
        check_err!(parser::ParameterPairParser, "123: Uint32");
        check_err!(parser::ParameterPairParser, "foo: bar: Uint32");
        check_err!(parser::ParameterPairParser, "foo: uint32");
        check_err!(parser::ParameterPairParser, "foo: mapUint32, Bool");
    }

    #[test]
    fn component_definition() {
        check_ok!(
            parser::ComponentDefinitionParser,
            "transition myTransition(param1: Uint32, param2: Uint32) end"
        );
        check_ok!(
            parser::ComponentDefinitionParser,
            "procedure myProcedure(param: Uint32) end"
        );
        check_ok!(
            parser::ComponentDefinitionParser,
            "procedure myProcedure(param: Map ByStr32 ByStr32) param end"
        );
        check_ok!(
          parser::ComponentDefinitionParser,
          "transition myTransition(param: Bool) match param with | False => True | _ => False end end"
      );

        check_err!(parser::ComponentDefinitionParser, "transition myTransition");
        check_err!(
            parser::ComponentDefinitionParser,
            "procedure myProcedure() returns Uint32"
        );
        check_err!(
            parser::ComponentDefinitionParser,
            "procedure myProcedure(param: Uint32) returns Uint32 {"
        );
        check_err!(
            parser::ComponentDefinitionParser,
            "transition myTransition() { state_1 -> state_2 }"
        );
        check_err!(
            parser::ComponentDefinitionParser,
            "transition myTransition(param1: Uint32 param2: Uint32)"
        );
    }

    #[test]
    fn procedure_definition() {
        check_ok!(parser::ProcedureDefinitionParser, "procedure foo() end");
        check_ok!(
            parser::ProcedureDefinitionParser,
            "procedure bar(x: Int32, y: Uint32) baz x y end"
        );

        check_err!(parser::ProcedureDefinitionParser, "procedure 42() { }");
        check_err!(parser::ProcedureDefinitionParser, "procedure foo(x, y) { }");
        check_err!(
            parser::ProcedureDefinitionParser,
            "procedure foo(x: Int32, y: Uint32)"
        );
        check_err!(
            parser::ProcedureDefinitionParser,
            "procedure foo(x: Int32, y: Uint32) foo x y"
        );
        check_err!(parser::ProcedureDefinitionParser, "procedure foo() {}");
    }

    #[test]
    fn transition_definition() {
        check_ok!(
            parser::TransitionDefinitionParser,
            "transition foo()  bar end"
        );
        check_ok!(
            parser::TransitionDefinitionParser,
            "transition bar(x: Int32, y: Uint32) foo end"
        );
        check_ok!(
            parser::TransitionDefinitionParser,
            "transition qux(bar: Bool) bar end"
        );
        check_ok!(parser::TransitionDefinitionParser, "transition empty() end");

        check_err!(
            parser::TransitionDefinitionParser,
            "transition 123() { foo() }"
        );
        check_err!(
            parser::TransitionDefinitionParser,
            "transition foo(bar) { foo() }"
        );
        check_err!(
            parser::TransitionDefinitionParser,
            "transition foo() { foo(); bar() }"
        );
        check_err!(
            parser::TransitionDefinitionParser,
            "transaction foo() { bar() }"
        );
        check_err!(
            parser::TransitionDefinitionParser,
            "transition foo() { bar }"
        );
        check_err!(
            parser::TransitionDefinitionParser,
            "transition foo() { bar( }"
        );
        check_err!(
            parser::TransitionDefinitionParser,
            "transition foo() { bar() };"
        );
        check_err!(parser::TransitionDefinitionParser, "transition foo() { ; }");
    }

    #[test]
    fn component_id() {
        check_ok!(parser::ComponentIdParser, "MyType");
        check_ok!(parser::ComponentIdParser, "Event42");
        check_ok!(parser::ComponentIdParser, "Foo_Bar");
        check_ok!(parser::ComponentIdParser, "ByStr42");
        check_ok!(parser::ComponentIdParser, "regular_id");

        check_err!(parser::ComponentIdParser, "42Event");
        check_err!(parser::ComponentIdParser, "my type");
        check_err!(parser::ComponentIdParser, "event+");
        check_err!(parser::ComponentIdParser, "ByStr");
    }

    #[test]
    fn component_parameters() {
        check_ok!(parser::ComponentParametersParser, "()");
        check_ok!(parser::ComponentParametersParser, "(a: Int32)");
        check_ok!(parser::ComponentParametersParser, "(a: Int32, b: Bool)");
        check_ok!(
            parser::ComponentParametersParser,
            "(a: Int32, b: Bool, c: String)"
        );
        check_ok!(
            parser::ComponentParametersParser,
            "(a: ByStr20, b: Map ByStr20 (Uint256))"
        );

        check_err!(parser::ComponentParametersParser, "a: Int32, b: Bool");
        check_err!(parser::ComponentParametersParser, "(a: Int32");
        check_err!(parser::ComponentParametersParser, "(a: Int32,, b: Bool)");
        check_err!(parser::ComponentParametersParser, "(())");
        check_err!(parser::ComponentParametersParser, "(a: )");
    }

    #[test]
    fn component_body() {
        check_ok!(
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
      "
        );

        check_ok!(
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
      "
        );

        check_ok!(parser::ComponentBodyParser, "
          current_init <-& init.dApp;
          xPointsDApp = \"xpoints\"; get_addr <-& current_init.dns[xPointsDApp]; addr = option_bystr20_value get_addr;
          is_xPoints = builtin eq _sender addr; match is_xPoints with
          | True => | False => e = { _exception : \"donate.tyron-WrongCaller\" }; throw e end;
          get_xPoints <- xpoints[_origin]; x_points = option_uint128_value get_xPoints; IsSufficient x_points amount;
          new_bal = builtin sub x_points amount; xpoints[_origin] := new_bal end
      ");
    }

    #[test]
    fn contract_fields() {
        check_ok!(parser::ContractFieldParser, "field foo: Int32 = Int32 42");
        check_ok!(
            parser::ContractFieldParser,
            "field bar: Map ByStr32 (List Uint32) = Emp ByStr32 (List Uint32)"
        );
        check_ok!(parser::ContractFieldParser, "field baz: Event = Event");

        check_err!(parser::ContractFieldParser, "field: Foo = Bar");
        check_err!(parser::ContractFieldParser, "field bar = 42");
        check_err!(parser::ContractFieldParser, "field baz: Event = 42");
        check_err!(parser::ContractFieldParser, "field qux = 'hello world'");
    }

    #[test]
    fn test_with_constraint() {
        check_ok!(
            parser::WithConstraintParser,
            "with builtin blt end_of_life =>"
        );
        check_ok!(
            parser::WithConstraintParser,
            "with builtin add {UInt32} one  =>"
        );
        check_ok!(parser::WithConstraintParser, "with true =>");
        check_ok!(parser::WithConstraintParser, "with variableIdentifier =>");

        check_err!(parser::WithConstraintParser, "foo");
        check_err!(
            parser::WithConstraintParser,
            "with variableIdentifier => foo"
        );
        check_err!(parser::WithConstraintParser, "with =>");
        check_err!(parser::WithConstraintParser, "with");
    }

    #[test]
    fn contract_definition() {
        check_ok!(parser::ContractDefinitionParser, "contract MyContract()");
        check_ok!(
            parser::ContractDefinitionParser,
            "contract MyContract(address: ByStr20)"
        );
        check_ok!(
            parser::ContractDefinitionParser,
            "contract MyContract(address: ByStr20) with true =>"
        );
        check_ok!(
            parser::ContractDefinitionParser,
            "contract MyContract(address: ByStr20) with true => field field1: Uint32 = Uint32 1"
        );
        check_ok!(parser::ContractDefinitionParser, "contract MyContract(address: ByStr20) with true => field field1: Uint32 = Uint32 1 transition a() end");
        check_ok!(parser::ContractDefinitionParser, "contract MyContract(address: ByStr20) with true => field field1: Uint32 = Uint32 1 procedure a() end");
        check_ok!(
            parser::ContractDefinitionParser,
            r#"contract MyContract(address: ByStr20) with true => field field1: Uint32 = Uint32 1 procedure RequireNotSelf(address: ByStr20)
      is_self = builtin eq address _sender;
      match is_self with
      | False =>
      | True =>
          error = SelfError;
          Throw error
      end
      end"#
        );

        check_err!(parser::ContractDefinitionParser, "contract MyContract");
        check_err!(parser::ContractDefinitionParser, "contract MyContract end");
        check_err!(parser::ContractDefinitionParser, "Contract MyContract");
        check_err!(parser::ContractDefinitionParser, "contract");
        check_err!(
            parser::ContractDefinitionParser,
            "contract MyContract(address: ByStr20,) with (true =>)"
        );
        check_err!(
            parser::ContractDefinitionParser,
            "contract MyContract(address: ByStr20) with true => field field1 = 1"
        );
        check_err!(
            parser::ContractDefinitionParser,
            "contract MyContract(address: ByStr20) with true => field field1: = 1"
        );
        check_err!(
          parser::ContractDefinitionParser,
          "contract MyContract(address: ByStr20) with true => field field1: Uint32 = 1 field2: 10"
      );
        check_err!(parser::ContractDefinitionParser, "contract MyContract(address: ByStr20) with true => field field1: Uint32 = 1 transition a(Void) {}");
        check_err!(parser::ContractDefinitionParser, "contract MyContract(address: ByStr20) with true => field field1: Uint32 = 1 procedure a(Void) {");
    }

    #[test]
    fn type_alternative_clause() {
        check_ok!(parser::TypeAlternativeClauseParser, "| MyType");
        check_ok!(parser::TypeAlternativeClauseParser, "| MyType of Int");
        check_ok!(
            parser::TypeAlternativeClauseParser,
            "| ByStr123 of Map MyType Int"
        );

        check_err!(parser::TypeAlternativeClauseParser, "| MyType of");
        check_err!(parser::TypeAlternativeClauseParser, "| 123MyType");
        check_err!(parser::TypeAlternativeClauseParser, "| ByStr");
    }

    #[test]
    fn library_single_definition() {
        check_ok!(parser::LibrarySingleDefinitionParser, "let foo = Int32 42");
        check_ok!(
            parser::LibrarySingleDefinitionParser,
            "let foo: Int32 = Int32 42"
        );
        check_ok!(parser::LibrarySingleDefinitionParser, "type Foo");
        check_ok!(
            parser::LibrarySingleDefinitionParser,
            "type Foo = | Bar | Baz"
        );

        check_err!(parser::LibrarySingleDefinitionParser, "let = Int32 42");
        check_err!(parser::LibrarySingleDefinitionParser, "let foo: = 42");
        check_err!(parser::LibrarySingleDefinitionParser, "type Int32 42");
        check_err!(
            parser::LibrarySingleDefinitionParser,
            "type Foo = | Bar Baz"
        );
    }

    #[test]
    fn library_definition() {
        check_ok!(parser::LibraryDefinitionParser, "library Foo");
        check_ok!(
            parser::LibraryDefinitionParser,
            "library Bar let x = Int32 10"
        );
        check_ok!(parser::LibraryDefinitionParser, "library Baz type Quux");
        check_ok!(
            parser::LibraryDefinitionParser,
            "library Qux type Quux = | Event"
        );
        check_ok!(
            parser::LibraryDefinitionParser,
            "library Quux type Quux = | Event of Uint256"
        );
        check_ok!(
            parser::LibraryDefinitionParser,
            "library Quuz type Quux = | Event of Uint256 | AnotherEvent of ByStr20"
        );
        check_ok!(
            parser::LibraryDefinitionParser,
            "library Qoorx let x: Int32 = Int32 42"
        );

        check_err!(parser::LibraryDefinitionParser, "library Foo Bar");
        check_err!(parser::LibraryDefinitionParser, "library");
        check_err!(parser::LibraryDefinitionParser, "library Foo bar");
        check_err!(parser::LibraryDefinitionParser, "library Foo type");
        check_err!(
            parser::LibraryDefinitionParser,
            "library Foo type = | Event"
        );
        check_err!(parser::LibraryDefinitionParser, "library Foo type Quux =");
        check_err!(parser::LibraryDefinitionParser, "library Foo type Quux = |");
        check_err!(
            parser::LibraryDefinitionParser,
            "library Foo type Quux = | Event of"
        );
        check_err!(parser::LibraryDefinitionParser, "library Foo let");
        check_err!(parser::LibraryDefinitionParser, "library Foo let x");
        check_err!(parser::LibraryDefinitionParser, "library Foo let = 42");
    }
}
