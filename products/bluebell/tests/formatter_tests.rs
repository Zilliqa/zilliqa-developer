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

    #[test]
    fn bytestring_formatter() {
        let ast = get_ast!(parser::ByteStringParser, "ByStr1234");
        println!("{}", ast.to_string());
    }
}
