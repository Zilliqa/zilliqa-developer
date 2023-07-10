use evm::ExitReason;
use evm::Opcode;
use evm::Trap;
use evm::{Capture, ExitSucceed, Machine};
use std::rc::Rc;

trait PatchVm {
    fn execute_with_runtime(&mut self) -> Capture<ExitReason, Trap>;
    fn step_with_runtime(&mut self) -> Result<(), Capture<ExitReason, Trap>>;
}

impl PatchVm for Machine {
    /// Loop stepping the machine, until it stops.
    fn execute_with_runtime(&mut self) -> Capture<ExitReason, Trap> {
        loop {
            match self.step_with_runtime() {
                Ok(()) => (),
                Err(res) => return res,
            }
        }
    }

    #[inline]
    /// Step the machine, executing one opcode. It then returns.
    fn step_with_runtime(&mut self) -> Result<(), Capture<ExitReason, Trap>> {
        let position = match self.position() {
            Ok(pos) => *pos,
            Err(err) => panic!("{:?}", err),
        };
        println!("{:?}", position);
        //			.
        //			.map_err(|reason| Capture::Exit(reason.clone()))?;

        self.step()
        /*
        match self.code.get(position).map(|v| Opcode(*v)) {
            Some(opcode) => match eval(self, opcode, position) {
                Control::Continue(p) => {
                    self.position = Ok(position + p);
                    Ok(())
                }
                Control::Exit(e) => {
                    self.position = Err(e.clone());
                    Err(Capture::Exit(e))
                }
                Control::Jump(p) => {
                    self.position = Ok(p);
                    Ok(())
                }
                Control::Trap(opcode) => {
                    self.position = Ok(position + 1);
                    Err(Capture::Trap(opcode))
                }
            },
            None => {
                self.position = Err(ExitSucceed::Stopped.into());
                Err(Capture::Exit(ExitSucceed::Stopped.into()))
            }
        }
        */
    }
}

pub fn evm_test() {
    let code = hex::decode("60e060020a6000350480632839e92814601e57806361047ff414603457005b602a6004356024356047565b8060005260206000f35b603d6004356099565b8060005260206000f35b600082600014605457605e565b8160010190506093565b81600014606957607b565b60756001840360016047565b90506093565b609060018403608c85600186036047565b6047565b90505b92915050565b6000816000148060a95750816001145b60b05760b7565b81905060cf565b60c1600283036099565b60cb600184036099565b0190505b91905056",
).unwrap();
    let data = hex::decode("2839e92800000000000000000000000000000000000000000000000000000000000000030000000000000000000000000000000000000000000000000000000000000001").unwrap();

    let mut vm = Machine::new(Rc::new(code), Rc::new(data), 1024, 10000);

    // Running
    vm.execute_with_runtime();
    //		assert_eq!(vm.run(), Capture::Exit(ExitSucceed::Returned.into()));
    println!("Ret: {:?}", vm.return_value())
}

/*



use evm::backend::{MemoryAccount, MemoryBackend, MemoryVicinity};
use evm::executor::stack::{MemoryStackState, StackExecutor, StackSubstateMetadata};
use evm::Config;
use primitive_types::{H160, U256};
use std::{collections::BTreeMap, str::FromStr};

pub fn evm_test() {
    let config = Config::istanbul();

    let vicinity = MemoryVicinity {
        gas_price: U256::zero(),
        origin: H160::default(),
        block_hashes: Vec::new(),
        block_number: Default::default(),
        block_coinbase: Default::default(),
        block_timestamp: Default::default(),
        block_difficulty: Default::default(),
        block_gas_limit: Default::default(),
        chain_id: U256::one(),
        block_base_fee_per_gas: U256::zero(),
        block_randomness: None,
    };

    let mut state = BTreeMap::new();
    state.insert(
        H160::from_str("0x1000000000000000000000000000000000000000").unwrap(),
        MemoryAccount {
            nonce: U256::one(),
            balance: U256::from(10000000),
            storage: BTreeMap::new(),
            code: hex::decode("6080604052348015600f57600080fd5b506004361060285760003560e01c80630f14a40614602d575b600080fd5b605660048036036020811015604157600080fd5b8101908080359060200190929190505050606c565b6040518082815260200191505060405180910390f35b6000806000905060005b83811015608f5760018201915080806001019150506076565b508091505091905056fea26469706673582212202bc9ec597249a9700278fe4ce78da83273cb236e76d4d6797b441454784f901d64736f6c63430007040033").unwrap(),
        }
    );
    state.insert(
        H160::from_str("0xf000000000000000000000000000000000000000").unwrap(),
        MemoryAccount {
            nonce: U256::one(),
            balance: U256::from(10000000),
            storage: BTreeMap::new(),
            code: Vec::new(),
        },
    );

    let backend = MemoryBackend::new(&vicinity, state);
    let metadata = StackSubstateMetadata::new(u64::MAX, &config);
    let state = MemoryStackState::new(metadata, &backend);
    let precompiles = BTreeMap::new();
    let mut executor = StackExecutor::new_with_precompiles(state, &config, &precompiles);


    let _reason = executor.transact_call(
        H160::from_str("0xf000000000000000000000000000000000000000").unwrap(),
        H160::from_str("0x1000000000000000000000000000000000000000").unwrap(),
        U256::zero(),
        hex::decode("0f14a4060000000000000000000000000000000000000000000000000000000000b71b00")
            .unwrap(),
        // hex::decode("0f14a4060000000000000000000000000000000000000000000000000000000000002ee0").unwrap(),
        u64::MAX,
        Vec::new(),
    );
}
*/
