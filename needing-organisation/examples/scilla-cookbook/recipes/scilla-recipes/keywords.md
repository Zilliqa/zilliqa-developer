---
tags:
  - keywords
---

# Glossary of keywords & conventions

This page is responsible for some of the most common development and blockchain terms and the conventions to follow onchain or with interactions.

## Glossary

`nonce`

An incrementing integer counter, safeguards from transactions being sent twice. The term stands for "number used once".

`gas price`

The amount paid for a unit of gas.

`gas limit`

The amount of gas should be expended before erroring.

`signature`

Cryptographic hash of some data generated using a keypair.

`QA`

1 Unit of QA is worth 0.000001 LI and 0.000000000001 ZIL.

QA is used when speaking about small values. Default Scilla unit that can be represented as a Uint128.

`LI`

1 unit of LI is worth 1000000 QA and 0.000001 ZIL.

LI is rarely used as a unit.

`ZIL`

1 unit of ZIL is worth 1000000 LI and 1000000000000 QA.

ZIL is used when speaking about large values.

## Keywords

`accept`

Accepts an amount of ZIL into the contract.

`_sender`

The address calling the contract.

`_origin`

The original address that started the chain of calling.

`_this_address`

Starts a pattern matching block

`in`

Used to chain a logical structure through multiple variables.

## Conventions

The standard convention to writing procedure names is in `PascalCase`.

The standard convention to writing constants and variables is in `snake_case`.

## Conventional transition names

### ZRC-5 - convention for depositing zil

For a smart contract to accept incoming ZIL, it needs to do so explicitly using the `accept` statement. As such, any transition that does not have the accept statement will not be able to accept any incoming ZIL transfer.

However, there is currently no naming convention for transitions that can accept ZIL. As a result, cryptocurrency exchanges or cryptocurrency wallet providers do not know which `_tag` to set, should they wish to transfer ZIL to a contract address.

By having a naming convention for transitions that can accept ZIL, one can easily transfer ZIL to a contract that follows this convention, thereby improving composability between multiple contracts.

```ocaml
(* Accepts funds *)
transition AddFunds ()
  accept
end
```

## Further reading
