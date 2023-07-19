use evm::Opcode;

#[derive(Debug)]
pub struct EvmInstruction {
    pub position: usize,
    pub opcode: Opcode,
    pub arguments: Vec<u8>,
    pub stack_consumed: usize,
    pub stack_produced: usize,
    pub is_terminator: bool,
}
