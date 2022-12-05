---
id: developer-introduction
title: Developer Introduction
keywords:
  - Developers
  - Dapps
description: ZILEVM for developers
---

---

## Developer Introduction

### Remix IDE

#### Example Smart Contract Deployment

Once you have set up Metamask to have the Zilliqa EVM configuration, then open [Remix](https://remix.ethereum.org/).

Open the Workspaces folder and right click compile the file called `1_Storage.sol` which will build some artifacts.

[Compiling a contract](/img/evm/compile_contract.png)

Next, open `Deploy & Run Transactions`. In `Environment` click the option `Injected Provider - Metamask` which will open a dialog with metamask for users to confirm the connection. This should read `Custom (33101) network` below the environment if your wallet is setup with the Zilliqa EVM connection.

[Deploying a compiled contract](/img/evm/deploy_contract.png)

Metamask should open a dialog and ask you to sign the deployment transaction.

On success metamask will open another dialog showing the transactionID of the deployed contract. Navigating to the code for the contract, you can see that it's compiled byte code.

[Metamask Deploying](/img/evm/metamask_deploy.png)

You've successfully deployed an EVM contract to Zilliqa EVM network.

Below the deploy button is all of the deployed contracts you own, along with an interactive ABI, so you can call and view the state of the contract.
