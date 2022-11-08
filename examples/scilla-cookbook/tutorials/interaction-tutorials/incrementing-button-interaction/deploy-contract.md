---
sidebar_position: 2
---
# Deploy Contract

Review the Javascript that defines [deployContract.js](https://github.com/Zilliqa/Zilliqa-JavaScript-Library-Examples/blob/master/node/deployContract.js)

## Deploy button Contract

Using the deployContract.js file defined in zilliqa-js we have the ability to deploy a contract to a network. We provide a private key of the address which is deploying, the previous network configuration to establish a connection, in our case this is testnet, but could be mainnet or isolated environment.

We refactor the `deployContract.js` method to fit our usecase.

```js
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
      const myGasPrice = units.toQa('2000', units.Units.Li); // Gas Price that will be used by all transactions
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
        
        const init = [
            // this parameter is mandatory for all init arrays
            {
            vname: '_scilla_version',
            type: 'Uint32',
            value: '0',
            },
            {
            vname: 'contract_owner',
            type: 'ByStr20',
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
            false,
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
```

## Setup Program Starting Point

With our objectives defined we can begin to write how our program will achieve the functionality required. We have the first part complete, so let's write where our program will enter from and begin to test the deployment of DeployButtonContract contract to testnet.

```js
// Application Definition
// DEPLOY a contract
// SEND a transaction
// LISTEN to events emitted
// MONITOR mutable state
async function DeploySendListenMonitor()
{
    deployed_contract_base_16 = await DeployButtonContract();
    bech_32_bystr = toBech32Address(deployed_contract_base_16);
    console.log(`got ${bech_32_bystr} from ${deployed_contract_base_16}`)
}

// Start
DeploySendListenMonitor();
```

Using the command ```node index.js``` we can run the Javascript in the console and get the output.

```js
our account balance is:
{ balance: '1996734999999850', nonce: 299 }
Current Minimum Gas Price: 2000000000
My Gas Price 2000000000
Is the gas price sufficient? true
Deploying a new contract....
The transaction id is: 2e253603e77bfd104ffefa5015267f158f967a29f733803aec69d0f23ce290de
Waiting transaction be confirmed
The transaction status is:
{ cumulative_gas: 401, epoch_num: '3726609', success: true }
Contract address is: 0x8074237359dddba429ff1d02633a5d01e637ed91
got zil1sp6zxu6emhd6g20lr5pxxwjaq8nr0mv38w8msm from 0x8074237359dddba429ff1d02633a5d01e637ed91
```
