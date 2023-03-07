---
sidebar_position: 1
tags:
  - zilliqa-js
  - interaction
  - state
  - deploy
  - event
  - listener
  - query
  - transaction
  - batch
  - sign
---

# zilliqa-js quickstart

## Getting started

You will need Node and NPM installed on your machine for this guide.

Read the [repo readme](https://github.com/Zilliqa/Zilliqa-JavaScript-Library) to get started with `npm installing` zilliqa-js to your local repo.

Create a burner testing wallet and fund it with tokens from the [faucet](https://dev-wallet.zilliqa.com/faucet?network=testnet).

:::danger
Do not publish your private key as a git artefact, the wallet is at risk.
:::

:::tip
Use a .env file and add it to your .gitignore
:::

## HelloWorld Interaction Example

Review the documentation from the [examples repo for zilliqa-js](https://github.com/Zilliqa/Zilliqa-JavaScript-Library-Examples)

Call a specific example by installing the dependencies and invoking the example command.

In this example we are calling the transition `setHello` with on the `HelloWorld` contract with one vname called `msg` which has the value `newMsg` the contract is at address `zil1v6tjt9s0nua80tvvays5m2g763npxgkez0gnnq` on the `dev-api testnet`.

```ocaml
transition setHello (msg : String)
```

```bash
git clone Zilliqa/Zilliqa-JavaScript-Library-Examples .
cd Zilliqa-JavaScript-Library-Examples
cd node
npm i
node helloWorld.js
```

```js
// Import zilliqa-js
const { BN, Long, bytes, units } = require("@zilliqa-js/util");
const { Zilliqa } = require("@zilliqa-js/zilliqa");
const {
  toBech32Address,
  getAddressFromPrivateKey,
} = require("@zilliqa-js/crypto");

// What network to connect on
const zilliqa = new Zilliqa("https://dev-api.zilliqa.com");

// These are set by the core protocol, and may vary per-chain and network.
// For more information: https://apidocs.zilliqa.com/?shell#getnetworkid
const chainId = 333; // chainId of the developer testnet
const msgVersion = 1; // current msgVersion
const VERSION = bytes.pack(chainId, msgVersion);

// Populate the wallet with an account
const privateKey =
  "deb5c896228f8515146aa16f94a558ba14e52d8496b4b267b2d59cd9036f39a6";

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

    console.log(`Your account balance is:`);
    console.log(balance.result);
    console.log(`Current Minimum Gas Price: ${minGasPrice.result}`);
    const myGasPrice = units.toQa("2000", units.Units.Li); // Gas Price that will be used by all transactions
    console.log(`My Gas Price ${myGasPrice.toString()}`);
    const isGasSufficient = myGasPrice.gte(new BN(minGasPrice.result)); // Checks if your gas price is less than the minimum gas price
    console.log(`Is the gas price sufficient? ${isGasSufficient}`);

    const deployedContract = zilliqa.contracts.at(
      "zil1v6tjt9s0nua80tvvays5m2g763npxgkez0gnnq"
    );

    // Create a new timebased message and call setHello
    // Also notice here we have a default function parameter named toDs as mentioned above.
    // For calling a smart contract, any transaction can be processed in the DS but not every transaction can be processed in the shards.
    // For those transactions are involved in chain call, the value of toDs should always be true.
    // If a transaction of contract invocation is sent to a shard and if the shard is not allowed to process it, then the transaction will be dropped.
    const newMsg = "Hello, the time is " + Date.now();
    console.log("Calling setHello transition with msg: " + newMsg);
    const callTx = await deployedContract.callWithoutConfirm(
      "setHello",
      [
        {
          vname: "msg",
          type: "String",
          value: newMsg,
        },
      ],
      {
        // amount, gasPrice and gasLimit must be explicitly provided
        version: VERSION,
        amount: new BN(0),
        gasPrice: myGasPrice,
        gasLimit: Long.fromNumber(8000),
      },
      false
    );

    console.log(callTx.bytes);

    // process confirm
    console.log(`The transaction id is:`, callTx.id);
    console.log(`Waiting transaction be confirmed`);
    const confirmedTxn = await callTx.confirm(callTx.id);

    console.log(`The transaction status is:`);
    console.log(confirmedTxn.receipt);
    if (confirmedTxn.receipt.success === true) {
      console.log(`Contract address is: ${deployedContract.address}`);
    }
  } catch (err) {
    console.log(err);
  }
}

testBlockchain();
```

To which, the console will respond with.

```bash
My account address is: 0x8254b2C9aCdf181d5d6796d63320fBb20D4Edd12
My account bech32 address is: zil1sf2t9jdvmuvp6ht8jmtrxg8mkgx5ahgj6h833r

Your account balance is:
{ balance: '10722145999990000', nonce: 1468 }

Current Minimum Gas Price: 2000000000
My Gas Price 2000000000
Is the gas price sufficient? true

Sending a payment transaction to the network...

The transaction id is: 035a2ae08d0b4d12f31ee6c0315d91b4bb9150c1f078fc88e0ee3b5640f2d318

The transaction status is:
{ cumulative_gas: 50, epoch_num: '3129050', success: true }

Deploying a new contract....
Deployment Transaction ID: 6a67c5d1fb65f7fae9c02ee81b37fb5855bbfd3d17d163a908bb5f667419d1df

Deployment Transaction Receipt:
{ cumulative_gas: 482, epoch_num: '3129052', success: true }
The contract address is:
0xD1F5c962F1c6A77253BFD799B3472D05de414ae2

Calling setHello transition with msg: Hello, the time is 1629086853792
{
    "accepted": false,
    "cumulative_gas": 357,
    "epoch_num": "3129054",
    "event_logs": [
        {
            "_eventname": "setHello()",
            "address": "0xd1f5c962f1c6a77253bfd799b3472d05de414ae2",
            "params": [
                {
                    "type": "Int32",
                    "value": "2",
                    "vname": "code"
                }
            ]
        }
    ],
    "success": true
}

Getting contract state...
The state of the contract is:
{
    "_balance": "0",
    "welcome_msg": "Hello, the time is 1629086853792"
}
```

## Address Interaction

```js
const { toBech32Address, toChecksumAddress } = require("@zilliqa-js/crypto");

// not checksummed address (will not be accepted by blockchain)
const address = "573EC96638C8BB1C386394602E1460634F02ADDA";

// checksummed ByStr20
const checksummedAddresses = toChecksumAddress(address);
// returns '0x573EC96638C8bB1c386394602E1460634F02aDdA'

const bech32_address = toBech32Address(address);
// returns zil12ulvje3ceza3cwrrj3szu9rqvd8s9tw69c978p
```

## Example Interaction Scripts

```js
//shows the javascript required to call a specific transition
node callContract.js

// shows the javascript required to create a list of signed batch transactions
node createBatchTransaction.js

//shows the javascript required to create a list of batch transactions with correct nonce
node createBatchTransactionWithoutConfirm.js

//shows the javascript required to post a raw transaction
createTransactionRaw.js

//shows the javascript required to deploy a contract
deployContract.js

//shows the javascript required to return all the particular transactions that happened for a particular block
getTxnBodiesForTxBlock.js

//shows the javascript required to interact with the helloWorld tutorial scilla
helloWorld.js

//shows the javascript required to listen for events
newEventLogSubscriptions.js

//shows the javascript required to listen for new block events
newTxBlockSubscriptions.js

//shows the javascript required to query init params, or state params including maps
queryState.js

//get a particular transaction from a given transaction hash
queryTransaction.js

//get the status of a particular transaction from a given transaction hash
queryTransactionStatus.js

//shows the javascript required to sign a unsigned batch transaction
signBatchTransaction.js

//shows examples of offline and online signs
walletSign.js
```

## zilliqa-js Interaction Examples

### zilliqa-js Examples

[zilliqa-js Example Interaction](https://github.com/Zilliqa/Zilliqa-JavaScript-Library-Examples/tree/master/node)

### TheDrBee contract Interactions

[zilliqa-js contract Interaction](https://github.com/TheDrBee/oSCILLAtor/tree/main/js)

### Zilpay Repositories

[Zilpay Browser Extension](https://github.com/zilpay/zil-pay)
