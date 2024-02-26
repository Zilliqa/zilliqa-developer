use std::{
    collections::{BTreeSet, HashMap},
    mem,
};

use evm::Opcode;
use log::info;
use primitive_types::U256;

use crate::{
    function_signature::EvmFunctionSignature,
    instruction::{EvmInstruction, EvmSourcePosition, RustPosition},
    opcode_spec::{OpcodeSpec, OpcodeSpecification},
    types::{EvmType, EvmTypeValue},
};

pub const ALLOCATION_POINTER: u8 = 0x40;
pub const MEMORY_OFFSET: u8 = 0x80;

#[derive(Debug, Clone)]
pub struct Scope {
    pub stack_counter: i32,
    pub arg_count: i32,
    entry_stack_counter: i32,
    name_location: HashMap<String, i32>,
    location_name: HashMap<i32, String>,
}

impl Scope {
    pub fn empty(arg_count: i32) -> Self {
        Scope {
            stack_counter: 0,
            entry_stack_counter: arg_count,
            arg_count,
            name_location: HashMap::new(),
            location_name: HashMap::new(),
        }
    }

    pub fn new(parent: Scope, arg_count: i32) -> Self {
        let mut ret = parent.clone();
        ret.entry_stack_counter = ret.stack_counter + arg_count;

        ret
    }

    pub fn relative_stack_counter(&self) -> i32 {
        (self.stack_counter - self.entry_stack_counter) as i32
    }

    pub fn arg_count(&self) -> i32 {
        self.arg_count
    }

    pub fn register_arg_name(&mut self, name: &str, arg_number: i32) -> Result<(), String> {
        if self.name_location.contains_key(name) {
            return Err(format!("SSA name {} already exists", name));
        }

        assert!(
            self.entry_stack_counter > self.stack_counter,
            "Attempting to register too many function arguments"
        );

        // TODO: assumes that args are first in, last out
        self.name_location.insert(name.to_string(), arg_number);
        self.location_name.insert(arg_number, name.to_string());

        self.stack_counter += 1;

        // TODO: Consider pruning of the names

        Ok(())
    }

    pub fn register_stack_name(&mut self, name: &str) -> Result<(), String> {
        if self.name_location.contains_key(name) {
            let depth = match self.name_location.get(name) {
                Some(depth) => depth.clone(),
                _ => return Err("Unable to find the depth of existing SSA name".to_string()),
            };

            self.location_name.remove(&depth);
            self.name_location.remove(name);
        }

        assert!(self.stack_counter + self.arg_count > 0);

        self.name_location
            .insert(name.to_string(), self.stack_counter - 1);
        self.location_name
            .insert(self.stack_counter - 1, name.to_string());

        // TODO: Consider pruning of the names

        Ok(())
    }

    pub fn register_alias(&mut self, source: &str, dest: &str) -> Result<(), String> {
        // TODO: Create separate alias record to deal with this
        if self.name_location.contains_key(dest) {
            return Err(format!("SSA name {} already exists", dest));
        }

        if let Some(value) = self.name_location.get(source) {
            let value = *value as i32;
            self.name_location.insert(dest.to_string(), value);
            self.location_name.insert(value, dest.to_string());
            Ok(())
        } else {
            return Err(format!("SSA name {} does not exists", dest));
        }
    }

    fn update_stack(&mut self, opcode: Opcode) -> i32 {
        let consumes: i32 = opcode.stack_consumed();
        let produces: i32 = opcode.stack_produced();

        let before = self.stack_counter;

        self.stack_counter -= consumes;
        let ret = self.entry_stack_counter - self.stack_counter;
        self.stack_counter += produces;

        // Note that we allow the stack to be exceed by exactly one element for the return address
        assert!(self.stack_counter + self.arg_count >= -1);

        let after = self.stack_counter;

        // Trimming locations
        for depth in after..before {
            let name = match self.location_name.get(&depth) {
                Some(name) => name.clone(),
                _ => continue,
            };

            self.location_name.remove(&depth);
            self.name_location.remove(&name);
        }

        ret
    }

    fn swap(&mut self, depth: i32) {
        let position = self.stack_counter - depth;

        let name_at_position: Option<String> = match self.location_name.get(&position) {
            Some(n) => Some(n.clone()),
            None => None,
        };

        let name_at_zero: Option<String> = match self.location_name.get(&self.stack_counter) {
            Some(n) => Some(n.clone()),
            None => None,
        };

        if let Some(name_at_position) = &name_at_position {
            self.location_name.remove(&position);
            self.name_location.remove(name_at_position);
        }

        if let Some(name_at_zero) = name_at_zero {
            self.location_name.remove(&self.stack_counter);
            self.name_location.remove(&name_at_zero);

            self.name_location
                .insert(name_at_zero.to_string(), position);
            self.location_name
                .insert(position, name_at_zero.to_string());
        }

        if let Some(name_at_position) = name_at_position {
            self.name_location.insert(name_at_position.to_string(), 0);
            self.location_name.insert(0, name_at_position.to_string());
        }
    }
}

#[derive(Debug, Clone)]
pub struct EvmBlock {
    pub name: String,
    pub position: Option<u32>,
    pub instructions: Vec<EvmInstruction>,
    pub entry_from: Vec<u32>,
    pub is_entry: bool,
    pub is_terminated: bool,
    pub is_lookup_table: bool,

    pub consumes: i32,
    pub produces: i32,

    pub scope: Scope,
    pub comment: Option<String>, // stack_counter: i32,
    // name_location: HashMap<String, i32>,

    // Debug info
    pub source_position: Option<EvmSourcePosition>,
    pub rust_position: Option<RustPosition>,
    pub block_arugments: Option<BTreeSet<String>>,
    pub next_label: Option<String>,
    pub label_counter: u32,
}

impl EvmBlock {
    pub fn to_string(&self) -> String {
        let mut ret: String = "".to_string();
        ret.push_str(&self.name);
        ret.push_str(":\n");
        for instr in &self.instructions {
            ret.push_str("  ");
            ret.push_str(&instr.to_opcode_string());
            ret.push_str("\n");
        }

        ret
    }

    pub fn new(position: Option<u32>, arg_names: BTreeSet<String>, name: &str) -> Self {
        let mut ret = Self {
            name: name.to_string(),
            position,
            instructions: Vec::new(),
            entry_from: Vec::new(),
            is_entry: false,
            is_terminated: false,
            is_lookup_table: false,
            consumes: 0,
            produces: 0,
            scope: Scope::empty(arg_names.len() as i32),
            comment: None,
            source_position: None,
            rust_position: None,
            block_arugments: Some(arg_names.clone()),
            next_label: None,
            label_counter: 0,
        };

        for (i, name) in arg_names.iter().enumerate() {
            match ret.register_arg_name(name, i as i32) {
                Err(e) => panic!("{}", e),
                _ => (),
            }
        }
        ret.jumpdest();

        ret
    }

    pub fn generate_label(&mut self, label: String) -> String {
        let label = format!("{}__{}__{}", self.name, label, self.label_counter).to_string();
        self.label_counter += 1;
        label
    }

    pub fn set_next_instruction_comment(&mut self, comment: String) {
        self.comment = Some(comment);
    }

    pub fn set_next_instruction_location(&mut self, position: EvmSourcePosition) {
        self.source_position = Some(position);
    }

    pub fn set_next_rust_position(&mut self, filename: String, line: usize) {
        self.rust_position = Some(RustPosition { line, filename });
    }

    pub fn register_arg_name(&mut self, name: &str, arg_number: i32) -> Result<(), String> {
        self.scope.register_arg_name(name, arg_number)
    }

    pub fn register_stack_name(&mut self, name: &str) -> Result<(), String> {
        self.scope.register_stack_name(name)
    }

    pub fn register_alias(&mut self, source: &str, dest: &str) -> Result<(), String> {
        self.scope.register_alias(source, dest)
    }

    fn update_stack(&mut self, opcode: Opcode) {
        let deepest_visit = self.scope.update_stack(opcode);

        // Updating how deeply in the stack we consume
        if deepest_visit > 0 {
            self.consumes = std::cmp::max(self.consumes, deepest_visit);
        }
    }

    pub fn move_value(&mut self, from: i32, to: i32) -> Result<(), String> {
        if from == to {
            return Ok(());
        }

        // Ensuring that we are handling the corner
        // case where eihter from or to is 0 correctly:
        // Net result will be a single swap since swap(0) is noop
        let (a, b) = if from < to { (from, to) } else { (to, from) };

        self.swap(a);
        self.swap(b);
        self.swap(a);

        Ok(())
    }

    pub fn move_stack_name(&mut self, name: &str, pos: i32) -> Result<(), String> {
        match self.scope.name_location.get(name) {
            Some(depth) => {
                let orig_pos = self.scope.stack_counter - depth - 1;
                self.move_value(orig_pos, pos)
            }
            None => Err("Stack overflow.".to_string()),
        }
    }

    pub fn duplicate_stack_name(&mut self, name: &str) -> Result<(), String> {
        match self.scope.name_location.get(name) {
            Some(pos) => {
                let distance = self.scope.stack_counter - pos;

                match distance {
                    1 => {
                        self.dup1();
                        Ok(())
                    }
                    2 => {
                        self.dup2();
                        Ok(())
                    }
                    3 => {
                        self.dup3();
                        Ok(())
                    }
                    4 => {
                        self.dup4();
                        Ok(())
                    }
                    5 => {
                        self.dup5();
                        Ok(())
                    }
                    6 => {
                        self.dup6();
                        Ok(())
                    }
                    7 => {
                        self.dup7();
                        Ok(())
                    }
                    8 => {
                        self.dup8();
                        Ok(())
                    }
                    9 => {
                        self.dup9();
                        Ok(())
                    }
                    10 => {
                        self.dup10();
                        Ok(())
                    }
                    11 => {
                        self.dup11();
                        Ok(())
                    }
                    12 => {
                        self.dup12();
                        Ok(())
                    }
                    13 => {
                        self.dup13();
                        Ok(())
                    }
                    14 => {
                        self.dup14();
                        Ok(())
                    }
                    15 => {
                        self.dup15();
                        Ok(())
                    }
                    16 => {
                        self.dup16();
                        Ok(())
                    }
                    _ => panic!("{}", "Stack overflow.".to_string()),
                }
            }
            None => Err(format!("Failed to find SSA name {} on stack", name)),
        }
    }

    pub fn alloca(&mut self) {
        // Stack args: [size]
        todo!("Implement alloca");
    }

    pub fn mem_copy(&mut self) {
        // Stack args: [dest, source, size]
    }

    pub fn copy_object(&mut self) {
        // Stack args: [dest, source]
        // p_dest => p_dest
        // p_src  => p_src
        //        => len
        //        => 0x0

        self.dup1();
        self.mload();
        self.push1([224].to_vec());
        self.shr();
        self.set_next_instruction_comment("Copy loop counter".to_string());
        // Increasing copy len by 4 to ensure that we also copy the length
        // of the object (stored in u32)
        self.push([0x04].to_vec());
        self.add();

        self.push1([0x0].to_vec());

        let lbl_condition = self.generate_label("copy_loop_condition".to_string());
        let lbl_done = self.generate_label("copy_done".to_string());
        let lbl_body = self.generate_label("copy_body".to_string());

        self.create_label(lbl_condition.clone());
        self.dup2();
        self.dup2();
        self.lt();
        self.jump_if_to(&lbl_body);
        self.jump_to(&lbl_done);

        self.create_label(lbl_body);
        // Stack:
        // p_dest  => p_dest + 0x20
        // p_src   => p_src + 0x20
        // len     => len
        // counter => counter + 0x20

        self.dup3();
        self.push([0x20].to_vec());
        self.add();
        self.swap3();

        self.mload();

        self.dup5();
        self.push([0x20].to_vec());
        self.add();
        self.swap5();

        self.mstore();

        self.push([0x20].to_vec());
        self.add();
        self.jump_to(&lbl_condition);

        // Stack:
        // p_dest
        // p_src
        // len
        // counter
        self.create_label(lbl_done);
        self.pop();
        self.swap2();
        self.pop();
        self.pop();
    }

    pub fn alloca_static(&mut self, size: u64) {
        self.push1([ALLOCATION_POINTER].to_vec());
        self.mload(); // Stack element is the pointer to be left on stack
        self.dup1();
        self.push_u64(size);
        self.add();
        self.push1([ALLOCATION_POINTER].to_vec());
        self.mstore();
    }

    pub fn allocate_object(&mut self, value: Vec<u8>) {
        let chunks = (4 + value.len() + 31) / 32;
        let padded_length = chunks * 32;

        self.alloca_static((padded_length).try_into().unwrap());

        // Storing size
        self.push_u32(value.len().try_into().unwrap());
        self.push1([224].to_vec());
        self.shl();
        self.dup2();
        self.mstore();

        self.dup1(); // Adding rolling pointer
        self.push1([4].to_vec());
        self.add();

        for i in 0..chunks {
            let start = i * 32;
            let mut end = (i + 1) * 32;
            if end > value.len() {
                end = value.len();
            }
            let mut byte_slice: Vec<u8> = value[start..end].into();
            while byte_slice.len() < 32 {
                byte_slice.push(0);
            }

            self.push(byte_slice);
            self.dup2();
            self.mstore();

            if i != chunks - 1 {
                self.push1([32].to_vec());
                self.add();
            }
        }

        self.pop(); // Removing rolling pointer
    }

    pub fn call_internal(
        &mut self,
        _function: &EvmFunctionSignature,
        _args: Vec<EvmTypeValue>,
    ) -> &mut Self {
        todo!()
    }

    pub fn call(&mut self, function: &EvmFunctionSignature, args: Vec<EvmType>) -> &mut Self {
        if let Some(generator) = function.inline_assembly_generator {
            generator(self);
            return self;
        }

        let address = match function.external_address {
            Some(a) => a,
            None => {
                info!("{:#?}", function);
                panic!(
                    "TODO: Internal calls' not supported yet. Attempted to call {}",
                    function.name
                )
            }
        };
        // TODO: Deal with internal calls
        // See https://medium.com/@rbkhmrcr/precompiles-solidity-e5d29bd428c4
        // Head-tail encoding https://medium.com/@hayeah/how-to-decipher-a-smart-contract-method-call-8ee980311603

        self.push1([ALLOCATION_POINTER].to_vec());
        // Stack:
        // arg N     => arg N
        // alloc_ptr => p
        //           => p_data

        self.mload(); // Stack element is the pointer

        self.dup1(); // p_data = p + 0x20 * len(args)
        self.push_u32((0x20 * args.len()).try_into().unwrap());
        self.add();

        for (i, arg) in args.iter().enumerate().rev() {
            match arg {
                EvmType::String => {
                    // By default we store in head:
                    // p_data - p ->  p + 0x20 * i (p_head)

                    // Stack:
                    // arg N  => p
                    // p      => p_data
                    // p_data => arg N
                    self.swap1();
                    self.swap2();

                    // Stack:
                    // p       => p
                    // p_data  => p_data
                    // arg N   => arg N
                    //         => tail offset
                    self.dup3();
                    self.dup3();
                    self.sub();

                    // Stack:
                    // p           => p
                    // p_data      => p_data
                    // arg N       => arg N
                    // tail offset => tail offset
                    //             => p_head
                    self.dup4();
                    self.push_u32((0x20 * i) as u32);
                    self.add();

                    // Stack:
                    // p           => p
                    // p_data      => p_data
                    // arg N       => arg N
                    // tail offset
                    // p_head
                    self.mstore();

                    // Storing the tail:
                    // arg N -> *p_data

                    // Stack:
                    // p      => p
                    // p_data => p_data
                    // arg N  => arg N (p_str)
                    //        => p_data
                    self.dup2();

                    // Stack:
                    // p      => p
                    // p_data => p_data
                    // arg N
                    // p_data

                    self.set_next_instruction_comment("Loading string argument".to_string());

                    // Stack:
                    // p             => p
                    // p_data        => p_data
                    // arg N (p_str) => p_data (dest)
                    // p_data        => p_str  (src)
                    self.swap1();

                    // Stack:
                    // p             => p
                    // p_data        => p_data
                    // p_data     (dest) => copy len
                    // p_str_data (src)
                    self.set_next_instruction_comment("Copying string to call data".to_string());
                    self.copy_object();
                    self.add();

                    // panic!("Strings not supported.");
                }
                _ => {
                    // By default we store in head:
                    // arg N - > p + 0x20 * i (p_head)

                    // Stack:
                    // arg N  => p
                    // p      => p_data
                    // p_data => arg N
                    self.swap1();
                    self.swap2();

                    // Stack:
                    // p      => p
                    // p_data => p_data
                    // arg N  => arg N
                    //        => p_head
                    self.dup3();
                    self.push_u32((0x20 * i) as u32);
                    self.add();

                    // Stack:
                    // p      => p
                    // p_data => p_data
                    // arg N
                    self.mstore();
                    continue;
                }
            }
        }

        // Target format: gas address argsOffset argsSize retOffset retSize

        // Stack:
        // p        => p
        // p_data   => data_size
        self.dup2();
        self.swap1();
        self.sub();

        let gas = EvmTypeValue::Uint32(0x1337); // TODO: How to compute this or where to get it from
        let address = EvmTypeValue::Uint32(address);

        // Stack:
        // p         => p
        // data_size => data_size
        //           => 0x20

        self.push([0x20].to_vec()); //return size, TODO: Compute the size of the return type

        // Stack:
        // p            => 0x20
        // data_size    => p
        // 0x20         => data_size
        self.swap2();
        self.swap1();

        // Stack:
        // 0x20         => 0x20
        // p            => p
        // data_size    => data_size
        //              => p
        self.dup2();

        // Stack:
        // 0x20         => 0x20
        // p            => p
        // data_size    => data_size
        // p            => p
        //              => address
        //              => gas
        self.push(address.to_bytes_unpadded());
        self.push(gas.to_bytes_unpadded());

        // TODO: How come self.external_call(); does not call the precompile?
        self.external_staticcall();

        self
    }

    pub fn extract_blocks_from_bytecode(
        bytecode: &Vec<u8>,
        _opcode_specs: &HashMap<u8, OpcodeSpecification>, // TODO: remove
    ) -> (Vec<EvmBlock>, Vec<u8>) {
        let mut blocks: Vec<EvmBlock> = Vec::new();
        let mut block_counter = 0;
        let mut current_block =
            EvmBlock::new(None, BTreeSet::new(), &format!("block{}", block_counter));
        current_block.is_entry = true;
        block_counter += 1;

        let offset = 0;
        let mut i = offset;
        while i < bytecode.len() {
            let opcode = Opcode(bytecode[i]);
            let is_terminator = opcode.is_terminator();
            let mut collect_args = opcode.bytecode_arguments();
            // TODO: Use write_instruction
            let mut instr = EvmInstruction {
                position: Some(i as u32),
                opcode,
                arguments: Vec::new(),
                unresolved_argument_label: None,

                stack_size: 0, // TODO: Should be calculated using write_instruction
                is_terminator,
                comment: None,
                source_position: None,
                rust_position: None,
                label: None,
            };

            i += 1;
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
                    BTreeSet::new(),
                    &format!("block{}", block_counter),
                );

                block_counter += 1;
            }

            current_block.instructions.push(instr);

            // A terminated block followed by an invalid opcode starts the data section.
            // TODO: Find some spec to confirm this assumption
            if is_terminator {
                if Opcode(bytecode[i]) == Opcode::INVALID {
                    i += 1;

                    // Encountered the auxilary data section
                    break;
                }
            }
        }

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
        unresolved_argument_label: Option<String>,
    ) -> &mut Self {
        let mut comment = None;
        mem::swap(&mut comment, &mut self.comment);

        let mut source_position = None;
        mem::swap(&mut source_position, &mut self.source_position);

        let mut rust_position = None;
        mem::swap(&mut rust_position, &mut self.rust_position);
        let mut label = None;
        mem::swap(&mut label, &mut self.next_label);

        self.instructions.push(EvmInstruction {
            position: None,
            opcode: opcode.clone(),
            arguments: [].to_vec(),
            unresolved_argument_label,

            stack_size: self.scope.stack_counter,
            is_terminator: false,
            comment,
            source_position,
            rust_position,
            label,
        });
        self.update_stack(opcode);

        self
    }

    pub fn write_instruction_with_args(&mut self, opcode: Opcode, arguments: Vec<u8>) -> &mut Self {
        assert!(opcode.bytecode_arguments() == arguments.len());

        let mut comment = None;
        mem::swap(&mut comment, &mut self.comment);

        let mut source_position = None;
        mem::swap(&mut source_position, &mut self.source_position);

        let mut rust_position = None;
        mem::swap(&mut rust_position, &mut self.rust_position);

        let mut label = None;
        mem::swap(&mut label, &mut self.next_label);

        self.instructions.push(EvmInstruction {
            position: None,
            opcode: opcode.clone(),
            arguments,

            unresolved_argument_label: None,

            stack_size: self.scope.stack_counter,
            is_terminator: false,
            comment,
            source_position,
            rust_position,
            label,
        });

        self.update_stack(opcode);
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

    pub fn push_label(&mut self, label: &str) -> &mut Self {
        self.write_instruction(Opcode::PUSH4, Some(label.to_string()))
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

    pub fn create_label(&mut self, label: String) -> &mut Self {
        self.next_label = Some(label);
        self.write_instruction(Opcode::JUMPDEST, None)
    }

    /*
    pub fn push0(&mut self) -> &mut Self {
        self.write_instruction(Opcode::PUSH0, None)
    }
    */

    pub fn push_u64(&mut self, arg: u64) -> &mut Self {
        self.push(arg.to_be_bytes().to_vec())
    }

    pub fn push_u32(&mut self, arg: u32) -> &mut Self {
        self.push(arg.to_be_bytes().to_vec())
    }

    pub fn push_u256(&mut self, arg: U256) -> &mut Self {
        let mut bytes = [0u8; 32];
        arg.to_big_endian(&mut bytes);
        self.push(Vec::from(bytes))
    }

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
        assert!(arguments.len() == 1);
        self.write_instruction_with_args(Opcode::PUSH1, arguments)
    }

    pub fn push2(&mut self, arguments: Vec<u8>) -> &mut Self {
        assert!(arguments.len() == 2);
        self.write_instruction_with_args(Opcode::PUSH2, arguments)
    }

    pub fn push3(&mut self, arguments: Vec<u8>) -> &mut Self {
        assert!(arguments.len() == 3);
        self.write_instruction_with_args(Opcode::PUSH3, arguments)
    }

    pub fn push4(&mut self, arguments: Vec<u8>) -> &mut Self {
        assert!(arguments.len() == 4);
        self.write_instruction_with_args(Opcode::PUSH4, arguments)
    }

    pub fn push5(&mut self, arguments: Vec<u8>) -> &mut Self {
        assert!(arguments.len() == 5);
        self.write_instruction_with_args(Opcode::PUSH5, arguments)
    }

    pub fn push6(&mut self, arguments: Vec<u8>) -> &mut Self {
        assert!(arguments.len() == 6);
        self.write_instruction_with_args(Opcode::PUSH6, arguments)
    }

    pub fn push7(&mut self, arguments: Vec<u8>) -> &mut Self {
        assert!(arguments.len() == 7);
        self.write_instruction_with_args(Opcode::PUSH7, arguments)
    }

    pub fn push8(&mut self, arguments: Vec<u8>) -> &mut Self {
        assert!(arguments.len() == 8);
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

    pub fn swap(&mut self, depth: i32) -> &mut Self {
        match depth {
            0 => self,
            1 => self.swap1(),
            2 => self.swap2(),
            3 => self.swap3(),
            4 => self.swap4(),
            5 => self.swap5(),
            6 => self.swap6(),
            7 => self.swap7(),
            8 => self.swap8(),
            9 => self.swap9(),
            10 => self.swap10(),
            11 => self.swap11(),
            12 => self.swap12(),
            13 => self.swap13(),
            14 => self.swap14(),
            15 => self.swap15(),
            _ => panic!("Swap depth must be at least 0 and lower than 16"),
        }
    }

    pub fn swap1(&mut self) -> &mut Self {
        self.scope.swap(1);
        self.write_instruction(Opcode::SWAP1, None)
    }

    pub fn swap2(&mut self) -> &mut Self {
        self.scope.swap(2);
        self.write_instruction(Opcode::SWAP2, None)
    }

    pub fn swap3(&mut self) -> &mut Self {
        self.scope.swap(3);
        self.write_instruction(Opcode::SWAP3, None)
    }

    pub fn swap4(&mut self) -> &mut Self {
        self.scope.swap(4);
        self.write_instruction(Opcode::SWAP4, None)
    }

    pub fn swap5(&mut self) -> &mut Self {
        self.scope.swap(5);
        self.write_instruction(Opcode::SWAP5, None)
    }

    pub fn swap6(&mut self) -> &mut Self {
        self.scope.swap(6);
        self.write_instruction(Opcode::SWAP6, None)
    }

    pub fn swap7(&mut self) -> &mut Self {
        self.scope.swap(7);
        self.write_instruction(Opcode::SWAP7, None)
    }

    pub fn swap8(&mut self) -> &mut Self {
        self.scope.swap(8);
        self.write_instruction(Opcode::SWAP8, None)
    }

    pub fn swap9(&mut self) -> &mut Self {
        self.scope.swap(9);
        self.write_instruction(Opcode::SWAP9, None)
    }

    pub fn swap10(&mut self) -> &mut Self {
        self.scope.swap(10);
        self.write_instruction(Opcode::SWAP10, None)
    }

    pub fn swap11(&mut self) -> &mut Self {
        self.scope.swap(11);
        self.write_instruction(Opcode::SWAP11, None)
    }

    pub fn swap12(&mut self) -> &mut Self {
        self.scope.swap(12);
        self.write_instruction(Opcode::SWAP12, None)
    }

    pub fn swap13(&mut self) -> &mut Self {
        self.scope.swap(13);
        self.write_instruction(Opcode::SWAP13, None)
    }

    pub fn swap14(&mut self) -> &mut Self {
        self.scope.swap(14);
        self.write_instruction(Opcode::SWAP14, None)
    }

    pub fn swap15(&mut self) -> &mut Self {
        self.scope.swap(15);
        self.write_instruction(Opcode::SWAP15, None)
    }

    pub fn swap16(&mut self) -> &mut Self {
        self.scope.swap(16);
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
