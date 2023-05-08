#[macro_use]
extern crate lalrpop_util;
pub mod ast;
pub mod lexer;
use crate::lexer::Lexer;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::Read;
use std::process;
lalrpop_mod!(pub bluebell);

#[test]
fn bytestring_parser() {
    let mut errors: Vec<lexer::ParseError> = [].to_vec();

    assert!(bluebell::ByteStringParser::new()
        .parse(&mut errors, lexer::Lexer::new("ByStr3"),)
        .is_ok());

    assert!(bluebell::ByteStringParser::new()
        .parse(&mut errors, lexer::Lexer::new("ByStr11"))
        .is_ok());
    assert!(bluebell::ByteStringParser::new()
        .parse(&mut errors, lexer::Lexer::new("ByStr0"))
        .is_ok());

    assert!(bluebell::ByteStringParser::new()
        .parse(&mut errors, lexer::Lexer::new("ByStr 3"))
        .is_err());
    assert!(bluebell::ByteStringParser::new()
        .parse(&mut errors, lexer::Lexer::new("ByStr 11"))
        .is_err());
    assert!(bluebell::ByteStringParser::new()
        .parse(&mut errors, lexer::Lexer::new("ByStr 0"))
        .is_err());
    assert!(bluebell::ByteStringParser::new()
        .parse(&mut errors, lexer::Lexer::new("ByStr"))
        .is_err());
}

#[test]
fn type_name_identifier_parser() {
    let mut errors: Vec<lexer::ParseError> = [].to_vec();

    assert!(bluebell::TypeNameIdentifierParser::new()
        .parse(&mut errors, lexer::Lexer::new("Event"))
        .is_ok());
    assert!(bluebell::TypeNameIdentifierParser::new()
        .parse(&mut errors, lexer::Lexer::new("Foo"))
        .is_ok());
    assert!(bluebell::TypeNameIdentifierParser::new()
        .parse(&mut errors, lexer::Lexer::new("ByStr42"))
        .is_ok());

    assert!(bluebell::TypeNameIdentifierParser::new()
        .parse(&mut errors, lexer::Lexer::new("bystr"))
        .is_err());
    assert!(bluebell::TypeNameIdentifierParser::new()
        .parse(&mut errors, lexer::Lexer::new("_foo"))
        .is_err());
    assert!(bluebell::TypeNameIdentifierParser::new()
        .parse(&mut errors, lexer::Lexer::new("ByStr 0"))
        .is_err());
    assert!(bluebell::TypeNameIdentifierParser::new()
        .parse(&mut errors, lexer::Lexer::new("Type With Spaces"))
        .is_err());
}

#[test]
fn imported_name_parser() {
    let mut errors: Vec<lexer::ParseError> = [].to_vec();

    assert!(bluebell::ImportedNameParser::new()
        .parse(&mut errors, lexer::Lexer::new("Event"))
        .is_ok());
    assert!(bluebell::ImportedNameParser::new()
        .parse(&mut errors, lexer::Lexer::new("Foo"))
        .is_ok());
    assert!(bluebell::ImportedNameParser::new()
        .parse(&mut errors, lexer::Lexer::new("ByStr42"))
        .is_ok());
    assert!(bluebell::ImportedNameParser::new()
        .parse(&mut errors, lexer::Lexer::new("Event as Alias"))
        .is_ok());
    assert!(bluebell::ImportedNameParser::new()
        .parse(&mut errors, lexer::Lexer::new("Foo as Alias"))
        .is_ok());
    assert!(bluebell::ImportedNameParser::new()
        .parse(&mut errors, lexer::Lexer::new("ByStr42 as Alias"))
        .is_ok());

    assert!(bluebell::ImportedNameParser::new()
        .parse(&mut errors, lexer::Lexer::new("foo"))
        .is_err());
    assert!(bluebell::ImportedNameParser::new()
        .parse(&mut errors, lexer::Lexer::new("Type With Spaces"))
        .is_err());
    assert!(bluebell::ImportedNameParser::new()
        .parse(&mut errors, lexer::Lexer::new("Foo As Alias"))
        .is_err());
    assert!(bluebell::ImportedNameParser::new()
        .parse(&mut errors, lexer::Lexer::new("Event as"))
        .is_err());
}

#[test]
fn import_declarations() {
    let mut errors: Vec<lexer::ParseError> = [].to_vec();

    assert!(bluebell::ImportDeclarationsParser::new()
        .parse(&mut errors, lexer::Lexer::new("import Foo"))
        .is_ok());
    assert!(bluebell::ImportDeclarationsParser::new()
        .parse(
            &mut errors,
            lexer::Lexer::new("import Event import ByStr42")
        )
        .is_ok());
    assert!(bluebell::ImportDeclarationsParser::new()
        .parse(&mut errors, lexer::Lexer::new("import Foo as Bar"))
        .is_ok());

    assert!(bluebell::ImportDeclarationsParser::new()
        .parse(&mut errors, lexer::Lexer::new("foo"))
        .is_err());
    assert!(bluebell::ImportDeclarationsParser::new()
        .parse(&mut errors, lexer::Lexer::new("import"))
        .is_err());
    assert!(bluebell::ImportDeclarationsParser::new()
        .parse(&mut errors, lexer::Lexer::new("import Foo import"))
        .is_err());
}

#[test]
fn meta_identifiers() {
    let mut errors: Vec<lexer::ParseError> = [].to_vec();

    assert!(bluebell::MetaIdentifierParser::new()
        .parse(&mut errors, lexer::Lexer::new("Event"))
        .is_ok());
    assert!(bluebell::MetaIdentifierParser::new()
        .parse(&mut errors, lexer::Lexer::new("Foo.Bar"))
        .is_ok());
    assert!(bluebell::MetaIdentifierParser::new()
        .parse(&mut errors, lexer::Lexer::new("ABCD.Event"))
        .is_ok());
    assert!(bluebell::MetaIdentifierParser::new()
        .parse(&mut errors, lexer::Lexer::new("0x1234.Event"))
        .is_ok());
    assert!(bluebell::MetaIdentifierParser::new()
        .parse(&mut errors, lexer::Lexer::new("ByStr"))
        .is_ok());

    assert!(bluebell::MetaIdentifierParser::new()
        .parse(&mut errors, lexer::Lexer::new("_foo"))
        .is_err());
    assert!(bluebell::MetaIdentifierParser::new()
        .parse(&mut errors, lexer::Lexer::new("Type With Spaces"))
        .is_err());
    assert!(bluebell::MetaIdentifierParser::new()
        .parse(&mut errors, lexer::Lexer::new("Foo .. Bar"))
        .is_err());
    assert!(bluebell::MetaIdentifierParser::new()
        .parse(&mut errors, lexer::Lexer::new("0x1234 - Bar"))
        .is_err());
    assert!(bluebell::MetaIdentifierParser::new()
        .parse(&mut errors, lexer::Lexer::new("Foo."))
        .is_err());
    assert!(bluebell::MetaIdentifierParser::new()
        .parse(&mut errors, lexer::Lexer::new("0x1234.42"))
        .is_err());
}

#[test]
fn variable_identifier() {
    let mut errors: Vec<lexer::ParseError> = [].to_vec();

    assert!(bluebell::VariableIdentifierParser::new()
        .parse(&mut errors, lexer::Lexer::new("foo"))
        .is_ok());

    assert!(bluebell::VariableIdentifierParser::new()
        .parse(&mut errors, lexer::Lexer::new("_bar"))
        .is_ok());

    assert!(bluebell::VariableIdentifierParser::new()
        .parse(&mut errors, lexer::Lexer::new("Foo.bar"))
        .is_ok());
    assert!(bluebell::VariableIdentifierParser::new()
        .parse(&mut errors, lexer::Lexer::new("ByStr42.baz"))
        .is_ok());
    assert!(bluebell::VariableIdentifierParser::new()
        .parse(&mut errors, lexer::Lexer::new("Event.qux"))
        .is_ok());

    assert!(bluebell::VariableIdentifierParser::new()
        .parse(&mut errors, lexer::Lexer::new("42"))
        .is_err());
    assert!(bluebell::VariableIdentifierParser::new()
        .parse(&mut errors, lexer::Lexer::new("_"))
        .is_err());
    assert!(bluebell::VariableIdentifierParser::new()
        .parse(&mut errors, lexer::Lexer::new("Type With Spaces.baz"))
        .is_err());
    //    assert!(bluebell::VariableIdentifierParser::new().parse(&mut errors,  lexer::Lexer::new("Bystr.qux")).is_err());
    assert!(bluebell::VariableIdentifierParser::new()
        .parse(&mut errors, lexer::Lexer::new("Event42.Bar"))
        .is_err());
    assert!(bluebell::VariableIdentifierParser::new()
        .parse(&mut errors, lexer::Lexer::new("Foo."))
        .is_err());
}

#[test]
fn builtin_arguments() {
    let mut errors: Vec<lexer::ParseError> = [].to_vec();

    assert!(bluebell::BuiltinArgumentsParser::new()
        .parse(&mut errors, lexer::Lexer::new("( )"))
        .is_ok());
    assert!(bluebell::BuiltinArgumentsParser::new()
        .parse(&mut errors, lexer::Lexer::new("foo"))
        .is_ok());
    assert!(bluebell::BuiltinArgumentsParser::new()
        .parse(&mut errors, lexer::Lexer::new("foo bar baz"))
        .is_ok());
    assert!(bluebell::BuiltinArgumentsParser::new()
        .parse(&mut errors, lexer::Lexer::new("Event.qux"))
        .is_ok());

    assert!(bluebell::BuiltinArgumentsParser::new()
        .parse(&mut errors, lexer::Lexer::new("42"))
        .is_err());
    assert!(bluebell::BuiltinArgumentsParser::new()
        .parse(&mut errors, lexer::Lexer::new("_"))
        .is_err());
    assert!(bluebell::BuiltinArgumentsParser::new()
        .parse(&mut errors, lexer::Lexer::new("Type With Spaces.baz"))
        .is_err());
    assert!(bluebell::BuiltinArgumentsParser::new()
        .parse(&mut errors, lexer::Lexer::new("Event42.Bar"))
        .is_err());
    assert!(bluebell::BuiltinArgumentsParser::new()
        .parse(&mut errors, lexer::Lexer::new("Foo."))
        .is_err());
}

#[test]
fn scilla_types() {
    let mut errors: Vec<lexer::ParseError> = [].to_vec();

    assert!(bluebell::ScillaTypeParser::new()
        .parse(&mut errors, lexer::Lexer::new("Uint32"))
        .is_ok());
    assert!(bluebell::ScillaTypeParser::new()
        .parse(&mut errors, lexer::Lexer::new("Foo(Bar)"))
        .is_ok());
    assert!(bluebell::ScillaTypeParser::new()
        .parse(&mut errors, lexer::Lexer::new("Map Uint32 String"))
        .is_ok());
    assert!(bluebell::ScillaTypeParser::new()
        .parse(&mut errors, lexer::Lexer::new("Uint32 -> Bool"))
        .is_ok());
    assert!(bluebell::ScillaTypeParser::new()
        .parse(&mut errors, lexer::Lexer::new("(Uint32)"))
        .is_ok());
    assert!(bluebell::ScillaTypeParser::new()
        .parse(&mut errors, lexer::Lexer::new("Address with end"))
        .is_ok());
    assert!(bluebell::ScillaTypeParser::new()
        .parse(
            &mut errors,
            lexer::Lexer::new("forall 'A. forall 'B. ( 'B -> 'A -> 'B) -> 'B -> List 'A -> 'B")
        )
        .is_ok());
    assert!(bluebell::ScillaTypeParser::new()
        .parse(&mut errors, lexer::Lexer::new("T"))
        .is_ok());

    assert!(bluebell::ScillaTypeParser::new()
        .parse(&mut errors, lexer::Lexer::new("Map"))
        .is_err());
    assert!(bluebell::ScillaTypeParser::new()
        .parse(&mut errors, lexer::Lexer::new("Uint32 ->"))
        .is_err());
    assert!(bluebell::ScillaTypeParser::new()
        .parse(&mut errors, lexer::Lexer::new("-> Bool"))
        .is_err());
    assert!(bluebell::ScillaTypeParser::new()
        .parse(&mut errors, lexer::Lexer::new("address with"))
        .is_err());
    assert!(bluebell::ScillaTypeParser::new()
        .parse(&mut errors, lexer::Lexer::new("address with Foo end"))
        .is_err());
    assert!(bluebell::ScillaTypeParser::new()
        .parse(&mut errors, lexer::Lexer::new("forall T. Map(T, Uint32)"))
        .is_err());
    assert!(bluebell::ScillaTypeParser::new()
        .parse(&mut errors, lexer::Lexer::new("Foo(Bar"))
        .is_err());
}

#[test]
fn test_address_type() {
    let mut errors: Vec<lexer::ParseError> = [].to_vec();

    assert!(bluebell::AddressTypeParser::new()
        .parse(&mut errors,  lexer::Lexer::new("Foo with end")
        ).is_ok());
    assert!(bluebell::AddressTypeParser::new()
        .parse(&mut errors,  lexer::Lexer::new("ByStr42 with end")
        ).is_ok());
    assert!(bluebell::AddressTypeParser::new()
        .parse(&mut errors,  lexer::Lexer::new("Event with end")
        ).is_ok());
    assert!(bluebell::AddressTypeParser::new()
        .parse(&mut errors,  lexer::Lexer::new("Foo with contract field field1: Uint32, field field2: Uint32 end")
        ).is_ok());
    assert!(bluebell::AddressTypeParser::new()
        .parse(&mut errors,  lexer::Lexer::new("ByStr42 with contract field field1: Uint32 end")
        ).is_ok());
    assert!(bluebell::AddressTypeParser::new()
        .parse(&mut errors,  lexer::Lexer::new("Event with contract end")
        ).is_ok());
    assert!(bluebell::AddressTypeParser::new()
        .parse(&mut errors,  lexer::Lexer::new("Foo with library end")
        ).is_ok());
    assert!(bluebell::AddressTypeParser::new()
        .parse(&mut errors,  lexer::Lexer::new("ByStr42 with library end")
        ).is_ok());
    assert!(bluebell::AddressTypeParser::new()
        .parse(&mut errors,  lexer::Lexer::new("Event with library end")
        ).is_ok());
    assert!(bluebell::AddressTypeParser::new()
        .parse(&mut errors,  lexer::Lexer::new("Foo with _foo end")
        ).is_ok());
    assert!(bluebell::AddressTypeParser::new()
        .parse(&mut errors,  lexer::Lexer::new("ByStr42 with _foo end")
        ).is_ok());
    assert!(bluebell::AddressTypeParser::new()
        .parse(&mut errors,  lexer::Lexer::new("Event with _foo end")
        ).is_ok());
    assert!(bluebell::AddressTypeParser::new()
        .parse(&mut errors,  lexer::Lexer::new("Foo with contract field field1: Uint32 end")
        ).is_ok());
    assert!(bluebell::AddressTypeParser::new().parse(&mut errors,  lexer::Lexer::new("ByStr42 with contract field field1: Uint32, field field2: Uint32, field field3: Uint32 end")).is_ok());
    assert!(bluebell::AddressTypeParser::new()
        .parse(&mut errors,  lexer::Lexer::new("Event with contract field field1: Uint32 end")
        ).is_ok());

    assert!(bluebell::AddressTypeParser::new()
        .parse(&mut errors,  lexer::Lexer::new("foo with end")
        ).is_err());
    assert!(bluebell::AddressTypeParser::new()
        .parse(&mut errors,  lexer::Lexer::new("Foo with ")
        ).is_err());
    assert!(bluebell::AddressTypeParser::new()
        .parse(&mut errors,  lexer::Lexer::new("Foo with foo bar: Uint32 end")
        ).is_err());
    assert!(bluebell::AddressTypeParser::new()
        .parse(&mut errors,  lexer::Lexer::new("Foo with contract field1: Uint32, field2: Uint32")
        ).is_err());
    assert!(bluebell::AddressTypeParser::new()
        .parse(&mut errors,  lexer::Lexer::new("Foo with contract, end")
        ).is_err());
}

#[test]
fn test_type_map_key() {
    let mut errors: Vec<lexer::ParseError> = [].to_vec();

    assert!(bluebell::TypeMapKeyParser::new().parse(&mut errors,  lexer::Lexer::new("Foo")).is_ok());
    assert!(bluebell::TypeMapKeyParser::new().parse(&mut errors,  lexer::Lexer::new("(Foo)")).is_ok());
    assert!(bluebell::TypeMapKeyParser::new()
        .parse(&mut errors,  lexer::Lexer::new("Foo with end")
        ).is_ok());
    assert!(bluebell::TypeMapKeyParser::new()
        .parse(&mut errors,  lexer::Lexer::new("(Foo with end)")
        ).is_ok());
    assert!(bluebell::TypeMapKeyParser::new()
        .parse(&mut errors,  lexer::Lexer::new("(ByStr42 with contract end)")
        ).is_ok());
    assert!(bluebell::TypeMapKeyParser::new()
        .parse(&mut errors,  lexer::Lexer::new("Foo with library end")
        ).is_ok());

    assert!(bluebell::TypeMapKeyParser::new().parse(&mut errors,  lexer::Lexer::new("foo")).is_err());
    assert!(bluebell::TypeMapKeyParser::new().parse(&mut errors,  lexer::Lexer::new("Foo()")).is_err());
    assert!(bluebell::TypeMapKeyParser::new()
        .parse(&mut errors,  lexer::Lexer::new("(Foo with bar end)")
        ).is_err());
    assert!(bluebell::TypeMapKeyParser::new().parse(&mut errors,  lexer::Lexer::new("(42)")).is_err());
}

#[test]
fn type_map_value() {
    let mut errors: Vec<lexer::ParseError> = [].to_vec();

    assert!(bluebell::TypeMapValueParser::new().parse(&mut errors,  lexer::Lexer::new("Uint32")).is_ok());
    assert!(bluebell::TypeMapValueParser::new()
        .parse(&mut errors,  lexer::Lexer::new("Map Foo Bar")
        ).is_ok());
    assert!(bluebell::TypeMapValueParser::new()
        .parse(&mut errors,  lexer::Lexer::new("(Uint32)")
        ).is_ok());
    assert!(bluebell::TypeMapValueParser::new()
        .parse(&mut errors,  lexer::Lexer::new("Address with end")
        ).is_ok());

    assert!(bluebell::TypeMapValueParser::new()
        .parse(&mut errors,  lexer::Lexer::new("Foo with contract field field1: Uint32 end")
        ).is_ok());

    assert!(bluebell::TypeMapValueParser::new().parse(&mut errors,  lexer::Lexer::new("foo")).is_err());
    assert!(bluebell::TypeMapValueParser::new().parse(&mut errors,  lexer::Lexer::new("bystr1")).is_err());
    assert!(bluebell::TypeMapValueParser::new().parse(&mut errors,  lexer::Lexer::new("event")).is_err());
    assert!(bluebell::TypeMapValueParser::new()
        .parse(&mut errors,  lexer::Lexer::new("map(foo, bar)")
        ).is_err());
    assert!(bluebell::TypeMapValueParser::new().parse(&mut errors,  lexer::Lexer::new("(42)")).is_err());
    assert!(bluebell::TypeMapValueParser::new()
        .parse(&mut errors,  lexer::Lexer::new("address with")
        ).is_err());
    assert!(bluebell::TypeMapValueParser::new()
        .parse(&mut errors,  lexer::Lexer::new("foo with foo bar")
        ).is_err());
}

#[test]
fn type_map_value_arguments() {

    let mut errors: Vec<lexer::ParseError> = [].to_vec();

    assert!(bluebell::TypeMapValueArgumentsParser::new()
        .parse(&mut errors,  lexer::Lexer::new("(Uint32)")
        ).is_ok());
    assert!(bluebell::TypeMapValueArgumentsParser::new()
        .parse(&mut errors,  lexer::Lexer::new("Foo")
        ).is_ok());
    assert!(bluebell::TypeMapValueArgumentsParser::new()
        .parse(&mut errors,  lexer::Lexer::new("Map Foo Bar")
        ).is_ok());

    assert!(bluebell::TypeMapValueArgumentsParser::new()
        .parse(&mut errors,  lexer::Lexer::new("Foo Bar")
        ).is_err());
    assert!(bluebell::TypeMapValueArgumentsParser::new()
        .parse(&mut errors,  lexer::Lexer::new("map(foo, bar)")
        ).is_err());
    assert!(bluebell::TypeMapValueArgumentsParser::new()
        .parse(&mut errors,  lexer::Lexer::new("(42)()")
        ).is_err());
    assert!(bluebell::TypeMapValueArgumentsParser::new()
        .parse(&mut errors,  lexer::Lexer::new("(Uint32")
        ).is_err());
    assert!(bluebell::TypeMapValueArgumentsParser::new()
        .parse(&mut errors,  lexer::Lexer::new("Map(Foo)")
        ).is_err());
    assert!(bluebell::TypeMapValueArgumentsParser::new()
        .parse(&mut errors,  lexer::Lexer::new("Map(Foo, Bar)")
        ).is_err());

}

#[test]
fn type_argument() {

    let mut errors: Vec<lexer::ParseError> = [].to_vec();

    assert!(bluebell::TypeArgumentParser::new().parse(&mut errors,  lexer::Lexer::new("Foo")).is_ok());
    assert!(bluebell::TypeArgumentParser::new().parse(&mut errors,  lexer::Lexer::new("(Bar)")).is_ok());
    assert!(bluebell::TypeArgumentParser::new().parse(&mut errors,  lexer::Lexer::new("Uint32")).is_ok());
    assert!(bluebell::TypeArgumentParser::new().parse(&mut errors,  lexer::Lexer::new("'A")).is_ok());
    assert!(bluebell::TypeArgumentParser::new()
        .parse(&mut errors,  lexer::Lexer::new("(Uint32)")
        ).is_ok());
    assert!(bluebell::TypeArgumentParser::new()
        .parse(&mut errors,  lexer::Lexer::new("Address with contract field field1: Uint32, field field2: Uint32 end")
        ).is_ok());
    assert!(bluebell::TypeArgumentParser::new()
        .parse(&mut errors,  lexer::Lexer::new("Map Uint32 Bool")
        ).is_ok());

    assert!(bluebell::TypeArgumentParser::new()
        .parse(&mut errors,  lexer::Lexer::new("foo bar")
        ).is_err());
    assert!(bluebell::TypeArgumentParser::new().parse(&mut errors,  lexer::Lexer::new("123")).is_err());
    assert!(bluebell::TypeArgumentParser::new()
        .parse(&mut errors,  lexer::Lexer::new("foo.bar")
        ).is_err());
    assert!(bluebell::TypeArgumentParser::new()
        .parse(&mut errors,  lexer::Lexer::new("mapUint32Uint32")
        ).is_err());
    assert!(bluebell::TypeArgumentParser::new().parse(&mut errors,  lexer::Lexer::new("'_A")).is_err());
    assert!(bluebell::TypeArgumentParser::new()
        .parse(&mut errors,  lexer::Lexer::new("(map(Int32, String))")
        ).is_err());
    assert!(bluebell::TypeArgumentParser::new()
        .parse(&mut errors,  lexer::Lexer::new("Foo.bar")
        ).is_err());
    assert!(bluebell::TypeArgumentParser::new()
        .parse(&mut errors,  lexer::Lexer::new("Map(Int32, String, Bool)")
        ).is_err());

}

#[test]
fn full_expressions() {

    let mut errors: Vec<lexer::ParseError> = [].to_vec();

    assert!(bluebell::FullExpressionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("let x = Int32 42 in X")
        ).is_ok()); // let-binding of a number
    assert!(bluebell::FullExpressionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("let x: Int32 = Int32 42 in X")
        ).is_ok()); // let-binding of a typed number
    assert!(bluebell::FullExpressionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("let x = Uint128 42 in builtin lt x x")
        ).is_ok()); // let-binding of a number
    assert!(bluebell::FullExpressionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("fun (x: Int32) => builtin lt x x")
        ).is_ok()); // lambda function that adds 1 to its argument
    assert!(bluebell::FullExpressionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("fun (x: Int32) => foo")
        ).is_ok()); // lambda function that adds 1 to its argument
    assert!(bluebell::FullExpressionParser::new().parse(&mut errors,  lexer::Lexer::new("foo")).is_ok()); // single variable identifier
    assert!(bluebell::FullExpressionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("foo bar baz")
        ).is_ok()); // multiple variable identifiers
    assert!(bluebell::FullExpressionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("UInt32 42")
        ).is_ok()); // number literal
    assert!(bluebell::FullExpressionParser::new().parse(&mut errors,  lexer::Lexer::new("true")).is_ok()); // boolean literal
    assert!(bluebell::FullExpressionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("builtin blabla a b")
        ).is_ok()); // builtin function call with arguments
    assert!(bluebell::FullExpressionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("{ foo: UInt32 42; bar: Int64 23 }")
        ).is_ok()); // record literal
    assert!(bluebell::FullExpressionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("match foo with \n| False => True \n| _ => False \nend")
        ).is_ok()); // match expression
    assert!(bluebell::FullExpressionParser::new().parse(&mut errors,  lexer::Lexer::new("Foo")).is_ok()); // meta-identifier
    assert!(bluebell::FullExpressionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("Foo  m  n")
        ).is_ok()); // meta-identifier with arguments
    assert!(bluebell::FullExpressionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("tfun 'T => fun (x: 'T) => x")
        ).is_ok()); // type-level function
    assert!(bluebell::FullExpressionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("@foo Type")
        ).is_ok()); // type-level function application

    assert!(bluebell::FullExpressionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("let 42 = x in x")
        ).is_err()); // variable name cannot be a number
    assert!(bluebell::FullExpressionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("let x: = 42 in x")
        ).is_err()); // missing type annotation after colon
    assert!(bluebell::FullExpressionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("fun x => x + 1")
        ).is_err()); // missing type annotation in lambda function
    assert!(bluebell::FullExpressionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("42foo")
        ).is_err()); // variable identifier cannot start with a number
    assert!(bluebell::FullExpressionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("42.foo")
        ).is_err()); // variable identifier cannot start with a number
    assert!(bluebell::FullExpressionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("42.23")
        ).is_err()); // invalid floating point literal
    assert!(bluebell::FullExpressionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("builtin noop")
        ).is_err()); // builtin function call without arguments
    assert!(bluebell::FullExpressionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("{ foo = 42; bar: 23 }")
        ).is_err()); // missing colon in record field
    assert!(bluebell::FullExpressionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("{ foo: 42, bar = 23 }")
        ).is_err()); // using equals sign instead of colon in record field
    assert!(bluebell::FullExpressionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("{ foo = 42, }")
        ).is_err()); // trailing comma in record literal
    assert!(bluebell::FullExpressionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("match foo with | 42 => true end")
        ).is_err()); // missing 'else' clause in match expression
    assert!(bluebell::FullExpressionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("Foo()")
        ).is_err()); // invalid number of arguments for meta-identifier
    assert!(bluebell::FullExpressionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("tfun T => Foo")
        ).is_err()); // missing expression after type-level function
    assert!(bluebell::FullExpressionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("@foo()")
        ).is_err()); // missing type argument in type-level function application

}

#[test]
fn atomic_expression() {
    let mut errors: Vec<lexer::ParseError> = [].to_vec();

    assert!(bluebell::AtomicExpressionParser::new().parse(&mut errors,  lexer::Lexer::new("foo")).is_ok());
    assert!(bluebell::AtomicExpressionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("Uint32 42")
        ).is_ok());
    assert!(bluebell::AtomicExpressionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("0x123abc")
        ).is_ok());
    assert!(bluebell::AtomicExpressionParser::new()
        .parse(&mut errors,  lexer::Lexer::new(r#""string""#)
        ).is_ok());

    assert!(bluebell::AtomicExpressionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("(foo)")
        ).is_err());
    assert!(bluebell::AtomicExpressionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("42.0")
        ).is_err());
    assert!(bluebell::AtomicExpressionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("True")
        ).is_err());
}

#[test]
fn value_literal() {
    let mut errors: Vec<lexer::ParseError> = [].to_vec();

    assert!(bluebell::ValueLiteralParser::new()
        .parse(&mut errors,  lexer::Lexer::new("Foo123 42")
        ).is_ok());
    assert!(bluebell::ValueLiteralParser::new().parse(&mut errors,  lexer::Lexer::new("0x1234")).is_ok());
    assert!(bluebell::ValueLiteralParser::new()
        .parse(&mut errors,  lexer::Lexer::new("\"hello world\"")
        ).is_ok());
    assert!(bluebell::ValueLiteralParser::new()
        .parse(&mut errors,  lexer::Lexer::new("Emp Uint32 Uint32")
        ).is_ok());
    assert!(bluebell::ValueLiteralParser::new()
        .parse(&mut errors,  lexer::Lexer::new("ByStr20 123")
        ).is_ok());

    assert!(bluebell::ValueLiteralParser::new()
        .parse(&mut errors,  lexer::Lexer::new("Foo Bar")
        ).is_err());
    assert!(bluebell::ValueLiteralParser::new()
        .parse(&mut errors,  lexer::Lexer::new("ByStr hello")
        ).is_err());
    assert!(bluebell::ValueLiteralParser::new().parse(&mut errors,  lexer::Lexer::new("1 + 2")).is_err());
}

#[test]
fn map_access() {
    let mut errors: Vec<lexer::ParseError> = [].to_vec();


    assert!(bluebell::MapAccessParser::new().parse(&mut errors,  lexer::Lexer::new("[foo]")).is_ok());
    assert!(bluebell::MapAccessParser::new().parse(&mut errors,  lexer::Lexer::new("[bar123]")).is_ok());
    assert!(bluebell::MapAccessParser::new().parse(&mut errors,  lexer::Lexer::new("[_result]")).is_ok());

    assert!(bluebell::MapAccessParser::new().parse(&mut errors,  lexer::Lexer::new("[0x0232]")).is_err());
    assert!(bluebell::MapAccessParser::new().parse(&mut errors,  lexer::Lexer::new(r#"["xx"]"#)).is_err());
    assert!(bluebell::MapAccessParser::new().parse(&mut errors,  lexer::Lexer::new("[Foo]")).is_err());
    assert!(bluebell::MapAccessParser::new().parse(&mut errors,  lexer::Lexer::new("[]")).is_err());
    assert!(bluebell::MapAccessParser::new().parse(&mut errors,  lexer::Lexer::new("[foo.bar]")).is_err());
    
}

#[test]
fn pattern() {
    let mut errors: Vec<lexer::ParseError> = [].to_vec();

    assert!(bluebell::PatternParser::new().parse(&mut errors,  lexer::Lexer::new("_")).is_ok());
    assert!(bluebell::PatternParser::new().parse(&mut errors,  lexer::Lexer::new("foo")).is_ok());
    assert!(bluebell::PatternParser::new().parse(&mut errors,  lexer::Lexer::new("Bar42")).is_ok());
    assert!(bluebell::PatternParser::new().parse(&mut errors,  lexer::Lexer::new("Bar42 _")).is_ok());
    assert!(bluebell::PatternParser::new().parse(&mut errors,  lexer::Lexer::new("Bar42 hello")).is_ok());
    assert!(bluebell::PatternParser::new().parse(&mut errors,  lexer::Lexer::new("Bar42(_)")).is_ok());
    assert!(bluebell::PatternParser::new().parse(&mut errors,  lexer::Lexer::new("Bar42(Foo)")).is_ok());
    assert!(bluebell::PatternParser::new()
        .parse(&mut errors,  lexer::Lexer::new("Bar42(Foo Bar)")
        ).is_ok());
    assert!(bluebell::PatternParser::new()
        .parse(&mut errors,  lexer::Lexer::new("Bar42(ByStr42 Int32)")
        ).is_ok());
    assert!(bluebell::PatternParser::new()
        .parse(&mut errors,  lexer::Lexer::new("Bar42(Foo.Bar Bar.Baz)")
        ).is_ok());

    assert!(bluebell::PatternParser::new().parse(&mut errors,  lexer::Lexer::new("_ _")).is_err());
    assert!(bluebell::PatternParser::new().parse(&mut errors,  lexer::Lexer::new("42Bar")).is_err());
    assert!(bluebell::PatternParser::new().parse(&mut errors,  lexer::Lexer::new("foo bar")).is_err());
    assert!(bluebell::PatternParser::new()
        .parse(&mut errors,  lexer::Lexer::new("Foo42(, Bar)")
        ).is_err());
    assert!(bluebell::PatternParser::new()
        .parse(&mut errors,  lexer::Lexer::new("Foo42(Map, Bar)")
        ).is_err());
    assert!(bluebell::PatternParser::new()
        .parse(&mut errors,  lexer::Lexer::new("Bar42(Map ByStr42 Int32)")
        ).is_err());
}

#[test]
fn argument_pattern() {
    let mut errors: Vec<lexer::ParseError> = [].to_vec();

    assert!(bluebell::ArgumentPatternParser::new().parse(&mut errors,  lexer::Lexer::new("_")).is_ok());
    assert!(bluebell::ArgumentPatternParser::new().parse(&mut errors,  lexer::Lexer::new("foo")).is_ok());
    assert!(bluebell::ArgumentPatternParser::new()
        .parse(&mut errors,  lexer::Lexer::new("MyType")
        ).is_ok());
    assert!(bluebell::ArgumentPatternParser::new()
        .parse(&mut errors,  lexer::Lexer::new("(baz)")
        ).is_ok());
    assert!(bluebell::ArgumentPatternParser::new()
        .parse(&mut errors,  lexer::Lexer::new("(Bar42 _)")
        ).is_ok());
    assert!(bluebell::ArgumentPatternParser::new()
        .parse(&mut errors,  lexer::Lexer::new("my_type")
        ).is_ok());

    assert!(bluebell::ArgumentPatternParser::new()
        .parse(&mut errors,  lexer::Lexer::new("2bar")
        ).is_err());
    assert!(bluebell::ArgumentPatternParser::new()
        .parse(&mut errors,  lexer::Lexer::new("MyType()")
        ).is_err());
    assert!(bluebell::ArgumentPatternParser::new()
        .parse(&mut errors,  lexer::Lexer::new("(2bar)")
        ).is_err());

}

#[test]
fn pattern_match_expression_clause() {
    let mut errors: Vec<lexer::ParseError> = [].to_vec();

    let lexer = lexer::Lexer::new("| Bar _ => Int32 -1");
    println!("Tokens {:?}", lexer.collect::<Vec<_>>());

    assert!(bluebell::PatternMatchExpressionClauseParser::new()
        .parse(&mut errors, lexer::Lexer::new("| _ => Uint32 42"))
        .is_ok());
    assert!(bluebell::PatternMatchExpressionClauseParser::new()
        .parse(&mut errors, lexer::Lexer::new("| Foo => Uint32 42"))
        .is_ok());
    assert!(bluebell::PatternMatchExpressionClauseParser::new()
        .parse(&mut errors, lexer::Lexer::new("| Foo _ Bar => Uint32 42"))
        .is_ok());
    assert!(bluebell::PatternMatchExpressionClauseParser::new()
        .parse(&mut errors, lexer::Lexer::new("| Bar _ => Uint32 42"))
        .is_ok());
    assert!(bluebell::PatternMatchExpressionClauseParser::new()
        .parse(&mut errors, lexer::Lexer::new("| Bar _ => Int32 -1"))
        .is_ok());    
    assert!(bluebell::PatternMatchExpressionClauseParser::new()
        .parse(
            &mut errors,
            lexer::Lexer::new("| Foo _ => let x = Uint32 1 in x")
        )
        .is_ok());

    assert!(bluebell::PatternMatchExpressionClauseParser::new()
        .parse(&mut errors, lexer::Lexer::new("| Foo =>"))
        .is_err());
    assert!(bluebell::PatternMatchExpressionClauseParser::new()
        .parse(&mut errors, lexer::Lexer::new("| => 42"))
        .is_err());
    assert!(bluebell::PatternMatchExpressionClauseParser::new()
        .parse(&mut errors, lexer::Lexer::new("| _ => "))
        .is_err());
    assert!(bluebell::PatternMatchExpressionClauseParser::new()
        .parse(&mut errors, lexer::Lexer::new("| () => 42"))
        .is_err());
    assert!(bluebell::PatternMatchExpressionClauseParser::new()
        .parse(&mut errors, lexer::Lexer::new("| Foo + Bar => 42"))
        .is_err());
}

#[test]
fn message_entry() {
    let mut errors: Vec<lexer::ParseError> = [].to_vec();

    assert!(bluebell::MessageEntryParser::new()
        .parse(&mut errors,  lexer::Lexer::new("foo: Uint32 42")
        ).is_ok());
    assert!(bluebell::MessageEntryParser::new()
        .parse(&mut errors,  lexer::Lexer::new("foo: bar")
        ).is_ok());
    assert!(bluebell::MessageEntryParser::new()
        .parse(&mut errors,  lexer::Lexer::new("foo: 0x1337")
        ).is_ok());

    assert!(bluebell::MessageEntryParser::new().parse(&mut errors,  lexer::Lexer::new("foo")).is_err());
    assert!(bluebell::MessageEntryParser::new()
        .parse(&mut errors,  lexer::Lexer::new("foo: bar: baz")
        ).is_err());
    assert!(bluebell::MessageEntryParser::new().parse(&mut errors,  lexer::Lexer::new(": 42")).is_err());
}

#[test]
fn test_type_annotation() {
    let mut errors: Vec<lexer::ParseError> = [].to_vec();

    assert!(bluebell::TypeAnnotationParser::new().parse(&mut errors,  lexer::Lexer::new(": Int")).is_ok());
    assert!(bluebell::TypeAnnotationParser::new()
        .parse(&mut errors,  lexer::Lexer::new(": MyCustomType")
        ).is_ok());
    assert!(bluebell::TypeAnnotationParser::new()
        .parse(&mut errors,  lexer::Lexer::new(": ByStr32")
        ).is_ok());
    assert!(bluebell::TypeAnnotationParser::new()
        .parse(&mut errors,  lexer::Lexer::new(": (Map ByStr32 Uint32)")
        ).is_ok());
    assert!(bluebell::TypeAnnotationParser::new()
        .parse(&mut errors,  lexer::Lexer::new(": (Map ByStr32 (Map ByStr32 Uint32))")
        ).is_ok());
    assert!(bluebell::TypeAnnotationParser::new()
        .parse(&mut errors,  lexer::Lexer::new(": (List MyCustomType)")
        ).is_ok());
    assert!(bluebell::TypeAnnotationParser::new()
        .parse(&mut errors,  lexer::Lexer::new(": (Pair MyCustomType1 MyCustomType2)")
        ).is_ok());
    assert!(bluebell::TypeAnnotationParser::new()
        .parse(&mut errors,  lexer::Lexer::new(": (Pair (Pair Int Bool) (List MyCustomType))")
        ).is_ok());

    assert!(bluebell::TypeAnnotationParser::new()
        .parse(&mut errors,  lexer::Lexer::new(": MyCustomType (")
        ).is_err());
    assert!(bluebell::TypeAnnotationParser::new()
        .parse(&mut errors,  lexer::Lexer::new(": (Map MyCustomType)")
        ).is_err());
    assert!(bluebell::TypeAnnotationParser::new()
        .parse(&mut errors,  lexer::Lexer::new(": (Pair MyCustomType1 MyCustomType2")
        ).is_err());
    assert!(bluebell::TypeAnnotationParser::new().parse(&mut errors,  lexer::Lexer::new("Int")).is_err());
    assert!(bluebell::TypeAnnotationParser::new().parse(&mut errors,  lexer::Lexer::new(": 42")).is_err());
}

#[test]
fn typed_identifier() {
    let mut errors: Vec<lexer::ParseError> = [].to_vec();

    assert!(bluebell::TypedIdentifierParser::new()
        .parse(&mut errors,  lexer::Lexer::new("foo: Int")
        ).is_ok());
    assert!(bluebell::TypedIdentifierParser::new()
        .parse(&mut errors,  lexer::Lexer::new("bar: ByStr20")
        ).is_ok());
    assert!(bluebell::TypedIdentifierParser::new()
        .parse(&mut errors,  lexer::Lexer::new("baz: (Int Bool)")
        ).is_ok());

    assert!(bluebell::TypedIdentifierParser::new()
        .parse(&mut errors,  lexer::Lexer::new("1foo: Int")
        ).is_err());
    assert!(bluebell::TypedIdentifierParser::new()
        .parse(&mut errors,  lexer::Lexer::new("foo: int")
        ).is_err());
    assert!(bluebell::TypedIdentifierParser::new()
        .parse(&mut errors,  lexer::Lexer::new("foo: (,)")
        ).is_err());
}

#[test]
fn test_statement() {
    let mut errors: Vec<lexer::ParseError> = [].to_vec();

    assert!(bluebell::StatementParser::new().parse(&mut errors,  lexer::Lexer::new("foo <- bar")).is_ok()); // REGULAR_ID '<-' variableIdentifier
    assert!(bluebell::StatementParser::new()
        .parse(&mut errors,  lexer::Lexer::new("remoteFetchStatement")
        ).is_ok()); // remoteFetchStatement
    assert!(bluebell::StatementParser::new().parse(&mut errors,  lexer::Lexer::new("foo := bar")).is_ok()); // REGULAR_ID ':=' variableIdentifier
    assert!(bluebell::StatementParser::new()
        .parse(&mut errors,  lexer::Lexer::new("foo = Uint32 42")
        ).is_ok()); // REGULAR_ID '=' fullExpression
    assert!(bluebell::StatementParser::new()
        .parse(&mut errors,  lexer::Lexer::new("foo <- &Event")
        ).is_ok()); // REGULAR_ID '<-' '&' (CUSTOM_TYPE_IDENTIFIER | BYSTR | 'Event') blockchainFetchArguments?
    assert!(bluebell::StatementParser::new()
        .parse(&mut errors,  lexer::Lexer::new("foo <- qux[baz]")
        ).is_ok()); // REGULAR_ID '<-' REGULAR_ID mapAccess+
    assert!(bluebell::StatementParser::new()
        .parse(&mut errors,  lexer::Lexer::new("foo <- exists baz[bar]")
        ).is_ok()); // REGULAR_ID '<-' 'exists' REGULAR_ID mapAccess+
    assert!(bluebell::StatementParser::new()
        .parse(&mut errors,  lexer::Lexer::new("foo[bar] := qux")
        ).is_ok()); // REGULAR_ID mapAccess+ ':=' variableIdentifier
    assert!(bluebell::StatementParser::new()
        .parse(&mut errors,  lexer::Lexer::new("delete foo[bar]")
        ).is_ok()); // 'delete' REGULAR_ID mapAccess+
    assert!(bluebell::StatementParser::new().parse(&mut errors,  lexer::Lexer::new("accept")).is_ok()); // 'accept'
    assert!(bluebell::StatementParser::new().parse(&mut errors,  lexer::Lexer::new("send foo")).is_ok()); // 'send' variableIdentifier
    assert!(bluebell::StatementParser::new().parse(&mut errors,  lexer::Lexer::new("event foo")).is_ok()); // 'event' variableIdentifier
    assert!(bluebell::StatementParser::new().parse(&mut errors,  lexer::Lexer::new("throw")).is_ok()); // 'throw' variableIdentifier?
    assert!(bluebell::StatementParser::new()
        .parse(&mut errors,  lexer::Lexer::new("match foo with | False => True | _ => False end")
        ).is_ok()); // 'match' variableIdentifier 'with' patternMatchClause 'end'
    assert!(bluebell::StatementParser::new()
        .parse(&mut errors,  lexer::Lexer::new("match foo with | _ => value end")
        ).is_ok()); // 'match' variableIdentifier 'with' patternMatchClause 'end'
    assert!(bluebell::StatementParser::new()
        .parse(&mut errors,  lexer::Lexer::new("Foo bar baz")
        ).is_ok()); // componentId variableIdentifier*
    assert!(bluebell::StatementParser::new()
        .parse(&mut errors,  lexer::Lexer::new("forall foo Event")
        ).is_ok()); // 'forall' variableIdentifier componentId

    assert!(bluebell::StatementParser::new().parse(&mut errors,  lexer::Lexer::new("foo < bar")).is_err()); // should be '<-'
    assert!(bluebell::StatementParser::new().parse(&mut errors,  lexer::Lexer::new("42 = foo")).is_err()); // invalid order of operands
    assert!(bluebell::StatementParser::new().parse(&mut errors,  lexer::Lexer::new("&Event")).is_err()); // missing identifier
    assert!(bluebell::StatementParser::new()
        .parse(&mut errors,  lexer::Lexer::new("foo[] <- bar")
        ).is_err()); // missing key
    assert!(bluebell::StatementParser::new()
        .parse(&mut errors,  lexer::Lexer::new("foo <- exists")
        ).is_err()); // missing identifier after 'exists'
    assert!(bluebell::StatementParser::new()
        .parse(&mut errors,  lexer::Lexer::new("foo[] := bar")
        ).is_err()); // missing key
    assert!(bluebell::StatementParser::new()
        .parse(&mut errors,  lexer::Lexer::new("foo := qux[bar][baz]")
        ).is_err()); // wrong assignment operator
    assert!(bluebell::StatementParser::new()
        .parse(&mut errors,  lexer::Lexer::new("foo.delete[bar]")
        ).is_err()); // missing operator between variable and method call
    assert!(bluebell::StatementParser::new().parse(&mut errors,  lexer::Lexer::new("send")).is_err()); // missing identifier
    assert!(bluebell::StatementParser::new().parse(&mut errors,  lexer::Lexer::new("event")).is_err()); // missing identifier
    assert!(bluebell::StatementParser::new()
        .parse(&mut errors,  lexer::Lexer::new("match with _ => 42 end")
        ).is_err()); // missing variable
    assert!(bluebell::StatementParser::new().parse(&mut errors,  lexer::Lexer::new("forall")).is_err()); // missing variable and component ID
}

#[test]
fn blockchain_fetch_arguments() {
    let mut errors: Vec<lexer::ParseError> = [].to_vec();

    assert!(bluebell::BlockchainFetchArgumentsParser::new()
        .parse(&mut errors,  lexer::Lexer::new("(foo bar)")
        ).is_ok());
    assert!(bluebell::BlockchainFetchArgumentsParser::new()
        .parse(&mut errors,  lexer::Lexer::new("(x)")
        ).is_ok());
    assert!(bluebell::BlockchainFetchArgumentsParser::new()
        .parse(&mut errors,  lexer::Lexer::new("(y z a)")
        ).is_ok());

    assert!(bluebell::BlockchainFetchArgumentsParser::new()
        .parse(&mut errors,  lexer::Lexer::new("()")
        ).is_err());
    assert!(bluebell::BlockchainFetchArgumentsParser::new()
        .parse(&mut errors,  lexer::Lexer::new("(123)")
        ).is_err());
    assert!(bluebell::BlockchainFetchArgumentsParser::new()
        .parse(&mut errors,  lexer::Lexer::new("(foo, 123)")
        ).is_err());
    assert!(bluebell::BlockchainFetchArgumentsParser::new()
        .parse(&mut errors,  lexer::Lexer::new("foo, bar")
        ).is_err());
    assert!(bluebell::BlockchainFetchArgumentsParser::new()
        .parse(&mut errors,  lexer::Lexer::new("(foo; bar)")
        ).is_err());
    assert!(bluebell::BlockchainFetchArgumentsParser::new()
        .parse(&mut errors,  lexer::Lexer::new("foo")
        ).is_err());
}

#[test]
fn statement_block() {
    let mut errors: Vec<lexer::ParseError> = [].to_vec();

    assert!(bluebell::StatementBlockParser::new()
        .parse(&mut errors,  lexer::Lexer::new("x <- y; z := a")
        ).is_ok());
    assert!(bluebell::StatementBlockParser::new()
        .parse(&mut errors,  lexer::Lexer::new("accept")
        ).is_ok());
    assert!(bluebell::StatementBlockParser::new()
        .parse(&mut errors,  lexer::Lexer::new("send x")
        ).is_ok());
    assert!(bluebell::StatementBlockParser::new()
        .parse(&mut errors,  lexer::Lexer::new("event myEvent")
        ).is_ok());
    assert!(bluebell::StatementBlockParser::new()
        .parse(&mut errors,  lexer::Lexer::new("match x with | _ => accept end")
        ).is_ok());
    assert!(bluebell::StatementBlockParser::new()
        .parse(&mut errors,  lexer::Lexer::new("match x with | _ => y <- z end")
        ).is_ok());
    assert!(bluebell::StatementBlockParser::new()
        .parse(&mut errors,  lexer::Lexer::new("MyComponent y")
        ).is_ok());
    assert!(bluebell::StatementBlockParser::new()
        .parse(&mut errors,  lexer::Lexer::new("MyComponent y; forall foo Event ; match x with | _ => y <- z end")
        ).is_ok());

    assert!(bluebell::StatementBlockParser::new()
        .parse(&mut errors,  lexer::Lexer::new("x < y")
        ).is_err());
    assert!(bluebell::StatementBlockParser::new().parse(&mut errors,  lexer::Lexer::new("x <-")).is_err());
    assert!(bluebell::StatementBlockParser::new()
        .parse(&mut errors,  lexer::Lexer::new("accept event")
        ).is_err());
    assert!(bluebell::StatementBlockParser::new().parse(&mut errors,  lexer::Lexer::new("send")).is_err());
    assert!(bluebell::StatementBlockParser::new()
        .parse(&mut errors,  lexer::Lexer::new("match x with _ => accept end")
        ).is_err());
    assert!(bluebell::StatementBlockParser::new()
        .parse(&mut errors,  lexer::Lexer::new("MyComponent X")
        ).is_err());
    assert!(bluebell::StatementBlockParser::new()
        .parse(&mut errors,  lexer::Lexer::new("MyComponent y forall x")
        ).is_err());
}

#[test]
fn parameter_pair() {
    let mut errors: Vec<lexer::ParseError> = [].to_vec();

    assert!(bluebell::ParameterPairParser::new()
        .parse(&mut errors,  lexer::Lexer::new("foo: Uint32")
        ).is_ok());
    assert!(bluebell::ParameterPairParser::new()
        .parse(&mut errors,  lexer::Lexer::new("bar: Bool")
        ).is_ok());
    assert!(bluebell::ParameterPairParser::new()
        .parse(&mut errors,  lexer::Lexer::new("baz: Address")
        ).is_ok());
    assert!(bluebell::ParameterPairParser::new()
        .parse(&mut errors,  lexer::Lexer::new("qux: Map Uint32 Bool")
        ).is_ok());

    assert!(bluebell::ParameterPairParser::new()
        .parse(&mut errors,  lexer::Lexer::new("foo Uint32")
        ).is_err());
    assert!(bluebell::ParameterPairParser::new().parse(&mut errors,  lexer::Lexer::new("foo")).is_err());
    assert!(bluebell::ParameterPairParser::new()
        .parse(&mut errors,  lexer::Lexer::new("123: Uint32")
        ).is_err());
    assert!(bluebell::ParameterPairParser::new()
        .parse(&mut errors,  lexer::Lexer::new("foo: bar: Uint32")
        ).is_err());
    assert!(bluebell::ParameterPairParser::new()
        .parse(&mut errors,  lexer::Lexer::new("foo: uint32")
        ).is_err());
    assert!(bluebell::ParameterPairParser::new()
        .parse(&mut errors,  lexer::Lexer::new("foo: mapUint32, Bool")
        ).is_err());
}

#[test]
fn component_definition() {
    let mut errors: Vec<lexer::ParseError> = [].to_vec();

    assert!(bluebell::ComponentDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("transition myTransition(param1: Uint32, param2: Uint32) end")
        ).is_ok());
    assert!(bluebell::ComponentDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("procedure myProcedure(param: Uint32) end")
        ).is_ok());
    assert!(bluebell::ComponentDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("procedure myProcedure(param: Map ByStr32 ByStr32) param end")
        ).is_ok());
    assert!(bluebell::ComponentDefinitionParser::new().parse(&mut errors,  lexer::Lexer::new("transition myTransition(param: Bool) match param with | False => True | _ => False end end")).is_ok());

    assert!(bluebell::ComponentDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("transition myTransition")
        ).is_err());
    assert!(bluebell::ComponentDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("procedure myProcedure() returns Uint32")
        ).is_err());
    assert!(bluebell::ComponentDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("procedure myProcedure(param: Uint32) returns Uint32 {")
        ).is_err());
    assert!(bluebell::ComponentDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("transition myTransition() { state_1 -> state_2 }")
        ).is_err());
    assert!(bluebell::ComponentDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("transition myTransition(param1: Uint32 param2: Uint32)")
        ).is_err());
}

#[test]
fn procedure_definition() {
    let mut errors: Vec<lexer::ParseError> = [].to_vec();

    assert!(bluebell::ProcedureDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("procedure foo() end")
        ).is_ok());
    assert!(bluebell::ProcedureDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("procedure bar(x: Int32, y: Uint32) baz x y end")
        ).is_ok());

    assert!(bluebell::ProcedureDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("procedure 42() { }")
        ).is_err());
    assert!(bluebell::ProcedureDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("procedure foo(x, y) { }")
        ).is_err());
    assert!(bluebell::ProcedureDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("procedure foo(x: Int32, y: Uint32)")
        ).is_err());
    assert!(bluebell::ProcedureDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("procedure foo(x: Int32, y: Uint32) foo x y")
        ).is_err());
    assert!(bluebell::ProcedureDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("procedure foo() {}")
        ).is_err());
}

#[test]
fn transition_definition() {
    let mut errors: Vec<lexer::ParseError> = [].to_vec();

    assert!(bluebell::TransitionDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("transition foo()  bar end")
        ).is_ok());
    assert!(bluebell::TransitionDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("transition bar(x: Int32, y: Uint32) foo end")
        ).is_ok());
    assert!(bluebell::TransitionDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("transition qux(bar: Bool) bar end")
        ).is_ok());
    assert!(bluebell::TransitionDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("transition empty() end")
        ).is_ok());

    assert!(bluebell::TransitionDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("transition 123() { foo() }")
        ).is_err());
    assert!(bluebell::TransitionDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("transition foo(bar) { foo() }")
        ).is_err());
    assert!(bluebell::TransitionDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("transition foo() { foo(); bar() }")
        ).is_err());
    assert!(bluebell::TransitionDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("transaction foo() { bar() }")
        ).is_err());
    assert!(bluebell::TransitionDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("transition foo() { bar }")
        ).is_err());
    assert!(bluebell::TransitionDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("transition foo() { bar( }")
        ).is_err());
    assert!(bluebell::TransitionDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("transition foo() { bar() };")
        ).is_err());
    assert!(bluebell::TransitionDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("transition foo() { ; }")
        ).is_err());
}

#[test]
fn component_id() {
    let mut errors: Vec<lexer::ParseError> = [].to_vec();

    assert!(bluebell::ComponentIdParser::new().parse(&mut errors,  lexer::Lexer::new("MyType")).is_ok());
    // TODO: assert!(bluebell::ComponentIdParser::new().parse(&mut errors,  lexer::Lexer::new("Event42")).is_ok());
    assert!(bluebell::ComponentIdParser::new().parse(&mut errors,  lexer::Lexer::new("Foo_Bar")).is_ok());
    assert!(bluebell::ComponentIdParser::new().parse(&mut errors,  lexer::Lexer::new("ByStr42")).is_ok());
    assert!(bluebell::ComponentIdParser::new()
        .parse(&mut errors,  lexer::Lexer::new("regular_id")
        ).is_ok());

    assert!(bluebell::ComponentIdParser::new().parse(&mut errors,  lexer::Lexer::new("42Event")).is_err());
    assert!(bluebell::ComponentIdParser::new().parse(&mut errors,  lexer::Lexer::new("my type")).is_err());
    assert!(bluebell::ComponentIdParser::new().parse(&mut errors,  lexer::Lexer::new("event+")).is_err());
    assert!(bluebell::ComponentIdParser::new().parse(&mut errors,  lexer::Lexer::new("ByStr")).is_err());
}

#[test]
fn component_parameters() {

    let mut errors: Vec<lexer::ParseError> = [].to_vec();

    assert!(bluebell::ComponentParametersParser::new()
        .parse(&mut errors,  lexer::Lexer::new("()")
        ).is_ok());
    assert!(bluebell::ComponentParametersParser::new()
        .parse(&mut errors,  lexer::Lexer::new("(a: Int32)")
        ).is_ok());
    assert!(bluebell::ComponentParametersParser::new()
        .parse(&mut errors,  lexer::Lexer::new("(a: Int32, b: Bool)")
        ).is_ok());
    assert!(bluebell::ComponentParametersParser::new()
        .parse(&mut errors,  lexer::Lexer::new("(a: Int32, b: Bool, c: String)")
        ).is_ok());
    assert!(bluebell::ComponentParametersParser::new()
        .parse(&mut errors,  lexer::Lexer::new("(a: ByStr20, b: Map ByStr20 (Uint256))")
        ).is_ok());

    assert!(bluebell::ComponentParametersParser::new()
        .parse(&mut errors,  lexer::Lexer::new("a: Int32, b: Bool")
        ).is_err());
    assert!(bluebell::ComponentParametersParser::new()
        .parse(&mut errors,  lexer::Lexer::new("(a: Int32")
        ).is_err());
    assert!(bluebell::ComponentParametersParser::new()
        .parse(&mut errors,  lexer::Lexer::new("(a: Int32,, b: Bool)")
        ).is_err());
    assert!(bluebell::ComponentParametersParser::new()
        .parse(&mut errors,  lexer::Lexer::new("(())")
        ).is_err());
    assert!(bluebell::ComponentParametersParser::new()
        .parse(&mut errors,  lexer::Lexer::new("(a: )")
        ).is_err());
}

#[test]
fn component_body() {

        let mut errors: Vec<lexer::ParseError> = [].to_vec();

        assert!(bluebell::ComponentBodyParser::new()
            .parse(&mut errors,  lexer::Lexer::new(
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
            )
            ).is_ok());

        assert!(bluebell::ComponentBodyParser::new()
            .parse(&mut errors,  lexer::Lexer::new(
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
            )
            ).is_ok());

        assert!(bluebell::ComponentBodyParser::new().parse(&mut errors,  lexer::Lexer::new("
        current_init <-& init.dApp;
        xPointsDApp = \"xpoints\"; get_addr <-& current_init.dns[xPointsDApp]; addr = option_bystr20_value get_addr;
        is_xPoints = builtin eq _sender addr; match is_xPoints with
        | True => | False => e = { _exception : \"donate.tyron-WrongCaller\" }; throw e end;
        get_xPoints <- xpoints[_origin]; x_points = option_uint128_value get_xPoints; IsSufficient x_points amount;
        new_bal = builtin sub x_points amount; xpoints[_origin] := new_bal end
        ")).is_ok());
}

#[test]
fn contract_fields() {
    let mut errors: Vec<lexer::ParseError> = [].to_vec();

    assert!(bluebell::ContractFieldParser::new()
        .parse(&mut errors,  lexer::Lexer::new("field foo: Int32 = Int32 42")
        ).is_ok());
    assert!(bluebell::ContractFieldParser::new()
        .parse(&mut errors,  lexer::Lexer::new("field bar: Map ByStr32 (List Uint32) = Emp ByStr32 (List Uint32)")
        ).is_ok());
    assert!(bluebell::ContractFieldParser::new()
        .parse(&mut errors,  lexer::Lexer::new("field baz: Event = Event")
        ).is_ok());

    assert!(bluebell::ContractFieldParser::new()
        .parse(&mut errors,  lexer::Lexer::new("field: Foo = Bar")
        ).is_err());
    assert!(bluebell::ContractFieldParser::new()
        .parse(&mut errors,  lexer::Lexer::new("field bar = 42")
        ).is_err());
    assert!(bluebell::ContractFieldParser::new()
        .parse(&mut errors,  lexer::Lexer::new("field baz: Event = 42")
        ).is_err());
    assert!(bluebell::ContractFieldParser::new()
        .parse(&mut errors,  lexer::Lexer::new("field qux = 'hello world'")
        ).is_err());
}

#[test]
fn test_with_constraint() {
    let mut errors: Vec<lexer::ParseError> = [].to_vec();

    assert!(bluebell::WithConstraintParser::new()
        .parse(&mut errors,  lexer::Lexer::new("with builtin blt end_of_life =>")
        ).is_ok());
    assert!(bluebell::WithConstraintParser::new()
        .parse(&mut errors,  lexer::Lexer::new("with builtin add {UInt32} one  =>")
        ).is_ok());
    assert!(bluebell::WithConstraintParser::new()
        .parse(&mut errors,  lexer::Lexer::new("with true =>")
        ).is_ok());
    assert!(bluebell::WithConstraintParser::new()
        .parse(&mut errors,  lexer::Lexer::new("with variableIdentifier =>")
        ).is_ok());

    assert!(bluebell::WithConstraintParser::new().parse(&mut errors,  lexer::Lexer::new("foo")).is_err());
    assert!(bluebell::WithConstraintParser::new()
        .parse(&mut errors,  lexer::Lexer::new("with variableIdentifier => foo")
        ).is_err());
    assert!(bluebell::WithConstraintParser::new()
        .parse(&mut errors,  lexer::Lexer::new("with =>")
        ).is_err());
    assert!(bluebell::WithConstraintParser::new().parse(&mut errors,  lexer::Lexer::new("with")).is_err());
}

#[test]
fn contract_definition() {

    let mut errors: Vec<lexer::ParseError> = [].to_vec();

    assert!(bluebell::ContractDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("contract MyContract()")
        ).is_ok());
    assert!(bluebell::ContractDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("contract MyContract(address: ByStr20)")
        ).is_ok());
    assert!(bluebell::ContractDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("contract MyContract(address: ByStr20) with true =>")
        ).is_ok());
    assert!(bluebell::ContractDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("contract MyContract(address: ByStr20) with true => field field1: Uint32 = Uint32 1")
        ).is_ok());
    assert!(bluebell::ContractDefinitionParser::new().parse(&mut errors,  lexer::Lexer::new("contract MyContract(address: ByStr20) with true => field field1: Uint32 = Uint32 1 transition a() end")).is_ok());
    assert!(bluebell::ContractDefinitionParser::new().parse(&mut errors,  lexer::Lexer::new("contract MyContract(address: ByStr20) with true => field field1: Uint32 = Uint32 1 procedure a() end")).is_ok());
    assert!(bluebell::ContractDefinitionParser::new().parse(&mut errors,  lexer::Lexer::new(
        r#"contract MyContract(address: ByStr20) with true => field field1: Uint32 = Uint32 1 procedure RequireNotSelf(address: ByStr20)
        is_self = builtin eq address _sender;
        match is_self with
        | False =>
        | True =>
            error = SelfError;
            Throw error
        end
        end"#)).is_ok());

    assert!(bluebell::ContractDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("contract MyContract")
        ).is_err());
    assert!(bluebell::ContractDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("contract MyContract end")
        ).is_err());
    assert!(bluebell::ContractDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("Contract MyContract")
        ).is_err());
    assert!(bluebell::ContractDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("contract")
        ).is_err());
    assert!(bluebell::ContractDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("contract MyContract(address: ByStr20,) with (true =>)")
        ).is_err());
    assert!(bluebell::ContractDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("contract MyContract(address: ByStr20) with true => field field1 = 1")
        ).is_err());
    assert!(bluebell::ContractDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("contract MyContract(address: ByStr20) with true => field field1: = 1")
        ).is_err());
    assert!(bluebell::ContractDefinitionParser::new().parse(&mut errors,  lexer::Lexer::new(
        "contract MyContract(address: ByStr20) with true => field field1: Uint32 = 1 field2: 10")).is_err());
    assert!(bluebell::ContractDefinitionParser::new().parse(&mut errors,  lexer::Lexer::new(
        "contract MyContract(address: ByStr20) with true => field field1: Uint32 = 1 transition a(Void) {}")).is_err());
    assert!(bluebell::ContractDefinitionParser::new().parse(&mut errors,  lexer::Lexer::new(
        "contract MyContract(address: ByStr20) with true => field field1: Uint32 = 1 procedure a(Void) {")).is_err());

}

#[test]
fn type_alternative_clause() {
    let mut errors: Vec<lexer::ParseError> = [].to_vec();

    assert!(bluebell::TypeAlternativeClauseParser::new()
        .parse(&mut errors, lexer::Lexer::new("| MyType"))
        .is_ok());
    assert!(bluebell::TypeAlternativeClauseParser::new()
        .parse(&mut errors, lexer::Lexer::new("| MyType of Int"))
        .is_ok());
    assert!(bluebell::TypeAlternativeClauseParser::new()
        .parse(
            &mut errors,
            lexer::Lexer::new("| ByStr123 of Map MyType Int")
        )
        .is_ok());

    assert!(bluebell::TypeAlternativeClauseParser::new()
        .parse(&mut errors, lexer::Lexer::new("| MyType of"))
        .is_err());
    assert!(bluebell::TypeAlternativeClauseParser::new()
        .parse(&mut errors, lexer::Lexer::new("| 123MyType"))
        .is_err());
    assert!(bluebell::TypeAlternativeClauseParser::new()
        .parse(&mut errors, lexer::Lexer::new("| ByStr"))
        .is_err());
}

#[test]
fn library_single_definition() {
    let mut errors: Vec<lexer::ParseError> = [].to_vec();

    assert!(bluebell::LibrarySingleDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("let foo = Int32 42")
        ).is_ok());
    assert!(bluebell::LibrarySingleDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("let foo: Int32 = Int32 42")
        ).is_ok());
    assert!(bluebell::LibrarySingleDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("type Foo")
        ).is_ok());
    assert!(bluebell::LibrarySingleDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("type Foo = | Bar | Baz")
        ).is_ok());

    assert!(bluebell::LibrarySingleDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("let = Int32 42")
        ).is_err());
    assert!(bluebell::LibrarySingleDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("let foo: = 42")
        ).is_err());
    assert!(bluebell::LibrarySingleDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("type Int32 42")
        ).is_err());
    assert!(bluebell::LibrarySingleDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("type Foo = | Bar Baz")
        ).is_err());
}

#[test]
fn library_definition() {
    let mut errors: Vec<lexer::ParseError> = [].to_vec();

    assert!(bluebell::LibraryDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("library Foo")
        ).is_ok());
    assert!(bluebell::LibraryDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("library Bar let x = Int32 10")
        ).is_ok());
    assert!(bluebell::LibraryDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("library Baz type Quux")
        ).is_ok());
    assert!(bluebell::LibraryDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("library Qux type Quux = | Event")
        ).is_ok());
    assert!(bluebell::LibraryDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("library Quux type Quux = | Event of Uint256")
        ).is_ok());
    assert!(bluebell::LibraryDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("library Quuz type Quux = | Event of Uint256 | AnotherEvent of ByStr20")
        ).is_ok());
    assert!(bluebell::LibraryDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("library Qoorx let x: Int32 = Int32 42")
        ).is_ok());

    assert!(bluebell::LibraryDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("library Foo Bar")
        ).is_err());
    assert!(bluebell::LibraryDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("library")
        ).is_err());
    assert!(bluebell::LibraryDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("library Foo bar")
        ).is_err());
    assert!(bluebell::LibraryDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("library Foo type")
        ).is_err());
    assert!(bluebell::LibraryDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("library Foo type = | Event")
        ).is_err());
    assert!(bluebell::LibraryDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("library Foo type Quux =")
        ).is_err());
    assert!(bluebell::LibraryDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("library Foo type Quux = |")
        ).is_err());
    assert!(bluebell::LibraryDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("library Foo type Quux = | Event of")
        ).is_err());
    assert!(bluebell::LibraryDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("library Foo let")
        ).is_err());
    assert!(bluebell::LibraryDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("library Foo let x")
        ).is_err());
    assert!(bluebell::LibraryDefinitionParser::new()
        .parse(&mut errors,  lexer::Lexer::new("library Foo let = 42")
        ).is_err());
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
    // for token in lexer {
    //     println!("{:?}", token);
    // }

    
    let parser = bluebell::ProgramParser::new();
    match parser.parse(&mut errors,  lexer::Lexer::new(&script)) {
        Ok(ast) => println!("{:?}", ast),
        Err(error) => {
            let message = format!("Syntax error {:?}", error);
            let mut pos : Vec<usize> = [].to_vec();
            error.map_location(|l|{
                pos.push(l);
             l });

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
                    }
                    else {
                        line_start = n + 1;
                    }
                }
                if !should_stop && n == pos[0] {
                    should_stop = true;
                }

                n += 1;
                if !should_stop
                {
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
            print!("{}", " ".repeat(char_counter+ format!("Line {},{}:", line_counter, char_counter).len()));
            println!("{}", "^".repeat(pos[1] - pos[0]));

            let my_error = ParserError {
                message,
                line:  0, //error.location_line(),
                column: 0, // err.location_column(),
            };
            println!("{}", my_error);

            process::exit(-1);
        }
    }

}
