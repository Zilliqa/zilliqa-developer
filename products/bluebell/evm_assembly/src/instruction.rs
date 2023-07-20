use evm::Opcode;

#[derive(Debug, Clone)]
pub struct EvmInstruction {
    pub position: Option<usize>,
    pub opcode: Opcode,
    pub arguments: Vec<u8>,

    pub unresolved_label: Option<String>,

    // TODO: Figure out whether we really need these:
    pub stack_consumed: usize,
    pub stack_produced: usize,
    pub is_terminator: bool,
}

impl EvmInstruction {
    fn arg_to_u64_big_endian(&self) -> Option<u64> {
        if self.arguments.len() > 8 {
            return None; // The input data is too large to fit into a u64
        }

        let mut buf = [0; 8];
        buf[8 - self.arguments.len()..].copy_from_slice(&self.arguments);

        Some(u64::from_be_bytes(buf))
    }

    pub fn push_value_as_u64(&self) -> Option<u64> {
        match self.opcode {
            // Opcode::PUSH0 => Some(0),
            Opcode::PUSH1
            | Opcode::PUSH2
            | Opcode::PUSH3
            | Opcode::PUSH4
            | Opcode::PUSH5
            | Opcode::PUSH6
            | Opcode::PUSH7
            | Opcode::PUSH8 => self.arg_to_u64_big_endian(),
            _ => None,
        }
    }

    pub fn push_value(&self) -> Option<Vec<u8>> {
        match self.opcode {
            // TODO: Opcode::PUSH0 |
            Opcode::PUSH1
            | Opcode::PUSH2
            | Opcode::PUSH3
            | Opcode::PUSH4
            | Opcode::PUSH5
            | Opcode::PUSH6
            | Opcode::PUSH7
            | Opcode::PUSH8 => Some(self.arguments.clone()),
            _ => None,
        }
    }

    pub fn to_opcode_string(&self) -> String {
        if self.arguments.len() > 0 {
            let argument: String = self
                .arguments
                .iter()
                .map(|byte| format!("{:02x}", byte).to_string())
                .collect();
            let position = match self.position {
                Some(v) => v,
                None => 0,
            };
            format!(
                "[0x{:02x}: 0x{:02x}] {} 0x{}",
                position,
                self.opcode.as_u8(),
                self.opcode.to_string(),
                argument
            )
        } else {
            let position = match self.position {
                Some(v) => v,
                None => 0,
            };
            format!(
                "[0x{:02x}: 0x{:02x}] {}",
                position,
                self.opcode.as_u8(),
                self.opcode.to_string()
            )
        }
    }
}
