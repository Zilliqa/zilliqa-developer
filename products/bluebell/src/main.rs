#[macro_use]
extern crate lalrpop_util;
pub mod ast;
pub mod lexer;
use crate::lexer::Lexer;

use std::env;
use std::fs::File;
use std::io::Read;
use std::process;
lalrpop_mod!(pub bluebell);

macro_rules! check_ok {
    ($parser:ty, $result:expr) => {
        let mut errors = vec![];
        assert!(<$parser>::new()
            .parse(&mut errors, lexer::Lexer::new($result))
            .is_ok());
    };
}

macro_rules! check_err {
    ($parser:ty, $result:expr) => {
        let mut errors = vec![];
        assert!(<$parser>::new()
            .parse(&mut errors, lexer::Lexer::new($result))
            .is_err());
    };
}

#[test]
fn bytestring_parser() {
    check_ok!(bluebell::ByteStringParser, "ByStr1234");
    check_ok!(bluebell::ByteStringParser, "ByStr11");
    check_ok!(bluebell::ByteStringParser, "ByStr0");

    check_err!(bluebell::ByteStringParser, "ByStr 3");
    check_err!(bluebell::ByteStringParser, "ByStr 11");
    check_err!(bluebell::ByteStringParser, "ByStr 0");
    check_err!(bluebell::ByteStringParser, "ByStr");
}

#[test]
fn type_name_identifier_parser() {
    check_ok!(bluebell::TypeNameIdentifierParser, "Event");
    check_ok!(bluebell::TypeNameIdentifierParser, "Foo");
    check_ok!(bluebell::TypeNameIdentifierParser, "ByStr42");

    check_err!(bluebell::TypeNameIdentifierParser, "bystr");
    check_err!(bluebell::TypeNameIdentifierParser, "_foo");
    check_err!(bluebell::TypeNameIdentifierParser, "ByStr 0");
    check_err!(bluebell::TypeNameIdentifierParser, "Type With Spaces");
}

#[test]
fn imported_name_parser() {
    check_ok!(bluebell::ImportedNameParser, "Event");
    check_ok!(bluebell::ImportedNameParser, "Foo");
    check_ok!(bluebell::ImportedNameParser, "ByStr42");
    check_ok!(bluebell::ImportedNameParser, "Event as Alias");
    check_ok!(bluebell::ImportedNameParser, "Foo as Alias");
    check_ok!(bluebell::ImportedNameParser, "ByStr42 as Alias");

    check_err!(bluebell::ImportedNameParser, "foo");
    check_err!(bluebell::ImportedNameParser, "Type With Spaces");
    check_err!(bluebell::ImportedNameParser, "Foo As Alias");
    check_err!(bluebell::ImportedNameParser, "Event as");
}

#[test]
fn import_declarations() {
    check_ok!(bluebell::ImportDeclarationsParser, "import Foo");
    check_ok!(
        bluebell::ImportDeclarationsParser,
        "import Event import ByStr42"
    );
    check_ok!(bluebell::ImportDeclarationsParser, "import Foo as Bar");

    check_err!(bluebell::ImportDeclarationsParser, "foo");
    check_err!(bluebell::ImportDeclarationsParser, "import");
    check_err!(bluebell::ImportDeclarationsParser, "import Foo import");
}

#[test]
fn meta_identifiers() {
    check_ok!(bluebell::MetaIdentifierParser, "Event");
    check_ok!(bluebell::MetaIdentifierParser, "Foo.Bar");
    check_ok!(bluebell::MetaIdentifierParser, "ABCD.Event");
    check_ok!(bluebell::MetaIdentifierParser, "0x1234.Event");
    check_ok!(bluebell::MetaIdentifierParser, "ByStr");

    check_err!(bluebell::MetaIdentifierParser, "_foo");
    check_err!(bluebell::MetaIdentifierParser, "Type With Spaces");
    check_err!(bluebell::MetaIdentifierParser, "Foo .. Bar");
    check_err!(bluebell::MetaIdentifierParser, "0x1234 - Bar");
    check_err!(bluebell::MetaIdentifierParser, "Foo.");
    check_err!(bluebell::MetaIdentifierParser, "0x1234.42");
}

#[test]
fn variable_identifier() {
    check_ok!(bluebell::VariableIdentifierParser, "foo");
    check_ok!(bluebell::VariableIdentifierParser, "_bar");
    check_ok!(bluebell::VariableIdentifierParser, "Foo.bar");
    check_ok!(bluebell::VariableIdentifierParser, "ByStr42.baz");
    check_ok!(bluebell::VariableIdentifierParser, "Event.qux");

    check_err!(bluebell::VariableIdentifierParser, "42");
    check_err!(bluebell::VariableIdentifierParser, "_");
    check_err!(bluebell::VariableIdentifierParser, "Type With Spaces.baz");
    // check_err!(bluebell::VariableIdentifierParser, "Bystr.qux");
    check_err!(bluebell::VariableIdentifierParser, "Event42.Bar");
    check_err!(bluebell::VariableIdentifierParser, "Foo.");
}

#[test]
fn builtin_arguments() {
    check_ok!(bluebell::BuiltinArgumentsParser, "( )");
    check_ok!(bluebell::BuiltinArgumentsParser, "foo");
    check_ok!(bluebell::BuiltinArgumentsParser, "foo bar baz");
    check_ok!(bluebell::BuiltinArgumentsParser, "Event.qux");

    check_err!(bluebell::BuiltinArgumentsParser, "42");
    check_err!(bluebell::BuiltinArgumentsParser, "_");
    check_err!(bluebell::BuiltinArgumentsParser, "Type With Spaces.baz");
    check_err!(bluebell::BuiltinArgumentsParser, "Event42.Bar");
    check_err!(bluebell::BuiltinArgumentsParser, "Foo.");
}

#[test]
fn scilla_types() {
    check_ok!(bluebell::ScillaTypeParser, "Uint32");
    check_ok!(bluebell::ScillaTypeParser, "Foo(Bar)");
    check_ok!(bluebell::ScillaTypeParser, "Map Uint32 String");
    check_ok!(bluebell::ScillaTypeParser, "Uint32 -> Bool");
    check_ok!(bluebell::ScillaTypeParser, "(Uint32)");
    check_ok!(bluebell::ScillaTypeParser, "Address with end");
    check_ok!(
        bluebell::ScillaTypeParser,
        "forall 'A. forall 'B. ( 'B -> 'A -> 'B) -> 'B -> List 'A -> 'B"
    );
    check_ok!(bluebell::ScillaTypeParser, "T");

    check_err!(bluebell::ScillaTypeParser, "Map");
    check_err!(bluebell::ScillaTypeParser, "Uint32 ->");
    check_err!(bluebell::ScillaTypeParser, "-> Bool");
    check_err!(bluebell::ScillaTypeParser, "address with");
    check_err!(bluebell::ScillaTypeParser, "address with Foo end");
    check_err!(bluebell::ScillaTypeParser, "forall T. Map(T, Uint32)");
    check_err!(bluebell::ScillaTypeParser, "Foo(Bar");
}

#[test]
fn test_address_type() {
    check_ok!(bluebell::AddressTypeParser, "Foo with end");
    check_ok!(bluebell::AddressTypeParser, "ByStr42 with end");
    check_ok!(bluebell::AddressTypeParser, "Event with end");
    check_ok!(
        bluebell::AddressTypeParser,
        "Foo with contract field field1: Uint32, field field2: Uint32 end"
    );
    check_ok!(
        bluebell::AddressTypeParser,
        "ByStr42 with contract field field1: Uint32 end"
    );
    check_ok!(bluebell::AddressTypeParser, "Event with contract end");
    check_ok!(bluebell::AddressTypeParser, "Foo with library end");
    check_ok!(bluebell::AddressTypeParser, "ByStr42 with library end");
    check_ok!(bluebell::AddressTypeParser, "Event with library end");
    check_ok!(bluebell::AddressTypeParser, "Foo with _foo end");
    check_ok!(bluebell::AddressTypeParser, "ByStr42 with _foo end");
    check_ok!(bluebell::AddressTypeParser, "Event with _foo end");
    check_ok!(
        bluebell::AddressTypeParser,
        "Foo with contract field field1: Uint32 end"
    );
    check_ok!(bluebell::AddressTypeParser, "ByStr42 with contract field field1: Uint32, field field2: Uint32, field field3: Uint32 end");
    check_ok!(
        bluebell::AddressTypeParser,
        "Event with contract field field1: Uint32 end"
    );

    check_err!(bluebell::AddressTypeParser, "foo with end");
    check_err!(bluebell::AddressTypeParser, "Foo with ");
    check_err!(bluebell::AddressTypeParser, "Foo with foo bar: Uint32 end");
    check_err!(
        bluebell::AddressTypeParser,
        "Foo with contract field1: Uint32, field2: Uint32"
    );
    check_err!(bluebell::AddressTypeParser, "Foo with contract, end");
}

#[test]
fn test_type_map_key() {
    check_ok!(bluebell::TypeMapKeyParser, "Foo");
    check_ok!(bluebell::TypeMapKeyParser, "(Foo)");
    check_ok!(bluebell::TypeMapKeyParser, "Foo with end");
    check_ok!(bluebell::TypeMapKeyParser, "(Foo with end)");
    check_ok!(bluebell::TypeMapKeyParser, "(ByStr42 with contract end)");
    check_ok!(bluebell::TypeMapKeyParser, "Foo with library end");

    check_err!(bluebell::TypeMapKeyParser, "foo");
    check_err!(bluebell::TypeMapKeyParser, "Foo()");
    check_err!(bluebell::TypeMapKeyParser, "(Foo with bar end)");
    check_err!(bluebell::TypeMapKeyParser, "(42)");
}

#[test]
fn type_map_value() {
    check_ok!(bluebell::TypeMapValueParser, "Uint32");
    check_ok!(bluebell::TypeMapValueParser, "Map Foo Bar");
    check_ok!(bluebell::TypeMapValueParser, "(Uint32)");
    check_ok!(bluebell::TypeMapValueParser, "Address with end");
    check_ok!(
        bluebell::TypeMapValueParser,
        "Foo with contract field field1: Uint32 end"
    );

    check_err!(bluebell::TypeMapValueParser, "foo");
    check_err!(bluebell::TypeMapValueParser, "bystr1");
    check_err!(bluebell::TypeMapValueParser, "event");
    check_err!(bluebell::TypeMapValueParser, "map(foo, bar)");
    check_err!(bluebell::TypeMapValueParser, "(42)");
    check_err!(bluebell::TypeMapValueParser, "address with");
    check_err!(bluebell::TypeMapValueParser, "foo with foo bar");
}

#[test]
fn type_map_value_arguments() {
    check_ok!(bluebell::TypeMapValueArgumentsParser, "(Uint32)");
    check_ok!(bluebell::TypeMapValueArgumentsParser, "Foo");
    check_ok!(bluebell::TypeMapValueArgumentsParser, "Map Foo Bar");

    check_err!(bluebell::TypeMapValueArgumentsParser, "Foo Bar");
    check_err!(bluebell::TypeMapValueArgumentsParser, "map(foo, bar)");
    check_err!(bluebell::TypeMapValueArgumentsParser, "(42)()");
    check_err!(bluebell::TypeMapValueArgumentsParser, "(Uint32");
    check_err!(bluebell::TypeMapValueArgumentsParser, "Map(Foo)");
    check_err!(bluebell::TypeMapValueArgumentsParser, "Map(Foo, Bar)");
}

#[test]
fn type_argument() {
    check_ok!(bluebell::TypeArgumentParser, "Foo");
    check_ok!(bluebell::TypeArgumentParser, "(Bar)");
    check_ok!(bluebell::TypeArgumentParser, "Uint32");
    check_ok!(bluebell::TypeArgumentParser, "'A");
    check_ok!(bluebell::TypeArgumentParser, "(Uint32)");
    check_ok!(
        bluebell::TypeArgumentParser,
        "Address with contract field field1: Uint32, field field2: Uint32 end"
    );
    check_ok!(bluebell::TypeArgumentParser, "Map Uint32 Bool");

    check_err!(bluebell::TypeArgumentParser, "foo bar");
    check_err!(bluebell::TypeArgumentParser, "123");
    check_err!(bluebell::TypeArgumentParser, "foo.bar");
    check_err!(bluebell::TypeArgumentParser, "mapUint32Uint32");
    check_err!(bluebell::TypeArgumentParser, "'_A");
    check_err!(bluebell::TypeArgumentParser, "(map(Int32, String))");
    check_err!(bluebell::TypeArgumentParser, "Foo.bar");
    check_err!(bluebell::TypeArgumentParser, "Map(Int32, String, Bool)");
}

#[test]
fn full_expressions() {
    check_ok!(bluebell::FullExpressionParser, "let x = Int32 42 in X");
    check_ok!(
        bluebell::FullExpressionParser,
        "let x: Int32 = Int32 42 in X"
    );
    check_ok!(
        bluebell::FullExpressionParser,
        "let x = Uint128 42 in builtin lt x x"
    );
    check_ok!(
        bluebell::FullExpressionParser,
        "fun (x: Int32) => builtin lt x x"
    );
    check_ok!(bluebell::FullExpressionParser, "fun (x: Int32) => foo");
    check_ok!(bluebell::FullExpressionParser, "foo");
    check_ok!(bluebell::FullExpressionParser, "foo bar baz");
    check_ok!(bluebell::FullExpressionParser, "UInt32 42");
    check_ok!(bluebell::FullExpressionParser, "true");
    check_ok!(bluebell::FullExpressionParser, "builtin blabla a b");
    check_ok!(
        bluebell::FullExpressionParser,
        "{ foo: UInt32 42; bar: Int64 23 }"
    );
    check_ok!(
        bluebell::FullExpressionParser,
        "match foo with \n| False => True \n| _ => False \nend"
    );
    check_ok!(bluebell::FullExpressionParser, "Foo");
    check_ok!(bluebell::FullExpressionParser, "Foo  m  n");
    check_ok!(
        bluebell::FullExpressionParser,
        "tfun 'T => fun (x: 'T) => x"
    );
    check_ok!(bluebell::FullExpressionParser, "@foo Type");

    check_err!(bluebell::FullExpressionParser, "let 42 = x in x");
    check_err!(bluebell::FullExpressionParser, "let x: = 42 in x");
    check_err!(bluebell::FullExpressionParser, "fun x => x + 1");
    check_err!(bluebell::FullExpressionParser, "42foo");
    check_err!(bluebell::FullExpressionParser, "42.foo");
    check_err!(bluebell::FullExpressionParser, "42.23");
    check_err!(bluebell::FullExpressionParser, "builtin noop");
    check_err!(bluebell::FullExpressionParser, "{ foo = 42; bar: 23 }");
    check_err!(bluebell::FullExpressionParser, "{ foo: 42, bar = 23 }");
    check_err!(bluebell::FullExpressionParser, "{ foo = 42, }");
    check_err!(
        bluebell::FullExpressionParser,
        "match foo with | 42 => true end"
    );
    check_err!(bluebell::FullExpressionParser, "Foo()");
    check_err!(bluebell::FullExpressionParser, "tfun T => Foo");
    check_err!(bluebell::FullExpressionParser, "@foo()");
}

#[test]
fn atomic_expression() {
    check_ok!(bluebell::AtomicExpressionParser, "foo");
    check_ok!(bluebell::AtomicExpressionParser, "Uint32 42");
    check_ok!(bluebell::AtomicExpressionParser, "0x123abc");
    check_ok!(bluebell::AtomicExpressionParser, r#""string""#);

    check_err!(bluebell::AtomicExpressionParser, "(foo)");
    check_err!(bluebell::AtomicExpressionParser, "42.0");
    check_err!(bluebell::AtomicExpressionParser, "True");
}

#[test]
fn value_literal() {
    check_ok!(bluebell::AtomicExpressionParser, "foo");
    check_ok!(bluebell::AtomicExpressionParser, "Uint32 42");
    check_ok!(bluebell::AtomicExpressionParser, "0x123abc");
    check_ok!(bluebell::AtomicExpressionParser, r#""string""#);

    check_err!(bluebell::AtomicExpressionParser, "(foo)");
    check_err!(bluebell::AtomicExpressionParser, "42.0");
    check_err!(bluebell::AtomicExpressionParser, "True");
}

#[test]
fn map_access() {
    check_ok!(bluebell::MapAccessParser, "[foo]");
    check_ok!(bluebell::MapAccessParser, "[bar123]");
    check_ok!(bluebell::MapAccessParser, "[_result]");

    check_err!(bluebell::MapAccessParser, "[0x0232]");
    check_err!(bluebell::MapAccessParser, r#"["xx"]"#);
    check_err!(bluebell::MapAccessParser, "[Foo]");
    check_err!(bluebell::MapAccessParser, "[]");
    check_err!(bluebell::MapAccessParser, "[foo.bar]");
}

#[test]
fn pattern() {
    check_ok!(bluebell::PatternParser, "_");
    check_ok!(bluebell::PatternParser, "foo");
    check_ok!(bluebell::PatternParser, "Bar42");
    check_ok!(bluebell::PatternParser, "Bar42 _");
    check_ok!(bluebell::PatternParser, "Bar42 hello");
    check_ok!(bluebell::PatternParser, "Bar42(_)");
    check_ok!(bluebell::PatternParser, "Bar42(Foo)");
    check_ok!(bluebell::PatternParser, "Bar42(Foo Bar)");
    check_ok!(bluebell::PatternParser, "Bar42(ByStr42 Int32)");
    check_ok!(bluebell::PatternParser, "Bar42(Foo.Bar Bar.Baz)");

    check_err!(bluebell::PatternParser, "_ _");
    check_err!(bluebell::PatternParser, "42Bar");
    check_err!(bluebell::PatternParser, "foo bar");
    check_err!(bluebell::PatternParser, "Foo42(, Bar)");
    check_err!(bluebell::PatternParser, "Foo42(Map, Bar)");
    check_err!(bluebell::PatternParser, "Bar42(Map ByStr42 Int32)");
}

#[test]
fn argument_pattern() {
    check_ok!(bluebell::ArgumentPatternParser, "_");
    check_ok!(bluebell::ArgumentPatternParser, "foo");
    check_ok!(bluebell::ArgumentPatternParser, "MyType");
    check_ok!(bluebell::ArgumentPatternParser, "(baz)");
    check_ok!(bluebell::ArgumentPatternParser, "(Bar42 _)");
    check_ok!(bluebell::ArgumentPatternParser, "my_type");

    check_err!(bluebell::ArgumentPatternParser, "2bar");
    check_err!(bluebell::ArgumentPatternParser, "MyType()");
    check_err!(bluebell::ArgumentPatternParser, "(2bar)");
}

#[test]
fn pattern_match_expression_clause() {
    check_ok!(
        bluebell::PatternMatchExpressionClauseParser,
        "| _ => Uint32 42"
    );
    check_ok!(
        bluebell::PatternMatchExpressionClauseParser,
        "| Foo => Uint32 42"
    );
    check_ok!(
        bluebell::PatternMatchExpressionClauseParser,
        "| Foo _ Bar => Uint32 42"
    );
    check_ok!(
        bluebell::PatternMatchExpressionClauseParser,
        "| Bar _ => Uint32 42"
    );
    check_ok!(
        bluebell::PatternMatchExpressionClauseParser,
        "| Bar _ => Int32 -1"
    );
    check_ok!(
        bluebell::PatternMatchExpressionClauseParser,
        "| Foo _ => let x = Uint32 1 in x"
    );

    check_err!(bluebell::PatternMatchExpressionClauseParser, "| Foo =>");
    check_err!(bluebell::PatternMatchExpressionClauseParser, "| => 42");
    check_err!(bluebell::PatternMatchExpressionClauseParser, "| _ => ");
    check_err!(bluebell::PatternMatchExpressionClauseParser, "| () => 42");
    check_err!(
        bluebell::PatternMatchExpressionClauseParser,
        "| Foo + Bar => 42"
    );
}

#[test]
fn message_entry() {
    check_ok!(bluebell::MessageEntryParser, "foo: Uint32 42");
    check_ok!(bluebell::MessageEntryParser, "foo: bar");
    check_ok!(bluebell::MessageEntryParser, "foo: 0x1337");

    check_err!(bluebell::MessageEntryParser, "foo");
    check_err!(bluebell::MessageEntryParser, "foo: bar: baz");
    check_err!(bluebell::MessageEntryParser, ": 42");
}

#[test]
fn test_type_annotation() {
    check_ok!(bluebell::TypeAnnotationParser, ": Int");
    check_ok!(bluebell::TypeAnnotationParser, ": MyCustomType");
    check_ok!(bluebell::TypeAnnotationParser, ": ByStr32");
    check_ok!(bluebell::TypeAnnotationParser, ": (Map ByStr32 Uint32)");
    check_ok!(
        bluebell::TypeAnnotationParser,
        ": (Map ByStr32 (Map ByStr32 Uint32))"
    );
    check_ok!(bluebell::TypeAnnotationParser, ": (List MyCustomType)");
    check_ok!(
        bluebell::TypeAnnotationParser,
        ": (Pair MyCustomType1 MyCustomType2)"
    );
    check_ok!(
        bluebell::TypeAnnotationParser,
        ": (Pair (Pair Int Bool) (List MyCustomType))"
    );

    check_err!(bluebell::TypeAnnotationParser, ": MyCustomType (");
    check_err!(bluebell::TypeAnnotationParser, ": (Map MyCustomType)");
    check_err!(
        bluebell::TypeAnnotationParser,
        ": (Pair MyCustomType1 MyCustomType2"
    );
    check_err!(bluebell::TypeAnnotationParser, "Int");
    check_err!(bluebell::TypeAnnotationParser, ": 42");
}

#[test]
fn typed_identifier() {
    check_ok!(bluebell::TypedIdentifierParser, "foo: Int");
    check_ok!(bluebell::TypedIdentifierParser, "bar: ByStr20");
    check_ok!(bluebell::TypedIdentifierParser, "baz: (Int Bool)");

    check_err!(bluebell::TypedIdentifierParser, "1foo: Int");
    check_err!(bluebell::TypedIdentifierParser, "foo: int");
    check_err!(bluebell::TypedIdentifierParser, "foo: (,)");
}

#[test]
fn test_statement() {
    check_ok!(bluebell::StatementParser, "foo <- bar");
    check_ok!(bluebell::StatementParser, "remoteFetchStatement");
    check_ok!(bluebell::StatementParser, "foo := bar");
    check_ok!(bluebell::StatementParser, "foo = Uint32 42");
    check_ok!(bluebell::StatementParser, "foo <- &Event");
    check_ok!(bluebell::StatementParser, "foo <- qux[baz]");
    check_ok!(bluebell::StatementParser, "foo <- exists baz[bar]");
    check_ok!(bluebell::StatementParser, "foo[bar] := qux");
    check_ok!(bluebell::StatementParser, "delete foo[bar]");
    check_ok!(bluebell::StatementParser, "accept");
    check_ok!(bluebell::StatementParser, "send foo");
    check_ok!(bluebell::StatementParser, "event foo");
    check_ok!(bluebell::StatementParser, "throw");
    check_ok!(
        bluebell::StatementParser,
        "match foo with | False => True | _ => False end"
    );
    check_ok!(bluebell::StatementParser, "match foo with | _ => value end");
    check_ok!(bluebell::StatementParser, "Foo bar baz");
    check_ok!(bluebell::StatementParser, "forall foo Event");

    check_err!(bluebell::StatementParser, "foo < bar");
    check_err!(bluebell::StatementParser, "42 = foo");
    check_err!(bluebell::StatementParser, "&Event");
    check_err!(bluebell::StatementParser, "foo[] <- bar");
    check_err!(bluebell::StatementParser, "foo <- exists");
    check_err!(bluebell::StatementParser, "foo[] := bar");
    check_err!(bluebell::StatementParser, "foo := qux[bar][baz]");
    check_err!(bluebell::StatementParser, "foo.delete[bar]");
    check_err!(bluebell::StatementParser, "send");
    check_err!(bluebell::StatementParser, "event");
    check_err!(bluebell::StatementParser, "match with _ => 42 end");
    check_err!(bluebell::StatementParser, "forall");
}

#[test]
fn blockchain_fetch_arguments() {
    check_ok!(bluebell::BlockchainFetchArgumentsParser, "(foo bar)");
    check_ok!(bluebell::BlockchainFetchArgumentsParser, "(x)");
    check_ok!(bluebell::BlockchainFetchArgumentsParser, "(y z a)");

    check_err!(bluebell::BlockchainFetchArgumentsParser, "()");
    check_err!(bluebell::BlockchainFetchArgumentsParser, "(123)");
    check_err!(bluebell::BlockchainFetchArgumentsParser, "(foo, 123)");
    check_err!(bluebell::BlockchainFetchArgumentsParser, "foo, bar");
    check_err!(bluebell::BlockchainFetchArgumentsParser, "(foo; bar)");
    check_err!(bluebell::BlockchainFetchArgumentsParser, "foo");
}

#[test]
fn statement_block() {
    check_ok!(bluebell::StatementBlockParser, "x <- y; z := a");
    check_ok!(bluebell::StatementBlockParser, "accept");
    check_ok!(bluebell::StatementBlockParser, "send x");
    check_ok!(bluebell::StatementBlockParser, "event myEvent");
    check_ok!(
        bluebell::StatementBlockParser,
        "match x with | _ => accept end"
    );
    check_ok!(
        bluebell::StatementBlockParser,
        "match x with | _ => y <- z end"
    );
    check_ok!(bluebell::StatementBlockParser, "MyComponent y");
    check_ok!(
        bluebell::StatementBlockParser,
        "MyComponent y; forall foo Event ; match x with | _ => y <- z end"
    );

    check_err!(bluebell::StatementBlockParser, "x < y");
    check_err!(bluebell::StatementBlockParser, "x <-");
    check_err!(bluebell::StatementBlockParser, "accept event");
    check_err!(bluebell::StatementBlockParser, "send");
    check_err!(
        bluebell::StatementBlockParser,
        "match x with _ => accept end"
    );
    check_err!(bluebell::StatementBlockParser, "MyComponent X");
    check_err!(bluebell::StatementBlockParser, "MyComponent y forall x");
}

#[test]
fn parameter_pair() {
    check_ok!(bluebell::ParameterPairParser, "foo: Uint32");
    check_ok!(bluebell::ParameterPairParser, "bar: Bool");
    check_ok!(bluebell::ParameterPairParser, "baz: Address");
    check_ok!(bluebell::ParameterPairParser, "qux: Map Uint32 Bool");

    check_err!(bluebell::ParameterPairParser, "foo Uint32");
    check_err!(bluebell::ParameterPairParser, "foo");
    check_err!(bluebell::ParameterPairParser, "123: Uint32");
    check_err!(bluebell::ParameterPairParser, "foo: bar: Uint32");
    check_err!(bluebell::ParameterPairParser, "foo: uint32");
    check_err!(bluebell::ParameterPairParser, "foo: mapUint32, Bool");
}

#[test]
fn component_definition() {
    check_ok!(
        bluebell::ComponentDefinitionParser,
        "transition myTransition(param1: Uint32, param2: Uint32) end"
    );
    check_ok!(
        bluebell::ComponentDefinitionParser,
        "procedure myProcedure(param: Uint32) end"
    );
    check_ok!(
        bluebell::ComponentDefinitionParser,
        "procedure myProcedure(param: Map ByStr32 ByStr32) param end"
    );
    check_ok!(
        bluebell::ComponentDefinitionParser,
        "transition myTransition(param: Bool) match param with | False => True | _ => False end end"
    );

    check_err!(
        bluebell::ComponentDefinitionParser,
        "transition myTransition"
    );
    check_err!(
        bluebell::ComponentDefinitionParser,
        "procedure myProcedure() returns Uint32"
    );
    check_err!(
        bluebell::ComponentDefinitionParser,
        "procedure myProcedure(param: Uint32) returns Uint32 {"
    );
    check_err!(
        bluebell::ComponentDefinitionParser,
        "transition myTransition() { state_1 -> state_2 }"
    );
    check_err!(
        bluebell::ComponentDefinitionParser,
        "transition myTransition(param1: Uint32 param2: Uint32)"
    );
}

#[test]
fn procedure_definition() {
    check_ok!(bluebell::ProcedureDefinitionParser, "procedure foo() end");
    check_ok!(
        bluebell::ProcedureDefinitionParser,
        "procedure bar(x: Int32, y: Uint32) baz x y end"
    );

    check_err!(bluebell::ProcedureDefinitionParser, "procedure 42() { }");
    check_err!(
        bluebell::ProcedureDefinitionParser,
        "procedure foo(x, y) { }"
    );
    check_err!(
        bluebell::ProcedureDefinitionParser,
        "procedure foo(x: Int32, y: Uint32)"
    );
    check_err!(
        bluebell::ProcedureDefinitionParser,
        "procedure foo(x: Int32, y: Uint32) foo x y"
    );
    check_err!(bluebell::ProcedureDefinitionParser, "procedure foo() {}");
}

#[test]
fn transition_definition() {
    check_ok!(
        bluebell::TransitionDefinitionParser,
        "transition foo()  bar end"
    );
    check_ok!(
        bluebell::TransitionDefinitionParser,
        "transition bar(x: Int32, y: Uint32) foo end"
    );
    check_ok!(
        bluebell::TransitionDefinitionParser,
        "transition qux(bar: Bool) bar end"
    );
    check_ok!(
        bluebell::TransitionDefinitionParser,
        "transition empty() end"
    );

    check_err!(
        bluebell::TransitionDefinitionParser,
        "transition 123() { foo() }"
    );
    check_err!(
        bluebell::TransitionDefinitionParser,
        "transition foo(bar) { foo() }"
    );
    check_err!(
        bluebell::TransitionDefinitionParser,
        "transition foo() { foo(); bar() }"
    );
    check_err!(
        bluebell::TransitionDefinitionParser,
        "transaction foo() { bar() }"
    );
    check_err!(
        bluebell::TransitionDefinitionParser,
        "transition foo() { bar }"
    );
    check_err!(
        bluebell::TransitionDefinitionParser,
        "transition foo() { bar( }"
    );
    check_err!(
        bluebell::TransitionDefinitionParser,
        "transition foo() { bar() };"
    );
    check_err!(
        bluebell::TransitionDefinitionParser,
        "transition foo() { ; }"
    );
}

#[test]
fn component_id() {
    check_ok!(bluebell::ComponentIdParser, "MyType");
    check_ok!(bluebell::ComponentIdParser, "Event42");
    check_ok!(bluebell::ComponentIdParser, "Foo_Bar");
    check_ok!(bluebell::ComponentIdParser, "ByStr42");
    check_ok!(bluebell::ComponentIdParser, "regular_id");

    check_err!(bluebell::ComponentIdParser, "42Event");
    check_err!(bluebell::ComponentIdParser, "my type");
    check_err!(bluebell::ComponentIdParser, "event+");
    check_err!(bluebell::ComponentIdParser, "ByStr");
}

#[test]
fn component_parameters() {
    check_ok!(bluebell::ComponentParametersParser, "()");
    check_ok!(bluebell::ComponentParametersParser, "(a: Int32)");
    check_ok!(bluebell::ComponentParametersParser, "(a: Int32, b: Bool)");
    check_ok!(
        bluebell::ComponentParametersParser,
        "(a: Int32, b: Bool, c: String)"
    );
    check_ok!(
        bluebell::ComponentParametersParser,
        "(a: ByStr20, b: Map ByStr20 (Uint256))"
    );

    check_err!(bluebell::ComponentParametersParser, "a: Int32, b: Bool");
    check_err!(bluebell::ComponentParametersParser, "(a: Int32");
    check_err!(bluebell::ComponentParametersParser, "(a: Int32,, b: Bool)");
    check_err!(bluebell::ComponentParametersParser, "(())");
    check_err!(bluebell::ComponentParametersParser, "(a: )");
}

#[test]
fn component_body() {
    check_ok!(
        bluebell::ComponentBodyParser,
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
        bluebell::ComponentBodyParser,
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

    check_ok!(bluebell::ComponentBodyParser, "
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
    check_ok!(bluebell::ContractFieldParser, "field foo: Int32 = Int32 42");
    check_ok!(
        bluebell::ContractFieldParser,
        "field bar: Map ByStr32 (List Uint32) = Emp ByStr32 (List Uint32)"
    );
    check_ok!(bluebell::ContractFieldParser, "field baz: Event = Event");

    check_err!(bluebell::ContractFieldParser, "field: Foo = Bar");
    check_err!(bluebell::ContractFieldParser, "field bar = 42");
    check_err!(bluebell::ContractFieldParser, "field baz: Event = 42");
    check_err!(bluebell::ContractFieldParser, "field qux = 'hello world'");
}

#[test]
fn test_with_constraint() {
    check_ok!(
        bluebell::WithConstraintParser,
        "with builtin blt end_of_life =>"
    );
    check_ok!(
        bluebell::WithConstraintParser,
        "with builtin add {UInt32} one  =>"
    );
    check_ok!(bluebell::WithConstraintParser, "with true =>");
    check_ok!(bluebell::WithConstraintParser, "with variableIdentifier =>");

    check_err!(bluebell::WithConstraintParser, "foo");
    check_err!(
        bluebell::WithConstraintParser,
        "with variableIdentifier => foo"
    );
    check_err!(bluebell::WithConstraintParser, "with =>");
    check_err!(bluebell::WithConstraintParser, "with");
}

#[test]
fn contract_definition() {
    check_ok!(bluebell::ContractDefinitionParser, "contract MyContract()");
    check_ok!(
        bluebell::ContractDefinitionParser,
        "contract MyContract(address: ByStr20)"
    );
    check_ok!(
        bluebell::ContractDefinitionParser,
        "contract MyContract(address: ByStr20) with true =>"
    );
    check_ok!(
        bluebell::ContractDefinitionParser,
        "contract MyContract(address: ByStr20) with true => field field1: Uint32 = Uint32 1"
    );
    check_ok!(bluebell::ContractDefinitionParser, "contract MyContract(address: ByStr20) with true => field field1: Uint32 = Uint32 1 transition a() end");
    check_ok!(bluebell::ContractDefinitionParser, "contract MyContract(address: ByStr20) with true => field field1: Uint32 = Uint32 1 procedure a() end");
    check_ok!(
        bluebell::ContractDefinitionParser,
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

    check_err!(bluebell::ContractDefinitionParser, "contract MyContract");
    check_err!(
        bluebell::ContractDefinitionParser,
        "contract MyContract end"
    );
    check_err!(bluebell::ContractDefinitionParser, "Contract MyContract");
    check_err!(bluebell::ContractDefinitionParser, "contract");
    check_err!(
        bluebell::ContractDefinitionParser,
        "contract MyContract(address: ByStr20,) with (true =>)"
    );
    check_err!(
        bluebell::ContractDefinitionParser,
        "contract MyContract(address: ByStr20) with true => field field1 = 1"
    );
    check_err!(
        bluebell::ContractDefinitionParser,
        "contract MyContract(address: ByStr20) with true => field field1: = 1"
    );
    check_err!(
        bluebell::ContractDefinitionParser,
        "contract MyContract(address: ByStr20) with true => field field1: Uint32 = 1 field2: 10"
    );
    check_err!(bluebell::ContractDefinitionParser, "contract MyContract(address: ByStr20) with true => field field1: Uint32 = 1 transition a(Void) {}");
    check_err!(bluebell::ContractDefinitionParser, "contract MyContract(address: ByStr20) with true => field field1: Uint32 = 1 procedure a(Void) {");
}

#[test]
fn type_alternative_clause() {
    check_ok!(bluebell::TypeAlternativeClauseParser, "| MyType");
    check_ok!(bluebell::TypeAlternativeClauseParser, "| MyType of Int");
    check_ok!(
        bluebell::TypeAlternativeClauseParser,
        "| ByStr123 of Map MyType Int"
    );

    check_err!(bluebell::TypeAlternativeClauseParser, "| MyType of");
    check_err!(bluebell::TypeAlternativeClauseParser, "| 123MyType");
    check_err!(bluebell::TypeAlternativeClauseParser, "| ByStr");
}

#[test]
fn library_single_definition() {
    check_ok!(
        bluebell::LibrarySingleDefinitionParser,
        "let foo = Int32 42"
    );
    check_ok!(
        bluebell::LibrarySingleDefinitionParser,
        "let foo: Int32 = Int32 42"
    );
    check_ok!(bluebell::LibrarySingleDefinitionParser, "type Foo");
    check_ok!(
        bluebell::LibrarySingleDefinitionParser,
        "type Foo = | Bar | Baz"
    );

    check_err!(bluebell::LibrarySingleDefinitionParser, "let = Int32 42");
    check_err!(bluebell::LibrarySingleDefinitionParser, "let foo: = 42");
    check_err!(bluebell::LibrarySingleDefinitionParser, "type Int32 42");
    check_err!(
        bluebell::LibrarySingleDefinitionParser,
        "type Foo = | Bar Baz"
    );
}

#[test]
fn library_definition() {
    check_ok!(bluebell::LibraryDefinitionParser, "library Foo");
    check_ok!(
        bluebell::LibraryDefinitionParser,
        "library Bar let x = Int32 10"
    );
    check_ok!(bluebell::LibraryDefinitionParser, "library Baz type Quux");
    check_ok!(
        bluebell::LibraryDefinitionParser,
        "library Qux type Quux = | Event"
    );
    check_ok!(
        bluebell::LibraryDefinitionParser,
        "library Quux type Quux = | Event of Uint256"
    );
    check_ok!(
        bluebell::LibraryDefinitionParser,
        "library Quuz type Quux = | Event of Uint256 | AnotherEvent of ByStr20"
    );
    check_ok!(
        bluebell::LibraryDefinitionParser,
        "library Qoorx let x: Int32 = Int32 42"
    );

    check_err!(bluebell::LibraryDefinitionParser, "library Foo Bar");
    check_err!(bluebell::LibraryDefinitionParser, "library");
    check_err!(bluebell::LibraryDefinitionParser, "library Foo bar");
    check_err!(bluebell::LibraryDefinitionParser, "library Foo type");
    check_err!(
        bluebell::LibraryDefinitionParser,
        "library Foo type = | Event"
    );
    check_err!(bluebell::LibraryDefinitionParser, "library Foo type Quux =");
    check_err!(
        bluebell::LibraryDefinitionParser,
        "library Foo type Quux = |"
    );
    check_err!(
        bluebell::LibraryDefinitionParser,
        "library Foo type Quux = | Event of"
    );
    check_err!(bluebell::LibraryDefinitionParser, "library Foo let");
    check_err!(bluebell::LibraryDefinitionParser, "library Foo let x");
    check_err!(bluebell::LibraryDefinitionParser, "library Foo let = 42");
}

#[derive(Debug)]
struct ParserError {
    message: String,
    line: usize,
    column: usize,
}

impl std::error::Error for ParserError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} (line {}, column {})",
            self.message, self.line, self.column
        )
    }
}
// */
fn main() {
    let mut errors: Vec<lexer::ParseError> = [].to_vec();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Missing file name");
    }
    let mut file = File::open(&args[1]).expect("Unable to open file");
    let mut script = String::new();
    file.read_to_string(&mut script)
        .expect("Unable to read file");

    let lexer = Lexer::new(&script);

    let parser = bluebell::ProgramParser::new();
    match parser.parse(&mut errors, lexer) {
        Ok(ast) => println!("{:?}", ast),
        Err(error) => {
            let message = format!("Syntax error {:?}", error);
            let mut pos: Vec<usize> = [].to_vec();
            error.map_location(|l| {
                pos.push(l);
                l
            });

            let mut n = 0;
            let mut line_counter = 0;
            let mut char_counter = 0;
            let mut line_start = 0;
            let mut line_end = 0;
            let mut should_stop = false;
            for ch in script.chars() {
                if ch == '\n' {
                    if should_stop {
                        line_end = n;
                        break;
                    } else {
                        line_start = n + 1;
                    }
                }
                if !should_stop && n == pos[0] {
                    should_stop = true;
                }

                n += 1;
                if !should_stop {
                    char_counter += 1;
                }

                if ch == '\n' {
                    line_counter += 1;
                    char_counter = 0;
                }
            }

            if line_end < line_start {
                line_end = script.len();
            }

            let line = &script[line_start..line_end];
            println!("Line {},{}:{}", line_counter, char_counter, line);
            print!(
                "{}",
                " ".repeat(char_counter + format!("Line {},{}:", line_counter, char_counter).len())
            );
            println!("{}", "^".repeat(pos[1] - pos[0]));

            let my_error = ParserError {
                message,
                line: 0,   //error.location_line(),
                column: 0, // err.location_column(),
            };
            println!("{}", my_error);

            process::exit(-1);
        }
    }
}
