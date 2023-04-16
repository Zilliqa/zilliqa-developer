# Burn ZRC2

This contract aims to allow users to burn ZRC2 tokens:

- The owner can pause the contract for a particular ZRC2 or overall
- The owner can change the number of blocks before the burn is final.
- The owner can assign owership to a new owner.

It is recommended that each project deploy its own burn contract, to keep the contract state size small.

## Burning tokens

With `ZRC2Burn` as the burn contract, and `ZRC2TokenAddress` as any
conformant ZRC2 token, `WalletAddress` as the address that holds the
tokens to burn and `AllowanceAddress` as an address which calls the
ZRC-2 `TransferFrom` mechanism:

To burn ZRC2s:

- Call `ZRC2Burn.UpdateBurnAllowance( ZRC2TokenAddress, token_amount)` from the wallet that wishes to burn the tokens.
- Call `ZRC2TokenAddress.Transfer( ZRC2BurnAddress, token_amount)` to burn the tokens.
- If you change your mind, call `ZRC2Burn.CancelBurn( ZRC2BurnAddress )` within `burncancelblocks` to change your mind. Your tokens will be returned.
- Anyone can then call `ZRC2Burn.FinaliseBurn(token_address, wallet_address)` to finalise the burn.
- Once the tokens are permanently burned, not even the `ZRC2Burn` contract owner can get them back.

To burn ZRC2 tokens via an allowance:

- Call `ZRC2Burn.UpdateBurnAllowance( ZRC2TokenAddress, token_amount)` from the wallet that wishes to burn the tokens.
- Call `ZRC2TokenAddress.IncreaseAllowance( AllowanceAddress, token_amount)` from the wallet that wishes to burn the tokens.
- Have `AllowanceAddress` call `ZRC2TokenAddress.TransferFrom( WalletAddress, ZRC2Burn, token_amount)` to transfer the tokens.
- If `WalletAddress` changes their mind, they can call `ZRC2Burn.CancelBurn( ZRC2BurnAddress )`.
- Anyone can then call `ZRC2Burn.FinaliseBurn(token_address, wallet_address)` to finalise the burn.
- Once the tokens are permanently burned, not even the `ZRC2Burn` contract owner can get them back.

## Inspecting totals

The total number of tokens burned per ZRC2 is stored in `token_total_burned`.

## Safety features

To transfer ownership, the existing owner calls `SetContractOwnershipRecipient` and the recipient calls `AcceptContractOwnership`.

The owner can pause the contract for particular ZRC2 tokens with `Pause(ZRC2TokenAddress)` and `UnPause(ZRC2TokenAddress)`.

The owner can pause and unpause the contract globally with `PauseAll()` and `UnPauseAll()`.

It is not defined if the owner can interact with the contract whilst
it is paused. The owner can reassign ownership and call the pause and
unpause transitions whilst the contract is paused.

## Tests

To run the tests against the isolated server on port 5555

```sh
npm install
npx hardhat test
```

The tests are in `test/BurnTokensAnyZRC2.test.ts` ; edit the parameters at the top of the file to run tests against localdev, testnet or mainnet.
