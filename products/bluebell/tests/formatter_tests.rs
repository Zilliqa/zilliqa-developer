use bluebell::{ast::*, formatter::ScillaFormatter, lexer::Lexer, lexer::*, type_inference::*};
#[cfg(test)]
mod tests {
    use bluebell::formatter::ScillaFormatter;
    use bluebell::lexer::Lexer;
    use bluebell::lexer::*;

    use std::collections::HashMap;
    use std::env;
    use std::fs::File;
    use std::io::Read;
    use std::process;

    macro_rules! get_ast {
        ($parser:ty, $result:expr) => {{
            let mut errors = vec![];
            let ast = <$parser>::new().parse(&mut errors, crate::lexer::Lexer::new($result));
            match ast {
                Ok(parsed_ast) => parsed_ast,
                Err(err) => panic!("Parsing error: {:?}", err),
            }
        }};
    }

    #[test]
    fn bytestring_formatter() {
        let ast = get_ast!(bluebell::ByteStringParser, "ByStr1234");
        println!("{}", ast.to_string());
    }
}
