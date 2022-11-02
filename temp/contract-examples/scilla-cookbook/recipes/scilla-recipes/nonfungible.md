---
tags:
  - non
  - fungible
  - token
  - reference
  - contract
  - zrc-1
---

# Non-fungible token

A [fungible token](fungible) is unit of currency that can be readily interchanged, it has properties like fiat where 1 unit of currency is equal to 1 unit of the same currency.

Non-fungible tokens have associated with it some unique properties like some particular attributes, image, asset and therefore makes each piece unique. The tokens can be thought of as certificates of ownership for virtual or physical assets. A Non-Fungible contract is a collection of associations between token ids, there associated asset and an owning address.

[ZRC-6](https://github.com/Zilliqa/ZRC/blob/master/zrcs/zrc-6.md) defines a minimum interface of an NFT smart contract. It has features including royalty payments and batch minting.

[ZRC-7](https://github.com/Zilliqa/ZRC/blob/master/zrcs/zrc-7.md) defines a metadata standard that developers should follow to ensure ecosystem partners can universally consume. The resource attached to a ZRC-1 / ZRC-6 should emit a JSON response which contains not only the image, but the traits and other relevant project and token information.

### ZRC-6

The ZRC-6 contract can be found [here](https://github.com/Zilliqa/ZRC/blob/main/reference-contracts/zrc6.scilla).

The ZRC-6 specification can be found [here](https://github.com/Zilliqa/ZRC/blob/master/zrcs/zrc-6.md).

#### ZRC-6 interface

```ocaml
procedure TransferToken(to: ByStr20, token_id: Uint256)
transition Pause()
transition Unpause()
transition SetRoyaltyRecipient(to: ByStr20)
transition SetRoyaltyFeeBPS(fee_bps: Uint128)
transition SetBaseURI(uri: String)
transition Mint(to: ByStr20)
transition BatchMint(to_list: List ByStr20)
transition Burn(token_id: Uint256)
transition BatchBurn(token_id_list: List Uint256)
transition AddMinter(minter: ByStr20)
transition RemoveMinter(minter: ByStr20)
transition SetSpender(spender: ByStr20, token_id: Uint256)
transition AddOperator(operator: ByStr20)
transition RemoveOperator(operator: ByStr20)
transition TransferFrom(to: ByStr20, token_id: Uint256)
transition BatchTransferFrom(to_token_id_pair_list: List (Pair ByStr20 Uint256))
transition SetContractOwnershipRecipient(to: ByStr20)
transition AcceptContractOwnership()
```

## Transferring Nonfungible Tokens

```ocaml
type TokenMove = | UserToContract | ContractToUser

procedure MoveNonFungibleTokenFromContractToUser(token_move: TokenMove, recipient_address: ByStr20, nonfungible: ByStr20, token_id: Uint256)
  match token_move with
    | ContractToUser =>
      transfer_to_user = {
        _tag: "TransferFrom";
        _recipient: nonfungible;
        _amount: uint128_zero;
        to: recipient_address;
        token_id: token_id
        };
      msgs = one_msg transfer_to_user;
      send msgs;
      e = {_eventname : "StoreMovedToUserSuccess"; nonfungible : nonfungible; token_id: token_id};
      event e

    | UserToContract =>
    end
end
```

## Further reading

[ZRC-1 specification](https://github.com/Zilliqa/ZRC/blob/master/zrcs/zrc-1.md)

[ZRC-6 specification](https://github.com/Zilliqa/ZRC/blob/master/zrcs/zrc-6.md)
