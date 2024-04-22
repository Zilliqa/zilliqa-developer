---
id: remix-project
title: Remix Project
keywords:
  - Remix
  - IDE
description: Remix is a browser-based compiler and IDE that enables users to deploy, interact with and debug Zilliqa Solidity contracts
---

---

## Remix IDE

### Example Smart Contract Deployment

Once you have set up Metamask to have the Zilliqa EVM configuration, then open [Remix](https://remix.ethereum.org/).

Open the Workspaces folder and right click compile the file called `1_Storage.sol` which will build some artifacts.

<img alt="Compiling a contract" src="/assets/img/evm/compile_contract.png" width="400px">

Next, open `Deploy & Run Transactions`. In `Environment` click the option `Injected Provider - Metamask` which will open a dialog with metamask for users to confirm the connection. This should read `Custom (33101) network` below the environment if your wallet is setup with the Zilliqa EVM connection.

<img alt="Deploying a compiled contract" src="/assets/img/evm/deploy_contract.png" width="400px">

Metamask should open a dialog and ask you to sign the deployment transaction.

On success metamask will open another dialog showing the transactionID of the deployed contract. Navigating to the code for the contract, you can see that it's compiled byte code.

<img alt="Metamask deployment tx" src="/assets/img/evm/metamask_deploy.png" width="400px">

You've successfully deployed an EVM contract to Zilliqa EVM network.

Below the deploy button is all of the deployed contracts you own, along with an interactive ABI, so you can call and view the state of the contract.
