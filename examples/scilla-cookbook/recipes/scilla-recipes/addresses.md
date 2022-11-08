---
tags:
  - Address
  - ByStr20
  - Bech32
  - Base16
---

# Addresses

Addresses on the network can either be a wallet or a contract. Addresses come in two format Bech32 or Base16. Bech32's start with the prefix ```zil1...``` whereas Base16's are prefixed with ```0x...```.

Converting a Bech32 into a Base16 and vice versa can be achieved manually though our [Neo-Savant IDE](https://ide.zilliqa.com/)`(Tools>AddressConverter)` or programmatically using programs like [zilliqa-js](https://github.com/Zilliqa/Zilliqa-JavaScript-Library). When we refer to a ByStr20 address in Scilla, we refer to the Base16 address.

```text
Bech32 = zil1zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3h6785s
Base16 = 0x1111111111111111111111111111111111111111
```

```ocaml
scilla_version 0

library Example

let constant_bystr = 0x1111111111111111111111111111111111111111

contract Example
(
    immutable_bystr: ByStr20
)

field addr_bystr : ByStr20 = constant_bystr
```

## Address subtypes

We can infer subtypes of Address such as 'A contract on the network' or 'A user or contract on the network' to give some type safety when trying to use contracts or addresses that might not exist

```ocaml
  (* any valid bytes of string length 20, not guaranteed to be valid *)
  ByStr20
  (* guarantees to be either a contract or a user with some balance or nonce >= 1*)
  ByStr20 with end
  (* guarantees to be a contract *)
  ByStr20 with contract end
```

Read more on [Address subtyping here.](https://scilla.readthedocs.io/en/latest/scilla-in-depth.html?highlight=with%20end#address-subtyping)

## Further reading

[Scilla Documentation - Addresses](https://scilla.readthedocs.io/en/latest/scilla-in-depth.html?highlight=bystr20#addresses)

[auction.scilla](https://github.com/Zilliqa/scilla/blob/master/tests/contracts/auction.scilla)

[unstoppabledomains_registry.scilla](https://github.com/Zilliqa/zli/blob/c35fbac35edb5c6987b8a5881490a7cacb4cb1be/testsuite/contracts/UnstoppableDomains/Registry.scilla)
