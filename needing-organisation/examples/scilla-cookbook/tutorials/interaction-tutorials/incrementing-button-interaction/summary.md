---
sidebar_position: 5
---

# Summary

We've met all the objectives set out in 300 lines of easy to read and understand Javascript and now have a form of primitive automation.

- [x] Use zilliqa-js to deploy a contract.
- [x] Use zilliqa-js to interact with and press the button.
- [x] Use zilliqa-js to get the mutable state of our contract.
- [x] Use zilliqa-js to listen for events in our contract.

```js
const { BN, Long, bytes, units } = require("@zilliqa-js/util");
const { Zilliqa } = require("@zilliqa-js/zilliqa");
const { StatusType, MessageType } = require("@zilliqa-js/subscriptions");
const {
  fromBech32Address,
  toBech32Address,
  getAddressFromPrivateKey,
} = require("@zilliqa-js/crypto");

// these values can change depending on what network you are connecting on
const zilliqa = new Zilliqa("https://dev-api.zilliqa.com");
// webhook to listen to events emited back to clients
const websocket = "wss://dev-ws.zilliqa.com";
const chainId = 333;
const msgVersion = 1;
const VERSION = bytes.pack(chainId, msgVersion);

// use a dotenv file for storing private keys, do not commit this directly to your repo
const privateKey = "";
// mutable state field name to monitor
const state_field_to_monitor = "total_count_clicks";

zilliqa.wallet.addByPrivateKey(privateKey);
const address_from_pk = getAddressFromPrivateKey(privateKey);

// Application Definition
// DEPLOY a contract,
// SEND a transaction
// LISTEN to events emitted
// MONITOR mutable state
async function DeploySendListenMonitor() {
  deployed_contract_base_16 = await DeployButtonContract();
  bech_32_bystr = toBech32Address(deployed_contract_base_16);
  console.log(`got ${bech_32_bystr} from ${deployed_contract_base_16}`);

  await Promise.all([
    ListenForEvents(deployed_contract_base_16),
    PressTheButton(bech_32_bystr),
  ]);
}

// Runs Application
DeploySendListenMonitor();

// Deploys a contract using a private key, returns the deployed_contract_base_16 of the contract deployed
async function DeployButtonContract() {
  try {
    // Get Balance
    const balance = await zilliqa.blockchain.getBalance(address_from_pk);
    // Get Minimum Gas Price from blockchain
    const minGasPrice = await zilliqa.blockchain.getMinimumGasPrice();

    // Account balance (See note 1)
    console.log(`Your account balance is:`);
    console.log(balance.result);
    console.log(`Current Minimum Gas Price: ${minGasPrice.result}`);
    const myGasPrice = units.toQa("2000", units.Units.Li); // Gas Price that will be used by all transactions
    console.log(`My Gas Price ${myGasPrice.toString()}`);
    const isGasSufficient = myGasPrice.gte(new BN(minGasPrice.result)); // Checks if your gas price is less than the minimum gas price
    console.log(`Is the gas price sufficient? ${isGasSufficient}`);

    // Deploy a contract
    console.log(`Deploying a new contract....`);
    const code = `
scilla_version 0
library MyFirstButton

let uint128_zero = Uint128 0
let uint128_one  = Uint128 1
let empty_bystr = 0x1111111111111111111111111111111111111111

(* Error exception *)
type Error =
| NotContractOwner
| NotUniqueClicker

let make_error =
fun (result : Error) =>
    let result_code = 
    match result with
    | NotContractOwner             => Int32 -1
    | NotUniqueClicker             => Int32 -2
    end
    in
    { _exception : "Error"; code : result_code }  
    
(*
* Create a scilla contract which models a button that can be pressed by anyone. 
* If you are the recent button presser you may not press the button again.
* When the button is pressed: Increment a counter and set the caller of the button press to be the most recent address.  
* The owner of the button has the ability to reset the counter to zero 
*)
contract MyFirstButton
(
contract_owner: ByStr20
)

field current_clicker    : ByStr20 = empty_bystr
field total_count_clicks : Uint128 = uint128_zero

(* 
@Dev: Emit Errors 
*)
procedure ThrowError(err : Error)
e = make_error err;
throw e
end

(*
@Dev : Throws an error if '_sender' is not 'contract_owner'
*)
procedure IsContractOwner()
is_contract_owner = builtin eq contract_owner _sender;
match is_contract_owner with
| True => 
    (* No Operation - Continue contract execution *)
| False =>
    err = NotContractOwner;
    ThrowError err
end
end

(*  
Dev: Increments 'current_clicker' by 'uint128_one'
*)
procedure IncrementCounter()
previous_click_count <- total_count_clicks;
new_click_count = builtin add previous_click_count uint128_one;
total_count_clicks := new_click_count;

e = {_eventname : "IncrementCounterSuccess"; pcc:previous_click_count; ncc: new_click_count  };
event e
end 


(*  
Dev: Throws an error if the current '_sender' is the previous 'current_clicker'
*)
procedure IsPreviousClicker(new_clicker: ByStr20)
previous_clicker <- current_clicker;
is_previous_clicker = builtin eq previous_clicker _sender;
match is_previous_clicker with
| True => 
    err = NotUniqueClicker;
    ThrowError err
| False =>
    (* No Operation - Continue contract execution *)
end
end

(*  
Dev: Sets 'new_clicker' as 'current_clicker'
*)
procedure SetNewClicker(new_clicker: ByStr20)
current_clicker := new_clicker;

e = {_eventname : "NewClickerState"; nc:new_clicker};
event e
end

(*  
Dev: Resets 'current_clicker' to 'uint128_zero'
*)
procedure ContractOwnerResetButton()
total_count_clicks  := uint128_zero
end

(*  
Dev: Sets 'new_clicker' as current_clicker
*)
transition PressTheButton()
IsPreviousClicker _sender;
SetNewClicker _sender;
IncrementCounter;

e = {_eventname : "PressTheButtonSuccess"; button_presser : _sender };
event e
end

(*  
Dev: Sets 'new_clicker' as current_clicker
*)
transition OwnerResetButton()
IsContractOwner;
ContractOwnerResetButton
end
`;

    const init = [
      // this parameter is mandatory for all init arrays
      {
        vname: "_scilla_version",
        type: "Uint32",
        value: "0",
      },
      {
        vname: "contract_owner",
        type: "ByStr20",
        value: `${address_from_pk}`,
      },
    ];

    const contract = zilliqa.contracts.new(code, init);

    // Deploy the contract.
    // Also notice here we have a default function parameter named toDs as mentioned above.
    // A contract can be deployed at either the shard or at the DS. Always set this value to false.
    const [deployTx, deployedContract] = await contract.deployWithoutConfirm(
      {
        version: VERSION,
        gasPrice: myGasPrice,
        gasLimit: Long.fromNumber(10000),
      },
      false
    );

    // process confirm
    console.log(`The transaction id is:`, deployTx.id);
    console.log(`Waiting transaction be confirmed`);
    const confirmedTxn = await deployTx.confirm(deployTx.id);

    console.log(`The transaction status is:`);
    console.log(confirmedTxn.receipt);
    if (confirmedTxn.receipt.success === true) {
      console.log(`Contract address is: 0x${deployedContract.address}`);
      return "0x" + deployedContract.address;
    }
  } catch (err) {
    console.log(err);
  }
}

// Listen for events from a contract - errors aren't caught
async function ListenForEvents(deployed_contract_base_16) {
  const subscriber = zilliqa.subscriptionBuilder.buildEventLogSubscriptions(
    websocket,
    {
      addresses: [deployed_contract_base_16],
    }
  );

  console.log("Listener started");

  subscriber.emitter.on(MessageType.EVENT_LOG, async (event) => {
    console.log("get new event log: ", JSON.stringify(event)); // this will emit 2/3 times before event emitted

    const current_button_click_count =
      await zilliqa.blockchain.getSmartContractSubState(
        deployed_contract_base_16,
        state_field_to_monitor
      );
    console.log(current_button_click_count.result);
    console.log("");
  });

  await subscriber.start();
}

// Calls the previously deployed contract transition
async function PressTheButton(bech_32_bystr) {
  try {
    const balance = await zilliqa.blockchain.getBalance(address_from_pk);
    const minGasPrice = await zilliqa.blockchain.getMinimumGasPrice();
    const myGasPrice = units.toQa("2000", units.Units.Li);
    const isGasSufficient = myGasPrice.gte(new BN(minGasPrice.result));
    const deployedContract = zilliqa.contracts.at(bech_32_bystr);

    console.log(`Pressing the button...`);
    const callTx = await deployedContract.callWithoutConfirm(
      "PressTheButton", // transition name
      [], // no vnames
      {
        version: VERSION,
        amount: new BN(0),
        gasPrice: myGasPrice,
        gasLimit: Long.fromNumber(8000),
      },
      false
    );

    console.log(`Waiting transaction be confirmed`);
    const confirmedTxn = await callTx.confirm(callTx.id);

    //console.log(`The transaction status is:`);
    //console.log(confirmedTxn.receipt);
    if (confirmedTxn.receipt.success === true) {
      console.log(`Button pressed by : ${address_from_pk}`);
    }
  } catch (err) {
    console.log(err);
  }
}
```

```js
Your account balance is:
{ balance: '1990534999999850', nonce: 307 }
Current Minimum Gas Price: 2000000000
My Gas Price 2000000000
Is the gas price sufficient? true
Deploying a new contract....
The transaction id is: 1548964c7d852fdb5c2b606a62aba83a5e89ce3ec047c3eac8986b0943e10da7
Waiting transaction be confirmed
The transaction status is:
{ cumulative_gas: 401, epoch_num: '3726858', success: true }
Contract address is: 0x60d5edfe4bcff11bde238c153853bcf358654b41
got zil1vr27mljtelc3hh3r3s2ns5au7dvx2j6pa6p9k0 from 0x60d5edfe4bcff11bde238c153853bcf358654b41
Listener started
Pressing the button...
Waiting transaction be confirmed

get new event log:  {"query":"EventLog"}
{ total_count_clicks: '0' }

get new event log:  {"query":"EventLog","value":[{"address":"60d5edfe4bcff11bde238c153853bcf358654b41","event_logs":[{"_eventname":"PressTheButtonSuccess","params":[{"type":"ByStr20","value":"0x428a2aa43456fe7fd2de66e48c1fbf372ec10eae","vname":"button_presser"}]},{"_eventname":"IncrementCounterSuccess","params":[{"type":"Uint128","value":"0","vname":"pcc"},{"type":"Uint128","value":"1","vname":"ncc"}]},{"_eventname":"NewClickerState","params":[{"type":"ByStr20","value":"0x428a2aa43456fe7fd2de66e48c1fbf372ec10eae","vname":"nc"}]}]}]}
{ total_count_clicks: '1' }

Button pressed by : 0x428A2aA43456FE7fd2De66E48C1fBf372eC10eAE
get new event log:  {"query":"EventLog"}
{ total_count_clicks: '1' }
```
