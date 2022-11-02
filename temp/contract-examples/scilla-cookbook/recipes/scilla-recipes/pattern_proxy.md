---
tags:
  - proxy
  - pattern
---

# Pattern (proxy)

Proxy patterns allows developers to adapt to a changing environment and to react to bugs and other errors. To overcome the limitations introduced by the immutability of contract code, a contract can be split up into modules, which are then virtually upgradeable.

They are only virtually upgradeable, because existing contracts still cannot be changed. However, a new version of the contract can be deployed and its address replaces the old one in storage. To avoid breaking dependencies of other contracts that are referencing the upgraded contract, or users who do not know about the release of a new contract version (that comes with a new address), we make use of a proxy contract that delegates calls to the specific modules

The proxy pattern is easily understood with an example of minting NFTs. An NFT contract requires an 'Minter' to handle the responsibility of being the trusted entity to Mint. In a dapp, if the caller tried to call Mint, they would be returned a 'NotMinter' error. We can get around this by creating a new contract which will be the 'Minter' and will contain logic about what conditions to mint an NFT.

![Docusaurus](/img/recipes/patterns/proxy-diagram.png)

```ocaml
(* configured with 'NFT_Proxy' as a minter before calls are made *)
contract NonfungibleToken()

(* internal proc to mint the token *)
procedure MintToken(to: ByStr20, token_uri: String)
  ...
end

(* (2) NFT checks the sender of the call, checking if they are an approved minter *)
transition Mint(to: ByStr20, token_uri: String)
  IsMinter _sender;
  MintToken to;
  ...
end
```

```ocaml
(* allows anyone to call ProxyBuy, when sending to NonfungibleToken the _sender 'NFT_Proxy' will be a minter *)
contract NFT_Proxy()

(* (1a) internal proc to ask the NFT contract to mint a token *)
procedure ProxyCallMint(token_uri: String)
  IsMinter _sender;
    e = {
    _tag: "MintToken";
    _recipient: NonfungibleToken;
    _amount: Uint128 0;
    to: _sender;
    token_uri: token_uri
  };
  msgs = one_msg msg_to_sender;
  send msgs
  ...
end

(* (1) user calls this function on the proxy *)
transition ProxyBuy()
  ProxyCallMint;
  ...
end
```

Consider if we wanted to deploy some new proxy logic to effect our NFT minting, we could easily deploy a new contract and upgrade the NonFungibleToken delegated Minter to point at our new proxy. We have managed to upgrade some immutable logic by using the proxy pattern.

## Further reading
