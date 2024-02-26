# Burn ZRC2

The purpose of this contract is to allow users to permanently burn ZRC2 tokens.

- The owner can pause the contract for a particular ZRC2 or overall
- The owner can change the number of blocks before the burn is final.
- The owner can assign owership to a new owner.

It is recommended that each project deploy its own burn contract, to keep the contract state size small.

## Use cases

It is envisioned that this would mainly be used by ZRC2 maintainers to
permanently reduce the supply of ZRC-2 tokens.

## Burning tokens

With `ZRC2Burn` as the burn contract, and `ZRC2TokenAddress` as any
conformant ZRC2 token, `WalletAddress` as the address that holds the
tokens to burn and `AllowanceAddress` as an address which calls the
ZRC-2 `TransferFrom` mechanism:

To burn ZRC2s:

- Call `ZRC2Burn.UpdateBurnAllowance( ZRC2TokenAddress, token_amount)` from the wallet that wishes to burn the tokens.
- Call `ZRC2TokenAddress.Transfer( ZRC2BurnAddress, token_amount)` to burn the tokens. This will start a cancel timer for `burncancelblocks` blocks.
- If you change your mind, call `ZRC2Burn.CancelBurn( ZRC2BurnAddress )` before the timer expires (ie. within `burncancelblocks`) to change your mind. Your tokens will be returned.
- If `Transfer()` or `TransferFrom()` is called again, a new timer will be started for all tokens waiting to be burned.
- Otherwise the burn becomes irrevocable.,
- Anyone can then call `ZRC2Burn.FinaliseBurn(token_address, wallet_address)` to finalise the burn.
- Once the tokens are permanently burned, not even the `ZRC2Burn` contract owner can get them back.

To burn ZRC2 tokens via an allowance:

- Call `ZRC2Burn.UpdateBurnAllowance( ZRC2TokenAddress, token_amount)` from the wallet that wishes to burn the tokens.
- Call `ZRC2TokenAddress.IncreaseAllowance( AllowanceAddress, token_amount)` from the wallet that wishes to burn the tokens.
- Have `AllowanceAddress` call `ZRC2TokenAddress.TransferFrom( WalletAddress, ZRC2Burn, token_amount)` to transfer the tokens. This will start a cancel timer for `burncancelblocks` blocks.
- If `WalletAddress` changes their mind before the timer expires (within `burncancelblocks`), they can call `ZRC2Burn.CancelBurn( ZRC2BurnAddress )` to cancel.
- If `Transfer()` or `TransferFrom` is called again, a new timer will be started for all tokens waiting to be burned.
- Otherwise the burn becomes irrevocable.
- Anyone can then call `ZRC2Burn.FinaliseBurn(token_address, wallet_address)` to finalise the burn - this just updates internal data structures.
- Once the tokens are permanently burned, not even the `ZRC2Burn` contract owner can get them back.

`burncancelblocks` is sampled when the timer is started or restarted.

## Inspecting totals

The total number of tokens burned per ZRC2 is stored in `token_total_burned`.

## Safety features

To transfer ownership, the existing owner calls
`SetContractOwnershipRecipient` and the recipient calls
`AcceptContractOwnership`.

The owner can pause the contract for particular ZRC2 tokens with
`Pause(ZRC2TokenAddress)` and `UnPause(ZRC2TokenAddress)`.

The owner can pause and unpause the contract globally with
`PauseAll()` and `UnPauseAll()`.

There is a quirk here: `CancelBurn` cannot be called if the contract
is globally paused, but can be called if particular ZRC2s are paused
(even on the paused ZRC2s). This is to allow the owner to pause new
burns whilst facilitating the cancellation of burns in progress, which
would otherwise become irrevocable.

It is not defined if the owner can interact with the
contract whilst it is paused. The owner can reassign ownership and
call the pause and unpause transitions whilst the contract is paused.

## Quirks

- When the contract is paused on only a particular ZRC2, you can cancel your burn, but not finalise it.
  This is odd, but arises from the logic that we would really like you not to be able to cancel it either,
  but we have to in case the timer expires on you. Thus, we protect the logic behind finalise with the
  pause flag, but not that behind cancel. This provides us with better error coverage at the cost of
  some more than slightly odd semantics.

## Tests and deployment instructions

### Tests

To run the tests against the isolated server on port 5555

```sh
npm install
npx hardhat test
```

The tests are in `test/BurnTokensAnyZRC2.test.ts` ; edit the parameters at the top of the file to run tests against localdev, testnet or mainnet.

### Deployment

Deploy `contracts/BurnTokensAnyZRC2.scilla` .

Use your usual way of deploying to the chain, depending on your custody requirements. This might include:

- Use the test script and provide a key via environment variables or in some other way.
- Deploy using a wallet via [Neosavant](https://ide.zilliqa.com/#/)
- Using [zli](https://github.com/Zilliqa/zli) .
