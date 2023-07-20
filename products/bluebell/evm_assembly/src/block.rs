use crate::instruction::EvmInstruction;
use crate::opcode_spec::OpcodeSpecification;
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
    pub fn new(position: Option<usize>, name: String) -> Self {
        Self {
            name,
            position,
            instructions: Vec::new(),
            entry_from: Vec::new(),
            is_entry: false,
            is_terminated: false,
            is_lookup_table: false,
        }
    }

    pub fn extract_blocks_from_bytecode(
        bytecode: &Vec<u8>,
        opcode_specs: &HashMap<u8, OpcodeSpecification>,
    ) -> (Vec<EvmBlock>, Vec<u8>) {
        let mut blocks: Vec<EvmBlock> = Vec::new();
        let mut block_counter = 0;
        let mut current_block =
            EvmBlock::new(Some(0), format!("block{}", block_counter).to_string());
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
                current_block = EvmBlock::new(
                    instr.position,
                    format!("block{}", block_counter).to_string(),
                );

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

    pub fn push(&mut self, opcode: Opcode, unresolved_label: Option<String>) -> &mut Self {
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

    pub fn stop(&mut self) -> &mut Self {
        self.push(Opcode::STOP, None)
    }

    pub fn add(&mut self) -> &mut Self {
        self.push(Opcode::ADD, None)
    }

    pub fn mul(&mut self) -> &mut Self {
        self.push(Opcode::MUL, None)
    }

    pub fn sub(&mut self) -> &mut Self {
        self.push(Opcode::SUB, None)
    }

    pub fn div(&mut self) -> &mut Self {
        self.push(Opcode::DIV, None)
    }

    pub fn sdiv(&mut self) -> &mut Self {
        self.push(Opcode::SDIV, None)
    }

    pub fn r#mod(&mut self) -> &mut Self {
        self.push(Opcode::MOD, None)
    }

    pub fn smod(&mut self) -> &mut Self {
        self.push(Opcode::SMOD, None)
    }

    pub fn addmod(&mut self) -> &mut Self {
        self.push(Opcode::ADDMOD, None)
    }

    pub fn mulmod(&mut self) -> &mut Self {
        self.push(Opcode::MULMOD, None)
    }

    pub fn exp(&mut self) -> &mut Self {
        self.push(Opcode::EXP, None)
    }

    pub fn signextend(&mut self) -> &mut Self {
        self.push(Opcode::SIGNEXTEND, None)
    }

    pub fn lt(&mut self) -> &mut Self {
        self.push(Opcode::LT, None)
    }

    pub fn gt(&mut self) -> &mut Self {
        self.push(Opcode::GT, None)
    }

    pub fn slt(&mut self) -> &mut Self {
        self.push(Opcode::SLT, None)
    }

    pub fn sgt(&mut self) -> &mut Self {
        self.push(Opcode::SGT, None)
    }

    pub fn eq(&mut self) -> &mut Self {
        self.push(Opcode::EQ, None)
    }

    pub fn iszero(&mut self) -> &mut Self {
        self.push(Opcode::ISZERO, None)
    }

    pub fn and(&mut self) -> &mut Self {
        self.push(Opcode::AND, None)
    }

    pub fn or(&mut self) -> &mut Self {
        self.push(Opcode::OR, None)
    }

    pub fn xor(&mut self) -> &mut Self {
        self.push(Opcode::XOR, None)
    }

    pub fn not(&mut self) -> &mut Self {
        self.push(Opcode::NOT, None)
    }

    pub fn byte(&mut self) -> &mut Self {
        self.push(Opcode::BYTE, None)
    }

    pub fn calldataload(&mut self) -> &mut Self {
        self.push(Opcode::CALLDATALOAD, None)
    }

    pub fn calldatasize(&mut self) -> &mut Self {
        self.push(Opcode::CALLDATASIZE, None)
    }

    pub fn calldatacopy(&mut self) -> &mut Self {
        self.push(Opcode::CALLDATACOPY, None)
    }

    pub fn codesize(&mut self) -> &mut Self {
        self.push(Opcode::CODESIZE, None)
    }

    pub fn codecopy(&mut self) -> &mut Self {
        self.push(Opcode::CODECOPY, None)
    }

    pub fn shl(&mut self) -> &mut Self {
        self.push(Opcode::SHL, None)
    }

    pub fn shr(&mut self) -> &mut Self {
        self.push(Opcode::SHR, None)
    }

    pub fn sar(&mut self) -> &mut Self {
        self.push(Opcode::SAR, None)
    }

    pub fn pop(&mut self) -> &mut Self {
        self.push(Opcode::POP, None)
    }

    pub fn mload(&mut self) -> &mut Self {
        self.push(Opcode::MLOAD, None)
    }

    pub fn mstore(&mut self) -> &mut Self {
        self.push(Opcode::MSTORE, None)
    }

    pub fn mstore8(&mut self) -> &mut Self {
        self.push(Opcode::MSTORE8, None)
    }

    pub fn jump(&mut self) -> &mut Self {
        self.push(Opcode::JUMP, None)
    }

    pub fn jumpi(&mut self) -> &mut Self {
        self.push(Opcode::JUMPI, None)
    }

    pub fn jump_to(&mut self, label: String) -> &mut Self {
        self.push(Opcode::JUMP, Some(label))
    }

    pub fn if_jump_to(&mut self, label: String) -> &mut Self {
        self.push(Opcode::JUMPI, Some(label))
    }

    pub fn pc(&mut self) -> &mut Self {
        self.push(Opcode::PC, None)
    }

    pub fn msize(&mut self) -> &mut Self {
        self.push(Opcode::MSIZE, None)
    }

    pub fn jumpdest(&mut self) -> &mut Self {
        self.push(Opcode::JUMPDEST, None)
    }

    /*
    pub fn push0(&mut self) -> &mut Self {
        self.push(Opcode::PUSH0, None)
    }
    */

    pub fn push1(&mut self) -> &mut Self {
        self.push(Opcode::PUSH1, None)
    }

    pub fn push2(&mut self) -> &mut Self {
        self.push(Opcode::PUSH2, None)
    }

    pub fn push3(&mut self) -> &mut Self {
        self.push(Opcode::PUSH3, None)
    }

    pub fn push4(&mut self) -> &mut Self {
        self.push(Opcode::PUSH4, None)
    }

    pub fn push5(&mut self) -> &mut Self {
        self.push(Opcode::PUSH5, None)
    }

    pub fn push6(&mut self) -> &mut Self {
        self.push(Opcode::PUSH6, None)
    }

    pub fn push7(&mut self) -> &mut Self {
        self.push(Opcode::PUSH7, None)
    }

    pub fn push8(&mut self) -> &mut Self {
        self.push(Opcode::PUSH8, None)
    }

    pub fn push9(&mut self) -> &mut Self {
        self.push(Opcode::PUSH9, None)
    }

    pub fn push10(&mut self) -> &mut Self {
        self.push(Opcode::PUSH10, None)
    }

    pub fn push11(&mut self) -> &mut Self {
        self.push(Opcode::PUSH11, None)
    }

    pub fn push12(&mut self) -> &mut Self {
        self.push(Opcode::PUSH12, None)
    }

    pub fn push13(&mut self) -> &mut Self {
        self.push(Opcode::PUSH13, None)
    }

    pub fn push14(&mut self) -> &mut Self {
        self.push(Opcode::PUSH14, None)
    }

    pub fn push15(&mut self) -> &mut Self {
        self.push(Opcode::PUSH15, None)
    }

    pub fn push16(&mut self) -> &mut Self {
        self.push(Opcode::PUSH16, None)
    }

    pub fn push17(&mut self) -> &mut Self {
        self.push(Opcode::PUSH17, None)
    }

    pub fn push18(&mut self) -> &mut Self {
        self.push(Opcode::PUSH18, None)
    }

    pub fn push19(&mut self) -> &mut Self {
        self.push(Opcode::PUSH19, None)
    }

    pub fn push20(&mut self) -> &mut Self {
        self.push(Opcode::PUSH20, None)
    }

    pub fn push21(&mut self) -> &mut Self {
        self.push(Opcode::PUSH21, None)
    }

    pub fn push22(&mut self) -> &mut Self {
        self.push(Opcode::PUSH22, None)
    }

    pub fn push23(&mut self) -> &mut Self {
        self.push(Opcode::PUSH23, None)
    }

    pub fn push24(&mut self) -> &mut Self {
        self.push(Opcode::PUSH24, None)
    }

    pub fn push25(&mut self) -> &mut Self {
        self.push(Opcode::PUSH25, None)
    }

    pub fn push26(&mut self) -> &mut Self {
        self.push(Opcode::PUSH26, None)
    }

    pub fn push27(&mut self) -> &mut Self {
        self.push(Opcode::PUSH27, None)
    }

    pub fn push28(&mut self) -> &mut Self {
        self.push(Opcode::PUSH28, None)
    }

    pub fn push29(&mut self) -> &mut Self {
        self.push(Opcode::PUSH29, None)
    }

    pub fn push30(&mut self) -> &mut Self {
        self.push(Opcode::PUSH30, None)
    }

    pub fn push31(&mut self) -> &mut Self {
        self.push(Opcode::PUSH31, None)
    }

    pub fn push32(&mut self) -> &mut Self {
        self.push(Opcode::PUSH32, None)
    }

    pub fn dup1(&mut self) -> &mut Self {
        self.push(Opcode::DUP1, None)
    }

    pub fn dup2(&mut self) -> &mut Self {
        self.push(Opcode::DUP2, None)
    }

    pub fn dup3(&mut self) -> &mut Self {
        self.push(Opcode::DUP3, None)
    }

    pub fn dup4(&mut self) -> &mut Self {
        self.push(Opcode::DUP4, None)
    }

    pub fn dup5(&mut self) -> &mut Self {
        self.push(Opcode::DUP5, None)
    }

    pub fn dup6(&mut self) -> &mut Self {
        self.push(Opcode::DUP6, None)
    }

    pub fn dup7(&mut self) -> &mut Self {
        self.push(Opcode::DUP7, None)
    }

    pub fn dup8(&mut self) -> &mut Self {
        self.push(Opcode::DUP8, None)
    }

    pub fn dup9(&mut self) -> &mut Self {
        self.push(Opcode::DUP9, None)
    }

    pub fn dup10(&mut self) -> &mut Self {
        self.push(Opcode::DUP10, None)
    }

    pub fn dup11(&mut self) -> &mut Self {
        self.push(Opcode::DUP11, None)
    }

    pub fn dup12(&mut self) -> &mut Self {
        self.push(Opcode::DUP12, None)
    }

    pub fn dup13(&mut self) -> &mut Self {
        self.push(Opcode::DUP13, None)
    }

    pub fn dup14(&mut self) -> &mut Self {
        self.push(Opcode::DUP14, None)
    }

    pub fn dup15(&mut self) -> &mut Self {
        self.push(Opcode::DUP15, None)
    }

    pub fn dup16(&mut self) -> &mut Self {
        self.push(Opcode::DUP16, None)
    }

    pub fn swap1(&mut self) -> &mut Self {
        self.push(Opcode::SWAP1, None)
    }

    pub fn swap2(&mut self) -> &mut Self {
        self.push(Opcode::SWAP2, None)
    }

    pub fn swap3(&mut self) -> &mut Self {
        self.push(Opcode::SWAP3, None)
    }

    pub fn swap4(&mut self) -> &mut Self {
        self.push(Opcode::SWAP4, None)
    }

    pub fn swap5(&mut self) -> &mut Self {
        self.push(Opcode::SWAP5, None)
    }

    pub fn swap6(&mut self) -> &mut Self {
        self.push(Opcode::SWAP6, None)
    }

    pub fn swap7(&mut self) -> &mut Self {
        self.push(Opcode::SWAP7, None)
    }

    pub fn swap8(&mut self) -> &mut Self {
        self.push(Opcode::SWAP8, None)
    }

    pub fn swap9(&mut self) -> &mut Self {
        self.push(Opcode::SWAP9, None)
    }

    pub fn swap10(&mut self) -> &mut Self {
        self.push(Opcode::SWAP10, None)
    }

    pub fn swap11(&mut self) -> &mut Self {
        self.push(Opcode::SWAP11, None)
    }

    pub fn swap12(&mut self) -> &mut Self {
        self.push(Opcode::SWAP12, None)
    }

    pub fn swap13(&mut self) -> &mut Self {
        self.push(Opcode::SWAP13, None)
    }

    pub fn swap14(&mut self) -> &mut Self {
        self.push(Opcode::SWAP14, None)
    }

    pub fn swap15(&mut self) -> &mut Self {
        self.push(Opcode::SWAP15, None)
    }

    pub fn swap16(&mut self) -> &mut Self {
        self.push(Opcode::SWAP16, None)
    }

    pub fn r#return(&mut self) -> &mut Self {
        self.push(Opcode::RETURN, None)
    }

    pub fn revert(&mut self) -> &mut Self {
        self.push(Opcode::REVERT, None)
    }

    pub fn invalid(&mut self) -> &mut Self {
        self.push(Opcode::INVALID, None)
    }

    pub fn eofmagic(&mut self) -> &mut Self {
        self.push(Opcode::EOFMAGIC, None)
    }

    // Externals
    pub fn external_sha3(&mut self) -> &mut Self {
        self.push(Opcode::SHA3, None)
    }
    pub fn external_address(&mut self) -> &mut Self {
        self.push(Opcode::ADDRESS, None)
    }
    pub fn external_balance(&mut self) -> &mut Self {
        self.push(Opcode::BALANCE, None)
    }
    pub fn external_selfbalance(&mut self) -> &mut Self {
        self.push(Opcode::SELFBALANCE, None)
    }
    pub fn external_basefee(&mut self) -> &mut Self {
        self.push(Opcode::BASEFEE, None)
    }
    pub fn external_origin(&mut self) -> &mut Self {
        self.push(Opcode::ORIGIN, None)
    }
    pub fn external_caller(&mut self) -> &mut Self {
        self.push(Opcode::CALLER, None)
    }
    pub fn external_callvalue(&mut self) -> &mut Self {
        self.push(Opcode::CALLVALUE, None)
    }
    pub fn external_gasprice(&mut self) -> &mut Self {
        self.push(Opcode::GASPRICE, None)
    }
    pub fn external_extcodesize(&mut self) -> &mut Self {
        self.push(Opcode::EXTCODESIZE, None)
    }
    pub fn external_extcodecopy(&mut self) -> &mut Self {
        self.push(Opcode::EXTCODECOPY, None)
    }
    pub fn external_extcodehash(&mut self) -> &mut Self {
        self.push(Opcode::EXTCODEHASH, None)
    }
    pub fn external_returndatasize(&mut self) -> &mut Self {
        self.push(Opcode::RETURNDATASIZE, None)
    }
    pub fn external_returndatacopy(&mut self) -> &mut Self {
        self.push(Opcode::RETURNDATACOPY, None)
    }
    pub fn external_blockhash(&mut self) -> &mut Self {
        self.push(Opcode::BLOCKHASH, None)
    }
    pub fn external_coinbase(&mut self) -> &mut Self {
        self.push(Opcode::COINBASE, None)
    }
    pub fn external_timestamp(&mut self) -> &mut Self {
        self.push(Opcode::TIMESTAMP, None)
    }
    pub fn external_number(&mut self) -> &mut Self {
        self.push(Opcode::NUMBER, None)
    }
    pub fn external_difficulty(&mut self) -> &mut Self {
        self.push(Opcode::DIFFICULTY, None)
    }
    pub fn external_gaslimit(&mut self) -> &mut Self {
        self.push(Opcode::GASLIMIT, None)
    }
    pub fn external_sload(&mut self) -> &mut Self {
        self.push(Opcode::SLOAD, None)
    }
    pub fn external_sstore(&mut self) -> &mut Self {
        self.push(Opcode::SSTORE, None)
    }
    pub fn external_gas(&mut self) -> &mut Self {
        self.push(Opcode::GAS, None)
    }
    pub fn external_log0(&mut self) -> &mut Self {
        self.push(Opcode::LOG0, None)
    }
    pub fn external_log1(&mut self) -> &mut Self {
        self.push(Opcode::LOG1, None)
    }
    pub fn external_log2(&mut self) -> &mut Self {
        self.push(Opcode::LOG2, None)
    }
    pub fn external_log3(&mut self) -> &mut Self {
        self.push(Opcode::LOG3, None)
    }
    pub fn external_log4(&mut self) -> &mut Self {
        self.push(Opcode::LOG4, None)
    }
    pub fn external_create(&mut self) -> &mut Self {
        self.push(Opcode::CREATE, None)
    }
    pub fn external_create2(&mut self) -> &mut Self {
        self.push(Opcode::CREATE2, None)
    }
    pub fn external_call(&mut self) -> &mut Self {
        self.push(Opcode::CALL, None)
    }
    pub fn external_callcode(&mut self) -> &mut Self {
        self.push(Opcode::CALLCODE, None)
    }
    pub fn external_delegatecall(&mut self) -> &mut Self {
        self.push(Opcode::DELEGATECALL, None)
    }
    pub fn external_staticcall(&mut self) -> &mut Self {
        self.push(Opcode::STATICCALL, None)
    }
    pub fn external_suicide(&mut self) -> &mut Self {
        self.push(Opcode::SUICIDE, None)
    }
    pub fn external_chainid(&mut self) -> &mut Self {
        self.push(Opcode::CHAINID, None)
    }
}

/*
// TODO: Everything block should be defined in block, not the builder

*/
