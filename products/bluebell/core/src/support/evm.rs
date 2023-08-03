use crate::ast::nodes::NodeProgram;

use crate::evm_bytecode_generator::EvmBytecodeGenerator;

use crate::intermediate_representation::emitter::IrEmitter;
use crate::intermediate_representation::pass_manager::PassManager;

use evm_assembly::compiler_context::EvmCompilerContext;
use evm_assembly::executor::EvmExecutor;

use evm::backend::Backend;
use evm::executor::stack::{PrecompileFailure, PrecompileOutput, PrecompileOutputType};
use evm::{Context as EvmContext, ExitError, ExitSucceed};

pub trait BluebellModule {
    fn attach(&self, context: &mut EvmCompilerContext);
}

pub struct ScillaDefaultTypes;
impl BluebellModule for ScillaDefaultTypes {
    // TODO: Generalise to support both LLVM and EVM
    fn attach(&self, context: &mut EvmCompilerContext) {
        context.declare_integer("Int8", 8);
        context.declare_integer("Int16", 16);
        context.declare_integer("Int32", 32);
        context.declare_integer("Int64", 64);
        context.declare_unsigned_integer("Uint8", 8);
        context.declare_unsigned_integer("Uint16", 16);
        context.declare_unsigned_integer("Uint32", 32);
        context.declare_unsigned_integer("Uint64", 64);
        context.declare_unsigned_integer("Uint256", 256);
    }
}

pub struct ScillaDefaultBuiltins;
impl BluebellModule for ScillaDefaultBuiltins {
    // TODO: Generalise to support both LLVM and EVM

    fn attach(&self, specification: &mut EvmCompilerContext) {
        let _ = specification
            .declare_function(
                "builtin__fibonacci::<Uint64,Uint64>",
                ["Uint256", "Uint256"].to_vec(),
                "Uint256",
            )
            .attach_runtime(|| {
                fn custom_runtime(
                    input: &[u8],
                    gas_limit: Option<u64>,
                    context: &EvmContext,
                    _backend: &dyn Backend,
                    is_static: bool,
                ) -> Result<(PrecompileOutput, u64), PrecompileFailure> {
                    println!("Running precompile {:?}!", input);
                    println!("Len: {} / {}", input.len() / 32, input.len());
                    println!("Context: {:#?}", context);
                    println!("Static: {}", is_static);
                    let gas_needed = 20;

                    if let Some(gas_limit) = gas_limit {
                        if gas_limit < gas_needed {
                            return Err(PrecompileFailure::Error {
                                exit_status: ExitError::OutOfGas,
                            });
                        }
                    }

                    Ok((
                        PrecompileOutput {
                            output_type: PrecompileOutputType::Exit(ExitSucceed::Returned),
                            output: input.to_vec(),
                        },
                        gas_needed,
                    ))
                }

                custom_runtime
            });
        let _ = specification.declare_inline_generics("builtin__add", |block, _arg_types| {
            block.add();
            Ok(())
        });
    }
}

pub struct EvmCompiler {
    context: EvmCompilerContext,
    ir_emitter: IrEmitter,
    pass_manager: PassManager,
}

impl EvmCompiler {
    pub fn new() -> Self {
        EvmCompiler {
            context: EvmCompilerContext::new(),
            ir_emitter: IrEmitter::new(),
            pass_manager: PassManager::default_pipeline(),
        }
    }

    pub fn attach(&mut self, module: &dyn BluebellModule) {
        module.attach(&mut self.context);
    }

    pub fn compile(&self, _script: String) -> Result<Vec<u8>, String> {
        todo!()
    }

    // TODO: Remove &mut self - needs to be removed from a number of places first
    pub fn compile_ast(&mut self, ast: &NodeProgram) -> Result<Vec<u8>, String> {
        let mut ir = self.ir_emitter.emit(ast)?;
        self.pass_manager.run(&mut ir)?;

        let mut generator = EvmBytecodeGenerator::new(&mut self.context, ir);

        generator.build_executable()
    }

    pub fn executable_from_ast(&mut self, ast: &NodeProgram) -> Result<EvmExecutor, String> {
        let executable = self.compile_ast(ast)?;
        Ok(EvmExecutor::new(&self.context, executable))
    }
}
