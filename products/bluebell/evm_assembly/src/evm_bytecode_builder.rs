use evm::Opcode;

use std::collections::HashMap;

use crate::evm_decompiler::EvmAssemblyGenerator;

pub struct EvmInstruction {
    opcode: Opcode,
    arguments: Vec<u8>,
}

#[derive(Debug, Clone)]
struct OpcodeSpecification {
    opcode: Opcode,
    stack_consumed: usize,
    stack_produced: usize,
    is_terminator: bool,
}

impl OpcodeSpecification {
    pub fn new(
        opcode: Opcode,
        stack_consumed: usize,
        stack_produced: usize,
        is_terminator: bool,
    ) -> OpcodeSpecification {
        OpcodeSpecification {
            opcode,
            stack_consumed,
            stack_produced,
            is_terminator,
        }
    }
}

pub struct EvmBlock {
    name: String,
    instructions: Vec<EvmInstruction>,
}
impl EvmBlock {
    pub fn new(name: String) -> Self {
        Self {
            name,
            instructions: Vec::new(),
        }
    }
}

pub struct EvmByteCodeBuilder {
    bytecode: Vec<u8>,
    labels: HashMap<String, usize>,

    blocks: Vec<EvmBlock>,
    opcode_specs: HashMap<u8, OpcodeSpecification>,
}

impl EvmByteCodeBuilder {
    pub fn new() -> Self {
        Self {
            bytecode: Vec::new(),
            blocks: Vec::new(),
            labels: HashMap::new(),
            opcode_specs: Self::create_opcode_spec(),
        }
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self {
            bytecode: bytes,
            blocks: Vec::new(),
            labels: HashMap::new(),
            opcode_specs: Self::create_opcode_spec(),
        }
    }

    fn create_opcode_spec() -> HashMap<u8, OpcodeSpecification> {
        let spec: HashMap<u8, OpcodeSpecification> = [
            (
                Opcode::STOP.as_u8(),
                OpcodeSpecification::new(Opcode::STOP, 0, 0, false),
            ),
            (
                Opcode::ADD.as_u8(),
                OpcodeSpecification::new(Opcode::ADD, 2, 1, false),
            ),
            (
                Opcode::MUL.as_u8(),
                OpcodeSpecification::new(Opcode::MUL, 2, 1, false),
            ),
            (
                Opcode::SUB.as_u8(),
                OpcodeSpecification::new(Opcode::SUB, 2, 1, false),
            ),
            (
                Opcode::DIV.as_u8(),
                OpcodeSpecification::new(Opcode::DIV, 2, 1, false),
            ),
            (
                Opcode::SDIV.as_u8(),
                OpcodeSpecification::new(Opcode::SDIV, 2, 1, false),
            ),
            (
                Opcode::MOD.as_u8(),
                OpcodeSpecification::new(Opcode::MOD, 2, 1, false),
            ),
            (
                Opcode::SMOD.as_u8(),
                OpcodeSpecification::new(Opcode::SMOD, 2, 1, false),
            ),
            (
                Opcode::ADDMOD.as_u8(),
                OpcodeSpecification::new(Opcode::ADDMOD, 3, 1, false),
            ),
            (
                Opcode::MULMOD.as_u8(),
                OpcodeSpecification::new(Opcode::MULMOD, 3, 1, false),
            ),
            (
                Opcode::EXP.as_u8(),
                OpcodeSpecification::new(Opcode::EXP, 2, 1, false),
            ),
            (
                Opcode::SIGNEXTEND.as_u8(),
                OpcodeSpecification::new(Opcode::SIGNEXTEND, 2, 1, false),
            ),
            (
                Opcode::LT.as_u8(),
                OpcodeSpecification::new(Opcode::LT, 2, 1, false),
            ),
            (
                Opcode::GT.as_u8(),
                OpcodeSpecification::new(Opcode::GT, 2, 1, false),
            ),
            (
                Opcode::SLT.as_u8(),
                OpcodeSpecification::new(Opcode::SLT, 2, 1, false),
            ),
            (
                Opcode::SGT.as_u8(),
                OpcodeSpecification::new(Opcode::SGT, 2, 1, false),
            ),
            (
                Opcode::EQ.as_u8(),
                OpcodeSpecification::new(Opcode::EQ, 2, 1, false),
            ),
            (
                Opcode::ISZERO.as_u8(),
                OpcodeSpecification::new(Opcode::ISZERO, 1, 1, false),
            ),
            (
                Opcode::AND.as_u8(),
                OpcodeSpecification::new(Opcode::AND, 2, 1, false),
            ),
            (
                Opcode::OR.as_u8(),
                OpcodeSpecification::new(Opcode::OR, 2, 1, false),
            ),
            (
                Opcode::XOR.as_u8(),
                OpcodeSpecification::new(Opcode::XOR, 2, 1, false),
            ),
            (
                Opcode::NOT.as_u8(),
                OpcodeSpecification::new(Opcode::NOT, 1, 1, false),
            ),
            (
                Opcode::BYTE.as_u8(),
                OpcodeSpecification::new(Opcode::BYTE, 2, 1, false),
            ),
            (
                Opcode::CALLDATALOAD.as_u8(),
                OpcodeSpecification::new(Opcode::CALLDATALOAD, 1, 1, false),
            ),
            (
                Opcode::CALLDATASIZE.as_u8(),
                OpcodeSpecification::new(Opcode::CALLDATASIZE, 0, 1, false),
            ),
            (
                Opcode::CALLDATACOPY.as_u8(),
                OpcodeSpecification::new(Opcode::CALLDATACOPY, 3, 0, false),
            ),
            (
                Opcode::CODESIZE.as_u8(),
                OpcodeSpecification::new(Opcode::CODESIZE, 0, 1, false),
            ),
            (
                Opcode::CODECOPY.as_u8(),
                OpcodeSpecification::new(Opcode::CODECOPY, 3, 0, false),
            ),
            // TODO:
            (
                Opcode::SHL.as_u8(),
                OpcodeSpecification::new(Opcode::SHL, 0, 0, false),
            ),
            (
                Opcode::SHR.as_u8(),
                OpcodeSpecification::new(Opcode::SHR, 0, 0, false),
            ),
            (
                Opcode::SAR.as_u8(),
                OpcodeSpecification::new(Opcode::SAR, 0, 0, false),
            ),
            (
                Opcode::POP.as_u8(),
                OpcodeSpecification::new(Opcode::POP, 0, 0, false),
            ),
            (
                Opcode::MLOAD.as_u8(),
                OpcodeSpecification::new(Opcode::MLOAD, 0, 0, false),
            ),
            (
                Opcode::MSTORE.as_u8(),
                OpcodeSpecification::new(Opcode::MSTORE, 0, 0, false),
            ),
            (
                Opcode::MSTORE8.as_u8(),
                OpcodeSpecification::new(Opcode::MSTORE8, 0, 0, false),
            ),
            (
                Opcode::JUMP.as_u8(),
                OpcodeSpecification::new(Opcode::JUMP, 0, 0, true),
            ),
            (
                Opcode::JUMPI.as_u8(),
                OpcodeSpecification::new(Opcode::JUMPI, 0, 0, true),
            ),
            (
                Opcode::PC.as_u8(),
                OpcodeSpecification::new(Opcode::PC, 0, 0, false),
            ),
            (
                Opcode::MSIZE.as_u8(),
                OpcodeSpecification::new(Opcode::MSIZE, 0, 0, false),
            ),
            (
                Opcode::JUMPDEST.as_u8(),
                OpcodeSpecification::new(Opcode::JUMPDEST, 0, 0, false),
            ),
            (
                Opcode::PUSH1.as_u8(),
                OpcodeSpecification::new(Opcode::PUSH1, 0, 0, false),
            ),
            (
                Opcode::PUSH2.as_u8(),
                OpcodeSpecification::new(Opcode::PUSH2, 0, 0, false),
            ),
            (
                Opcode::PUSH3.as_u8(),
                OpcodeSpecification::new(Opcode::PUSH3, 0, 0, false),
            ),
            (
                Opcode::PUSH4.as_u8(),
                OpcodeSpecification::new(Opcode::PUSH4, 0, 0, false),
            ),
            (
                Opcode::PUSH5.as_u8(),
                OpcodeSpecification::new(Opcode::PUSH5, 0, 0, false),
            ),
            (
                Opcode::PUSH6.as_u8(),
                OpcodeSpecification::new(Opcode::PUSH6, 0, 0, false),
            ),
            (
                Opcode::PUSH7.as_u8(),
                OpcodeSpecification::new(Opcode::PUSH7, 0, 0, false),
            ),
            (
                Opcode::PUSH8.as_u8(),
                OpcodeSpecification::new(Opcode::PUSH8, 0, 0, false),
            ),
            (
                Opcode::PUSH9.as_u8(),
                OpcodeSpecification::new(Opcode::PUSH9, 0, 0, false),
            ),
            (
                Opcode::PUSH10.as_u8(),
                OpcodeSpecification::new(Opcode::PUSH10, 0, 0, false),
            ),
            (
                Opcode::PUSH11.as_u8(),
                OpcodeSpecification::new(Opcode::PUSH11, 0, 0, false),
            ),
            (
                Opcode::PUSH12.as_u8(),
                OpcodeSpecification::new(Opcode::PUSH12, 0, 0, false),
            ),
            (
                Opcode::PUSH13.as_u8(),
                OpcodeSpecification::new(Opcode::PUSH13, 0, 0, false),
            ),
            (
                Opcode::PUSH14.as_u8(),
                OpcodeSpecification::new(Opcode::PUSH14, 0, 0, false),
            ),
            (
                Opcode::PUSH15.as_u8(),
                OpcodeSpecification::new(Opcode::PUSH15, 0, 0, false),
            ),
            (
                Opcode::PUSH16.as_u8(),
                OpcodeSpecification::new(Opcode::PUSH16, 0, 0, false),
            ),
            (
                Opcode::PUSH17.as_u8(),
                OpcodeSpecification::new(Opcode::PUSH17, 0, 0, false),
            ),
            (
                Opcode::PUSH18.as_u8(),
                OpcodeSpecification::new(Opcode::PUSH18, 0, 0, false),
            ),
            (
                Opcode::PUSH19.as_u8(),
                OpcodeSpecification::new(Opcode::PUSH19, 0, 0, false),
            ),
            (
                Opcode::PUSH20.as_u8(),
                OpcodeSpecification::new(Opcode::PUSH20, 0, 0, false),
            ),
            (
                Opcode::PUSH21.as_u8(),
                OpcodeSpecification::new(Opcode::PUSH21, 0, 0, false),
            ),
            (
                Opcode::PUSH22.as_u8(),
                OpcodeSpecification::new(Opcode::PUSH22, 0, 0, false),
            ),
            (
                Opcode::PUSH23.as_u8(),
                OpcodeSpecification::new(Opcode::PUSH23, 0, 0, false),
            ),
            (
                Opcode::PUSH24.as_u8(),
                OpcodeSpecification::new(Opcode::PUSH24, 0, 0, false),
            ),
            (
                Opcode::PUSH25.as_u8(),
                OpcodeSpecification::new(Opcode::PUSH25, 0, 0, false),
            ),
            (
                Opcode::PUSH26.as_u8(),
                OpcodeSpecification::new(Opcode::PUSH26, 0, 0, false),
            ),
            (
                Opcode::PUSH27.as_u8(),
                OpcodeSpecification::new(Opcode::PUSH27, 0, 0, false),
            ),
            (
                Opcode::PUSH28.as_u8(),
                OpcodeSpecification::new(Opcode::PUSH28, 0, 0, false),
            ),
            (
                Opcode::PUSH29.as_u8(),
                OpcodeSpecification::new(Opcode::PUSH29, 0, 0, false),
            ),
            (
                Opcode::PUSH30.as_u8(),
                OpcodeSpecification::new(Opcode::PUSH30, 0, 0, false),
            ),
            (
                Opcode::PUSH31.as_u8(),
                OpcodeSpecification::new(Opcode::PUSH31, 0, 0, false),
            ),
            (
                Opcode::PUSH32.as_u8(),
                OpcodeSpecification::new(Opcode::PUSH32, 0, 0, false),
            ),
            (
                Opcode::DUP1.as_u8(),
                OpcodeSpecification::new(Opcode::DUP1, 0, 0, false),
            ),
            (
                Opcode::DUP2.as_u8(),
                OpcodeSpecification::new(Opcode::DUP2, 0, 0, false),
            ),
            (
                Opcode::DUP3.as_u8(),
                OpcodeSpecification::new(Opcode::DUP3, 0, 0, false),
            ),
            (
                Opcode::DUP4.as_u8(),
                OpcodeSpecification::new(Opcode::DUP4, 0, 0, false),
            ),
            (
                Opcode::DUP5.as_u8(),
                OpcodeSpecification::new(Opcode::DUP5, 0, 0, false),
            ),
            (
                Opcode::DUP6.as_u8(),
                OpcodeSpecification::new(Opcode::DUP6, 0, 0, false),
            ),
            (
                Opcode::DUP7.as_u8(),
                OpcodeSpecification::new(Opcode::DUP7, 0, 0, false),
            ),
            (
                Opcode::DUP8.as_u8(),
                OpcodeSpecification::new(Opcode::DUP8, 0, 0, false),
            ),
            (
                Opcode::DUP9.as_u8(),
                OpcodeSpecification::new(Opcode::DUP9, 0, 0, false),
            ),
            (
                Opcode::DUP10.as_u8(),
                OpcodeSpecification::new(Opcode::DUP10, 0, 0, false),
            ),
            (
                Opcode::DUP11.as_u8(),
                OpcodeSpecification::new(Opcode::DUP11, 0, 0, false),
            ),
            (
                Opcode::DUP12.as_u8(),
                OpcodeSpecification::new(Opcode::DUP12, 0, 0, false),
            ),
            (
                Opcode::DUP13.as_u8(),
                OpcodeSpecification::new(Opcode::DUP13, 0, 0, false),
            ),
            (
                Opcode::DUP14.as_u8(),
                OpcodeSpecification::new(Opcode::DUP14, 0, 0, false),
            ),
            (
                Opcode::DUP15.as_u8(),
                OpcodeSpecification::new(Opcode::DUP15, 0, 0, false),
            ),
            (
                Opcode::DUP16.as_u8(),
                OpcodeSpecification::new(Opcode::DUP16, 0, 0, false),
            ),
            (
                Opcode::SWAP1.as_u8(),
                OpcodeSpecification::new(Opcode::SWAP1, 0, 0, false),
            ),
            (
                Opcode::SWAP2.as_u8(),
                OpcodeSpecification::new(Opcode::SWAP2, 0, 0, false),
            ),
            (
                Opcode::SWAP3.as_u8(),
                OpcodeSpecification::new(Opcode::SWAP3, 0, 0, false),
            ),
            (
                Opcode::SWAP4.as_u8(),
                OpcodeSpecification::new(Opcode::SWAP4, 0, 0, false),
            ),
            (
                Opcode::SWAP5.as_u8(),
                OpcodeSpecification::new(Opcode::SWAP5, 0, 0, false),
            ),
            (
                Opcode::SWAP6.as_u8(),
                OpcodeSpecification::new(Opcode::SWAP6, 0, 0, false),
            ),
            (
                Opcode::SWAP7.as_u8(),
                OpcodeSpecification::new(Opcode::SWAP7, 0, 0, false),
            ),
            (
                Opcode::SWAP8.as_u8(),
                OpcodeSpecification::new(Opcode::SWAP8, 0, 0, false),
            ),
            (
                Opcode::SWAP9.as_u8(),
                OpcodeSpecification::new(Opcode::SWAP9, 0, 0, false),
            ),
            (
                Opcode::SWAP10.as_u8(),
                OpcodeSpecification::new(Opcode::SWAP10, 0, 0, false),
            ),
            (
                Opcode::SWAP11.as_u8(),
                OpcodeSpecification::new(Opcode::SWAP11, 0, 0, false),
            ),
            (
                Opcode::SWAP12.as_u8(),
                OpcodeSpecification::new(Opcode::SWAP12, 0, 0, false),
            ),
            (
                Opcode::SWAP13.as_u8(),
                OpcodeSpecification::new(Opcode::SWAP13, 0, 0, false),
            ),
            (
                Opcode::SWAP14.as_u8(),
                OpcodeSpecification::new(Opcode::SWAP14, 0, 0, false),
            ),
            (
                Opcode::SWAP15.as_u8(),
                OpcodeSpecification::new(Opcode::SWAP15, 0, 0, false),
            ),
            (
                Opcode::SWAP16.as_u8(),
                OpcodeSpecification::new(Opcode::SWAP16, 0, 0, false),
            ),
            (
                Opcode::RETURN.as_u8(),
                OpcodeSpecification::new(Opcode::RETURN, 0, 0, true),
            ),
            (
                Opcode::REVERT.as_u8(),
                OpcodeSpecification::new(Opcode::REVERT, 0, 0, true),
            ), // TODO: Determine if this is a terminator - assuming true for now.
            (
                Opcode::INVALID.as_u8(),
                OpcodeSpecification::new(Opcode::INVALID, 0, 0, false),
            ),
            (
                Opcode::EOFMAGIC.as_u8(),
                OpcodeSpecification::new(Opcode::EOFMAGIC, 0, 0, false),
            ),
            (
                Opcode::SHA3.as_u8(),
                OpcodeSpecification::new(Opcode::SHA3, 0, 0, false),
            ),
            (
                Opcode::ADDRESS.as_u8(),
                OpcodeSpecification::new(Opcode::ADDRESS, 0, 0, false),
            ),
            (
                Opcode::BALANCE.as_u8(),
                OpcodeSpecification::new(Opcode::BALANCE, 0, 0, false),
            ),
            (
                Opcode::SELFBALANCE.as_u8(),
                OpcodeSpecification::new(Opcode::SELFBALANCE, 0, 0, false),
            ),
            (
                Opcode::BASEFEE.as_u8(),
                OpcodeSpecification::new(Opcode::BASEFEE, 0, 0, false),
            ),
            (
                Opcode::ORIGIN.as_u8(),
                OpcodeSpecification::new(Opcode::ORIGIN, 0, 0, false),
            ),
            (
                Opcode::CALLER.as_u8(),
                OpcodeSpecification::new(Opcode::CALLER, 0, 0, false),
            ),
            (
                Opcode::CALLVALUE.as_u8(),
                OpcodeSpecification::new(Opcode::CALLVALUE, 0, 0, false),
            ),
            (
                Opcode::GASPRICE.as_u8(),
                OpcodeSpecification::new(Opcode::GASPRICE, 0, 0, false),
            ),
            (
                Opcode::EXTCODESIZE.as_u8(),
                OpcodeSpecification::new(Opcode::EXTCODESIZE, 0, 0, false),
            ),
            (
                Opcode::EXTCODECOPY.as_u8(),
                OpcodeSpecification::new(Opcode::EXTCODECOPY, 0, 0, false),
            ),
            (
                Opcode::EXTCODEHASH.as_u8(),
                OpcodeSpecification::new(Opcode::EXTCODEHASH, 0, 0, false),
            ),
            (
                Opcode::RETURNDATASIZE.as_u8(),
                OpcodeSpecification::new(Opcode::RETURNDATASIZE, 0, 0, false),
            ),
            (
                Opcode::RETURNDATACOPY.as_u8(),
                OpcodeSpecification::new(Opcode::RETURNDATACOPY, 0, 0, false),
            ),
            (
                Opcode::BLOCKHASH.as_u8(),
                OpcodeSpecification::new(Opcode::BLOCKHASH, 0, 0, false),
            ),
            (
                Opcode::COINBASE.as_u8(),
                OpcodeSpecification::new(Opcode::COINBASE, 0, 0, false),
            ),
            (
                Opcode::TIMESTAMP.as_u8(),
                OpcodeSpecification::new(Opcode::TIMESTAMP, 0, 0, false),
            ),
            (
                Opcode::NUMBER.as_u8(),
                OpcodeSpecification::new(Opcode::NUMBER, 0, 0, false),
            ),
            (
                Opcode::DIFFICULTY.as_u8(),
                OpcodeSpecification::new(Opcode::DIFFICULTY, 0, 0, false),
            ),
            (
                Opcode::GASLIMIT.as_u8(),
                OpcodeSpecification::new(Opcode::GASLIMIT, 0, 0, false),
            ),
            (
                Opcode::SLOAD.as_u8(),
                OpcodeSpecification::new(Opcode::SLOAD, 0, 0, false),
            ),
            (
                Opcode::SSTORE.as_u8(),
                OpcodeSpecification::new(Opcode::SSTORE, 0, 0, false),
            ),
            (
                Opcode::GAS.as_u8(),
                OpcodeSpecification::new(Opcode::GAS, 0, 0, false),
            ),
            (
                Opcode::LOG0.as_u8(),
                OpcodeSpecification::new(Opcode::LOG0, 0, 0, false),
            ),
            (
                Opcode::LOG1.as_u8(),
                OpcodeSpecification::new(Opcode::LOG1, 0, 0, false),
            ),
            (
                Opcode::LOG2.as_u8(),
                OpcodeSpecification::new(Opcode::LOG2, 0, 0, false),
            ),
            (
                Opcode::LOG3.as_u8(),
                OpcodeSpecification::new(Opcode::LOG3, 0, 0, false),
            ),
            (
                Opcode::LOG4.as_u8(),
                OpcodeSpecification::new(Opcode::LOG4, 0, 0, false),
            ),
            (
                Opcode::CREATE.as_u8(),
                OpcodeSpecification::new(Opcode::CREATE, 0, 0, false),
            ),
            (
                Opcode::CREATE2.as_u8(),
                OpcodeSpecification::new(Opcode::CREATE2, 0, 0, false),
            ),
            (
                Opcode::CALL.as_u8(),
                OpcodeSpecification::new(Opcode::CALL, 0, 0, false),
            ),
            (
                Opcode::CALLCODE.as_u8(),
                OpcodeSpecification::new(Opcode::CALLCODE, 0, 0, false),
            ),
            (
                Opcode::DELEGATECALL.as_u8(),
                OpcodeSpecification::new(Opcode::DELEGATECALL, 0, 0, false),
            ),
            (
                Opcode::STATICCALL.as_u8(),
                OpcodeSpecification::new(Opcode::STATICCALL, 0, 0, false),
            ),
            (
                Opcode::SUICIDE.as_u8(),
                OpcodeSpecification::new(Opcode::SUICIDE, 0, 0, false),
            ),
            (
                Opcode::CHAINID.as_u8(),
                OpcodeSpecification::new(Opcode::CHAINID, 0, 0, false),
            ),
        ]
        .iter()
        .cloned()
        .collect();
        spec
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

    fn opcode_to_assembly(opcode: Opcode) -> &'static str {
        match opcode {
            Opcode::STOP => "STOP",
            Opcode::ADD => "ADD",
            Opcode::MUL => "MUL",
            Opcode::SUB => "SUB",
            Opcode::DIV => "DIV",
            Opcode::SDIV => "SDIV",
            Opcode::MOD => "MOD",
            Opcode::SMOD => "SMOD",
            Opcode::ADDMOD => "ADDMOD",
            Opcode::MULMOD => "MULMOD",
            Opcode::EXP => "EXP",
            Opcode::SIGNEXTEND => "SIGNEXTEND",

            Opcode::LT => "LT",
            Opcode::GT => "GT",
            Opcode::SLT => "SLT",
            Opcode::SGT => "SGT",
            Opcode::EQ => "EQ",
            Opcode::ISZERO => "ISZERO",
            Opcode::AND => "AND",
            Opcode::OR => "OR",
            Opcode::XOR => "XOR",
            Opcode::NOT => "NOT",
            Opcode::BYTE => "BYTE",

            Opcode::CALLDATALOAD => "CALLDATALOAD",
            Opcode::CALLDATASIZE => "CALLDATASIZE",
            Opcode::CALLDATACOPY => "CALLDATACOPY",
            Opcode::CODESIZE => "CODESIZE",
            Opcode::CODECOPY => "CODECOPY",

            Opcode::SHL => "SHL",
            Opcode::SHR => "SHR",
            Opcode::SAR => "SAR",

            Opcode::POP => "POP",
            Opcode::MLOAD => "MLOAD",
            Opcode::MSTORE => "MSTORE",
            Opcode::MSTORE8 => "MSTORE8",
            Opcode::JUMP => "JUMP",
            Opcode::JUMPI => "JUMPI",
            Opcode::PC => "PC",
            Opcode::MSIZE => "MSIZE",
            Opcode::JUMPDEST => "JUMPDEST",

            Opcode::PUSH1 => "PUSH1",
            Opcode::PUSH2 => "PUSH2",
            Opcode::PUSH3 => "PUSH3",
            Opcode::PUSH4 => "PUSH4",
            Opcode::PUSH5 => "PUSH5",
            Opcode::PUSH6 => "PUSH6",
            Opcode::PUSH7 => "PUSH7",
            Opcode::PUSH8 => "PUSH8",
            Opcode::PUSH9 => "PUSH9",
            Opcode::PUSH10 => "PUSH10",
            Opcode::PUSH11 => "PUSH11",
            Opcode::PUSH12 => "PUSH12",
            Opcode::PUSH13 => "PUSH13",
            Opcode::PUSH14 => "PUSH14",
            Opcode::PUSH15 => "PUSH15",
            Opcode::PUSH16 => "PUSH16",
            Opcode::PUSH17 => "PUSH17",
            Opcode::PUSH18 => "PUSH18",
            Opcode::PUSH19 => "PUSH19",
            Opcode::PUSH20 => "PUSH20",
            Opcode::PUSH21 => "PUSH21",
            Opcode::PUSH22 => "PUSH22",
            Opcode::PUSH23 => "PUSH23",
            Opcode::PUSH24 => "PUSH24",
            Opcode::PUSH25 => "PUSH25",
            Opcode::PUSH26 => "PUSH26",
            Opcode::PUSH27 => "PUSH27",
            Opcode::PUSH28 => "PUSH28",
            Opcode::PUSH29 => "PUSH29",
            Opcode::PUSH30 => "PUSH30",
            Opcode::PUSH31 => "PUSH31",
            Opcode::PUSH32 => "PUSH32",

            Opcode::DUP1 => "DUP1",
            Opcode::DUP2 => "DUP2",
            Opcode::DUP3 => "DUP3",
            Opcode::DUP4 => "DUP4",
            Opcode::DUP5 => "DUP5",
            Opcode::DUP6 => "DUP6",
            Opcode::DUP7 => "DUP7",
            Opcode::DUP8 => "DUP8",
            Opcode::DUP9 => "DUP9",
            Opcode::DUP10 => "DUP10",
            Opcode::DUP11 => "DUP11",
            Opcode::DUP12 => "DUP12",
            Opcode::DUP13 => "DUP13",
            Opcode::DUP14 => "DUP14",
            Opcode::DUP15 => "DUP15",
            Opcode::DUP16 => "DUP16",

            Opcode::SWAP1 => "SWAP1",
            Opcode::SWAP2 => "SWAP2",
            Opcode::SWAP3 => "SWAP3",
            Opcode::SWAP4 => "SWAP4",
            Opcode::SWAP5 => "SWAP5",
            Opcode::SWAP6 => "SWAP6",
            Opcode::SWAP7 => "SWAP7",
            Opcode::SWAP8 => "SWAP8",
            Opcode::SWAP9 => "SWAP9",
            Opcode::SWAP10 => "SWAP10",
            Opcode::SWAP11 => "SWAP11",
            Opcode::SWAP12 => "SWAP12",
            Opcode::SWAP13 => "SWAP13",
            Opcode::SWAP14 => "SWAP14",
            Opcode::SWAP15 => "SWAP15",
            Opcode::SWAP16 => "SWAP16",

            Opcode::RETURN => "RETURN",
            Opcode::REVERT => "REVERT",

            Opcode::INVALID => "INVALID",

            Opcode::EOFMAGIC => "EOFMAGIC",

            Opcode::SHA3 => "SHA3",
            Opcode::ADDRESS => "ADDRESS",
            Opcode::BALANCE => "BALANCE",
            Opcode::SELFBALANCE => "SELFBALANCE",
            Opcode::BASEFEE => "BASEFEE",
            Opcode::ORIGIN => "ORIGIN",
            Opcode::CALLER => "CALLER",
            Opcode::CALLVALUE => "CALLVALUE",
            Opcode::GASPRICE => "GASPRICE",
            Opcode::EXTCODESIZE => "EXTCODESIZE",
            Opcode::EXTCODECOPY => "EXTCODECOPY",
            Opcode::EXTCODEHASH => "EXTCODEHASH",
            Opcode::RETURNDATASIZE => "RETURNDATASIZE",
            Opcode::RETURNDATACOPY => "RETURNDATACOPY",
            Opcode::BLOCKHASH => "BLOCKHASH",
            Opcode::COINBASE => "COINBASE",
            Opcode::TIMESTAMP => "TIMESTAMP",
            Opcode::NUMBER => "NUMBER",
            Opcode::DIFFICULTY => "DIFFICULTY",
            Opcode::GASLIMIT => "GASLIMIT",
            Opcode::SLOAD => "SLOAD",
            Opcode::SSTORE => "SSTORE",
            Opcode::GAS => "GAS",
            Opcode::LOG0 => "LOG0",
            Opcode::LOG1 => "LOG1",
            Opcode::LOG2 => "LOG2",
            Opcode::LOG3 => "LOG3",
            Opcode::LOG4 => "LOG4",
            Opcode::CREATE => "CREATE",
            Opcode::CREATE2 => "CREATE2",
            Opcode::CALL => "CALL",
            Opcode::CALLCODE => "CALLCODE",
            Opcode::DELEGATECALL => "DELEGATECALL",
            Opcode::STATICCALL => "STATICCALL",
            Opcode::SUICIDE => "SUICIDE",
            Opcode::CHAINID => "CHAINID",

            // Continue for all EVM opcodes...
            _ => "UNKNOWN",
        }
    }
}

impl EvmAssemblyGenerator for EvmByteCodeBuilder {
    fn generate_evm_assembly(&self) -> String {
        let mut blocks: Vec<EvmBlock> = Vec::new();
        let mut current_block = EvmBlock::new("block0".to_string());
        let mut i = 0;
        while i < self.bytecode.len() {
            let mut instr = EvmInstruction {
                opcode: Opcode(self.bytecode[i]),
                arguments: Vec::new(),
            };
            i += 1;

            let mut collect_args = match instr.opcode {
                Opcode::PUSH1 => 1,
                Opcode::PUSH2 => 2,
                Opcode::PUSH3 => 3,
                Opcode::PUSH4 => 4,
                Opcode::PUSH5 => 5,
                Opcode::PUSH6 => 6,
                Opcode::PUSH7 => 7,
                Opcode::PUSH8 => 8,
                Opcode::PUSH9 => 9,
                Opcode::PUSH10 => 10,
                Opcode::PUSH11 => 11,
                Opcode::PUSH12 => 12,
                Opcode::PUSH13 => 13,
                Opcode::PUSH14 => 14,
                Opcode::PUSH15 => 15,
                Opcode::PUSH16 => 16,
                // TODO: Add all
                Opcode::INVALID => break,
                _ => 0,
            };

            if i + collect_args > self.bytecode.len() {
                panic!("This is not good - we exceed the byte code");
            }

            while collect_args > 0 {
                instr.arguments.push(self.bytecode[i]);
                i += 1;
                collect_args -= 1;
            }

            if instr.opcode == Opcode::JUMPDEST {
                blocks.push(current_block);
                current_block = EvmBlock::new("block0".to_string()); // TODO: Name
            }

            current_block.instructions.push(instr);
        }

        let mut data: Vec<u8> = Vec::new();
        while i < self.bytecode.len() {
            data.push(self.bytecode[i]);
            i += 1;
        }

        blocks.push(current_block);

        let code_blocks = blocks
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
                                .map(|byte| format!("{:02x}", byte))
                                .collect();
                            format!("{} 0x{}", Self::opcode_to_assembly(instr.opcode), argument)
                        } else {
                            format!("{}", Self::opcode_to_assembly(instr.opcode))
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("\n");
                format!("{}:\n{}", block.name, code)
            })
            .collect::<Vec<String>>()
            .join("\n\n");

        let data: String = data.iter().map(|byte| format!("{:02x}", byte)).collect();

        format!("{}\n\nauxdata: 0x{}", code_blocks, data)
    }
}
