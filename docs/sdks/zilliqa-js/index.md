# Zilliqa JS SDK

## Introduction

`zilliqa-js` is structured as a Lerna monorepo. Each package roughly corresponds
to a discrete `ZilliqaModule`, most of which can be used independently.

The only required package is `@zilliqa-js/core`, which contains the default
`HTTPProvider`, and other core abstractions that are necessary for other modules
to function.

The following table provides a description of each module and what you may want
to use it for.

| Package                                                                         | Version                                                                                                                               | Description                                                                                                                                                               | Dependencies                                                  |
| ------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------- |
| [`@zilliqa-js/core`](./zilliqa/typescript/zilliqa-js-core)                      | [![npm](https://img.shields.io/npm/v/@zilliqa-js/core.svg)](https://www.npmjs.com/package/@zilliqa-js/core)                           | Core abstractions and base classes, such as `HTTPProvider` and network logic for interfacing with the Zilliqa JSON-RPC.                                                   |                                                               |
| [`@zilliqa-js/util`](./zilliqa/typescript/zilliqa-js-util)                      | [![npm](https://img.shields.io/npm/v/@zilliqa-js/util.svg)](https://www.npmjs.com/package/@zilliqa-js/util)                           | Miscellaneous functions that take care of serialisation/deserialisation and validation.                                                                                   |                                                               |
| [`@zilliqa-js/crypto`](./zilliqa/typescript/zilliqa-js-crypto)                  | [![npm](https://img.shields.io/npm/v/@zilliqa-js/crypto.svg)](https://www.npmjs.com/package/@zilliqa-js/crypto)                       | Exposes several loosely-coupled cryptographic convenience functions for working with the Zilliqa blockchain and its cryptographic primitives, such as Schnorr signatures. | `@zilliqa-js/util`                                            |
| [`@zilliqa-js/proto`](./zilliqa/typescript/zilliqa-js-proto)                    | [![npm](https://img.shields.io/npm/v/@zilliqa-js/proto.svg)](https://www.npmjs.com/package/@zilliqa-js/proto)                         | Protobuf source files and corresponding generated JS modules.                                                                                                             |                                                               |
| [`@zilliqa-js/account`](./zilliqa/typescript/zilliqa-js-account)                | [![npm](https://img.shields.io/npm/v/@zilliqa-js/account.svg)](https://www.npmjs.com/package/@zilliqa-js/account)                     | `Wallet`, `Account` and `Transaction` abstractions.                                                                                                                       | `@zilliqa-js/core`, `@zilliqa-js/crypto`, `@zilliqa-js/proto` |
| [`@zilliqa-js/blockchain`](./zilliqa/typescript/zilliqa-js-blockchain)          | [![npm](https://img.shields.io/npm/v/@zilliqa-js/blockchain.svg)](https://www.npmjs.com/package/@zilliqa-js/blockchain)               | Main interface to the Zilliqa `JSON-RPC`.                                                                                                                                 | `@zilliqa-js/account`                                         |
| [`@zilliqa-js/contract`](./zilliqa/typescript/zilliqa-js-contract)              | [![npm](https://img.shields.io/npm/v/@zilliqa-js/contract.svg)](https://www.npmjs.com/package/@zilliqa-js/contract)                   | Exposes a `Contracts` module that takes care of smart contract deployment and interaction.                                                                                | `@zilliqa-js/blockchain`                                      |
| [`@zilliqa-js/subscriptions`](./zilliqa/typescript/zilliqa-js-util)             | [![npm](https://img.shields.io/npm/v/@zilliqa-js/subscriptions.svg)](https://www.npmjs.com/package/@zilliqa-js/subscriptions)         | Classes to interact with the websocket API of Zilliqa node.                                                                                                               |                                                               |
| [`@zilliqa-js/scilla-json-utils`](https://github.com/Zilliqa/scilla-json-utils) | [![npm](https://img.shields.io/npm/v/@zilliqa-js/scilla-json-utils.svg)](https://www.npmjs.com/package/@zilliqa-js/scilla-json-utils) | Simplifies the way you construct the Scilla JSON data.                                                                                                                    |                                                               |
| [`@zilliqa-js/viewblock`](https://github.com/Ashlar/zilliqa-js-viewblock)       | [![npm](https://img.shields.io/npm/v/@zilliqa-js/viewblock.svg)](https://www.npmjs.com/package/@zilliqa-js/viewblock)                 | Library interfacing with ViewBlock's APIs. This package is maintained by [Ashlar](https://github.com/Ashlar)                                                              | `@zilliqa-js/crypto`                                          |

### Package dependency graph

```mermaid
 graph TD
   scilla-json-utils
   proto
   util
   core
   contract --> blockchain
   blockchain --> account
   crypto --> util
   account --> crypto
   account ---> core
   account ---> proto
   subscriptions
   viewblock
```

## Pre-Requisite (Windows Users)

`zilliqa-js` uses [`scrypt`](https://www.npmjs.com/package/scrypt) library which
depends on `node-gyp` in order to compile the binaries from source on Windows.
`node-gyp` on Windows requires users to install additional Visual Studio Build
tools.

To install the required Visual Studio Build tools:

```shell
npm install --global --production windows-build-tools # from an elevated PowerShell or CMD.exe as Administrator

npm config set msvs_version 2015 # 2015 is more compatible; though 2017 may work too
```

Refer to
[the installation guide](https://github.com/nodejs/node-gyp#installation) for
more information about `node-gyp` installation on Windows.

## Installation

It is recommended that developers install the JavaScript client by making use of
the umbrella package `@zilliqa-js/zilliqa`. This takes care of bootstrapping the
various modules, which are then accessible as members of the `Zilliqa` class.

```shell
npm i @zilliqa-js/zilliqa
# or
yarn add @zilliqa-js/zilliqa
```

## Quick Start

You can create a test account by using
[Zilliqa Dev Wallet](https://github.com/Zilliqa/dev-wallet) and request funds by
using [ZIL Faucet service](https://dev-wallet.zilliqa.com/faucet).

```javascript
const { BN, Long, bytes, units } = require("@zilliqa-js/util");
const { Zilliqa } = require("@zilliqa-js/zilliqa");
const {
  toBech32Address,
  getAddressFromPrivateKey,
} = require("@zilliqa-js/crypto");

const zilliqa = new Zilliqa("https://dev-api.zilliqa.com");

// These are set by the core protocol, and may vary per-chain.
// You can manually pack the bytes according to chain id and msg version.
// For more information: https://apidocs.zilliqa.com/?shell#getnetworkid

const chainId = 333; // chainId of the developer testnet
const msgVersion = 1; // current msgVersion
const VERSION = bytes.pack(chainId, msgVersion);

// Populate the wallet with an account
const privateKey =
  "3375F915F3F9AE35E6B301B7670F53AD1A5BE15D8221EC7FD5E503F21D3450C8";

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
    const myGasPrice = units.toQa("2000", units.Units.Li); // Gas Price that will be used by all transactions
    console.log(`My Gas Price ${myGasPrice.toString()}`);
    const isGasSufficient = myGasPrice.gte(new BN(minGasPrice.result)); // Checks if your gas price is less than the minimum gas price
    console.log(`Is the gas price sufficient? ${isGasSufficient}`);

    // Send a transaction to the network
    console.log("Sending a payment transaction to the network...");
    const tx = await zilliqa.blockchain.createTransaction(
      // Notice here we have a default function parameter named toDs which means the priority of the transaction.
      // If the value of toDs is false, then the transaction will be sent to a normal shard, otherwise, the transaction.
      // will be sent to ds shard. More info on design of sharding for smart contract can be found in.
      // https://blog.zilliqa.com/provisioning-sharding-for-smart-contracts-a-design-for-zilliqa-cd8d012ee735.
      // For payment transaction, it should always be false.
      zilliqa.transactions.new(
        {
          version: VERSION,
          toAddr: "0xA54E49719267E8312510D7b78598ceF16ff127CE",
          amount: new BN(units.toQa("1", units.Units.Zil)), // Sending an amount in Zil (1) and converting the amount to Qa
          gasPrice: myGasPrice, // Minimum gasPrice veries. Check the `GetMinimumGasPrice` on the blockchain
          gasLimit: Long.fromNumber(50),
        },
        false
      )
    );

    console.log(`The transaction status is:`);
    console.log(tx.receipt);

    // Deploy a contract
    console.log(`Deploying a new contract....`);
    const code = `scilla_version 0

    (* HelloWorld contract *)

    import ListUtils

    (***************************************************)
    (*               Associated library                *)
    (***************************************************)
    library HelloWorld

    let not_owner_code = Int32 1
    let set_hello_code = Int32 2

    (***************************************************)
    (*             The contract definition             *)
    (***************************************************)

    contract HelloWorld
    (owner: ByStr20)

    field welcome_msg : String = ""

    transition setHello (msg : String)
      is_owner = builtin eq owner _sender;
      match is_owner with
      | False =>
        e = {_eventname : "setHello()"; code : not_owner_code};
        event e
      | True =>
        welcome_msg := msg;
        e = {_eventname : "setHello()"; code : set_hello_code};
        event e
      end
    end


    transition getHello ()
        r <- welcome_msg;
        e = {_eventname: "getHello()"; msg: r};
        event e
    end`;

    const init = [
      // this parameter is mandatory for all init arrays
      {
        vname: "_scilla_version",
        type: "Uint32",
        value: "0",
      },
      {
        vname: "owner",
        type: "ByStr20",
        value: `${address}`,
      },
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
      false
    );

    // Introspect the state of the underlying transaction
    console.log(`Deployment Transaction ID: ${deployTx.id}`);
    console.log(`Deployment Transaction Receipt:`);
    console.log(deployTx.txParams.receipt);

    // Get the deployed contract address
    console.log("The contract address is:");
    console.log(hello.address);
    //Following line added to fix issue https://github.com/Zilliqa/zilliqa-js/issues/168
    const deployedContract = zilliqa.contracts.at(hello.address);

    // Create a new timebased message and call setHello
    // Also notice here we have a default function parameter named toDs as mentioned above.
    // For calling a smart contract, any transaction can be processed in the DS but not every transaction can be processed in the shards.
    // For those transactions are involved in chain call, the value of toDs should always be true.
    // If a transaction of contract invocation is sent to a shard and if the shard is not allowed to process it, then the transaction will be dropped.
    const newMsg = "Hello, the time is " + Date.now();
    console.log("Calling setHello transition with msg: " + newMsg);
    const callTx = await hello.call(
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
      33,
      1000,
      false
    );

    // Retrieving the transaction receipt (See note 2)
    console.log(JSON.stringify(callTx.receipt, null, 4));

    //Get the contract state
    console.log("Getting contract state...");
    const state = await deployedContract.getState();
    console.log("The state of the contract is:");
    console.log(JSON.stringify(state, null, 4));
  } catch (err) {
    console.log(err);
  }
}

testBlockchain();
```

### Notes on the Quick Start script

#### Note 1: Account balance

The account balance is an object with two fields, `balance` and `nonce`.

`balance` is the account balance in Qa, which is the lowest denomination in
Zilliqa. For more information about gas accounting,
[please refer to here](https://forum.zilliqa.com/t/gas-accounting-in-zilliqa/199)

`nonce` is a counter that keeps track of how many transactions are sent from a
given address. In Zilliqa, every transaction sent from an address must have a
unique nonce.

```json
{ "balance": "296505000000000", "nonce": "3" }
```

#### Note 2: Retrieving transaction receipt

An example of a transaction receipt is this:

```json
{
  "cumulative_gas": 357,
  "epoch_num": "676201",
  "event_logs": [
    {
      "_eventname": "setHello()",
      "address": "0x7a4aa130650396ab7c4006c471576a8404f5092b",
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
```

`event_logs` comprises of all the events emitted in the transaction. For
example, if your transaction calls a transition which emits 3 events, it will be
an array of three events. The `address` is the contract address of the contract
which emits the event.

`success` indicates if the transaction is successful.

## Examples

For more examples, visit this
[repository](https://github.com/Zilliqa/zilliqa-js-Examples).

## API Documentation

Each package contains API documentation. For convenience, these are links to the
respective `README` documents.

- [`account`](./zilliqa/typescript/zilliqa-js-account/README.md)
- [`blockchain`](./zilliqa/typescript/zilliqa-js-blockchain/README.md)
- [`contract`](./zilliqa/typescript/zilliqa-js-contract/README.md)
- [`core`](./zilliqa/typescript/zilliqa-js-core/README.md)
- [`crypto`](./zilliqa/typescript/zilliqa-js-crypto/README.md)
- [`proto`](./zilliqa/typescript/zilliqa-js-proto/README.md)
- [`util`](./zilliqa/typescript/zilliqa-js-util/README.md)

Also, you can use
[viewblock library](https://github.com/Ashlar/zilliqa-js-viewblock) to interface
with ViewBlock's APIs.

## Development

This repository makes use of several technologies to provide a better and faster
development experience for contributors, and has to be bootstrapped before one
can do productive work.

### Bootstrapping

`zilliqa-js` leverages Project References, which is available in TypeScript from
version `3.x`. As such, the build process is slightly different.

```shell
# install all dependencies and shared devDependencies
yarn install

# symlink packages, compile TS source files, and generate protobuf files.
yarn bootstrap

# watch TS source files and recompile on change
yarn build:ts -w
```

### Unit Tests

Tests for each package reside in `zilliqa/typescript/src/*/tests`, and are run
using `jest`.

### E2E Tests

We can easily simulate a publish using [Verdaccio](https://verdaccio.org/) which
is a private npm proxy registry. For more details check
[tasks/local-registry.sh](tasks/local-registry.sh)

### Bundling

#### rollup

`zilliqa-js` is bundled using `rollup`. To build the distributable bundles,
simple run `yarn bundle`. This will output two bundles, `*.umd.js` and
`*.esm.js`, to `zilliqa/typescript/*/dist`. Node.js clients are pointed to the
`umd` bundle, and bundlers are pointed to `esm`.

NOTE: these bundles are _not_ minified.

#### webpack

To build an all-in-one static js file, first install `webpack` globally using
`yarn global add webpack`. Then run `yarn build:web`. This will generate a
`dist` folder in the current path, which contains a file called
`zilliqa.min.js`. It can be used in normal html file. (A more specific example
please refer to `example/webpack`)

_NOTE: there may be some issue to install webpack with npm, thus using yarn is a
recommended way_

## Licence

You can view our [licence here](LICENSE).
