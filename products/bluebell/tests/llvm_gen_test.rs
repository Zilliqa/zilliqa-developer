#[cfg(test)]
mod tests {
    extern crate diffy;
    use bluebell::highlevel_ir::IrLowering;
    use bluebell::highlevel_ir_debug_printer::HighlevelIrDebugPrinter;
    use bluebell::highlevel_ir_emitter::HighlevelIrEmitter;
    use bluebell::highlevel_ir_pass_executor::HighlevelIrPassExecutor;
    use bluebell::highlevel_ir_type_collection::HighlevelIrTypeCollection;

    use bluebell::llvm_ir_generator::LlvmIrGenerator;

    use bluebell::lexer;
    use bluebell::lexer::Lexer;
    use bluebell::parser;
    use bluebell::ParserError;
    use bluebell::*;
    use diffy::{create_patch, PatchFormatter};
    use inkwell::context::Context;
    use std::fs;
    use std::fs::File;
    use std::io::Read;
    use std::process::Command;

    fn strip_comments(input: &str) -> String {
        let re = regex::Regex::new(r"[ ]*\(\*([^*]|\*+[^*)])*\*+\)\n*").unwrap();
        let result = re.replace_all(input, "");
        result.to_string()
    }

    fn parse_and_emit(path: String) -> bool {
        let mut file = File::open(&path).expect("Unable to open file");
        let mut script = String::new();
        file.read_to_string(&mut script)
            .expect("Unable to read file");
        let lexer = Lexer::new(&script);

        let mut errors: Vec<lexer::ParseError> = [].to_vec();
        let parser = parser::ProgramParser::new();
        assert!(errors.len() == 0);

        match parser.parse(&mut errors, lexer) {
            Ok(ast) => {
                let mut generator = HighlevelIrEmitter::new();
                let mut ast2 = ast.clone();
                // println!("AST: {:#?}\n\n", ast2);
                let mut ir = generator
                    .emit(&mut ast2)
                    .expect("Failed generating highlevel IR");

                // let context = Context::create();
                // let mut generator = LlvmIrGenerator::new(&context, ir);
                // generator.write_function_definitions_to_module();

                let mut type_collector = HighlevelIrTypeCollection::new();
                ir.visit(&mut type_collector);

                // println!("\n\nDefined types:\n{:#?}\n\n", ir.type_definitions);
                // println!("\n\nDefined functions:\n{:#?}\n\n", ir.function_definitions);

                let mut debug_printer = HighlevelIrDebugPrinter::new();
                ir.visit(&mut debug_printer);

                // let context = Context::create();
                // let mut generator = LlvmIrGenerator::new(&context, ir);
                // generator.write_function_definitions_to_module();

                assert!(false);
                true
            }
            Err(error) => {
                let ret = error.clone();
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
                    " ".repeat(
                        char_counter + format!("Line {},{}:", line_counter, char_counter).len()
                    )
                );
                println!("{}", "^".repeat(pos[1] - pos[0]));

                let my_error = ParserError {
                    message,
                    line: 0,   //error.location_line(),
                    column: 0, // err.location_column(),
                };
                println!("{}", my_error);

                false
            }
        }
    }

    #[test]
    fn test_scilla_files() {
        let mut success = true;

        let mut entries: Vec<_> = fs::read_dir("./tests/data/llvm/gold")
            .expect("read_dir call failed")
            .collect::<Result<_, _>>()
            .expect("Failed to collect directory entries");
        entries.sort_by_key(|entry| entry.path().to_string_lossy().into_owned());
        for entry in entries {
            let path = entry.path();
            let path_str = path.to_string_lossy().into_owned();

            // Run the .scilla file using your Scilla execution command
            println!("- Compiling to LLVM IR {}", path_str);
            let ret = parse_and_emit(path_str);
            success = ret && success;
        }

        assert!(success)
    }
}
