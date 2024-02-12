use std::{
    collections::{BTreeSet, HashMap, HashSet},
    mem,
};

use evm::Opcode;

use crate::{
    block::EvmBlock,
    bytecode_ir::EvmBytecodeIr,
    compiler_context::EvmCompilerContext,
    evm_decompiler::EvmAssemblyGenerator,
    executable::EvmExecutable,
    function::EvmFunction,
    opcode_spec::{create_opcode_spec, OpcodeSpec, OpcodeSpecification},
    types::EvmType,
};

pub struct FunctionBuilder<'a, 'ctx> {
    pub builder: &'a mut EvmByteCodeBuilder<'ctx>,
    function: EvmFunction,
}

impl<'a, 'ctx> FunctionBuilder<'a, 'ctx> {
    pub fn build<F>(mut self, builder: F)
    where
        F: Fn(&mut EvmByteCodeBuilder<'ctx>) -> Vec<EvmBlock>,
    {
        let signature = &self.function.signature.clone().unwrap();

        self.builder
            .set_current_function_name(signature.name.clone());
        self.function.blocks = builder(&mut self.builder);
        self.builder.clear_current_function_name();

        // if first_block.consumes

        match self.function.compute_stack_difference() {
            Err(e) => panic!("{}", e),
            _ => (),
        }

        /*
        TODO:
        if self.function.consumes != signature.arguments.len().try_into().unwrap() {
           panic!("{}", format!("Function consumes {} but expects {}",self.function.consumes,signature.arguments.len() ))
        }
        */

        if self.function.produces > 1 {
            panic!(
                "{}",
                format!(
                    "Function produces {} but at the moment we only support 1 return argument",
                    self.function.produces
                )
            )
        }

        self.builder.ir.functions.push_back(self.function);
    }
}

/*
impl<'a, 'ctx> FunctionBuilder<'a, 'ctx>> {

}
*/

pub struct EvmByteCodeBuilder<'ctx> {
    pub context: &'ctx mut EvmCompilerContext,
    pub ir: EvmBytecodeIr,

    pub bytecode: Vec<u8>,
    pub opcode_specs: HashMap<u8, OpcodeSpecification>, // TODO: Should be deleted
    pub auxiliary_data: Vec<u8>,
    pub was_finalized: bool,
    pub create_abi_boilerplate: bool,
    pub label_positions: HashMap<String, u32>,

    pub current_function_name: Option<String>,
    pub used_block_names: HashSet<String>,
}

impl<'ctx> EvmByteCodeBuilder<'ctx> {
    pub fn new(context: &'ctx mut EvmCompilerContext, create_abi_boilerplate: bool) -> Self {
        let mut ret = Self {
            context,
            ir: EvmBytecodeIr::new(),
            bytecode: Vec::new(),
            opcode_specs: create_opcode_spec(),
            auxiliary_data: Vec::new(),
            was_finalized: false,
            create_abi_boilerplate,
            label_positions: HashMap::new(),
            current_function_name: None,
            used_block_names: HashSet::new(),
        };

        // Reserving the start of the bytecode for the "entry" function
        ret.define_function("__main__", [].to_vec(), "Uint256")
            .build(|code_builder| {
                // Placeholder block for the main function
                [code_builder.new_evm_block("main_entry")].to_vec()
                // EvmBlock::new(None, BTreeSet::new(), &code_builder.add_scope_to_label("main_entry"))
            });

        ret
    }

    pub fn new_evm_block(&mut self, name: &str) -> EvmBlock {
        EvmBlock::new(
            None,
            BTreeSet::new(),
            &self.generate_unique_block_name(name),
        )
    }

    pub fn new_evm_block_with_args(&mut self, name: &str, args: BTreeSet<String>) -> EvmBlock {
        EvmBlock::new(None, args, &self.generate_unique_block_name(name))
    }

    pub fn set_current_function_name(&mut self, name: String) {
        self.current_function_name = Some(name);
    }

    pub fn clear_current_function_name(&mut self) {
        self.current_function_name = None;
    }

    pub fn generate_unique_block_name(&mut self, name: &str) -> String {
        let base = self.add_scope_to_label(name);
        let mut candidate = base.clone();
        let mut i = 1;
        while self.used_block_names.contains(&candidate) {
            candidate = format!("{}.{}", base, i).to_string();
            i += 1;
        }
        self.used_block_names.insert(candidate.clone());

        candidate
    }

    pub fn add_scope_to_label(&mut self, name: &str) -> String {
        let candidate = if let Some(current_function_name) = &self.current_function_name {
            format!("{}::{}", current_function_name, name)
        } else {
            name.to_string()
        };

        candidate
    }

    pub fn define_function<'a>(
        &'a mut self,
        name: &str,
        arg_types: Vec<&str>,
        return_type: &str,
    ) -> FunctionBuilder<'a, 'ctx> {
        let signature = {
            let prototype = self.context.declare_function(name, arg_types, return_type);
            prototype.signature
        };

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
        let ir = EvmBytecodeIr {
            functions: functions.into(),
            data: Vec::new(),
            unused_blocks,
        };
        Self {
            context,
            ir,
            opcode_specs,
            auxiliary_data,
            bytecode: bytes,

            was_finalized: false,
            create_abi_boilerplate: false,
            label_positions: HashMap::new(),
            current_function_name: None,
            used_block_names: HashSet::new(),
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

    pub fn build(&mut self) -> EvmExecutable {
        let mut bytecode = Vec::new();
        self.finalize_blocks();

        // Generating bytecode
        for function in self.ir.functions.iter_mut() {
            for block in function.blocks.iter_mut() {
                for instruction in block.instructions.iter_mut() {
                    // Sanity check that arguments matches
                    assert!(
                        instruction.opcode.bytecode_arguments() == instruction.arguments.len(),
                        "Sanity check failed while writing byte code."
                    );

                    // Writing code
                    bytecode.push(instruction.opcode.as_u8());
                    bytecode.extend(instruction.arguments.clone());
                }
            }
        }

        bytecode.push(Opcode::STOP.as_u8());

        for (_, payload) in &self.ir.data {
            bytecode.extend(payload);
        }

        let mut ir = EvmBytecodeIr::new();
        mem::swap(&mut ir, &mut self.ir);

        // TODO: Make block table
        EvmExecutable {
            bytecode,
            label_positions: self.label_positions.clone(),
            ir,
        }
    }

    pub fn finalize_blocks(&mut self) {
        if self.was_finalized {
            return;
        }

        // Building entry function
        let mut main = {
            let mut binding = self.ir.functions.front_mut();
            let main = match binding {
                Some(ref mut main) => main,
                _ => panic!("Expected the reserved main function."),
            };

            main.clone()
        };

        let mut binding_block = main.blocks.first_mut();
        let first_block = match binding_block {
            Some(ref mut block) => block,
            None => panic!("Function does not have a main block."),
        };

        self.set_current_function_name("__entry_function__".to_string());
        let mut switch_block = self.new_evm_block("switch");
        let mut fail_block = self.new_evm_block("fail");
        let mut success_block = self.new_evm_block("success");
        self.clear_current_function_name();

        // Making sure that there is value attached to the contract call
        if self.create_abi_boilerplate {
            first_block.push1([0x80].to_vec());
            first_block.push1([0x40].to_vec());
            first_block.mstore();
            first_block.external_callvalue();
            first_block.dup1();
            first_block.iszero();
            first_block.jump_if_to(&switch_block.name);
            first_block.push1([0x00].to_vec());
            first_block.dup1();
            first_block.revert();
        } else {
            first_block.push1([0x80].to_vec());
            first_block.push1([0x40].to_vec());
            first_block.mstore();

            // Adding return address
            first_block.push_label(&success_block.name);
            if let Some(fnc) = &self.ir.functions.get(1) {
                if let Some(block) = fnc.blocks.first() {
                    first_block.jump_to(&block.name);
                } else {
                    first_block.jump();
                }
            } else {
                first_block.jump();
            }
        }

        switch_block.pop(); // 0 Oribabky remove dup1()?
        switch_block.push1([0x04].to_vec()); // Checking that the size of call args
        switch_block.calldatasize();
        switch_block.lt();
        switch_block.jump_if_to(&fail_block.name);
        switch_block.push1([0x00].to_vec());
        switch_block.calldataload();
        switch_block.push1([0xe0].to_vec());
        switch_block.shr();

        let mut data_loading_blocks: Vec<EvmBlock> = Vec::new();
        let functions = self.ir.functions.clone();
        for (i, function) in functions.iter().enumerate() {
            // Skipping the entry function (the one we are building now)
            if i > 0 {
                // Ensuring that we are producing unique block names
                let signature = &function.signature.clone().unwrap();
                self.set_current_function_name(signature.name.clone());

                switch_block.dup1();
                switch_block.push(function.selector.clone());
                switch_block.eq();
                match function.blocks.first() {
                    Some(block) => {
                        let mut load_data_block = self.new_evm_block("load_args");
                        switch_block.jump_if_to(&load_data_block.name);

                        let signature = function.signature.clone().unwrap();

                        load_data_block.pop(); // Remove the user function selector from the stack

                        // Checking that the size of call args
                        // TODO: Assumptino is that all arguments are 256-bits
                        let args_size = 0x04 + 0x20 * signature.arguments.len();
                        load_data_block.push_u64(args_size.try_into().unwrap());
                        load_data_block.calldatasize();
                        load_data_block.lt();
                        load_data_block.jump_if_to(&fail_block.name);

                        // Adding return address
                        load_data_block.push_label(&success_block.name);

                        // Loading data
                        for (i, arg) in signature.arguments.iter().enumerate() {
                            if let EvmType::String = arg {
                                // TODO: We only support loading of 32 byte strings.
                                // TODO: Count non-zero characters and push to the stack
                                let size = 32;
                                load_data_block.alloca_static(4 + size);
                                // First four bytes store size
                                load_data_block.push_u32(size as u32);
                                load_data_block.push_u32(256 - 32);
                                load_data_block.shl();
                                load_data_block.dup2();
                                load_data_block.mstore();

                                load_data_block.dup1();
                                load_data_block.push1([0x04].to_vec());
                                load_data_block.add();

                                load_data_block.push_u64((0x04 + 0x20 * i).try_into().unwrap());
                                load_data_block.calldataload();
                                load_data_block.swap1();
                                load_data_block.mstore();
                            } else {
                                load_data_block.push_u64((0x04 + 0x20 * i).try_into().unwrap());
                                load_data_block.calldataload();
                            }
                        }

                        load_data_block.jump_to(&block.name);
                        data_loading_blocks.push(load_data_block);
                    }
                    _ => panic!("Function does not have any blocks."),
                };
            }

            self.clear_current_function_name();
        }

        switch_block.jump_to(&fail_block.name);

        fail_block.push1([0x00].to_vec());
        fail_block.dup1();
        fail_block.revert();

        success_block.push1([0x00].to_vec());
        success_block.dup1();
        success_block.r#return();

        if self.create_abi_boilerplate {
            main.blocks.push(switch_block);
            for block in data_loading_blocks {
                main.blocks.push(block);
            }
            main.blocks.push(fail_block);
            main.blocks.push(success_block);
        } else {
            main.blocks.push(success_block);
        }

        self.ir.functions[0] = main;

        // Resolving labels
        self.resolve_positions();

        // TODO: Test that all stack positions zero out

        self.was_finalized = true;
    }

    pub fn resolve_positions(&mut self) {
        let mut position: u32 = 0;
        self.label_positions = HashMap::new();

        // Creating code positions
        for function in self.ir.functions.iter_mut() {
            for (i, block) in function.blocks.iter_mut().enumerate() {
                block.position = Some(position);
                self.label_positions.insert(block.name.clone(), position);
                if i == 0 {
                    let function_name = match &function.signature {
                        Some(v) => v.name.clone(),
                        None => panic!("Invalid function signature {:?}", function),
                    };
                    self.label_positions.insert(function_name, position);
                }
                for instruction in block.instructions.iter_mut() {
                    instruction.position = Some(position);
                    if let Some(label) = &instruction.label {
                        self.label_positions.insert(label.to_string(), position);
                    }
                    position += 1 + instruction.expected_args_length() as u32;
                }
            }
        }

        // Position reserved for STOP
        position += 1;

        // Creating data positions
        for (name, payload) in &self.ir.data {
            self.label_positions.insert(name.to_string(), position);
            position += payload.len() as u32;
        }

        // Updating labels
        for function in self.ir.functions.iter_mut() {
            for block in function.blocks.iter_mut() {
                for instruction in block.instructions.iter_mut() {
                    if let Some(name) = &instruction.unresolved_argument_label {
                        match self.label_positions.get(name) {
                            Some(p) => {
                                instruction.u32_to_arg_big_endian(*p);
                            }
                            None => {
                                println!("Available labels: {:#?}", self.label_positions);
                                panic!("Label not found {:#?}!", name);
                            }
                        }
                    }
                }
            }
        }
    }
}

impl EvmAssemblyGenerator for EvmByteCodeBuilder<'_> {
    fn generate_evm_assembly(&self) -> String {
        let mut script = "Unused blocks:\n\n".to_string();

        let unused_blocks = self
            .ir
            .unused_blocks
            .iter()
            .map(|block| {
                let code = block
                    .instructions
                    .iter()
                    .map(|instr| {
                        let position = match instr.position {
                            Some(v) => v,
                            None => 0,
                        };
                        let instruction_value = if instr.arguments.len() > 0 {
                            let argument: String = instr
                                .arguments
                                .iter()
                                .map(|byte| format!("{:02x}", byte).to_string())
                                .collect();

                            format!("{} 0x{}", instr.opcode.to_string(), argument).to_string()
                        } else {
                            instr.opcode.to_string()
                        };

                        format!(
                            "[0x{:02x}: 0x{:02x}] {:<width$}  ;; Stack: {}, Comment: {}, Source: {:?}",
                            position,
                            instr.opcode.as_u8(),
                            instruction_value,
                            instr.stack_size,
                            instr.comment.clone().unwrap_or("".to_string()).trim(),
                            instr.source_position,
                            width = 40
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("\n");
                let position = match block.position {
                    Some(v) => v,
                    None => 0,
                };
                format!(
                    "{}: ;; Starts at 0x{:02x}  u8[{}] \n{}",
                    block.name, position, block.consumes, code
                )
            })
            .collect::<Vec<String>>()
            .join("\n\n");
        script.push_str(&unused_blocks);

        for function in &self.ir.functions {
            let code_blocks = function
                .blocks
                .iter()
                .map(|block| {
                    let code = block
                        .instructions
                        .iter()
                        .map(|instr| {
                            let position = match instr.position {
                                Some(v) => v,
                                None => 0,
                            };

                            let instruction_value = if instr.arguments.len() > 0 {
                                let argument: String = instr
                                    .arguments
                                    .iter()
                                    .map(|byte| format!("{:02x}", byte).to_string())
                                    .collect();

                                format!("{} 0x{}", instr.opcode.to_string(), argument).to_string()
                            } else {
                                instr.opcode.to_string()
                            };

                            format!(
                                "[0x{:02x}: 0x{:02x}] {:<width$} ;; Stack: {}, Comment: {}, Source: {:?}",
                                position,
                                instr.opcode.as_u8(),
                                instruction_value,
                                instr.stack_size,
                                instr.comment.clone().unwrap_or("".to_string()).trim(),
                                instr.source_position,
                                width = 40
                            )
                        })
                        .collect::<Vec<String>>()
                        .join("\n");
                    let position = match block.position {
                        Some(v) => v,
                        None => 0,
                    };
                    format!(
                        "{}: ;; Starts at 0x{:02x} u8[{}]\n{}",
                        block.name, position, block.consumes, code
                    )
                })
                .collect::<Vec<String>>()
                .join("\n\n");

            script.push_str("\n\nFunction:\n");
            script.push_str(&code_blocks);
        }

        script
    }
}
