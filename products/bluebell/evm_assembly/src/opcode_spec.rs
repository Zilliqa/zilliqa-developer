use evm::Opcode;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct OpcodeSpecification {
    pub opcode: Opcode,
    pub stack_consumed: usize,
    pub stack_produced: usize,
    pub is_terminator: bool,
    pub bytecode_arguments: usize,
}

impl OpcodeSpecification {
    pub fn new(
        opcode: Opcode,
        stack_consumed: usize,
        stack_produced: usize,
        is_terminator: bool,
        bytecode_arguments: usize,
    ) -> OpcodeSpecification {
        OpcodeSpecification {
            opcode,
            stack_consumed,
            stack_produced,
            is_terminator,
            bytecode_arguments,
        }
    }
}

// TODO: Finish the spec
pub fn create_opcode_spec() -> HashMap<u8, OpcodeSpecification> {
    let spec: HashMap<u8, OpcodeSpecification> = [
        (
            // TODO: Change PUSH0 once the Zilliqa EVM is upgraded
            // TODO: Ugly hack PUSH0 -> PUSH1
            0x5f,
            OpcodeSpecification::new(Opcode::PUSH1, 0, 0, false, 0),
        ),
        (
            Opcode::STOP.as_u8(),
            OpcodeSpecification::new(Opcode::STOP, 0, 0, false, 0),
        ),
        (
            Opcode::ADD.as_u8(),
            OpcodeSpecification::new(Opcode::ADD, 2, 1, false, 0),
        ),
        (
            Opcode::MUL.as_u8(),
            OpcodeSpecification::new(Opcode::MUL, 2, 1, false, 0),
        ),
        (
            Opcode::SUB.as_u8(),
            OpcodeSpecification::new(Opcode::SUB, 2, 1, false, 0),
        ),
        (
            Opcode::DIV.as_u8(),
            OpcodeSpecification::new(Opcode::DIV, 2, 1, false, 0),
        ),
        (
            Opcode::SDIV.as_u8(),
            OpcodeSpecification::new(Opcode::SDIV, 2, 1, false, 0),
        ),
        (
            Opcode::MOD.as_u8(),
            OpcodeSpecification::new(Opcode::MOD, 2, 1, false, 0),
        ),
        (
            Opcode::SMOD.as_u8(),
            OpcodeSpecification::new(Opcode::SMOD, 2, 1, false, 0),
        ),
        (
            Opcode::ADDMOD.as_u8(),
            OpcodeSpecification::new(Opcode::ADDMOD, 3, 1, false, 0),
        ),
        (
            Opcode::MULMOD.as_u8(),
            OpcodeSpecification::new(Opcode::MULMOD, 3, 1, false, 0),
        ),
        (
            Opcode::EXP.as_u8(),
            OpcodeSpecification::new(Opcode::EXP, 2, 1, false, 0),
        ),
        (
            Opcode::SIGNEXTEND.as_u8(),
            OpcodeSpecification::new(Opcode::SIGNEXTEND, 2, 1, false, 0),
        ),
        (
            Opcode::LT.as_u8(),
            OpcodeSpecification::new(Opcode::LT, 2, 1, false, 0),
        ),
        (
            Opcode::GT.as_u8(),
            OpcodeSpecification::new(Opcode::GT, 2, 1, false, 0),
        ),
        (
            Opcode::SLT.as_u8(),
            OpcodeSpecification::new(Opcode::SLT, 2, 1, false, 0),
        ),
        (
            Opcode::SGT.as_u8(),
            OpcodeSpecification::new(Opcode::SGT, 2, 1, false, 0),
        ),
        (
            Opcode::EQ.as_u8(),
            OpcodeSpecification::new(Opcode::EQ, 2, 1, false, 0),
        ),
        (
            Opcode::ISZERO.as_u8(),
            OpcodeSpecification::new(Opcode::ISZERO, 1, 1, false, 0),
        ),
        (
            Opcode::AND.as_u8(),
            OpcodeSpecification::new(Opcode::AND, 2, 1, false, 0),
        ),
        (
            Opcode::OR.as_u8(),
            OpcodeSpecification::new(Opcode::OR, 2, 1, false, 0),
        ),
        (
            Opcode::XOR.as_u8(),
            OpcodeSpecification::new(Opcode::XOR, 2, 1, false, 0),
        ),
        (
            Opcode::NOT.as_u8(),
            OpcodeSpecification::new(Opcode::NOT, 1, 1, false, 0),
        ),
        (
            Opcode::BYTE.as_u8(),
            OpcodeSpecification::new(Opcode::BYTE, 2, 1, false, 0),
        ),
        (
            Opcode::CALLDATALOAD.as_u8(),
            OpcodeSpecification::new(Opcode::CALLDATALOAD, 1, 1, false, 0),
        ),
        (
            Opcode::CALLDATASIZE.as_u8(),
            OpcodeSpecification::new(Opcode::CALLDATASIZE, 0, 1, false, 0),
        ),
        (
            Opcode::CALLDATACOPY.as_u8(),
            OpcodeSpecification::new(Opcode::CALLDATACOPY, 3, 0, false, 0),
        ),
        (
            Opcode::CODESIZE.as_u8(),
            OpcodeSpecification::new(Opcode::CODESIZE, 0, 1, false, 0),
        ),
        (
            Opcode::CODECOPY.as_u8(),
            OpcodeSpecification::new(Opcode::CODECOPY, 3, 0, false, 0),
        ),
        (
            Opcode::SHL.as_u8(),
            OpcodeSpecification::new(Opcode::SHL, 2, 1, false, 0),
        ),
        (
            Opcode::SHR.as_u8(),
            OpcodeSpecification::new(Opcode::SHR, 2, 1, false, 0),
        ),
        (
            Opcode::SAR.as_u8(),
            OpcodeSpecification::new(Opcode::SAR, 2, 1, false, 0),
        ),
        (
            Opcode::POP.as_u8(),
            OpcodeSpecification::new(Opcode::POP, 1, 0, false, 0),
        ),
        (
            Opcode::MLOAD.as_u8(),
            OpcodeSpecification::new(Opcode::MLOAD, 1, 1, false, 0),
        ),
        (
            Opcode::MSTORE.as_u8(),
            OpcodeSpecification::new(Opcode::MSTORE, 2, 0, false, 0),
        ),
        (
            Opcode::MSTORE8.as_u8(),
            OpcodeSpecification::new(Opcode::MSTORE8, 2, 0, false, 0),
        ),
        (
            Opcode::JUMP.as_u8(),
            OpcodeSpecification::new(Opcode::JUMP, 1, 0, true, 0),
        ),
        (
            Opcode::JUMPI.as_u8(),
            OpcodeSpecification::new(Opcode::JUMPI, 2, 0, false, 0),
        ),
        (
            Opcode::PC.as_u8(),
            OpcodeSpecification::new(Opcode::PC, 0, 1, false, 0),
        ),
        (
            Opcode::MSIZE.as_u8(),
            OpcodeSpecification::new(Opcode::MSIZE, 0, 1, false, 0),
        ),
        (
            Opcode::JUMPDEST.as_u8(),
            OpcodeSpecification::new(Opcode::JUMPDEST, 0, 0, false, 0),
        ),
        // TODO: Push0
        (
            Opcode::PUSH1.as_u8(),
            OpcodeSpecification::new(Opcode::PUSH1, 0, 1, false, 1),
        ),
        (
            Opcode::PUSH2.as_u8(),
            OpcodeSpecification::new(Opcode::PUSH2, 0, 1, false, 2),
        ),
        (
            Opcode::PUSH3.as_u8(),
            OpcodeSpecification::new(Opcode::PUSH3, 0, 1, false, 3),
        ),
        (
            Opcode::PUSH4.as_u8(),
            OpcodeSpecification::new(Opcode::PUSH4, 0, 1, false, 4),
        ),
        (
            Opcode::PUSH5.as_u8(),
            OpcodeSpecification::new(Opcode::PUSH5, 0, 1, false, 5),
        ),
        (
            Opcode::PUSH6.as_u8(),
            OpcodeSpecification::new(Opcode::PUSH6, 0, 1, false, 6),
        ),
        (
            Opcode::PUSH7.as_u8(),
            OpcodeSpecification::new(Opcode::PUSH7, 0, 1, false, 7),
        ),
        (
            Opcode::PUSH8.as_u8(),
            OpcodeSpecification::new(Opcode::PUSH8, 0, 1, false, 8),
        ),
        (
            Opcode::PUSH9.as_u8(),
            OpcodeSpecification::new(Opcode::PUSH9, 0, 1, false, 9),
        ),
        (
            Opcode::PUSH10.as_u8(),
            OpcodeSpecification::new(Opcode::PUSH10, 0, 1, false, 10),
        ),
        (
            Opcode::PUSH11.as_u8(),
            OpcodeSpecification::new(Opcode::PUSH11, 0, 1, false, 11),
        ),
        (
            Opcode::PUSH12.as_u8(),
            OpcodeSpecification::new(Opcode::PUSH12, 0, 1, false, 12),
        ),
        (
            Opcode::PUSH13.as_u8(),
            OpcodeSpecification::new(Opcode::PUSH13, 0, 1, false, 13),
        ),
        (
            Opcode::PUSH14.as_u8(),
            OpcodeSpecification::new(Opcode::PUSH14, 0, 1, false, 14),
        ),
        (
            Opcode::PUSH15.as_u8(),
            OpcodeSpecification::new(Opcode::PUSH15, 0, 1, false, 15),
        ),
        (
            Opcode::PUSH16.as_u8(),
            OpcodeSpecification::new(Opcode::PUSH16, 0, 1, false, 16),
        ),
        (
            Opcode::PUSH17.as_u8(),
            OpcodeSpecification::new(Opcode::PUSH17, 0, 1, false, 17),
        ),
        (
            Opcode::PUSH18.as_u8(),
            OpcodeSpecification::new(Opcode::PUSH18, 0, 1, false, 18),
        ),
        (
            Opcode::PUSH19.as_u8(),
            OpcodeSpecification::new(Opcode::PUSH19, 0, 1, false, 19),
        ),
        (
            Opcode::PUSH20.as_u8(),
            OpcodeSpecification::new(Opcode::PUSH20, 0, 1, false, 20),
        ),
        (
            Opcode::PUSH21.as_u8(),
            OpcodeSpecification::new(Opcode::PUSH21, 0, 1, false, 21),
        ),
        (
            Opcode::PUSH22.as_u8(),
            OpcodeSpecification::new(Opcode::PUSH22, 0, 1, false, 22),
        ),
        (
            Opcode::PUSH23.as_u8(),
            OpcodeSpecification::new(Opcode::PUSH23, 0, 1, false, 23),
        ),
        (
            Opcode::PUSH24.as_u8(),
            OpcodeSpecification::new(Opcode::PUSH24, 0, 1, false, 24),
        ),
        (
            Opcode::PUSH25.as_u8(),
            OpcodeSpecification::new(Opcode::PUSH25, 0, 1, false, 25),
        ),
        (
            Opcode::PUSH26.as_u8(),
            OpcodeSpecification::new(Opcode::PUSH26, 0, 1, false, 26),
        ),
        (
            Opcode::PUSH27.as_u8(),
            OpcodeSpecification::new(Opcode::PUSH27, 0, 1, false, 27),
        ),
        (
            Opcode::PUSH28.as_u8(),
            OpcodeSpecification::new(Opcode::PUSH28, 0, 1, false, 28),
        ),
        (
            Opcode::PUSH29.as_u8(),
            OpcodeSpecification::new(Opcode::PUSH29, 0, 1, false, 29),
        ),
        (
            Opcode::PUSH30.as_u8(),
            OpcodeSpecification::new(Opcode::PUSH30, 0, 1, false, 30),
        ),
        (
            Opcode::PUSH31.as_u8(),
            OpcodeSpecification::new(Opcode::PUSH31, 0, 1, false, 31),
        ),
        (
            Opcode::PUSH32.as_u8(),
            OpcodeSpecification::new(Opcode::PUSH32, 0, 1, false, 32),
        ),
        (
            Opcode::DUP1.as_u8(),
            OpcodeSpecification::new(Opcode::DUP1, 1, 2, false, 0),
        ),
        (
            Opcode::DUP2.as_u8(),
            OpcodeSpecification::new(Opcode::DUP2, 2, 3, false, 0),
        ),
        (
            Opcode::DUP3.as_u8(),
            OpcodeSpecification::new(Opcode::DUP3, 3, 4, false, 0),
        ),
        (
            Opcode::DUP4.as_u8(),
            OpcodeSpecification::new(Opcode::DUP4, 4, 5, false, 0),
        ),
        (
            Opcode::DUP5.as_u8(),
            OpcodeSpecification::new(Opcode::DUP5, 5, 6, false, 0),
        ),
        (
            Opcode::DUP6.as_u8(),
            OpcodeSpecification::new(Opcode::DUP6, 6, 7, false, 0),
        ),
        (
            Opcode::DUP7.as_u8(),
            OpcodeSpecification::new(Opcode::DUP7, 7, 8, false, 0),
        ),
        (
            Opcode::DUP8.as_u8(),
            OpcodeSpecification::new(Opcode::DUP8, 8, 9, false, 0),
        ),
        (
            Opcode::DUP9.as_u8(),
            OpcodeSpecification::new(Opcode::DUP9, 9, 10, false, 0),
        ),
        (
            Opcode::DUP10.as_u8(),
            OpcodeSpecification::new(Opcode::DUP10, 10, 11, false, 0),
        ),
        (
            Opcode::DUP11.as_u8(),
            OpcodeSpecification::new(Opcode::DUP11, 11, 12, false, 0),
        ),
        (
            Opcode::DUP12.as_u8(),
            OpcodeSpecification::new(Opcode::DUP12, 12, 13, false, 0),
        ),
        (
            Opcode::DUP13.as_u8(),
            OpcodeSpecification::new(Opcode::DUP13, 13, 14, false, 0),
        ),
        (
            Opcode::DUP14.as_u8(),
            OpcodeSpecification::new(Opcode::DUP14, 14, 15, false, 0),
        ),
        (
            Opcode::DUP15.as_u8(),
            OpcodeSpecification::new(Opcode::DUP15, 15, 16, false, 0),
        ),
        (
            Opcode::DUP16.as_u8(),
            OpcodeSpecification::new(Opcode::DUP16, 16, 17, false, 0),
        ),
        (
            Opcode::SWAP1.as_u8(),
            OpcodeSpecification::new(Opcode::SWAP1, 2, 2, false, 0),
        ),
        (
            Opcode::SWAP2.as_u8(),
            OpcodeSpecification::new(Opcode::SWAP2, 3, 3, false, 0),
        ),
        (
            Opcode::SWAP3.as_u8(),
            OpcodeSpecification::new(Opcode::SWAP3, 4, 4, false, 0),
        ),
        (
            Opcode::SWAP4.as_u8(),
            OpcodeSpecification::new(Opcode::SWAP4, 5, 5, false, 0),
        ),
        (
            Opcode::SWAP5.as_u8(),
            OpcodeSpecification::new(Opcode::SWAP5, 6, 6, false, 0),
        ),
        (
            Opcode::SWAP6.as_u8(),
            OpcodeSpecification::new(Opcode::SWAP6, 7, 7, false, 0),
        ),
        (
            Opcode::SWAP7.as_u8(),
            OpcodeSpecification::new(Opcode::SWAP7, 8, 8, false, 0),
        ),
        (
            Opcode::SWAP8.as_u8(),
            OpcodeSpecification::new(Opcode::SWAP8, 9, 9, false, 0),
        ),
        (
            Opcode::SWAP9.as_u8(),
            OpcodeSpecification::new(Opcode::SWAP9, 10, 10, false, 0),
        ),
        (
            Opcode::SWAP10.as_u8(),
            OpcodeSpecification::new(Opcode::SWAP10, 11, 11, false, 0),
        ),
        (
            Opcode::SWAP11.as_u8(),
            OpcodeSpecification::new(Opcode::SWAP11, 12, 12, false, 0),
        ),
        (
            Opcode::SWAP12.as_u8(),
            OpcodeSpecification::new(Opcode::SWAP12, 13, 13, false, 0),
        ),
        (
            Opcode::SWAP13.as_u8(),
            OpcodeSpecification::new(Opcode::SWAP13, 14, 14, false, 0),
        ),
        (
            Opcode::SWAP14.as_u8(),
            OpcodeSpecification::new(Opcode::SWAP14, 15, 15, false, 0),
        ),
        (
            Opcode::SWAP15.as_u8(),
            OpcodeSpecification::new(Opcode::SWAP15, 16, 16, false, 0),
        ),
        (
            Opcode::SWAP16.as_u8(),
            OpcodeSpecification::new(Opcode::SWAP16, 17, 17, false, 0),
        ),
        (
            Opcode::RETURN.as_u8(),
            OpcodeSpecification::new(Opcode::RETURN, 2, 0, true, 0),
        ),
        (
            Opcode::REVERT.as_u8(),
            OpcodeSpecification::new(Opcode::REVERT, 2, 0, true, 0),
        ), // TODO: Determine if this is a terminator - assuming true for now.
        (
            Opcode::INVALID.as_u8(),
            OpcodeSpecification::new(Opcode::INVALID, 0, 0, false, 0),
        ),
        (
            Opcode::EOFMAGIC.as_u8(),
            OpcodeSpecification::new(Opcode::EOFMAGIC, 0, 0, false, 0),
        ),
        (
            Opcode::SHA3.as_u8(),
            OpcodeSpecification::new(Opcode::SHA3, 2, 1, false, 0),
        ),
        (
            Opcode::ADDRESS.as_u8(),
            OpcodeSpecification::new(Opcode::ADDRESS, 0, 1, false, 0),
        ),
        (
            Opcode::BALANCE.as_u8(),
            OpcodeSpecification::new(Opcode::BALANCE, 1, 1, false, 0),
        ),
        (
            Opcode::SELFBALANCE.as_u8(),
            OpcodeSpecification::new(Opcode::SELFBALANCE, 0, 1, false, 0),
        ),
        (
            Opcode::BASEFEE.as_u8(),
            OpcodeSpecification::new(Opcode::BASEFEE, 0, 1, false, 0),
        ),
        (
            Opcode::ORIGIN.as_u8(),
            OpcodeSpecification::new(Opcode::ORIGIN, 0, 1, false, 0),
        ),
        (
            Opcode::CALLER.as_u8(),
            OpcodeSpecification::new(Opcode::CALLER, 0, 1, false, 0),
        ),
        (
            Opcode::CALLVALUE.as_u8(),
            OpcodeSpecification::new(Opcode::CALLVALUE, 0, 1, false, 0),
        ),
        (
            Opcode::GASPRICE.as_u8(),
            OpcodeSpecification::new(Opcode::GASPRICE, 0, 1, false, 0),
        ),
        (
            Opcode::EXTCODESIZE.as_u8(),
            OpcodeSpecification::new(Opcode::EXTCODESIZE, 1, 1, false, 0),
        ),
        (
            Opcode::EXTCODECOPY.as_u8(),
            OpcodeSpecification::new(Opcode::EXTCODECOPY, 4, 0, false, 0),
        ),
        (
            Opcode::EXTCODEHASH.as_u8(),
            OpcodeSpecification::new(Opcode::EXTCODEHASH, 1, 1, false, 0),
        ),
        (
            Opcode::RETURNDATASIZE.as_u8(),
            OpcodeSpecification::new(Opcode::RETURNDATASIZE, 0, 1, false, 0),
        ),
        (
            Opcode::RETURNDATACOPY.as_u8(),
            OpcodeSpecification::new(Opcode::RETURNDATACOPY, 3, 0, false, 0),
        ),
        (
            Opcode::BLOCKHASH.as_u8(),
            OpcodeSpecification::new(Opcode::BLOCKHASH, 1, 1, false, 0),
        ),
        (
            Opcode::COINBASE.as_u8(),
            OpcodeSpecification::new(Opcode::COINBASE, 0, 1, false, 0),
        ),
        (
            Opcode::TIMESTAMP.as_u8(),
            OpcodeSpecification::new(Opcode::TIMESTAMP, 0, 1, false, 0),
        ),
        (
            Opcode::NUMBER.as_u8(),
            OpcodeSpecification::new(Opcode::NUMBER, 0, 1, false, 0),
        ),
        (
            Opcode::DIFFICULTY.as_u8(), // TODO: Consume and produce not found on evm.code
            OpcodeSpecification::new(Opcode::DIFFICULTY, 0, 0, false, 0),
        ),
        (
            Opcode::GASLIMIT.as_u8(),
            OpcodeSpecification::new(Opcode::GASLIMIT, 0, 1, false, 0),
        ),
        (
            Opcode::SLOAD.as_u8(),
            OpcodeSpecification::new(Opcode::SLOAD, 1, 1, false, 0),
        ),
        (
            Opcode::SSTORE.as_u8(),
            OpcodeSpecification::new(Opcode::SSTORE, 1, 1, false, 0),
        ),
        (
            Opcode::GAS.as_u8(),
            OpcodeSpecification::new(Opcode::GAS, 0, 1, false, 0),
        ),
        (
            Opcode::LOG0.as_u8(),
            OpcodeSpecification::new(Opcode::LOG0, 2, 0, false, 0),
        ),
        (
            Opcode::LOG1.as_u8(),
            OpcodeSpecification::new(Opcode::LOG1, 3, 0, false, 0),
        ),
        (
            Opcode::LOG2.as_u8(),
            OpcodeSpecification::new(Opcode::LOG2, 4, 0, false, 0),
        ),
        (
            Opcode::LOG3.as_u8(),
            OpcodeSpecification::new(Opcode::LOG3, 5, 0, false, 0),
        ),
        (
            Opcode::LOG4.as_u8(),
            OpcodeSpecification::new(Opcode::LOG4, 6, 0, false, 0),
        ),
        (
            Opcode::CREATE.as_u8(),
            OpcodeSpecification::new(Opcode::CREATE, 3, 1, false, 0),
        ),
        (
            Opcode::CREATE2.as_u8(),
            OpcodeSpecification::new(Opcode::CREATE2, 4, 1, false, 0),
        ),
        (
            Opcode::CALL.as_u8(),
            OpcodeSpecification::new(Opcode::CALL, 7, 1, false, 0),
        ),
        (
            Opcode::CALLCODE.as_u8(),
            OpcodeSpecification::new(Opcode::CALLCODE, 7, 1, false, 0),
        ),
        (
            Opcode::DELEGATECALL.as_u8(),
            OpcodeSpecification::new(Opcode::DELEGATECALL, 6, 1, false, 0),
        ),
        (
            Opcode::STATICCALL.as_u8(),
            OpcodeSpecification::new(Opcode::STATICCALL, 6, 1, false, 0),
        ),
        (
            Opcode::SUICIDE.as_u8(), // TODO: Not found on evm.code
            OpcodeSpecification::new(Opcode::SUICIDE, 0, 0, false, 0),
        ),
        (
            Opcode::CHAINID.as_u8(),
            OpcodeSpecification::new(Opcode::CHAINID, 0, 1, false, 0),
        ),
    ]
    .iter()
    .cloned()
    .collect();
    spec
}
