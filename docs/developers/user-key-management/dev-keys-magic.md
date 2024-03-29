---
id: dev-keys-magic
title: Magic
keywords:
  - magic
  - zilliqa
  - email login
  - simple singup
  - fortmatic
description: Zilliqa Magic Integration Email Login Without Private Key
---

---

[Magic](https://docs.magic.link/) is a developer SDK that you can integrate into your application to enable passwordless authentication using magic links - similar to Slack and Medium.

When users want to sign up or log in to your application:

1. User requests a magic link sent to their email address
2. User clicks on that magic link
3. User is securely logged into the application

If it's a web application, users are logged into the original tab, even if the user clicked on the magic link on a different browser or mobile device!

!!! note

    You can skip straight to our reference example:

    👉 [Magic Zilliqa Integration](https://github.com/Zilliqa/dev-portal-examples)

## Installation

Magic interacts with the Zilliqa blockchain via Magic's extension NPM package @magic-ext/zilliqa. The Zilliqa extension also lets you interact with the blockchain using methods from Zilliqa's Javascript SDK.

=== "Npm"

    ```js
    npm install --save @magic-ext/zilliqa
    ```

=== "Yarn"

    ```js
    yarn add @magic-ext/zilliqa
    ```

## Initializing Extension

To initialize the magic extension, you would need to specify the Zilliqa RPC Node URL which can be found [here](../getting-started/dev-started-env.md).
You would need to specify your API Key for Magic, which you'll get by signing up on Magic's [dashboard](https://dashboard.magic.link/signup) - if you face any issues, refer to Magic's [documentation](https://dashboard.magic.link/signup).

=== "JavaScript"

    ```js
    import { Magic } from "magic-sdk";
    import { ZilliqaExtension } from "@magic-ext/zilliqa";

    const magic = new Magic("YOUR_API_KEY", {
      extensions: [
        new ZilliqaExtension({
          rpcUrl: "Zilliqa_RPC_NODE_URL",
        }),
      ],
    });
    ```

## Get User Wallet

### Get Wallet

Using getWallet function to get a Zilliqa wallet for the current user.

=== "JavaScript"

    ```js
    import { Magic } from "magic-sdk";
    import { ZilliqaExtension } from "@magic-ext/zilliqa";

    const magic = new Magic("YOUR_API_KEY", {
      extensions: [
        new ZilliqaExtension({
          rpcUrl: "Zilliqa_RPC_NODE_URL",
        }),
      ],
    });

    // Get user's Zilliqa wallet info
    const wallet = await magic.zilliqa.getWallet();
    console.log("Zilliqa wallet: ", wallet);
    ```

## Send Transaction

### Getting Test ZIL

Before you can send transaction on the Zilliqa blockchain, you'll need to acquire some test ZIL (Zilliqa's native cryptocurrency for test network).

1. Go to our [Magic Example](https://github.com/Zilliqa/dev-portal-examples/tree/master/magic-example) application
2. Login with your email address
3. Copy your Zilliqa public address
4. Go to the ZIL Faucet
5. Paste your copied Zilliqa public address in the text input
6. You can receive 300 test ZIL
7. Now you can use your test ZIL in our example app

### Call Extension Method

To send a standard Zilliqa blockchain transaction, you can call the magic.zil.sendTransaction method.

=== "JavaScript"

    ```js
    import { Magic } from "magic-sdk";
    import { ZilliqaExtension } from "@magic-ext/zilliqa";
    const { BN, Long, bytes, units } = require("@zilliqa-js/util");

    const magic = new Magic("YOUR_API_KEY", {
      extensions: [
        new ZilliqaExtension({
          rpcUrl: "Zilliqa_RPC_NODE_URL",
        }),
      ],
    });

    const chainId = 333; // chainId of the developer testnet
    const msgVersion = 1; // current msgVersion
    const VERSION = bytes.pack(chainId, msgVersion);

    const myGasPrice = units.toQa("1000", units.Units.Li);

    const params = {
      version: VERSION,
      toAddr: "zil14vut0rh7q78ydc0g7yt7e5zkfyrmmps00lk6r7",
      amount: new BN(units.toQa("0.5", units.Units.Zil)),
      gasPrice: myGasPrice,
      gasLimit: Long.fromNumber(1),
    };

    // Send a transaction
    const tx = await magic.zil.sendTransaction(params, false);
    console.log("send transaction", tx);
    ```

## Deploy Smart Contract

### Getting Test ZIL

Before you can send transaction on the Zilliqa blockchain, you'll need to acquire some test ZIL (Zilliqa's native cryptocurrency for test network).

1. Go to our [Magic Example](https://github.com/Zilliqa/dev-portal-examples/tree/master/magic-example) application
2. Login with your email address
3. Copy your Zilliqa public address
4. Go to the ZIL Faucet
5. Paste your copied Zilliqa public address in the text input
6. You can receive 300 test ZIL
7. Now you can use your test ZIL in our example app

### Call Extension Method

To deploy a smart contract, you can call the magic.zilliqa.deployContract method.

=== "JavaScript"

    ```js
    import { Magic } from "magic-sdk";
    import { ZilliqaExtension } from "@magic-ext/zilliqa";
    const { BN, Long, bytes, units } = require("@zilliqa-js/util");

    const magic = new Magic("YOUR_API_KEY", {
      extensions: [
        new ZilliqaExtension({
          rpcUrl: "Zilliqa_RPC_NODE_URL",
        }),
      ],
    });

    const wallet = await magic.zilliqa.getWallet();

    const address = wallet.address;

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

    const chainId = 333; // chainId of the developer testnet
    const msgVersion = 1; // current msgVersion
    const VERSION = bytes.pack(chainId, msgVersion);

    const myGasPrice = units.toQa("1000", units.Units.Li);

    const params = {
      version: VERSION,
      gasPrice: myGasPrice,
      gasLimit: Long.fromNumber(10000),
    };

    const result = await magic.zil.deployContract(
      init,
      code,
      params,
      33,
      1000,
      false
    );

    console.log("deploy contract", result);
    ```
