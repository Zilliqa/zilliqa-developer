---
id: dev-hardhat
title: Developing with Hardhat
keywords:
  - zilliqa
  - scilla
  - development
  - hardhat
  - smart contract
description: Getting started with Scilla and hardhat
---

---

<!-- markdownlint-disable MD036 -->

**Note that the Scilla hardhat plugin is currently experimental; use at your own risk**

Developing, testing and deploying contracts for both Scilla and EVM is
conveniently done with [hardhat](https://hardhat.org). There is a
(currently experimental) hardhat plugin allowing you to test Scilla
contracts.

To get this working:

### 1. Install the scilla tools

```sh
git clone https://github.com/Zilliqa/scilla.git
```

And follow the instructions in the `INSTALL.md`. This should leave you with the
`scilla-checker` and `scilla-fmt` binaries in your path.

### 2. Initialise a new hardhat project

```sh
mkdir contract_dev
cd contract_dev
yarn init
yarn add --dev hardhat
npx hardhat
```

Select `create a new typescript project`.

Now:

```sh
yarn add git+https://github.com/zilliqa/hardhat-scilla-plugin
```

and add the following to `hardhat.config.js`:

```sh
import "hardhat-scilla-plugin"
```

Now you can deploy a contract by adding the source code as `contracts/Hello.scilla`:

```scilla
scilla_version 0

library MetaSayHello

type Error =
| NotOwner

let make_error =
  fun (result: Error) =>
    let result_code =
      match result with
     | NotOwner => Int32 -1
      end
    in
    { _exception: "Error"; code: result_code }


contract SayHello(
 init_contract_owner: ByStr20,
 init_string : String
 )

field message : String = init_string
field owner : ByStr20 = init_contract_owner

procedure Throw(error: Error)
  e = make_error error;
  throw e
end

procedure AssertOwner(address: ByStr20)
   my_owner <- owner;
   is_owner = builtin eq my_owner address;
   match is_owner with
   | True => (* Yep *)
   | False =>
     err = NotOwner;
     Throw err
   end
end

transition SetMessage(in_message: String)
  AssertOwner _sender;
  message := in_message
end

transition SayHello()
  a_msg <- message;
  e = { _eventname: "Hello";
       message: a_msg
       };
  event e
end
```

and a deployment script, `scripts/deploy.ts`:

```typescript
import { expect } from 'chai';
import { ScillaContract, initZilliqa } from 'hardhat-scilla-plugin';
import hre, { ethers } from 'hardhat';

describe("Hello", function () {
    const privateKeys = [ "<your private key>" ];
    const network_url = "<network URL>";
    const chain_id = <chain_id>;

    before("set up the network", async function () {
        initZilliqa(network_url, chain_id, privateKeys);

        let contract: ScillaContract = await hre.deployScilla("SayHello", "5c2d46955de58033638f552bfd1bca408e6fc8ac", "TestA");
        console.log(`Contract ${JSON.stringify(contract)}`)
    });

    it("should do nothing", async function() {
    });
});
```

Run your script:

```sh
npx hardhat scripts/deploy.ts
```

It's often useful to run:

```sh
mitmweb --reverse:https://dev-api.zilliqa.com -p 8082
```

So you can monitor what calls are being made. You can generate keys for testing using `zli` (but one day there will be a plugin function to allow you to do this).

There are examples of this in [zilliqa-developer](https://github.com/zilliqa/zilliqa-developer) and in our acceptance test suite in our [acceptance tests](https://github.com/Zilliqa/Zilliqa/tree/master/tests/EvmAcceptanceTests).

Here is a sample `networks` stanza for your `hardhat.config.ts`; the keys configured for isolated server and other environments are the default test private keys, which are loaded with large balances at network startup:

```json
  networks: {
    isolated_server: {
      url: "http://localhost:5555/",
      websocketUrl: "ws://localhost:5555/",
      accounts: [
        "d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba",
        "589417286a3213dceb37f8f89bd164c3505a4cec9200c61f7c6db13a30a71b45",
        "e7f59a4beb997a02a13e0d5e025b39a6f0adc64d37bb1e6a849a4863b4680411",
        "410b0e0a86625a10c554f8248a77c7198917bd9135c15bb28922684826bb9f14"
      ],
      chainId: 0x8001,
      web3ClientVersion: "Zilliqa/v8.2",
      protocolVersion: 0x41,
      zilliqaNetwork: true,
      miningState: false
    },
    ganache: {
      url: "http://localhost:7545",
      websocketUrl: "ws://localhost:7545",
      chainId: 1337,
      web3ClientVersion: "Ganache/v7.4.1/EthereumJS TestRPC/v7.4.1/ethereum-js",
      protocolVersion: 0x3f,
      accounts: [
        // memonic: guard same cactus near figure photo remove letter target alien initial remove
        "67545ce31f5ca86719cf3743730435768515ebf014f84811463edcf7dcfaf91e",
        "9be4f8840833f64d4881027f4a53961d75bc649ac4801b33f746487ca8873f14",
        "32a75b674cc41405c914de1fe7b031b832dfd9203e1a287d09122bab689519e3",
        "dd8ce58f8cecd59fde7000fff9944908e89364b2ef36921c35725957617ddd32"
      ],
      zilliqaNetwork: false,
      miningState: true
    },
    public_testnet: {
      url: "https://evm-api-dev.zilliqa.com",
      websocketUrl: "https://evm-api-dev.zilliqa.com",
      accounts: [
        "d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba",
        "db11cfa086b92497c8ed5a4cc6edb3a5bfe3a640c43ffb9fc6aa0873c56f2ee3",
        "410b0e0a86625a10c554f8248a77c7198917bd9135c15bb28922684826bb9f14",
        "589417286a3213dceb37f8f89bd164c3505a4cec9200c61f7c6db13a30a71b45"
      ],
      chainId: 33101,
      zilliqaNetwork: true,
      web3ClientVersion: "Zilliqa/v8.2",
      protocolVersion: 0x41,
      miningState: false
    },
    localdev: {
      url: "http://localhost:5301",
      websocketUrl: "ws://localhost:5301",
      accounts: [
        "d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba",
        "589417286a3213dceb37f8f89bd164c3505a4cec9200c61f7c6db13a30a71b45",
        "e7f59a4beb997a02a13e0d5e025b39a6f0adc64d37bb1e6a849a4863b4680411",
        "410b0e0a86625a10c554f8248a77c7198917bd9135c15bb28922684826bb9f14"
      ],
      chainId: 0x8001,
      web3ClientVersion: "Zilliqa/v8.2",
      protocolVersion: 0x41,
      zilliqaNetwork: true,
      miningState: false
    }
  },
```
