use crate::parser::lexer;
use crate::parser::lexer::Lexer;
use crate::parser::parser;

use crate::contract_executor::UnsafeContractExecutor;
use crate::intermediate_representation::emitter::IrEmitter;
use crate::intermediate_representation::pass_manager::PassManager;
use crate::llvm_ir_generator::LlvmIrGenerator;

use inkwell::context::Context;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::module::Module;
use inkwell::types::{BasicTypeEnum /*, StructType */, VoidType};
use inkwell::values::FunctionValue;
use inkwell::OptimizationLevel;
use std::collections::HashMap;

enum LlvmType<'ctx> {
    Unit(VoidType<'ctx>),
    Basic(BasicTypeEnum<'ctx>),
    // TODO: Struct(StructType<'ctx>),
}

pub struct LlvmExecutable<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
}

impl<'ctx> LlvmExecutable<'ctx> {
    pub fn new(context: &'ctx Context, module: Module<'ctx>) -> Self {
        Self { context, module }
    }
}

pub struct LlvmBackendSpecification<'ctx> {
    context: &'ctx Context,
    pub function_declarations: HashMap<String, FunctionValue<'ctx>>, // TODO: Not used yet - pub to silence warning
    type_declarations: HashMap<String, LlvmType<'ctx>>,
    runtime_function_addresses: HashMap<String, usize>,
}

pub struct LlvmBackendSpecificationFunctionPrototype<'a, 'ctx> {
    name: String,
    runtime: &'a mut LlvmBackendSpecification<'ctx>,
}

impl<'a, 'ctx> LlvmBackendSpecificationFunctionPrototype<'a, 'ctx> {
    pub fn new(name: String, runtime: &'a mut LlvmBackendSpecification<'ctx>) -> Self {
        Self { name, runtime }
    }

    pub fn attach_runtime<F>(&mut self, get_pointer: F) -> Result<(), String>
    where
        F: FnOnce() -> usize,
    {
        let pointer = get_pointer();
        // Check if the function name already exists
        if self
            .runtime
            .runtime_function_addresses
            .contains_key(&self.name)
        {
            return Err(format!("Function '{}' already exists", self.name));
        }
        self.runtime
            .runtime_function_addresses
            .insert(self.name.clone(), pointer);
        Ok(())
    }
}

impl<'ctx> LlvmBackendSpecification<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        Self {
            context,
            function_declarations: HashMap::new(),
            type_declarations: HashMap::new(),
            runtime_function_addresses: HashMap::new(),
        }
    }

    pub fn declare_integer(&mut self, name: &str, bits: u32) {
        let int_type = self.context.custom_width_int_type(bits);
        self.type_declarations
            .insert(name.to_string(), LlvmType::Basic(int_type.into()));
    }

    pub fn declare_unsigned_integer(&mut self, name: &str, bits: u32) {
        let uint_type = self.context.custom_width_int_type(bits);
        self.type_declarations
            .insert(name.to_string(), LlvmType::Basic(uint_type.into()));
    }

    pub fn declare_void(&mut self, name: &str) {
        let void_type = self.context.void_type();
        self.type_declarations
            .insert(name.to_string(), LlvmType::Unit(void_type.into()));
    }

    pub fn declare_intrinsic<'b>(
        &'b mut self,
        name: &str,
        arg_types: Vec<&str>,
        return_type: &str,
    ) -> LlvmBackendSpecificationFunctionPrototype<'b, 'ctx> {
        // Resolve the return type
        let _return_type = match self
            .type_declarations
            .get(return_type)
            .expect("Return type not found.")
        {
            // LlvmType::Unit(void_type) => void_type.into(),
            LlvmType::Basic(basic_type) => basic_type.clone(),
            _ => unimplemented!(), // Handle other LlvmType variants here
        };
        // Resolve argument types
        let _arg_types: Vec<_> = arg_types
            .iter()
            .map(|&type_name| {
                match self
                    .type_declarations
                    .get(type_name)
                    .expect("Arg type not found.")
                {
                    LlvmType::Basic(basic_type) => basic_type.clone(),
                    _ => panic!("Only basic types are supported for function args."),
                }
            })
            .collect();
        // TODO: Fix this

        LlvmBackendSpecificationFunctionPrototype {
            name: name.to_string(),
            runtime: self,
        }
    }
}

pub struct UnsafeLlvmTestExecutor<'ctx, 'module> {
    execution_engine: ExecutionEngine<'ctx>,
    module: &'module mut Module<'ctx>,
}

impl<'ctx, 'module> UnsafeLlvmTestExecutor<'ctx, 'module> {
    pub fn new(module: &'module mut Module<'ctx>) -> Self {
        let execution_engine = module
            .create_jit_execution_engine(OptimizationLevel::None)
            .expect("Unable to create execution engine");
        UnsafeLlvmTestExecutor {
            execution_engine,
            module,
        }
    }
}

impl<'ctx, 'module> UnsafeContractExecutor for UnsafeLlvmTestExecutor<'ctx, 'module> {
    unsafe fn execute(&self, name: &str) -> f64 {
        let function = self
            .execution_engine
            .get_function::<unsafe extern "C" fn() -> f64>(name)
            .expect("Unable to find the function");
        function.call()
    }

    unsafe fn link_symbol(&self, name: &str, addr: usize) {
        let function = self
            .module
            .get_function(name)
            .expect("Unable to link the function");
        self.execution_engine
            .add_global_mapping(&function, addr as usize);
    }
}

pub struct LlvmBackend {
    context: Box<Context>,
}

impl LlvmBackend {
    pub fn new() -> Self {
        let context = Box::new(Context::create());
        Self { context }
    }

    pub fn create_backend_specification(&self) -> LlvmBackendSpecification {
        LlvmBackendSpecification::new(&self.context)
    }

    pub fn compile(&self, name: String, script: String) -> Result<Module, String> {
        let mut errors: Vec<lexer::ParseError> = [].to_vec();
        let lexer = Lexer::new(&script);

        let parser = parser::ProgramParser::new();
        let ast = match parser.parse(&mut errors, lexer) {
            Ok(ast) => ast,
            Err(error) => {
                // TODO: This code is ineffecient and we should attach the
                // the token position (line, char) to the ast nodes / parser
                let _message = format!("Syntax error {:?}", error);
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

                /*
                TODO:
                let my_error = ParserError {
                    message,
                    line: line_counter,
                    column: char_counter,
                };
                println!("{}", my_error);
                */
                return Err("Failed to compile".to_string());
            }
        };

        let mut module = self.context.create_module(&name);
        let mut generator = IrEmitter::new();
        let mut ir = generator
            .emit(&ast)
            .expect("Failed generating highlevel IR");

        /*** Analysis ***/
        let mut pass_manager = PassManager::default_pipeline();

        if let Err(err) = pass_manager.run(&mut ir) {
            panic!("{}", err);
        }

        /*
        let mut debug_printer = DebugPrinter::new();
        let _ = ir.run_pass(&mut debug_printer);
        */

        let mut generator = LlvmIrGenerator::new(&self.context, ir, &mut module);
        match generator.build_module() {
            Err(e) => {
                let llvm_str = module.print_to_string();
                let output = llvm_str.to_str().expect("Failed converting to UTF8");
                println!("{}", output);

                return Err(e);
            }
            Ok(_) => (),
        };

        // LlvmExecutable::new( &self.context, module );
        Ok(module)
    }
}
