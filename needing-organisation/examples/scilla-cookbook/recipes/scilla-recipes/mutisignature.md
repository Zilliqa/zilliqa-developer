---
tags:
  - mutisignature
  - mutisig
  - contract
  - reference
  - zrc4
---

# Muti-signature contracts

In blockchain-based applications, activities might need to be authorised by multiple blockchain addresses. For example, a monetary transaction may require authorisation from multiple blockchain addresses.

All the addresses that can authorise an activity might not be reachable at the time of issuing the transaction due to the sporadic or limited availability of some authorities. Also, it may not be possible to decide on all possible approvers in advance. How can a transaction be authorised by a subset (> 1) of approvers?

[ZRC-4](https://github.com/Zilliqa/ZRC/blob/master/zrcs/zrc-4.md) defines the technical specification document that defines the multisignature implementation.

## How does it work?

This contract holds funds that can be paid out to arbitrary users, provided that enough people in the collection of owners sign off on the payout.

The transaction must be added to the contract before signatures can be collected. Once enough signatures are collected,the recipient can ask for the transaction to be executed and the money paid out.

If an owner changes his mind about a transaction, the signature can be revoked until the transaction is executed.

:::note
This wallet does not allow adding or removing owners, or changing the number of required signatures. To do any of those things, read the readme baked into the contract for further information.
:::

## What could go wrong?

:::warning
If a sufficient number of owners lose their private keys, or for any other reason are unable or unwilling to sign for new transactions, the funds in the wallet will be locked forever. It is therefore a good idea to set required_signatures to a value strictly less than the number of owners, so that the remaining owners can retrieve the funds should such a scenario occur.

If an owner loses his private key, the remaining owners should move the funds to a new wallet (using the workflow baked into the readme of the contract) to ensure that funds are not locked if another owner loses his private key. The owner who originally lost his private key can generate a new key, and the corresponding address be added to the new wallet, so that the same set of persons own the new wallet.
:::

## Zilliqa multisig

### Webportal

The [Zilliqa multisig wallet](https://multisig.zilliqa.com/) is the easiest way of interfacing using a multisig contract. It provides a user interface over the transitions this contract exposes.

### Contract

ZRC-4 has a reference [contract](https://raw.githubusercontent.com/Zilliqa/ZRC/4e92efd31c5d9a43db8bf5a7d25f7e2916d54c1d/reference/multisig_wallet.scilla) that can be examined.

The contract takes two immutable parameters. One for the List of ByStr20's used for signing, and the total number of required signatures needed. The contract then performs some validation to ensure the amount of signatures required is less than or equal to the required amount before continuing to setup the state variables. The state of the multisig is concerned with capturing data relating to signatures, this is used later to check if enough signatures have been added to submit a transaction.

The contract has logical stubs related to the operation of the multisig.

```ocaml
(* Add signature to signature list *)
procedure AddSignature (transactionId : Uint32, signee : ByStr20)
```

```ocaml
(* Submit a transaction for future signoff *)
transition SubmitTransaction (recipient : ByStr20, amount : Uint128, tag : String)

(* Sign off on an existing transaction *)
transition SignTransaction (transactionId : Uint32)

(* Delete transaction and signatures *)
procedure DeleteTransaction (transactionId : Uint32)

(* Execute signed-off transaction *)
transition ExecuteTransaction (transactionId : Uint32)

(* Revoke signature of existing transaction, if it has not yet been executed. *)
transition RevokeSignature (transactionId : Uint32)

(* Add funds to wallet *)
transition AddFunds ()
```

### Interaction

ZRC-4 has a reference [interaction](https://github.com/Zilliqa/ZRC/tree/4e92efd31c5d9a43db8bf5a7d25f7e2916d54c1d/example/zrc4) library that can be used if users wanted to deploy or use the multisig without interacting through the UI or IDE.

## Testing the Zilliqa multisig

### Setup

- Create three wallets (say A, B, C) using the [dev wallet generator](https://dev-wallet.zilliqa.com/generate), store the corresponding keystore files on your machine and note the BECH32(zil...) addresses. (you can import keystores into Zilpay at a later convenience)

- Request testnet Zil for these addresses [via the faucet](https://dev-wallet.zilliqa.com/faucet).

### Deployment via UI

- Login to the multisig wallet portal using the keystore for address A. Deploy a multisig wallet in a 2 out of 3 mode (i.e., funds from the multisig wallet can be transferred only when 2 out of 3 wallets have signed off).

- Use addresses corresponding to A, B and C (from Step 1) as the owner of the multisig wallet. Owners are the ones who can sign off a payment. Once deployed, save the multisig contract address for further testing.

- Once the multisig wallet is deployed, you will have to add funds to the contract. For this, go to the multisig wallet interface and click Add Funds button from the right-hand panel. Add funds to the multisig wallet using the funds from address A using the UI.

:::note
You cannot send funds to the contract address through Zilpay, it returns the error: `contract account wont accept normal transactions`

Note the transition for accepting payments in the contract transition stubs as an alternative.
:::

:::note
There is not a multisignature contract that currently accepts ZRC-2 tokens
:::

### Approve a transaction

- Let’s say you want to pay 5 Zil to account C via the multisig wallet. In order to do this, log in the multisig wallet using account A. And click Transaction from the right-hand panel. Fill in the recipient address and the amount and send the transaction.

- Since it is a multisig wallet, any transfer of money from the wallet will require 2 out of 3 sign offs. Since A submitted the transaction, it is counted as a sign off. We now need either B or C to sign off.

- Log in to the multisig wallet using account B, import the multisig wallet by providing the multisig wallet address (from Step 3.) Once the wallet is imported, you can click sign to sign of this transaction. Do check the transaction details.

- Once the two signatures are obtained, you can use account B to execute the transaction. Execution is the final step in the transfer process and it will transfer 5 Zil to C.

## Further reading

[Multisignature](https://research.csiro.au/blockchainpatterns/general-patterns/security-patterns/multiple-authorization/)

[ZRC-4 Specification](https://github.com/Zilliqa/ZRC/blob/4e92efd31c5d9a43db8bf5a7d25f7e2916d54c1d/zrcs/zrc-4.md)

[ZRC-4 Interaction](https://github.com/Zilliqa/ZRC/tree/4e92efd31c5d9a43db8bf5a7d25f7e2916d54c1d/example/zrc4)

[Zilliqa Multisignature Wallet UI](https://multisig.zilliqa.com/)
