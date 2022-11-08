---
tags:
  - BNum
  - block
  - number
  - date
  - time
---

# BNum

Block Numbers (BNum) are a type used for tracking the current block number. Blockchain do not have any concept of dates or times without external oracles. Blocks can be used as a primitive way to determine how much time has passed.

```ocaml
scilla_version 0
library BlockNumbers

let constant_bnum = BNum 1337
let fifty_uint128 = Uint128 50
let zero_block = BNum 0

contract BlockNumbers
(
    immutable_bnum: BNum
)

field state_block : BNum = some_block_number


transition BNumMaths()
    block_tx_is_processed_at <- & BLOCKNUMBER;

    cast_uint_to_block = builtin badd zero_block zero_uint128 (* BNum(0) + UintX(N) = BNum(N) *)

    fifty_block_in_future = builtin badd block_tx_is_processed_at fifty_uint128; (* BNum + UintX = BNum *)

    fifty_block_in_past = builtin bsub block_tx_is_processed_at fifty_uint128; (* BNum - BNum = Uint256 *)

    are_blocks_equal = builtin eq  block_tx_is_processed_at cast_uint_to_block; (* BNum == BNum = Bool *)

    is_a_less_than_b = builtin blt block_tx_is_processed_at fifty_uint128 (* BNum < BNum = Bool *)
end
```

## Further reading

[Scilla documentation - BNum](https://scilla.readthedocs.io/en/latest/scilla-in-depth.html?highlight=block#block-numbers)

[auction.scilla](https://github.com/Zilliqa/scilla/blob/master/tests/contracts/auction.scilla)
