# Burn ZRC2

This contract aims to allow users to burn ZRC2 tokens:

- The owner can pause the contract for a particular ZRC2 or overall
- The owner can change the number of blocks before the burn is final.
- The owner can assign owership to a new owner.

It is recommended that each project deploy its own burn contract, to keep the contract state size small.

## Burning tokens

To burn ZRC2s:

- Call `ZRC2Burn.UpdateBurnAllowance( ZRC2TokenAddress, token_amount)` from the wallet that wishes to burn the tokens.
- Call `ZRC2TokenAddress.Transfer( ZRC2BurnAddress, token_amount)` to burn the tokens.
- If you change your mind, call `ZRCBurn.CancelBurn( ZRC2BurnAddress )` within `burncancelblocks` to change your mind. Your tokens will be returned.
- Otherwise your tokens will be permanently burned - not even the `ZRC2Burn` contract owner can get them back.

You can also use the ZRC2 allowance mechanism to burn tokens directly from a contract, should you wish to do so.

## Inspecting totals

The total number of tokens burned per ZRC2 is stored in `token_total_burned`.

## Tests

There will be tests for this contract in due course.
