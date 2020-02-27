const { BN, Long, bytes, units } = require('@zilliqa-js/util');
const { Zilliqa } = require('@zilliqa-js/zilliqa');

const {
    toBech32Address,
    getAddressFromPrivateKey,
} = require('@zilliqa-js/crypto');

const zils_per_account = '10000000000000000'

const zilliqa = new Zilliqa('http://localhost:5555');

// These are set by the core protocol, and may vary per-chain.
// You can manually pack the bytes according to chain id and msg version.
// For more information: https://apidocs.zilliqa.com/?shell#getnetworkid

const chainId = 333; // chainId of the developer testnet
const msgVersion = 1; // current msgVersion
const VERSION = bytes.pack(chainId, msgVersion);

// Populate the wallet with an account
const privateKey = 'db11cfa086b92497c8ed5a4cc6edb3a5bfe3a640c43ffb9fc6aa0873c56f2ee3';

zilliqa.wallet.addByPrivateKey(privateKey);

const address = getAddressFromPrivateKey(privateKey);
console.log(`My account address is: ${address}`);
console.log(`My account bech32 address is: ${toBech32Address(address)}`);

async function testBlockchain() {
    try {
        // Get Balance
        const balance = await zilliqa.blockchain.getBalance(address);
        // Get Minimum Gas Price from blockchain
        const minGasPrice = await zilliqa.blockchain.getMinimumGasPrice();

        // Account balance (See note 1)
        console.log(`Your account balance is:`);
        console.log(balance.result);
        console.log(`Current Minimum Gas Price: ${minGasPrice.result}`);
        const myGasPrice = units.toQa('1000', units.Units.Li); // Gas Price that will be used by all transactions
        console.log(`My Gas Price ${myGasPrice.toString()}`);
        const isGasSufficient = myGasPrice.gte(new BN(minGasPrice.result)); // Checks if your gas price is less than the minimum gas price
        console.log(`Is the gas price sufficient? ${isGasSufficient}`);
        // Deploy a contract
        console.log(`Deploying the contract....`);
        const code = `scilla_version 0

import BoolUtils IntUtils

library FundAccounts

let one_msg =
    fun (msg : Message) =>
    let nil_msg = Nil {Message} in
    Cons {Message} msg nil_msg

let one = Uint32 1

let not_owner_code = Uint32 0
let deposit_success_code = Uint32 1
let registration_error_code = Uint32 2
let registered_success_code = Uint32 3
let user_not_exists_code = Uint32 4

let mk_deposit_failed_event =
  fun (sender : ByStr20) =>
    {_eventname : "deposit_failed";
     sender: sender;
     code: not_owner_code }

let mk_deposit_success_event =
  fun(sender : ByStr20) =>
    {_eventname : "deposit_successful";
     sender: sender;
     code: deposit_success_code }
     
let mk_registration_failed_event =
  fun (user_address : ByStr20) =>
  fun (sender : ByStr20) =>
  {_eventname : "user_registration_failed";
   user_address: user_address;
   sender: sender;
   code: registration_error_code }

let mk_registration_success_event =
  fun (user_address : ByStr20) =>
  fun (sender : ByStr20) =>
  {_eventname : "user_registration_successful";
   user_address: user_address;
   sender: sender;
   code: registered_success_code }

contract FundAccounts
(owner: ByStr20,
zils_per_account : Uint128)

field users: Map ByStr20 ByStr20 = Emp ByStr20 ByStr20

(* Only owner can deposit ZIL *)
transition deposit()
    sender_is_owner = builtin eq owner _sender;
    match sender_is_owner with
    | False =>
        e = mk_deposit_failed_event _sender;
        event e
    | True =>
        accept;
        e = mk_deposit_success_event _sender;
        event e
    end
end

transition register_user(user_address: ByStr20)
    user_exists <- exists users[user_address];
        users[user_address] := user_address;
        e = mk_registration_success_event user_address _sender;
        event e;
        msg = {_tag: "";
               _recipient: user_address;
               _amount: zils_per_account;
               code: registered_success_code };
        msgs = one_msg msg;
        send msgs
end`;

        const init = [
            // this parameter is mandatory for all init arrays
            {
                vname: '_scilla_version',
                type: 'Uint32',
                value: '0',
            },
            {
                vname: 'owner',
                type: 'ByStr20',
                value: `${address}`,
            },
            {
                vname: 'zils_per_account',
                type: 'Uint128',
                value: `${zils_per_account}`
            }
      ];

        // Instance of class Contract
        const contract = zilliqa.contracts.new(code, init);

        // Deploy the contract.
        // Also notice here we have a default function parameter named toDs as mentioned above.
        // A contract can be deployed at either the shard or at the DS. Always set this value to false.
        const [deployTx, hello] = await contract.deploy(
            {
                version: VERSION,
                gasPrice: myGasPrice,
                gasLimit: Long.fromNumber(10000),
            },
            33,
            1000,
            false,
        );

        // Introspect the state of the underlying transaction
        console.log(`Deployment Transaction ID: ${deployTx.id}`);
        console.log(`Deployment Transaction Receipt:`);
        console.log(deployTx.txParams.receipt);

        // Get the deployed contract address
        console.log('The contract address is:');
        console.log(hello.address);
        //Following line added to fix issue https://github.com/Zilliqa/Zilliqa-JavaScript-Library/issues/168
        const deployedContract = zilliqa.contracts.at(hello.address);

        // Create a new timebased message and call setHello
        // Also notice here we have a default function parameter named toDs as mentioned above.
        // For calling a smart contract, any transaction can be processed in the DS but not every transaction can be processed in the shards.
        // For those transactions are involved in chain call, the value of toDs should always be true.
        // If a transaction of contract invocation is sent to a shard and if the shard is not allowed to process it, then the transaction will be dropped.
        const newMsg = 'Hello, the time is ' + Date.now();
        console.log('Calling deposit transaction with msg: ' + newMsg);
        const callTx = await hello.call(
            'deposit',
            [],
            {
                // amount, gasPrice and gasLimit must be explicitly provided
                version: VERSION,
                amount: new BN(8999999999999999999000),
                gasPrice: myGasPrice,
                gasLimit: Long.fromNumber(8000),
            },
            33,
            1000,
            false,
        );

        // Retrieving the transaction receipt (See note 2)
        console.log(JSON.stringify(callTx.receipt, null, 4));

        //Get the contract state
        console.log('Getting contract state...');
        const state = await deployedContract.getState();
        console.log('The state of the contract is:');
        console.log(JSON.stringify(state, null, 4));
    } catch (err) {
        console.log(err);
    }
}

testBlockchain();