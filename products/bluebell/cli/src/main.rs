use std::{fs::File, io::Read, process};

use bluebell::support::{
    evm::EvmCompiler,
    modules::{ScillaDebugBuiltins, ScillaDefaultBuiltins, ScillaDefaultTypes},
};
use clap::{Parser, Subcommand, ValueEnum};
use evm_assembly::types::EvmTypeValue;
use log::{Log, Metadata, Record};
use scilla_parser::{
    ast::nodes::NodeProgram,
    parser::{lexer, lexer::Lexer, parser, ParserError},
};

// Logger struct to capture logs
struct CaptureLogger {}

// Implementation of logger
impl CaptureLogger {
    // Constructor for CaptureLogger
    fn new() -> Self {
        Self {}
    }
}

// Implementing Log trait for CaptureLogger
impl Log for CaptureLogger {
    // Method to check if logging is enabled
    fn enabled(&self, _metadata: &Metadata) -> bool {
        // self.delegate.enabled(metadata)
        true
    }

    // Method to log a record
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            print!("{}", record.args().to_string());
        }
    }

    // Method to flush the logger
    fn flush(&self) {}
}

// Function to setup the logger
fn setup_logger() {
    let logger = Box::new(CaptureLogger::new());
    log::set_boxed_logger(logger).unwrap();
    log::set_max_level(log::LevelFilter::Info);
}

// Enum to define the output format of Bluebell
#[derive(Clone, Debug, Subcommand)]
enum BluebellOutputFormat {
    FormattedScilla,
}

// Enum to define the backend of Bluebell
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum BluebellBackend {
    Evm,
}

// Enum to define the command of Bluebell
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
        #[arg(short, long, default_value_t= String::new())]
        args: String,
    },
}

// Struct to hold the arguments for Scilla compiler and executor
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the source file
    filename: String,

    /// Whether or not to produce debug information
    #[arg(long, default_value_t = false)]
    debug: bool,

    /// Features to enable at runtime
    #[arg(long = "runtime-enable")]
    features_raw: Option<String>,

    /// Command to execute
    #[command(subcommand)]
    mode: BluebellCommand,
}

// Implementation of Args struct
impl Args {
    // Method to get the features
    fn features(&self) -> Vec<String> {
        match &self.features_raw {
            Some(v) => v.split(",").map(|s| s.to_string()).collect(),
            _ => Vec::new(),
        }
    }
}

// Function to run Bluebell with EVM backend
fn bluebell_evm_run(
    ast: &NodeProgram,
    entry_point: String,
    args: String,
    features: Vec<String>,
    _debug: bool,
) {
    let mut compiler = EvmCompiler::new();

    // Defining capabilities
    let default_types = ScillaDefaultTypes {};
    let default_builtins = ScillaDefaultBuiltins {};

    compiler.attach(&default_types);
    compiler.attach(&default_builtins);

    for feature in features {
        match &feature[..] {
            "debug" => {
                let feature = ScillaDebugBuiltins {};
                compiler.attach(&feature);
            }
            _ => {
                panic!("Unknown feature {}", feature)
            }
        }
    }

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

// Main function
fn main() {
    // Setting up the logger
    setup_logger();
    // Parsing the arguments
    let args = Args::parse();

    // Getting the features
    let features = args.features();
    // Accessing the values
    let mut errors: Vec<lexer::ParseError> = [].to_vec();
    // Opening the file
    let mut file = File::open(args.filename).expect("Unable to open file");
    let mut script = String::new();
    // Reading the file
    file.read_to_string(&mut script)
        .expect("Unable to read file");

    // Creating a new lexer
    let lexer = Lexer::new(&script);

    // Creating a new parser
    let parser = parser::ProgramParser::new();

    // Parsing the script
    match parser.parse(&mut errors, lexer) {
        Ok(ast) => {
            // Running the appropriate command based on the mode
            match args.mode {
                BluebellCommand::Run {
                    entry_point,
                    args: arguments,
                    backend,
                } => match backend {
                    // Running with EVM backend
                    BluebellBackend::Evm => {
                        bluebell_evm_run(&ast, entry_point, arguments, features, args.debug)
                    }
                },
                _ => unimplemented!(),
            }

            /*
            //let _inferred_types = infer_types(&ast).unwrap();
            let mut formatter = BluebellFormatter::new();
            let mut ast2 = ast.clone();
            let formatted_ast = formatter.emit(&mut ast2); // Call to_string on the top-level AST node to get formatted output

            let mut formatter = BluebellFormatter::new();
            let mut ast2 = ast.clone();
            formatter.emit(&mut ast2);
            */
        }
        Err(error) => {
            // Handling syntax errors
            let message = format!("Syntax error {:?}", error);
            let mut pos: Vec<lexer::SourcePosition> = [].to_vec();
            error.map_location(|l| {
                pos.push(l.clone());
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
                if !should_stop && n == pos[0].position {
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

            // Printing the line with the error
            let line = &script[line_start..line_end];
            println!("Line {},{}:{}", line_counter, char_counter, line);
            print!(
                "{}",
                " ".repeat(char_counter + format!("Line {},{}:", line_counter, char_counter).len())
            );
            if pos.len() > 1 {
                println!("{}", "^".repeat(pos[1].position - pos[0].position));
            }

            // Creating a new ParserError
            let my_error = ParserError {
                message,
                line: 0,   //error.location_line(),
                column: 0, // err.location_column(),
            };
            // Printing the error
            println!("{}", my_error);

            // Exiting the process with an error code
            process::exit(-1);
        }
    }
}
