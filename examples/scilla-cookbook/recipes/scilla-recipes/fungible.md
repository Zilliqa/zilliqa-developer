---
tags:
  - fungible
  - token
  - reference
  - contract
  - zrc-2
---

# Fungible token

Fungible tokens are an open standard for creating currencies. Fungibility is the property of goods or commodities whose individual units are essentially interchangeable, and each of its parts is indistinguishable from another part. An example is 1 ZIL can be traded for an equal value of 1 ZIL.

ZRC-2 defines a minimum interface that a smart contract must implement to allow fungible tokens to be managed, tracked, owned, and traded peer-to-peer via wallets or exchanges.

## How does it work?

The fungible contract has a state map called ```balances``` where it maps addresses (ByStr20) to a value of tokens represented by a Uint128. The amount of tokens that a user can be influenced by logic the fungible contract has. When a user wants to use these transactions in a sale, the contract spending the amount will check the fungible balance state to check if you have funds available to deduct.

The fungible contract gives a token holder the ability to set an ```allowance``` for any address. This allowance address will have a spend limit it is allowed to spend on behalf of the token holder.

:::note
You cannot arbitrarily move fungible tokens without you being either the owner or by giving an address some allowance.
:::

The fungible contract comes in different flavours depending on what the user needs to leverage. The contracts have the following public transitions :

## Types of fungible contracts

### Non-mintable

:::tip
The 'non-mintable' fungible contract defines the most basic interface a ZRC-2 token can have. It is considered the safest fungible contract as there is no ```Mint``` transition to be abused.
:::

:::info
The fungible contracts have been audited by PwC
:::

```ocaml
(* @dev: Increase the allowance of an approved_spender over the caller tokens. Only token_owner allowed to invoke.   *)
(* param spender:      Address of the designated approved_spender.                                                   *)
(* param amount:       Number of tokens to be increased as allowance for the approved_spender.                       *)
transition IncreaseAllowance(spender: ByStr20, amount: Uint128)

(* @dev: Decrease the allowance of an approved_spender over the caller tokens. Only token_owner allowed to invoke. *)
(* param spender:      Address of the designated approved_spender.                                                 *)
(* param amount:       Number of tokens to be decreased as allowance for the approved_spender.                     *)
transition DecreaseAllowance(spender: ByStr20, amount: Uint128)

(* @dev: Moves an amount tokens from _sender to the recipient. Used by token_owner. *)
(* @dev: Balance of recipient will increase. Balance of _sender will decrease.      *)
(* @param to:  Address of the recipient whose balance is increased.                 *)
(* @param amount:     Amount of tokens to be sent.                                  *)
transition Transfer(to: ByStr20, amount: Uint128)

(* @dev: Move a given amount of tokens from one address to another using the allowance mechanism. The caller must be an approved_spender. *)
(* @dev: Balance of recipient will increase. Balance of token_owner will decrease.                                                        *)
(* @param from:    Address of the token_owner whose balance is decreased.                                                                 *)
(* @param to:      Address of the recipient whose balance is increased.                                                                   *)
(* @param amount:  Amount of tokens to be transferred.                                                                                    *)
transition TransferFrom(from: ByStr20, to: ByStr20, amount: Uint128)
```

### Mintable

:::warning
The 'mintable' fungible contract should be viewed sceptically as the ```contract_owner``` can arbitrarily ```Mint``` new tokens.
:::

```ocaml
...
(* @dev: Mint new tokens. Only contract_owner can mint.                      *)
(* @param recipient: Address of the recipient whose balance is to increase.  *)
(* @param amount:    Number of tokens to be minted.                          *)
transition Mint(recipient: ByStr20, amount: Uint128)

(* @dev: Burn existing tokens. Only contract_owner can burn.                      *)
(* @param burn_account: Address of the token_owner whose balance is to decrease.  *)
(* @param amount:       Number of tokens to be burned.                            *)
transition Burn(burn_account: ByStr20, amount: Uint128)
```

### Operatable

:::warning
The 'operatable' fungible contract should be viewed sceptically as an ```operator``` can arbitrarily move tokens.
:::

```ocaml
(* @dev: Moves amount tokens from token_owner to recipient. _sender must be an operator of token_owner. *)
(* @dev: Balance of recipient will increase. Balance of token_owner will decrease.                      *)
(* @param from:        Address of the token_owner whose balance is decreased.                           *)
(* @param to:          Address of the recipient whose balance is increased.                             *)
(* @param amount:      Amount of tokens to be sent.                                                     *)
transition OperatorSend(from: ByStr20, to: ByStr20, amount: Uint128)
```

### Transferring Fungible Tokens

```ocaml
procedure TransferFundsFromXToY(x: ByStr20, fungible_contract: ByStr20, y: ByStr20, funds: Uint128)
    send_funds = {
      _tag: "TransferFrom";
      _recipient: fungible_contract;
      _amount: uint128_zero;
      from: x;
      to: y;
      amount: funds
      };
    msgs = one_msg send_funds;
    send msgs;
end
```

## Decimal Example

Lets say we make a token with 2 decimals with a total supply of 1000. There are 100 sub-units (max of two decimals) to 1 full token. The total sub-unit supply of would be 1000*100=100000.

| Amount to send | Fungible value |
|----------------|----------------|
| 0.01           | 1              |
| 0.1            | 10             |
| 1              | 100            |
| 10             | 1000           |

Lets take an example where we have 12 decimals with a supply of 1000. there are 1000000000000 sub-units (max of 12 decimals) to 1 full token. the total sub-unit supply would be 1000*1000000000000=100000000000000

| Amount to send | Fungible value |
|----------------|----------------|
| 0.000000000001 | 1              |
| 0.000000001    | 1000           |
| 0.000001       | 1000000        |
| 1              | 1000000000000  |

## Further reading

[Scilla Documentation - Fungible Tokens](https://scilla.readthedocs.io/en/latest/scilla-by-example.html?highlight=fungible#fungible-tokens)
