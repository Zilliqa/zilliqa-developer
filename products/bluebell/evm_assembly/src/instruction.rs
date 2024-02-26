use evm::Opcode;

#[derive(Debug, Clone)]
pub struct EvmSourcePosition {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone)]
pub struct RustPosition {
    pub filename: String,
    pub line: usize,
}

#[derive(Debug, Clone)]
pub struct EvmInstruction {
    pub position: Option<u32>,
    pub opcode: Opcode,
    pub arguments: Vec<u8>,

    pub unresolved_argument_label: Option<String>,

    pub stack_size: i32, // The number of elements on the stack since the start of the block before this instruction is executed
    pub is_terminator: bool,
    pub comment: Option<String>,
    pub source_position: Option<EvmSourcePosition>,
    pub rust_position: Option<RustPosition>,

    pub label: Option<String>,
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

    pub fn u32_to_arg_big_endian(&mut self, value: u32) {
        let bytes = value.to_be_bytes();
        let mut leading_zeros = 0;
        for byte in &bytes {
            if *byte == 0 {
                leading_zeros += 1;
            } else {
                break;
            }
        }

        let argument_size = self.expected_args_length();

        if leading_zeros == 4 {
            // If the u64 value is zero, we still need to ensure that the argument size is correct
            let leading_zero_bytes = argument_size - 4;
            if leading_zero_bytes > 0 {
                self.arguments = vec![0; leading_zero_bytes];
            } else {
                self.arguments = vec![];
            }
        } else {
            let actual_size = 4 - leading_zeros;
            if actual_size > argument_size {
                // If the actual size is greater than the expected size, remove leading zeros
                self.arguments = bytes[leading_zeros..].to_vec();
            } else {
                // If the actual size is less than the expected size, add leading zeros
                let leading_zero_bytes = argument_size - actual_size;
                let mut new_arguments = vec![0; leading_zero_bytes];
                new_arguments.extend_from_slice(&bytes[leading_zeros..]);
                self.arguments = new_arguments;
            }
        }
    }

    pub fn expected_args_length(&self) -> usize {
        match self.opcode {
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
            Opcode::PUSH17 => 17,
            Opcode::PUSH18 => 18,
            Opcode::PUSH19 => 19,
            Opcode::PUSH20 => 20,
            Opcode::PUSH21 => 21,
            Opcode::PUSH22 => 22,
            Opcode::PUSH23 => 23,
            Opcode::PUSH24 => 24,
            Opcode::PUSH25 => 25,
            Opcode::PUSH26 => 26,
            Opcode::PUSH27 => 27,
            Opcode::PUSH28 => 28,
            Opcode::PUSH29 => 29,
            Opcode::PUSH30 => 30,
            Opcode::PUSH31 => 31,
            Opcode::PUSH32 => 32,
            _ => 0,
        }
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
