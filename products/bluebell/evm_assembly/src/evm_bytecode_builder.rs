use evm::Opcode;

use std::collections::HashMap;

use self::evm_decompiler::EvmBytecodeToAssembly;

pub struct EvmByteCodeBuilder {
    bytecode: Vec<u8>,
    labels: HashMap<String, usize>,
}

impl EvmByteCodeBuilder {
    pub fn new() -> Self {
        Self {
            bytecode: Vec::new(),
            labels: HashMap::new(),
        }
    }

    pub fn from_bytes(_bytes: Vec<u8>) -> Self {
        unimplemented!();
    }

    pub fn from_asm(script: &str) -> Self {
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

    pub fn stop(&mut self) -> &mut Self {
        self.push(Opcode::STOP);
        self
    }

    pub fn add(&mut self) -> &mut Self {
        self.push(Opcode::ADD);
        self
    }

    pub fn mul(&mut self) -> &mut Self {
        self.push(Opcode::MUL);
        self
    }

    pub fn sub(&mut self) -> &mut Self {
        self.push(Opcode::SUB);
        self
    }

    pub fn div(&mut self) -> &mut Self {
        self.push(Opcode::DIV);
        self
    }

    pub fn sdiv(&mut self) -> &mut Self {
        self.push(Opcode::SDIV);
        self
    }

    pub fn r#mod(&mut self) -> &mut Self {
        self.push(Opcode::MOD);
        self
    }

    pub fn smod(&mut self) -> &mut Self {
        self.push(Opcode::SMOD);
        self
    }

    pub fn addmod(&mut self) -> &mut Self {
        self.push(Opcode::ADDMOD);
        self
    }

    pub fn mulmod(&mut self) -> &mut Self {
        self.push(Opcode::MULMOD);
        self
    }

    pub fn exp(&mut self) -> &mut Self {
        self.push(Opcode::EXP);
        self
    }

    pub fn signextend(&mut self) -> &mut Self {
        self.push(Opcode::SIGNEXTEND);
        self
    }

    pub fn lt(&mut self) -> &mut Self {
        self.push(Opcode::LT);
        self
    }

    pub fn gt(&mut self) -> &mut Self {
        self.push(Opcode::GT);
        self
    }

    pub fn slt(&mut self) -> &mut Self {
        self.push(Opcode::SLT);
        self
    }

    pub fn sgt(&mut self) -> &mut Self {
        self.push(Opcode::SGT);
        self
    }

    pub fn eq(&mut self) -> &mut Self {
        self.push(Opcode::EQ);
        self
    }

    pub fn iszero(&mut self) -> &mut Self {
        self.push(Opcode::ISZERO);
        self
    }

    pub fn and(&mut self) -> &mut Self {
        self.push(Opcode::AND);
        self
    }

    pub fn or(&mut self) -> &mut Self {
        self.push(Opcode::OR);
        self
    }

    pub fn xor(&mut self) -> &mut Self {
        self.push(Opcode::XOR);
        self
    }

    pub fn not(&mut self) -> &mut Self {
        self.push(Opcode::NOT);
        self
    }

    pub fn byte(&mut self) -> &mut Self {
        self.push(Opcode::BYTE);
        self
    }

    pub fn calldataload(&mut self) -> &mut Self {
        self.push(Opcode::CALLDATALOAD);
        self
    }

    pub fn calldatasize(&mut self) -> &mut Self {
        self.push(Opcode::CALLDATASIZE);
        self
    }

    pub fn calldatacopy(&mut self) -> &mut Self {
        self.push(Opcode::CALLDATACOPY);
        self
    }

    pub fn codesize(&mut self) -> &mut Self {
        self.push(Opcode::CODESIZE);
        self
    }

    pub fn codecopy(&mut self) -> &mut Self {
        self.push(Opcode::CODECOPY);
        self
    }

    pub fn shl(&mut self) -> &mut Self {
        self.push(Opcode::SHL);
        self
    }

    pub fn shr(&mut self) -> &mut Self {
        self.push(Opcode::SHR);
        self
    }

    pub fn sar(&mut self) -> &mut Self {
        self.push(Opcode::SAR);
        self
    }

    pub fn pop(&mut self) -> &mut Self {
        self.push(Opcode::POP);
        self
    }

    pub fn mload(&mut self) -> &mut Self {
        self.push(Opcode::MLOAD);
        self
    }

    pub fn mstore(&mut self) -> &mut Self {
        self.push(Opcode::MSTORE);
        self
    }

    pub fn mstore8(&mut self) -> &mut Self {
        self.push(Opcode::MSTORE8);
        self
    }

    pub fn jump(&mut self) -> &mut Self {
        self.push(Opcode::JUMP);
        self
    }

    pub fn jumpi(&mut self) -> &mut Self {
        self.push(Opcode::JUMPI);
        self
    }

    pub fn pc(&mut self) -> &mut Self {
        self.push(Opcode::PC);
        self
    }

    pub fn msize(&mut self) -> &mut Self {
        self.push(Opcode::MSIZE);
        self
    }

    pub fn jumpdest(&mut self) -> &mut Self {
        self.push(Opcode::JUMPDEST);
        self
    }

    /*
    pub fn push0(&mut self) -> &mut Self {
        self.push(Opcode::PUSH0);
        self
    }
    */

    pub fn push1(&mut self) -> &mut Self {
        self.push(Opcode::PUSH1);
        self
    }

    pub fn push2(&mut self) -> &mut Self {
        self.push(Opcode::PUSH2);
        self
    }

    pub fn push3(&mut self) -> &mut Self {
        self.push(Opcode::PUSH3);
        self
    }

    pub fn push4(&mut self) -> &mut Self {
        self.push(Opcode::PUSH4);
        self
    }

    pub fn push5(&mut self) -> &mut Self {
        self.push(Opcode::PUSH5);
        self
    }

    pub fn push6(&mut self) -> &mut Self {
        self.push(Opcode::PUSH6);
        self
    }

    pub fn push7(&mut self) -> &mut Self {
        self.push(Opcode::PUSH7);
        self
    }

    pub fn push8(&mut self) -> &mut Self {
        self.push(Opcode::PUSH8);
        self
    }

    pub fn push9(&mut self) -> &mut Self {
        self.push(Opcode::PUSH9);
        self
    }

    pub fn push10(&mut self) -> &mut Self {
        self.push(Opcode::PUSH10);
        self
    }

    pub fn push11(&mut self) -> &mut Self {
        self.push(Opcode::PUSH11);
        self
    }

    pub fn push12(&mut self) -> &mut Self {
        self.push(Opcode::PUSH12);
        self
    }

    pub fn push13(&mut self) -> &mut Self {
        self.push(Opcode::PUSH13);
        self
    }

    pub fn push14(&mut self) -> &mut Self {
        self.push(Opcode::PUSH14);
        self
    }

    pub fn push15(&mut self) -> &mut Self {
        self.push(Opcode::PUSH15);
        self
    }

    pub fn push16(&mut self) -> &mut Self {
        self.push(Opcode::PUSH16);
        self
    }

    pub fn push17(&mut self) -> &mut Self {
        self.push(Opcode::PUSH17);
        self
    }

    pub fn push18(&mut self) -> &mut Self {
        self.push(Opcode::PUSH18);
        self
    }

    pub fn push19(&mut self) -> &mut Self {
        self.push(Opcode::PUSH19);
        self
    }

    pub fn push20(&mut self) -> &mut Self {
        self.push(Opcode::PUSH20);
        self
    }

    pub fn push21(&mut self) -> &mut Self {
        self.push(Opcode::PUSH21);
        self
    }

    pub fn push22(&mut self) -> &mut Self {
        self.push(Opcode::PUSH22);
        self
    }

    pub fn push23(&mut self) -> &mut Self {
        self.push(Opcode::PUSH23);
        self
    }

    pub fn push24(&mut self) -> &mut Self {
        self.push(Opcode::PUSH24);
        self
    }

    pub fn push25(&mut self) -> &mut Self {
        self.push(Opcode::PUSH25);
        self
    }

    pub fn push26(&mut self) -> &mut Self {
        self.push(Opcode::PUSH26);
        self
    }

    pub fn push27(&mut self) -> &mut Self {
        self.push(Opcode::PUSH27);
        self
    }

    pub fn push28(&mut self) -> &mut Self {
        self.push(Opcode::PUSH28);
        self
    }

    pub fn push29(&mut self) -> &mut Self {
        self.push(Opcode::PUSH29);
        self
    }

    pub fn push30(&mut self) -> &mut Self {
        self.push(Opcode::PUSH30);
        self
    }

    pub fn push31(&mut self) -> &mut Self {
        self.push(Opcode::PUSH31);
        self
    }

    pub fn push32(&mut self) -> &mut Self {
        self.push(Opcode::PUSH32);
        self
    }

    pub fn dup1(&mut self) -> &mut Self {
        self.push(Opcode::DUP1);
        self
    }

    pub fn dup2(&mut self) -> &mut Self {
        self.push(Opcode::DUP2);
        self
    }

    pub fn dup3(&mut self) -> &mut Self {
        self.push(Opcode::DUP3);
        self
    }

    pub fn dup4(&mut self) -> &mut Self {
        self.push(Opcode::DUP4);
        self
    }

    pub fn dup5(&mut self) -> &mut Self {
        self.push(Opcode::DUP5);
        self
    }

    pub fn dup6(&mut self) -> &mut Self {
        self.push(Opcode::DUP6);
        self
    }

    pub fn dup7(&mut self) -> &mut Self {
        self.push(Opcode::DUP7);
        self
    }

    pub fn dup8(&mut self) -> &mut Self {
        self.push(Opcode::DUP8);
        self
    }

    pub fn dup9(&mut self) -> &mut Self {
        self.push(Opcode::DUP9);
        self
    }

    pub fn dup10(&mut self) -> &mut Self {
        self.push(Opcode::DUP10);
        self
    }

    pub fn dup11(&mut self) -> &mut Self {
        self.push(Opcode::DUP11);
        self
    }

    pub fn dup12(&mut self) -> &mut Self {
        self.push(Opcode::DUP12);
        self
    }

    pub fn dup13(&mut self) -> &mut Self {
        self.push(Opcode::DUP13);
        self
    }

    pub fn dup14(&mut self) -> &mut Self {
        self.push(Opcode::DUP14);
        self
    }

    pub fn dup15(&mut self) -> &mut Self {
        self.push(Opcode::DUP15);
        self
    }

    pub fn dup16(&mut self) -> &mut Self {
        self.push(Opcode::DUP16);
        self
    }

    pub fn swap1(&mut self) -> &mut Self {
        self.push(Opcode::SWAP1);
        self
    }

    pub fn swap2(&mut self) -> &mut Self {
        self.push(Opcode::SWAP2);
        self
    }

    pub fn swap3(&mut self) -> &mut Self {
        self.push(Opcode::SWAP3);
        self
    }

    pub fn swap4(&mut self) -> &mut Self {
        self.push(Opcode::SWAP4);
        self
    }

    pub fn swap5(&mut self) -> &mut Self {
        self.push(Opcode::SWAP5);
        self
    }

    pub fn swap6(&mut self) -> &mut Self {
        self.push(Opcode::SWAP6);
        self
    }

    pub fn swap7(&mut self) -> &mut Self {
        self.push(Opcode::SWAP7);
        self
    }

    pub fn swap8(&mut self) -> &mut Self {
        self.push(Opcode::SWAP8);
        self
    }

    pub fn swap9(&mut self) -> &mut Self {
        self.push(Opcode::SWAP9);
        self
    }

    pub fn swap10(&mut self) -> &mut Self {
        self.push(Opcode::SWAP10);
        self
    }

    pub fn swap11(&mut self) -> &mut Self {
        self.push(Opcode::SWAP11);
        self
    }

    pub fn swap12(&mut self) -> &mut Self {
        self.push(Opcode::SWAP12);
        self
    }

    pub fn swap13(&mut self) -> &mut Self {
        self.push(Opcode::SWAP13);
        self
    }

    pub fn swap14(&mut self) -> &mut Self {
        self.push(Opcode::SWAP14);
        self
    }

    pub fn swap15(&mut self) -> &mut Self {
        self.push(Opcode::SWAP15);
        self
    }

    pub fn swap16(&mut self) -> &mut Self {
        self.push(Opcode::SWAP16);
        self
    }

    pub fn r#return(&mut self) -> &mut Self {
        self.push(Opcode::RETURN);
        self
    }

    pub fn revert(&mut self) -> &mut Self {
        self.push(Opcode::REVERT);
        self
    }

    pub fn invalid(&mut self) -> &mut Self {
        self.push(Opcode::INVALID);
        self
    }

    pub fn eofmagic(&mut self) -> &mut Self {
        self.push(Opcode::EOFMAGIC);
        self
    }

    // Externals
    pub fn external_sha3(&mut self) -> &mut Self {
        self.push(Opcode::SHA3);
        self
    }
    pub fn external_address(&mut self) -> &mut Self {
        self.push(Opcode::ADDRESS);
        self
    }
    pub fn external_balance(&mut self) -> &mut Self {
        self.push(Opcode::BALANCE);
        self
    }
    pub fn external_selfbalance(&mut self) -> &mut Self {
        self.push(Opcode::SELFBALANCE);
        self
    }
    pub fn external_basefee(&mut self) -> &mut Self {
        self.push(Opcode::BASEFEE);
        self
    }
    pub fn external_origin(&mut self) -> &mut Self {
        self.push(Opcode::ORIGIN);
        self
    }
    pub fn external_caller(&mut self) -> &mut Self {
        self.push(Opcode::CALLER);
        self
    }
    pub fn external_callvalue(&mut self) -> &mut Self {
        self.push(Opcode::CALLVALUE);
        self
    }
    pub fn external_gasprice(&mut self) -> &mut Self {
        self.push(Opcode::GASPRICE);
        self
    }
    pub fn external_extcodesize(&mut self) -> &mut Self {
        self.push(Opcode::EXTCODESIZE);
        self
    }
    pub fn external_extcodecopy(&mut self) -> &mut Self {
        self.push(Opcode::EXTCODECOPY);
        self
    }
    pub fn external_extcodehash(&mut self) -> &mut Self {
        self.push(Opcode::EXTCODEHASH);
        self
    }
    pub fn external_returndatasize(&mut self) -> &mut Self {
        self.push(Opcode::RETURNDATASIZE);
        self
    }
    pub fn external_returndatacopy(&mut self) -> &mut Self {
        self.push(Opcode::RETURNDATACOPY);
        self
    }
    pub fn external_blockhash(&mut self) -> &mut Self {
        self.push(Opcode::BLOCKHASH);
        self
    }
    pub fn external_coinbase(&mut self) -> &mut Self {
        self.push(Opcode::COINBASE);
        self
    }
    pub fn external_timestamp(&mut self) -> &mut Self {
        self.push(Opcode::TIMESTAMP);
        self
    }
    pub fn external_number(&mut self) -> &mut Self {
        self.push(Opcode::NUMBER);
        self
    }
    pub fn external_difficulty(&mut self) -> &mut Self {
        self.push(Opcode::DIFFICULTY);
        self
    }
    pub fn external_gaslimit(&mut self) -> &mut Self {
        self.push(Opcode::GASLIMIT);
        self
    }
    pub fn external_sload(&mut self) -> &mut Self {
        self.push(Opcode::SLOAD);
        self
    }
    pub fn external_sstore(&mut self) -> &mut Self {
        self.push(Opcode::SSTORE);
        self
    }
    pub fn external_gas(&mut self) -> &mut Self {
        self.push(Opcode::GAS);
        self
    }
    pub fn external_log0(&mut self) -> &mut Self {
        self.push(Opcode::LOG0);
        self
    }
    pub fn external_log1(&mut self) -> &mut Self {
        self.push(Opcode::LOG1);
        self
    }
    pub fn external_log2(&mut self) -> &mut Self {
        self.push(Opcode::LOG2);
        self
    }
    pub fn external_log3(&mut self) -> &mut Self {
        self.push(Opcode::LOG3);
        self
    }
    pub fn external_log4(&mut self) -> &mut Self {
        self.push(Opcode::LOG4);
        self
    }
    pub fn external_create(&mut self) -> &mut Self {
        self.push(Opcode::CREATE);
        self
    }
    pub fn external_create2(&mut self) -> &mut Self {
        self.push(Opcode::CREATE2);
        self
    }
    pub fn external_call(&mut self) -> &mut Self {
        self.push(Opcode::CALL);
        self
    }
    pub fn external_callcode(&mut self) -> &mut Self {
        self.push(Opcode::CALLCODE);
        self
    }
    pub fn external_delegatecall(&mut self) -> &mut Self {
        self.push(Opcode::DELEGATECALL);
        self
    }
    pub fn external_staticcall(&mut self) -> &mut Self {
        self.push(Opcode::STATICCALL);
        self
    }
    pub fn external_suicide(&mut self) -> &mut Self {
        self.push(Opcode::SUICIDE);
        self
    }
    pub fn external_chainid(&mut self) -> &mut Self {
        self.push(Opcode::CHAINID);
        self
    }

    pub fn build(self) -> Vec<u8> {
        self.bytecode
    }
}

impl EvmBytecodeToAssembly for EvmByteCodeBuilder {
    fn generate_evm_assembly(&self) -> String {
        self.bytecode
            .iter()
            .map(|opcode| format!("{}", Self::opcode_to_assembly(*opcode)))
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn opcode_to_assembly(opcode: u8) -> &'static str {
        match opcode {
            0x00 => "STOP",
            0x01 => "ADD",
            0x02 => "MUL",
            0x03 => "SUB",
            0x04 => "DIV",
            0x05 => "SDIV",
            // Continue for all EVM opcodes...
            _ => "UNKNOWN",
        }
    }
}
