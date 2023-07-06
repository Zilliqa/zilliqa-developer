#[cfg(test)]
mod tests {
    extern crate diffy;
    use bluebell::contract_executor::{UnsafeContractExecutor, UnsafeLlvmTestExecutor};
    use bluebell::highlevel_ir_debug_printer::HighlevelIrDebugPrinter;
    use bluebell::highlevel_ir_emitter::HighlevelIrEmitter;
    use bluebell::highlevel_ir_pass_executor::HighlevelIrPassExecutor;
    use bluebell::intermediate_name_generator::IntermediateNameGenerator;
    use bluebell::llvm_ir_generator::LlvmIrGenerator;
    use bluebell::passes::annotate_base_types::AnnotateBaseTypes;
    use bluebell::passes::collect_type_definitions::CollectTypeDefinitionsPass;
    use bluebell::symbol_table::SymbolTable;
    use inkwell::context::Context;
    use inkwell::targets::{InitializationConfig, Target};
    use inkwell::OptimizationLevel;

    use bluebell::lexer;
    use bluebell::lexer::Lexer;
    use bluebell::parser;
    use bluebell::ParserError;

    use std::fs;
    use std::fs::File;
    use std::io::Read;

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
                let mut name_generator = IntermediateNameGenerator::new();

                let mut generator = HighlevelIrEmitter::new(&mut name_generator);
                let mut ast2 = ast.clone();
                // println!("AST: {:#?}\n\n", ast2);
                let mut ir = generator
                    .emit(&mut ast2)
                    .expect("Failed generating highlevel IR");

                // let context = Context::create();
                // let mut generator = LlvmIrGenerator::new(&context, ir);
                // generator.write_function_definitions_to_module();

                let mut symbol_table = SymbolTable::new(&mut name_generator);

                let mut type_collector = CollectTypeDefinitionsPass::new(&mut symbol_table);
                if let Err(err) = ir.visit(&mut type_collector) {
                    panic!("{}", err);
                }

                let mut type_annotator = AnnotateBaseTypes::new(&mut symbol_table);
                if let Err(err) = ir.visit(&mut type_annotator) {
                    panic!("{}", err);
                }

                // println!("\n\nDefined types:\n{:#?}\n\n", ir.type_definitions);
                // println!("\n\nDefined functions:\n{:#?}\n\n", ir.function_definitions);

                let mut debug_printer = HighlevelIrDebugPrinter::new();
                let _ = ir.visit(&mut debug_printer);

                let context = Context::create();
                let mut generator = LlvmIrGenerator::new(&context, ir);
                /*
                let module = match generator.build_module() {
                    Err(e) => {
                        panic!("Error: {:?}",e);
                    }
                    Ok(module) => {
                        let llvm_str = module.print_to_string();
                        let _output = llvm_str.to_str().expect("Failed converting to UTF8");
                        // println!("{}", output);
                        module
                    }
                };
                */

                println!("A");
                println!("B");
                extern "C" fn sumf(a: f64, b: f64) -> f64 {
                    a + b
                }

                Target::initialize_native(&InitializationConfig::default()).unwrap();

                let mut module = context.create_module("test");
                let builder = context.create_builder();

                let ft = context.f64_type();
                let fnt = ft.fn_type(&[], false);

                let f = module.add_function("test_fn", fnt, None);
                let b = context.append_basic_block(f, "entry");

                builder.position_at_end(b);

                let extf =
                    module.add_function("sumf", ft.fn_type(&[ft.into(), ft.into()], false), None);

                let argf = ft.const_float(64.);
                let call_site_value = builder.build_call(extf, &[argf.into(), argf.into()], "retv");
                let retv = call_site_value
                    .try_as_basic_value()
                    .left()
                    .unwrap()
                    .into_float_value();

                builder.build_return(Some(&retv));

                let llvm_str = module.print_to_string();
                let output = llvm_str.to_str().expect("Failed converting to UTF8");
                println!("{}", output);

                let contract_executor = UnsafeLlvmTestExecutor::new(&mut module);
                unsafe {
                    contract_executor.link_symbol("sumf", sumf as usize);
                    let result = contract_executor.execute("test_fn");
                    println!("{:?}", result);
                }
                /*
                let ee = module.create_jit_execution_engine(OptimizationLevel::None).unwrap();
                ee.add_global_mapping(&extf, sumf as usize);

                let result = unsafe { ee.run_function(f, &[]) }.as_float(&ft);
                */
                // let result = unsafe { execution_engine.run_function(f, &[]) }.as_float(&float32);
                // println!("Result: {:?}", result);

                assert!(false);
                true
            }
            Err(error) => {
                let _ret = error.clone();
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
