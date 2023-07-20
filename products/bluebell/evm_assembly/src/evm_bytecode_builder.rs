use crate::compiler_context::EvmCompilerContext;
use evm::Opcode;

use std::collections::HashMap;

use crate::block::EvmBlock;
use crate::evm_decompiler::EvmAssemblyGenerator;

use crate::function::EvmFunction;
use crate::opcode_spec::{create_opcode_spec, OpcodeSpecification};

/*
    codebuilder
        .new_function("name", ["arg1", "arg2"])
        .build(|block_builder| {
            block.if(|block_builder| {
            })

        })
*/

pub struct FunctionBuilder<'a, 'ctx> {
    pub builder: &'a mut EvmByteCodeBuilder<'ctx>,
    function: EvmFunction,
}

impl<'a, 'ctx> FunctionBuilder<'a, 'ctx> {
    pub fn build<F>(&mut self, builder: F)
    where
        F: Fn() -> i32,
    {
        builder();
    }
}

/*
impl<'a, 'ctx> FunctionBuilder<'a, 'ctx>> {

}
*/

pub struct EvmByteCodeBuilder<'ctx> {
    pub context: &'ctx mut EvmCompilerContext,
    pub functions: Vec<EvmFunction>,
    pub unused_blocks: Vec<EvmBlock>,
    pub bytecode: Vec<u8>,
    pub opcode_specs: HashMap<u8, OpcodeSpecification>,
    pub auxiliary_data: Vec<u8>,
}

impl<'ctx> EvmByteCodeBuilder<'ctx> {
    pub fn new(context: &'ctx mut EvmCompilerContext) -> Self {
        Self {
            context,
            functions: Vec::new(),
            unused_blocks: Vec::new(),
            bytecode: Vec::new(),
            opcode_specs: create_opcode_spec(),
            auxiliary_data: Vec::new(),
        }
    }

    pub fn define_function<'a>(
        &'a mut self,
        name: &str,
        arg_types: Vec<&str>,
        return_type: &str,
    ) -> FunctionBuilder<'a, 'ctx> {
        let signature = self.context.declare_function(name, arg_types, return_type);
        FunctionBuilder {
            builder: self,
            function: EvmFunction::from_signature(signature),
        }
    }

    pub fn from_bytes(context: &'ctx mut EvmCompilerContext, bytes: Vec<u8>) -> Self {
        let opcode_specs = create_opcode_spec();
        let (blocks, auxiliary_data) =
            EvmBlock::extract_blocks_from_bytecode(&bytes, &opcode_specs);

        let (functions, unused_blocks) = EvmFunction::extract_functions(&blocks);
        Self {
            context,
            functions,
            bytecode: bytes,
            unused_blocks,
            opcode_specs,
            auxiliary_data,
        }
    }

    pub fn from_asm(_script: &str) -> Self {
        unimplemented!();
    }

    pub fn push_u8(&mut self, opcode: u8) -> &mut Self {
        self.bytecode.push(opcode);
        self
    }
    pub fn push(&mut self, opcode: Opcode) -> &mut Self {
        self.bytecode.push(opcode.as_u8());
        self
    }

    pub fn push_bytes(&mut self, bytes: &[u8]) -> &mut Self {
        self.bytecode.extend_from_slice(bytes);
        self
    }

    pub fn build(self) -> Vec<u8> {
        self.bytecode
    }
}

impl EvmAssemblyGenerator for EvmByteCodeBuilder<'_> {
    fn generate_evm_assembly(&self) -> String {
        let mut script = "Unused blocks:\n\n".to_string();

        let unused_blocks = self
            .unused_blocks
            .iter()
            .map(|block| {
                let code = block
                    .instructions
                    .iter()
                    .map(|instr| {
                        if instr.arguments.len() > 0 {
                            let argument: String = instr
                                .arguments
                                .iter()
                                .map(|byte| format!("{:02x}", byte).to_string())
                                .collect();
                            let position = match instr.position {
                                Some(v) => v,
                                None => 0,
                            };
                            format!(
                                "[0x{:02x}: 0x{:02x}] {} 0x{}",
                                position,
                                instr.opcode.as_u8(),
                                instr.opcode.to_string(),
                                argument
                            )
                        } else {
                            let position = match instr.position {
                                Some(v) => v,
                                None => 0,
                            };
                            format!(
                                "[0x{:02x}: 0x{:02x}] {}",
                                position,
                                instr.opcode.as_u8(),
                                instr.opcode.to_string()
                            )
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("\n");
                let position = match block.position {
                    Some(v) => v,
                    None => 0,
                };
                format!("{}: ;; Starts at 0x{:02x} \n{}", block.name, position, code)
            })
            .collect::<Vec<String>>()
            .join("\n\n");
        script.push_str(&unused_blocks);

        for function in &self.functions {
            let code_blocks = function
                .blocks
                .iter()
                .map(|block| {
                    let code = block
                        .instructions
                        .iter()
                        .map(|instr| {
                            if instr.arguments.len() > 0 {
                                let argument: String = instr
                                    .arguments
                                    .iter()
                                    .map(|byte| format!("{:02x}", byte).to_string())
                                    .collect();
                                let position = match instr.position {
                                    Some(v) => v,
                                    None => 0,
                                };
                                format!(
                                    "[0x{:02x}: 0x{:02x}] {} 0x{}",
                                    position,
                                    instr.opcode.as_u8(),
                                    instr.opcode.to_string(),
                                    argument
                                )
                            } else {
                                let position = match instr.position {
                                    Some(v) => v,
                                    None => 0,
                                };
                                format!(
                                    "[0x{:02x}: 0x{:02x}] {}",
                                    position,
                                    instr.opcode.as_u8(),
                                    instr.opcode.to_string()
                                )
                            }
                        })
                        .collect::<Vec<String>>()
                        .join("\n");
                    let position = match block.position {
                        Some(v) => v,
                        None => 0,
                    };
                    format!("{}: ;; Starts at 0x{:02x} \n{}", block.name, position, code)
                })
                .collect::<Vec<String>>()
                .join("\n\n");

            script.push_str("\n\nFunction:\n");
            script.push_str(&code_blocks);
        }

        script
    }
}
