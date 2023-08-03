use std::ffi::CStr;

use clap::{Parser, Subcommand, ValueEnum};
use inkwell::context::Context;
use inkwell::targets::{InitializationConfig, Target};
use std::fs::File;
use std::io::Read;
use std::os::raw::c_char;
use std::process;

use bluebell::ast::nodes::NodeProgram;
use bluebell::contract_executor::UnsafeContractExecutor;
use bluebell::passes::debug_printer::DebugPrinter;

use bluebell::support::evm::{EvmCompiler, ScillaDefaultBuiltins, ScillaDefaultTypes};

use bluebell::llvm_ir_generator::LlvmIrGenerator;
use bluebell::support::llvm::{LlvmBackend, UnsafeLlvmTestExecutor};

use bluebell::intermediate_representation::emitter::IrEmitter;
use bluebell::intermediate_representation::pass_manager::PassManager;

use bluebell::parser::lexer;
use bluebell::parser::lexer::Lexer;
use bluebell::parser::{parser, ParserError};

use evm_assembly::types::EvmTypeValue;

#[derive(Clone, Debug, Subcommand)]
enum BluebellOutputFormat {
    LlvmIr,
    FormattedScilla,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum BluebellBackend {
    Llvm,
    Evm,
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
        /// Backend to use
        #[arg(long, value_enum)]
        backend: BluebellBackend,

        /// Function to name to invoke
        #[arg(short, long)]
        entry_point: String,

        /// Arguments to pass to function
        #[arg(short, long, default_value_t= ("".to_string()))]
        args: String,
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

fn bluebell_evm_run(ast: &NodeProgram, entry_point: String, args: String, _debug: bool) {
    let mut compiler = EvmCompiler::new();

    // Defining capabilities
    let default_types = ScillaDefaultTypes {};
    let default_builtins = ScillaDefaultBuiltins {};

    compiler.attach(&default_types);
    compiler.attach(&default_builtins);

    // Creating executable

    let executable = match compiler.executable_from_ast(ast) {
        Err(e) => panic!("{:?}", e),
        Ok(v) => v,
    };

    let arguments: Vec<EvmTypeValue> = if args == "" {
        [].to_vec()
    } else {
        serde_json::from_str(&args).expect("Failed to deserialize arguments")
    };

    executable.execute(&entry_point, arguments);
}

fn bluebell_llvm_run(ast: &NodeProgram, entry_point: String, debug: bool) {
    /****** Executable *****/
    ///////
    let backend = LlvmBackend::new();
    // TODO: runtime is a poor name.
    let mut specification = backend.create_backend_specification();

    specification.declare_integer("Int8", 8);
    specification.declare_integer("Int16", 16);
    specification.declare_integer("Int32", 32);
    specification.declare_integer("Int64", 64);
    specification.declare_unsigned_integer("Uint8", 8);
    specification.declare_unsigned_integer("Uint16", 16);
    specification.declare_unsigned_integer("Uint32", 32);
    specification.declare_unsigned_integer("Uint64", 64);

    let _ = specification
        .declare_intrinsic("add", ["Int32", "Int32"].to_vec(), "Int32")
        .attach_runtime(|| {
            extern "C" fn addi32(a: i32, b: i32) -> i32 {
                a + b
            }

            addi32 as usize
        });

    // let _executable = backend.create_executable("test");
    // let executable = backend.compile(name, script);

    let context = Context::create();
    let mut module = context.create_module("main");

    // Runtime struct <- contains Context
    // VM / Executor
    // Executable <- contains Module
    // Compiler

    // Declaring runtime
    let ft = context.f64_type();
    let i8_ptr_type = context.i8_type().ptr_type(inkwell::AddressSpace::default());
    let fn_type = context.void_type().fn_type(&[i8_ptr_type.into()], false);

    module.add_function("sumf", ft.fn_type(&[ft.into(), ft.into()], false), None);
    module.add_function("builtin__print<msg>", fn_type, None);

    let setup_runtime = |contract_executor: &UnsafeLlvmTestExecutor| {
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
        unsafe {
            contract_executor.link_symbol("sumf", sumf as usize);
            contract_executor.link_symbol("builtin__print<msg>", print_string as usize);
        }
    };

    /*** Compiling ***/

    /////
    // Frontend: AST -> Highlevel IR
    let mut generator = IrEmitter::new();
    let mut ir = generator.emit(ast).expect("Failed generating highlevel IR");

    /*** Analysis ***/
    let mut pass_manager = PassManager::default_pipeline();

    if let Err(err) = pass_manager.run(&mut ir) {
        panic!("{}", err);
    }

    let mut debug_printer = DebugPrinter::new();
    let _ = ir.run_pass(&mut debug_printer);

    ///////
    // Lowering/"backend": Generating LLVM IR
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

    //////
    // Executing

    let contract_executor = UnsafeLlvmTestExecutor::new(&mut module);
    setup_runtime(&contract_executor);

    unsafe {
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
                BluebellCommand::Run {
                    entry_point,
                    args: arguments,
                    backend,
                } => match backend {
                    BluebellBackend::Llvm => bluebell_llvm_run(&ast, entry_point, args.debug),
                    BluebellBackend::Evm => {
                        bluebell_evm_run(&ast, entry_point, arguments, args.debug)
                    }
                },
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
            if pos.len() > 1 {
                println!("{}", "^".repeat(pos[1] - pos[0]));
            }

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
