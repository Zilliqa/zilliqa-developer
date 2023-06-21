use std::env;
use std::fs::File;
use std::io::Read;
use std::process;

use bluebell::formatter::ScillaFormatter;
use bluebell::lexer::Lexer;
use bluebell::ng_formatter::ScillaCodeEmitter;
use bluebell::ParserError;
use bluebell::*;

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

    let parser = parser::ProgramParser::new();
    match parser.parse(&mut errors, lexer) {
        Ok(ast) => {
            //let _inferred_types = infer_types(&ast).unwrap();
            let formatted_ast = ast.to_string(); // Call to_string on the top-level AST node to get formatted output
            println!("{}", formatted_ast);

            let mut formatter = ScillaCodeEmitter::new();
            let mut ast2 = ast.clone();
            formatter.emit(&mut ast2);
        }
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
