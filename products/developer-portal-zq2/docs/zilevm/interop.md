---
id: interop
title: Zilliqa EVM/native interoperability
---

# Zilliqa EVM characteristics and interop

This page details the quirks you might see in Zilliqa as an EVM user, and describes how interoperability between EVM and the native Zilliqa layer is achieved.

## Address formats

Zilliqa addresses are derived from the SHA256 of your public key,
giving a hex string, `0x70b16b656fc1759193366dab9a56bee486feffda`,
which is then conventionally expressed in bech32 form - see
[ZIP-1](https://github.com/Zilliqa/ZIP/blob/master/zips/zip-1.md) -
`zil1wzckket0c96eryekdk4e5447ujr0al76fd6ax4`.

Ethereum addresses are derived from the Keccak256 of your public key,
giving a hex string, possibly with a checksum embedded in the
capitalisation - `0xB85fF091342e2e7a7461238796d5224fA81ca556`.

Though Zilliqa and Ethereum both use secp256k1, Zilliqa native uses
Schnorr signatures, whilst Ethereum uses ECDSA.

Sadly, this means that:

- It isn't possible to derive the EVM address of a wallet from its Zilliqa address, or vice versa.
- It isn't possible to derive a key which will sign EVM transactions on a Zilliqa address, or vice versa.

This means that you need to be very careful not to send tokens (or anything else!) to a Zilliqa address in an EVM contract or vice versa.

Sometimes you can escape the issue by using interop to call down from
EVM to Scilla (or, in Zilliqa 2, Scilla to EVM), but sometimes this
won't be possible and your assets will be gone.

We are exploring the possibility of allowing cross-signing of
transactions so as to escape this trap - this may be possible with
account abstraction.

## Gas

### Charges

We use the London fork configuration of gas units.

However, there is a small modification - a ZIL API native token transfer costs 50 gas; an EVM transfer costs 21000 gas. We'd like these to be the same, so we divide Eth gas costs by 420 (== 21000/50).

### Rounding

Because ETH and ZIL differ in the number of decimals they support, we
scale EVM balances by 6 decimal places.

If we maintained a 1-1 relationship between ethereum gas and ZIL gas ,
EVM would be dramatically more expensive than Scilla; in order to
ameliorate this, we scale ethereum gas when charging and for technical
reasons, the scaled value (ie. the scilla-equivalent gas price is what
we track under the hood).

As a result, rounding occurs, and the amount of ethereum gas you are
charged for a transaction will be slightly less than the amount
estimated, in order that the `effectiveGasPrice * gasUsed` is equal to
the deduction made for gas.

## Scilla/EVM interoperability

It is possible to call Scilla from EVM (though not to pass native
tokens between the two) via
[ZIP-21](https://github.com/Zilliqa/ZIP/blob/master/zips/zip-21.md).

## Opcode differences

There are a few opcodes which will give you different results on Zilliqa than on Ethereum:

| OP code    | Description                                                                                                                                                                                                            |
| ---------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| COINBASE   | Returns 0 (this opcode returns the address that gets the current block reward; since the reward is split among multiple participating parties in Zilliqa, it's not possible to implement this opcode, and we return 0) |
| CHAINID    | Returns 0x8000 + Zilliqa ChainID. Existing Zilliqa chain ids are incompatible with Ethereum ids (where 1 means mainnet), so we shift our chain id space up by 0x8000.                                                  |
| BASEFEE    | Returns the current ZIL gas price of 0.02 ZIL                                                                                                                                                                          |
| DIFFICULTY | Return current difficulty                                                                                                                                                                                              |

## EVM precompiles

We support a number of new precompiles. Documentation TBD.
