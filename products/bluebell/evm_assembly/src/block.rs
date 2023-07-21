use crate::function_signature::EvmFunctionSignature;
use crate::instruction::EvmInstruction;
use crate::opcode_spec::OpcodeSpecification;
use crate::types::EvmTypeValue;
use evm::Opcode;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct EvmBlock {
    pub name: String,
    pub position: Option<usize>,
    pub instructions: Vec<EvmInstruction>,
    pub entry_from: Vec<usize>,
    pub is_entry: bool,
    pub is_terminated: bool,
    pub is_lookup_table: bool,
}

impl EvmBlock {
    pub fn new(position: Option<usize>, name: &str) -> Self {
        let mut ret = Self {
            name: name.to_string(),
            position,
            instructions: Vec::new(),
            entry_from: Vec::new(),
            is_entry: false,
            is_terminated: false,
            is_lookup_table: false,
        };

        ret.jumpdest();

        ret
    }

    pub fn call(&mut self, function: &EvmFunctionSignature, args: Vec<EvmTypeValue>) -> &mut Self {
        println!("Calling {:?}", function);
        println!("Args: {:?}", args);
        let address = match function.external_address {
            Some(a) => a,
            None => panic!("TODO: Internal calls not supported yet."),
        };
        // TODO: Deal with internal calls
        self.push1([0x40].to_vec());
        self.mload(); // Stack element is the pointer

        for (i, _) in args.iter().enumerate().rev() {
            self.dup1();
            self.push1([(i * 0x20) as u8].to_vec());
            self.add();
            self.push1([0x20].to_vec()); // Length of the argument
            self.mstore();
        }

        for (i, arg) in args.iter().enumerate().rev() {
            let j = i + args.len();
            self.dup1();
            self.push1([(j * 0x20) as u8].to_vec());
            self.add();
            self.push(arg.to_bytes_unpadded());
            self.mstore();
        }

        let gas = EvmTypeValue::Uint32(21000);
        let address = EvmTypeValue::Uint32(address);
        let argsize = EvmTypeValue::Uint32(2 * (args.len() * 0x20) as u32); // Each argument is 32 byte long

        self.push([0x20].to_vec()); //return size, TODO
        self.dup2(); //
        self.push(argsize.to_bytes_unpadded());
        self.dup4(); // p
        self.push(address.to_bytes_unpadded());
        self.push(gas.to_bytes_unpadded());

        self.external_staticcall();

        self
    }

    pub fn extract_blocks_from_bytecode(
        bytecode: &Vec<u8>,
        opcode_specs: &HashMap<u8, OpcodeSpecification>,
    ) -> (Vec<EvmBlock>, Vec<u8>) {
        let mut blocks: Vec<EvmBlock> = Vec::new();
        let mut block_counter = 0;
        let mut current_block = EvmBlock::new(Some(0), &format!("block{}", block_counter));
        current_block.is_entry = true;
        block_counter += 1;

        let offset = 0;
        let mut i = offset;
        while i < bytecode.len() {
            let spec = match opcode_specs.get(&bytecode[i]) {
                Some(spec) => spec,
                _ => {
                    if let Some(instr) = current_block.instructions.last() {
                        println!("Last instruction:\n{:#?}", instr);
                        println!("Opcode name: {}", instr.opcode.to_string());
                    }
                    panic!("No spec found for opcode 0x{:02x}", bytecode[i]);
                }
            };

            let mut instr = EvmInstruction {
                position: Some(i),
                opcode: Opcode(bytecode[i]),
                arguments: Vec::new(),
                unresolved_label: None,

                stack_consumed: spec.stack_consumed,
                stack_produced: spec.stack_produced,
                is_terminator: spec.is_terminator,
            };

            i += 1;
            let mut collect_args = spec.bytecode_arguments;
            if i + collect_args > bytecode.len() {
                panic!("This is not good - we exceed the byte code");
            }

            while collect_args > 0 {
                instr.arguments.push(bytecode[i]);
                i += 1;
                collect_args -= 1;
            }

            if instr.opcode == Opcode::JUMPDEST {
                blocks.push(current_block);
                current_block = EvmBlock::new(instr.position, &format!("block{}", block_counter));

                block_counter += 1;
            }

            current_block.instructions.push(instr);

            // A terminated block followed by an invalid opcode starts the data section.
            // TODO: Find some spec to confirm this assumption
            if spec.is_terminator {
                if Opcode(bytecode[i]) == Opcode::INVALID {
                    i += 1;

                    // Encountered the auxilary data section
                    break;
                }
            }
        }
        println!("Terminating {} / {}", i, bytecode.len());
        let mut data: Vec<u8> = Vec::new();
        while i < bytecode.len() {
            data.push(bytecode[i]);
            i += 1;
        }

        blocks.push(current_block);
        (blocks, data)
    }

    pub fn write_instruction(
        &mut self,
        opcode: Opcode,
        unresolved_label: Option<String>,
    ) -> &mut Self {
        self.instructions.push(EvmInstruction {
            position: None,
            opcode,
            arguments: [].to_vec(),

            unresolved_label,

            stack_consumed: 0,
            stack_produced: 0,
            is_terminator: false,
        });
        self
    }

    pub fn write_instruction_with_args(&mut self, opcode: Opcode, arguments: Vec<u8>) -> &mut Self {
        self.instructions.push(EvmInstruction {
            position: None,
            opcode,
            arguments,

            unresolved_label: None,

            stack_consumed: 0,
            stack_produced: 0,
            is_terminator: false,
        });
        self
    }

    pub fn stop(&mut self) -> &mut Self {
        self.write_instruction(Opcode::STOP, None)
    }

    pub fn add(&mut self) -> &mut Self {
        self.write_instruction(Opcode::ADD, None)
    }

    pub fn mul(&mut self) -> &mut Self {
        self.write_instruction(Opcode::MUL, None)
    }

    pub fn sub(&mut self) -> &mut Self {
        self.write_instruction(Opcode::SUB, None)
    }

    pub fn div(&mut self) -> &mut Self {
        self.write_instruction(Opcode::DIV, None)
    }

    pub fn sdiv(&mut self) -> &mut Self {
        self.write_instruction(Opcode::SDIV, None)
    }

    pub fn r#mod(&mut self) -> &mut Self {
        self.write_instruction(Opcode::MOD, None)
    }

    pub fn smod(&mut self) -> &mut Self {
        self.write_instruction(Opcode::SMOD, None)
    }

    pub fn addmod(&mut self) -> &mut Self {
        self.write_instruction(Opcode::ADDMOD, None)
    }

    pub fn mulmod(&mut self) -> &mut Self {
        self.write_instruction(Opcode::MULMOD, None)
    }

    pub fn exp(&mut self) -> &mut Self {
        self.write_instruction(Opcode::EXP, None)
    }

    pub fn signextend(&mut self) -> &mut Self {
        self.write_instruction(Opcode::SIGNEXTEND, None)
    }

    pub fn lt(&mut self) -> &mut Self {
        self.write_instruction(Opcode::LT, None)
    }

    pub fn gt(&mut self) -> &mut Self {
        self.write_instruction(Opcode::GT, None)
    }

    pub fn slt(&mut self) -> &mut Self {
        self.write_instruction(Opcode::SLT, None)
    }

    pub fn sgt(&mut self) -> &mut Self {
        self.write_instruction(Opcode::SGT, None)
    }

    pub fn eq(&mut self) -> &mut Self {
        self.write_instruction(Opcode::EQ, None)
    }

    pub fn iszero(&mut self) -> &mut Self {
        self.write_instruction(Opcode::ISZERO, None)
    }

    pub fn and(&mut self) -> &mut Self {
        self.write_instruction(Opcode::AND, None)
    }

    pub fn or(&mut self) -> &mut Self {
        self.write_instruction(Opcode::OR, None)
    }

    pub fn xor(&mut self) -> &mut Self {
        self.write_instruction(Opcode::XOR, None)
    }

    pub fn not(&mut self) -> &mut Self {
        self.write_instruction(Opcode::NOT, None)
    }

    pub fn byte(&mut self) -> &mut Self {
        self.write_instruction(Opcode::BYTE, None)
    }

    pub fn calldataload(&mut self) -> &mut Self {
        self.write_instruction(Opcode::CALLDATALOAD, None)
    }

    pub fn calldatasize(&mut self) -> &mut Self {
        self.write_instruction(Opcode::CALLDATASIZE, None)
    }

    pub fn calldatacopy(&mut self) -> &mut Self {
        self.write_instruction(Opcode::CALLDATACOPY, None)
    }

    pub fn codesize(&mut self) -> &mut Self {
        self.write_instruction(Opcode::CODESIZE, None)
    }

    pub fn codecopy(&mut self) -> &mut Self {
        self.write_instruction(Opcode::CODECOPY, None)
    }

    pub fn shl(&mut self) -> &mut Self {
        self.write_instruction(Opcode::SHL, None)
    }

    pub fn shr(&mut self) -> &mut Self {
        self.write_instruction(Opcode::SHR, None)
    }

    pub fn sar(&mut self) -> &mut Self {
        self.write_instruction(Opcode::SAR, None)
    }

    pub fn pop(&mut self) -> &mut Self {
        self.write_instruction(Opcode::POP, None)
    }

    pub fn mload(&mut self) -> &mut Self {
        self.write_instruction(Opcode::MLOAD, None)
    }

    pub fn mstore(&mut self) -> &mut Self {
        self.write_instruction(Opcode::MSTORE, None)
    }

    pub fn mstore8(&mut self) -> &mut Self {
        self.write_instruction(Opcode::MSTORE8, None)
    }

    pub fn jump(&mut self) -> &mut Self {
        self.write_instruction(Opcode::JUMP, None)
    }

    pub fn jumpi(&mut self) -> &mut Self {
        self.write_instruction(Opcode::JUMPI, None)
    }

    pub fn jump_to(&mut self, label: &str) -> &mut Self {
        self.write_instruction(Opcode::PUSH4, Some(label.to_string()));
        self.write_instruction(Opcode::JUMP, None)
    }

    pub fn jump_if_to(&mut self, label: &str) -> &mut Self {
        self.write_instruction(Opcode::PUSH4, Some(label.to_string()));
        self.write_instruction(Opcode::JUMPI, None)
    }

    pub fn pc(&mut self) -> &mut Self {
        self.write_instruction(Opcode::PC, None)
    }

    pub fn msize(&mut self) -> &mut Self {
        self.write_instruction(Opcode::MSIZE, None)
    }

    pub fn jumpdest(&mut self) -> &mut Self {
        self.write_instruction(Opcode::JUMPDEST, None)
    }

    /*
    pub fn push0(&mut self) -> &mut Self {
        self.write_instruction(Opcode::PUSH0, None)
    }
    */

    pub fn push(&mut self, arguments: Vec<u8>) -> &mut Self {
        match arguments.len() {
            // TODO: 0 => self.push0(arguments),
            1 => self.push1(arguments),
            2 => self.push2(arguments),
            3 => self.push3(arguments),
            4 => self.push4(arguments),
            5 => self.push5(arguments),
            6 => self.push6(arguments),
            7 => self.push7(arguments),
            8 => self.push8(arguments),
            9 => self.push9(arguments),

            10 => self.push10(arguments),
            11 => self.push11(arguments),
            12 => self.push12(arguments),
            13 => self.push13(arguments),
            14 => self.push14(arguments),
            15 => self.push15(arguments),
            16 => self.push16(arguments),
            17 => self.push17(arguments),
            18 => self.push18(arguments),
            19 => self.push19(arguments),

            20 => self.push20(arguments),
            21 => self.push21(arguments),
            22 => self.push22(arguments),
            23 => self.push23(arguments),
            24 => self.push24(arguments),
            25 => self.push25(arguments),
            26 => self.push26(arguments),
            27 => self.push27(arguments),
            28 => self.push28(arguments),
            29 => self.push29(arguments),

            30 => self.push30(arguments),
            31 => self.push31(arguments),
            32 => self.push32(arguments),
            _ => panic!("Push size not supported."),
        }
    }

    pub fn push1(&mut self, arguments: Vec<u8>) -> &mut Self {
        self.write_instruction_with_args(Opcode::PUSH1, arguments)
    }

    pub fn push2(&mut self, arguments: Vec<u8>) -> &mut Self {
        self.write_instruction_with_args(Opcode::PUSH2, arguments)
    }

    pub fn push3(&mut self, arguments: Vec<u8>) -> &mut Self {
        self.write_instruction_with_args(Opcode::PUSH3, arguments)
    }

    pub fn push4(&mut self, arguments: Vec<u8>) -> &mut Self {
        self.write_instruction_with_args(Opcode::PUSH4, arguments)
    }

    pub fn push5(&mut self, arguments: Vec<u8>) -> &mut Self {
        self.write_instruction_with_args(Opcode::PUSH5, arguments)
    }

    pub fn push6(&mut self, arguments: Vec<u8>) -> &mut Self {
        self.write_instruction_with_args(Opcode::PUSH6, arguments)
    }

    pub fn push7(&mut self, arguments: Vec<u8>) -> &mut Self {
        self.write_instruction_with_args(Opcode::PUSH7, arguments)
    }

    pub fn push8(&mut self, arguments: Vec<u8>) -> &mut Self {
        self.write_instruction_with_args(Opcode::PUSH8, arguments)
    }

    pub fn push9(&mut self, arguments: Vec<u8>) -> &mut Self {
        self.write_instruction_with_args(Opcode::PUSH9, arguments)
    }

    pub fn push10(&mut self, arguments: Vec<u8>) -> &mut Self {
        self.write_instruction_with_args(Opcode::PUSH10, arguments)
    }

    pub fn push11(&mut self, arguments: Vec<u8>) -> &mut Self {
        self.write_instruction_with_args(Opcode::PUSH11, arguments)
    }

    pub fn push12(&mut self, arguments: Vec<u8>) -> &mut Self {
        self.write_instruction_with_args(Opcode::PUSH12, arguments)
    }

    pub fn push13(&mut self, arguments: Vec<u8>) -> &mut Self {
        self.write_instruction_with_args(Opcode::PUSH13, arguments)
    }

    pub fn push14(&mut self, arguments: Vec<u8>) -> &mut Self {
        self.write_instruction_with_args(Opcode::PUSH14, arguments)
    }

    pub fn push15(&mut self, arguments: Vec<u8>) -> &mut Self {
        self.write_instruction_with_args(Opcode::PUSH15, arguments)
    }

    pub fn push16(&mut self, arguments: Vec<u8>) -> &mut Self {
        self.write_instruction_with_args(Opcode::PUSH16, arguments)
    }

    pub fn push17(&mut self, arguments: Vec<u8>) -> &mut Self {
        self.write_instruction_with_args(Opcode::PUSH17, arguments)
    }

    pub fn push18(&mut self, arguments: Vec<u8>) -> &mut Self {
        self.write_instruction_with_args(Opcode::PUSH18, arguments)
    }

    pub fn push19(&mut self, arguments: Vec<u8>) -> &mut Self {
        self.write_instruction_with_args(Opcode::PUSH19, arguments)
    }

    pub fn push20(&mut self, arguments: Vec<u8>) -> &mut Self {
        self.write_instruction_with_args(Opcode::PUSH20, arguments)
    }

    pub fn push21(&mut self, arguments: Vec<u8>) -> &mut Self {
        self.write_instruction_with_args(Opcode::PUSH21, arguments)
    }

    pub fn push22(&mut self, arguments: Vec<u8>) -> &mut Self {
        self.write_instruction_with_args(Opcode::PUSH22, arguments)
    }

    pub fn push23(&mut self, arguments: Vec<u8>) -> &mut Self {
        self.write_instruction_with_args(Opcode::PUSH23, arguments)
    }

    pub fn push24(&mut self, arguments: Vec<u8>) -> &mut Self {
        self.write_instruction_with_args(Opcode::PUSH24, arguments)
    }

    pub fn push25(&mut self, arguments: Vec<u8>) -> &mut Self {
        self.write_instruction_with_args(Opcode::PUSH25, arguments)
    }

    pub fn push26(&mut self, arguments: Vec<u8>) -> &mut Self {
        self.write_instruction_with_args(Opcode::PUSH26, arguments)
    }

    pub fn push27(&mut self, arguments: Vec<u8>) -> &mut Self {
        self.write_instruction_with_args(Opcode::PUSH27, arguments)
    }

    pub fn push28(&mut self, arguments: Vec<u8>) -> &mut Self {
        self.write_instruction_with_args(Opcode::PUSH28, arguments)
    }

    pub fn push29(&mut self, arguments: Vec<u8>) -> &mut Self {
        self.write_instruction_with_args(Opcode::PUSH29, arguments)
    }

    pub fn push30(&mut self, arguments: Vec<u8>) -> &mut Self {
        self.write_instruction_with_args(Opcode::PUSH30, arguments)
    }

    pub fn push31(&mut self, arguments: Vec<u8>) -> &mut Self {
        self.write_instruction_with_args(Opcode::PUSH31, arguments)
    }

    pub fn push32(&mut self, arguments: Vec<u8>) -> &mut Self {
        self.write_instruction_with_args(Opcode::PUSH32, arguments)
    }

    pub fn dup1(&mut self) -> &mut Self {
        self.write_instruction(Opcode::DUP1, None)
    }

    pub fn dup2(&mut self) -> &mut Self {
        self.write_instruction(Opcode::DUP2, None)
    }

    pub fn dup3(&mut self) -> &mut Self {
        self.write_instruction(Opcode::DUP3, None)
    }

    pub fn dup4(&mut self) -> &mut Self {
        self.write_instruction(Opcode::DUP4, None)
    }

    pub fn dup5(&mut self) -> &mut Self {
        self.write_instruction(Opcode::DUP5, None)
    }

    pub fn dup6(&mut self) -> &mut Self {
        self.write_instruction(Opcode::DUP6, None)
    }

    pub fn dup7(&mut self) -> &mut Self {
        self.write_instruction(Opcode::DUP7, None)
    }

    pub fn dup8(&mut self) -> &mut Self {
        self.write_instruction(Opcode::DUP8, None)
    }

    pub fn dup9(&mut self) -> &mut Self {
        self.write_instruction(Opcode::DUP9, None)
    }

    pub fn dup10(&mut self) -> &mut Self {
        self.write_instruction(Opcode::DUP10, None)
    }

    pub fn dup11(&mut self) -> &mut Self {
        self.write_instruction(Opcode::DUP11, None)
    }

    pub fn dup12(&mut self) -> &mut Self {
        self.write_instruction(Opcode::DUP12, None)
    }

    pub fn dup13(&mut self) -> &mut Self {
        self.write_instruction(Opcode::DUP13, None)
    }

    pub fn dup14(&mut self) -> &mut Self {
        self.write_instruction(Opcode::DUP14, None)
    }

    pub fn dup15(&mut self) -> &mut Self {
        self.write_instruction(Opcode::DUP15, None)
    }

    pub fn dup16(&mut self) -> &mut Self {
        self.write_instruction(Opcode::DUP16, None)
    }

    pub fn swap1(&mut self) -> &mut Self {
        self.write_instruction(Opcode::SWAP1, None)
    }

    pub fn swap2(&mut self) -> &mut Self {
        self.write_instruction(Opcode::SWAP2, None)
    }

    pub fn swap3(&mut self) -> &mut Self {
        self.write_instruction(Opcode::SWAP3, None)
    }

    pub fn swap4(&mut self) -> &mut Self {
        self.write_instruction(Opcode::SWAP4, None)
    }

    pub fn swap5(&mut self) -> &mut Self {
        self.write_instruction(Opcode::SWAP5, None)
    }

    pub fn swap6(&mut self) -> &mut Self {
        self.write_instruction(Opcode::SWAP6, None)
    }

    pub fn swap7(&mut self) -> &mut Self {
        self.write_instruction(Opcode::SWAP7, None)
    }

    pub fn swap8(&mut self) -> &mut Self {
        self.write_instruction(Opcode::SWAP8, None)
    }

    pub fn swap9(&mut self) -> &mut Self {
        self.write_instruction(Opcode::SWAP9, None)
    }

    pub fn swap10(&mut self) -> &mut Self {
        self.write_instruction(Opcode::SWAP10, None)
    }

    pub fn swap11(&mut self) -> &mut Self {
        self.write_instruction(Opcode::SWAP11, None)
    }

    pub fn swap12(&mut self) -> &mut Self {
        self.write_instruction(Opcode::SWAP12, None)
    }

    pub fn swap13(&mut self) -> &mut Self {
        self.write_instruction(Opcode::SWAP13, None)
    }

    pub fn swap14(&mut self) -> &mut Self {
        self.write_instruction(Opcode::SWAP14, None)
    }

    pub fn swap15(&mut self) -> &mut Self {
        self.write_instruction(Opcode::SWAP15, None)
    }

    pub fn swap16(&mut self) -> &mut Self {
        self.write_instruction(Opcode::SWAP16, None)
    }

    pub fn r#return(&mut self) -> &mut Self {
        self.write_instruction(Opcode::RETURN, None)
    }

    pub fn revert(&mut self) -> &mut Self {
        self.write_instruction(Opcode::REVERT, None)
    }

    pub fn invalid(&mut self) -> &mut Self {
        self.write_instruction(Opcode::INVALID, None)
    }

    pub fn eofmagic(&mut self) -> &mut Self {
        self.write_instruction(Opcode::EOFMAGIC, None)
    }

    // Externals
    pub fn external_sha3(&mut self) -> &mut Self {
        self.write_instruction(Opcode::SHA3, None)
    }
    pub fn external_address(&mut self) -> &mut Self {
        self.write_instruction(Opcode::ADDRESS, None)
    }
    pub fn external_balance(&mut self) -> &mut Self {
        self.write_instruction(Opcode::BALANCE, None)
    }
    pub fn external_selfbalance(&mut self) -> &mut Self {
        self.write_instruction(Opcode::SELFBALANCE, None)
    }
    pub fn external_basefee(&mut self) -> &mut Self {
        self.write_instruction(Opcode::BASEFEE, None)
    }
    pub fn external_origin(&mut self) -> &mut Self {
        self.write_instruction(Opcode::ORIGIN, None)
    }
    pub fn external_caller(&mut self) -> &mut Self {
        self.write_instruction(Opcode::CALLER, None)
    }
    pub fn external_callvalue(&mut self) -> &mut Self {
        self.write_instruction(Opcode::CALLVALUE, None)
    }
    pub fn external_gasprice(&mut self) -> &mut Self {
        self.write_instruction(Opcode::GASPRICE, None)
    }
    pub fn external_extcodesize(&mut self) -> &mut Self {
        self.write_instruction(Opcode::EXTCODESIZE, None)
    }
    pub fn external_extcodecopy(&mut self) -> &mut Self {
        self.write_instruction(Opcode::EXTCODECOPY, None)
    }
    pub fn external_extcodehash(&mut self) -> &mut Self {
        self.write_instruction(Opcode::EXTCODEHASH, None)
    }
    pub fn external_returndatasize(&mut self) -> &mut Self {
        self.write_instruction(Opcode::RETURNDATASIZE, None)
    }
    pub fn external_returndatacopy(&mut self) -> &mut Self {
        self.write_instruction(Opcode::RETURNDATACOPY, None)
    }
    pub fn external_blockhash(&mut self) -> &mut Self {
        self.write_instruction(Opcode::BLOCKHASH, None)
    }
    pub fn external_coinbase(&mut self) -> &mut Self {
        self.write_instruction(Opcode::COINBASE, None)
    }
    pub fn external_timestamp(&mut self) -> &mut Self {
        self.write_instruction(Opcode::TIMESTAMP, None)
    }
    pub fn external_number(&mut self) -> &mut Self {
        self.write_instruction(Opcode::NUMBER, None)
    }
    pub fn external_difficulty(&mut self) -> &mut Self {
        self.write_instruction(Opcode::DIFFICULTY, None)
    }
    pub fn external_gaslimit(&mut self) -> &mut Self {
        self.write_instruction(Opcode::GASLIMIT, None)
    }
    pub fn external_sload(&mut self) -> &mut Self {
        self.write_instruction(Opcode::SLOAD, None)
    }
    pub fn external_sstore(&mut self) -> &mut Self {
        self.write_instruction(Opcode::SSTORE, None)
    }
    pub fn external_gas(&mut self) -> &mut Self {
        self.write_instruction(Opcode::GAS, None)
    }
    pub fn external_log0(&mut self) -> &mut Self {
        self.write_instruction(Opcode::LOG0, None)
    }
    pub fn external_log1(&mut self) -> &mut Self {
        self.write_instruction(Opcode::LOG1, None)
    }
    pub fn external_log2(&mut self) -> &mut Self {
        self.write_instruction(Opcode::LOG2, None)
    }
    pub fn external_log3(&mut self) -> &mut Self {
        self.write_instruction(Opcode::LOG3, None)
    }
    pub fn external_log4(&mut self) -> &mut Self {
        self.write_instruction(Opcode::LOG4, None)
    }
    pub fn external_create(&mut self) -> &mut Self {
        self.write_instruction(Opcode::CREATE, None)
    }
    pub fn external_create2(&mut self) -> &mut Self {
        self.write_instruction(Opcode::CREATE2, None)
    }
    pub fn external_call(&mut self) -> &mut Self {
        self.write_instruction(Opcode::CALL, None)
    }
    pub fn external_callcode(&mut self) -> &mut Self {
        self.write_instruction(Opcode::CALLCODE, None)
    }
    pub fn external_delegatecall(&mut self) -> &mut Self {
        self.write_instruction(Opcode::DELEGATECALL, None)
    }
    pub fn external_staticcall(&mut self) -> &mut Self {
        self.write_instruction(Opcode::STATICCALL, None)
    }
    pub fn external_suicide(&mut self) -> &mut Self {
        self.write_instruction(Opcode::SUICIDE, None)
    }
    pub fn external_chainid(&mut self) -> &mut Self {
        self.write_instruction(Opcode::CHAINID, None)
    }
}

/*
// TODO: Everything block should be defined in block, not the builder

*/
