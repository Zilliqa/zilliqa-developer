use std::ffi::CStr;

use clap::{Parser, Subcommand};
use inkwell::context::Context;
use inkwell::targets::{InitializationConfig, Target};
use std::fs::File;
use std::io::Read;
use std::os::raw::c_char;
use std::process;

use bluebell::ast::NodeProgram;
use bluebell::contract_executor::{UnsafeContractExecutor, UnsafeLlvmTestExecutor};
use bluebell::highlevel_ir_debug_printer::HighlevelIrDebugPrinter;
use bluebell::highlevel_ir_emitter::HighlevelIrEmitter;
use bluebell::highlevel_ir_pass_executor::HighlevelIrPassExecutor;
use bluebell::intermediate_name_generator::IntermediateNameGenerator;
use bluebell::llvm_ir_generator::LlvmIrGenerator;
use bluebell::passes::annotate_base_types::AnnotateBaseTypes;
use bluebell::passes::collect_type_definitions::CollectTypeDefinitionsPass;
use bluebell::symbol_table::SymbolTable;

use bluebell::lexer::Lexer;
use bluebell::ParserError;
use bluebell::*;

#[derive(Clone, Debug, Subcommand)]
enum BluebellOutputFormat {
    LlvmIr,
    FormattedScilla,
}

#[derive(Clone, Debug, Subcommand)]
enum BluebellCommand {
    Emit {
        /// Format to output
        #[command(subcommand)]
        format: BluebellOutputFormat,

        /// Filename of output file
        #[arg(short, long)]
        output: Option<String>,
    },
    Run {
        /// Function to name to invoke
        #[arg(short, long)]
        entry_point: String,
    },
}

/// Scilla compiler and executor
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the source file
    filename: String,

    /// Whether or not to produce debug information
    #[arg(long, default_value_t = false)]
    debug: bool,

    /// Command to execute
    #[command(subcommand)]
    mode: BluebellCommand,
}

fn bluebell_run(ast: &NodeProgram, entry_point: String, debug: bool) {
    let mut name_generator = IntermediateNameGenerator::new();

    /****** Executable *****/
    ///////
    // Declaring runtime
    let context = Context::create();
    let mut module = context.create_module("main");

    let ft = context.f64_type();
    module.add_function("sumf", ft.fn_type(&[ft.into(), ft.into()], false), None);
    let i8_ptr_type = context.i8_type().ptr_type(inkwell::AddressSpace::default());
    let fn_type = context.void_type().fn_type(&[i8_ptr_type.into()], false);
    module.add_function("builtin__print<msg>", fn_type, None);

    /*** Parsing ***/

    /////
    // AST -> Highlevel IR
    let mut generator = HighlevelIrEmitter::new(&mut name_generator);
    let mut ast2 = ast.clone();

    let mut ir = generator
        .emit(&mut ast2)
        .expect("Failed generating highlevel IR");

    // let context = Context::create();
    // let mut generator = LlvmIrGenerator::new(&context, ir);
    // generator.write_function_definitions_to_module();

    /*** Analysis ***/

    /////
    // Creating type symbols in symbol table
    let mut symbol_table = SymbolTable::new(&mut name_generator);
    let mut type_collector = CollectTypeDefinitionsPass::new(&mut symbol_table);
    if let Err(err) = ir.visit(&mut type_collector) {
        panic!("{}", err);
    }

    /////
    // Annotate with types
    let mut type_annotator = AnnotateBaseTypes::new(&mut symbol_table);
    if let Err(err) = ir.visit(&mut type_annotator) {
        panic!("{}", err);
    }

    // println!("\n\nDefined types:\n{:#?}\n\n", ir.type_definitions);
    // println!("\n\nDefined functions:\n{:#?}\n\n", ir.function_definitions);

    /////
    // Debug pass
    let mut debug_printer = HighlevelIrDebugPrinter::new();
    let _ = ir.visit(&mut debug_printer);

    /*** IR generation ***/

    ///////
    // Generating IR
    let mut generator = LlvmIrGenerator::new(&context, ir, &mut module);

    match generator.build_module() {
        Err(e) => {
            let llvm_str = module.print_to_string();
            let output = llvm_str.to_str().expect("Failed converting to UTF8");
            println!("{}", output);

            panic!("Error: {:?}", e);
        }
        Ok(_) => (),
    };

    if debug {
        let llvm_str = module.print_to_string();
        let output = llvm_str.to_str().expect("Failed converting to UTF8");
        println!("{}", output);
    }

    /****** Execution *****/
    //////
    // Initializing
    Target::initialize_native(&InitializationConfig::default()).unwrap();

    //////
    // Defining runtime

    extern "C" fn sumf(a: f64, b: f64) -> f64 {
        a + b
    }
    extern "C" fn print_string(s: *const c_char) {
        let c_str = unsafe { CStr::from_ptr(s) };
        let str_slice: &str = c_str.to_str().unwrap();
        println!("{}", str_slice);
    }

    //////
    // Executing

    let contract_executor = UnsafeLlvmTestExecutor::new(&mut module);
    unsafe {
        contract_executor.link_symbol("sumf", sumf as usize);
        contract_executor.link_symbol("builtin__print<msg>", print_string as usize);
        contract_executor.execute(&entry_point);
    }
}

fn main() {
    let args = Args::parse();

    // Accessing the values
    let mut errors: Vec<lexer::ParseError> = [].to_vec();
    let mut file = File::open(args.filename).expect("Unable to open file");
    let mut script = String::new();
    file.read_to_string(&mut script)
        .expect("Unable to read file");

    let lexer = Lexer::new(&script);

    let parser = parser::ProgramParser::new();
    match parser.parse(&mut errors, lexer) {
        Ok(ast) => {
            match args.mode {
                BluebellCommand::Run { entry_point } => bluebell_run(&ast, entry_point, args.debug),
                _ => unimplemented!(),
            }

            /*
            //let _inferred_types = infer_types(&ast).unwrap();
            let mut formatter = BluebellFormatter::new();
            let mut ast2 = ast.clone();
            let formatted_ast = formatter.emit(&mut ast2); // Call to_string on the top-level AST node to get formatted output
            println!("{}", formatted_ast);

            let mut formatter = BluebellFormatter::new();
            let mut ast2 = ast.clone();
            formatter.emit(&mut ast2);
            */
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
