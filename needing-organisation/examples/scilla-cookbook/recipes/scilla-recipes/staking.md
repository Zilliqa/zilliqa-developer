---
tags:
  - staking
---

# Staking

| Contract Name | File and Location                                                                                                  | Description                                                                                                                                                                                                                                         | Mainnet                                                                                                                                    |
| ------------- | ------------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| SSNList       | [`ssnlist.scilla`](https://github.com/Zilliqa/staking-contract/blob/main/contracts/ssnlist.scilla)                 | The main contract that keeps track of Staked Seed Nodes _aka_ SSNs, the delegators, the amount staked by a delegator with an SSN, and available rewards, etc.                                                                                       | [zil15lr86jwg937urdeayvtypvhy6pnp6d7p8n5z09](https://viewblock.io/zilliqa/address/zil15lr86jwg937urdeayvtypvhy6pnp6d7p8n5z09?txsType=subs) |
| SSNListProxy  | [`proxy.scilla`](https://github.com/Zilliqa/staking-contract/blob/main/contracts/proxy.scilla)                     | A proxy contract that sits on top of the SSNList contract. Any call to the `SSNList` contract must come from `SSNListProxy`. This contract facilitates upgradeability of the `SSNList` contract in case a bug is found.                             | [zil1v25at4s3eh9w34uqqhe3vdvfsvcwq6un3fupc2](https://viewblock.io/zilliqa/address/zil1v25at4s3eh9w34uqqhe3vdvfsvcwq6un3fupc2?tab=state)    |
| Wallet        | [`multisig_wallet.scilla`](https://github.com/Zilliqa/staking-contract/blob/main/contracts/multisig_wallet.scilla) | A multisig wallet contract tailored to work with the `SSNListproxy` contract. Certain transitions in the `SSNListProxy` contract can only be invoked when k-out-of-n users have agreed to do so. This logic is handled using the `Wallet` contract. | [zil1ur8ehr9qeqrgkgf3qj3ruv5dyt0w8nj53drvuz](https://viewblock.io/zilliqa/address/zil1ur8ehr9qeqrgkgf3qj3ruv5dyt0w8nj53drvuz)              |

## Further reading

[Viewblock Staking SSN Overview](https://viewblock.io/zilliqa/staking)

[Zilliqa Staking Github](https://github.com/Zilliqa/staking-contract)
