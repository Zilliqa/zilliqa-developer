# **ZRC6 to ERC712 NFT Swap** Contracts

## Contracts

## burnScillaAndMintEVMNFTSwap.sol

This contract enables to do a burn of ZRC6 NFT (through Zilliqa interop), and mint of a new `erc721` NFT in one transaction.
This contract handles a single one specific pair of Scilla NFTs collection and its corresponding EVM NFTs collection. Addresses of those collections are specified through constructor.
The contract is using Zilliqa interop to call the Scilla NFTs collection contract. 
This contract allows user to call the `swapZRC6NFTForErc721NFTByByrningZRC6` method using EVM wallet. This method takes the owner' ZilPay wallet address (the current address that owns Scilla NFTs), signature, and a `list of NFT ids to be burned and swapped`. The signature data is a signed owner' ZilPay wallet address signed with ZilPay. The signature is provided as a proof that EVM wallet that calls the contract also owns the the ZilPay address.
Successful `swapZRC6NFTForErc721NFTByByrningZRC6` call:
1. makes all the NFTs on Scilla NFTs collection among `list of Scilla NFT ids to be burned and swapped` owned by zero address
2. makes all the NFTs on EVM NFTs collection among `list of NFT ids to be burned and swapped` owned by the EVM wallet address of the caller.

## Playgrond contracts

Playground contracts directory contains pre deployed testnet contracts that can be used to run the web application and test it out.

### notScarceZRC6.scilla

TODO add some description for `notScarceZRC6.scilla`that can be minted by anyone.

### erc721.sol

TODO add some description for `erc721.sol` that can mint tokens on the fly by addresses set as allowed minters. One of the addresses is the `burnScillaAndMintEVMNFTSwap.sol` contract  

## Foundry usage

**Foundry is a blazing fast, portable and modular toolkit for Ethereum application development written in Rust.**

Foundry consists of:

-   **Forge**: Ethereum testing framework (like Truffle, Hardhat and DappTools).
-   **Cast**: Swiss army knife for interacting with EVM smart contracts, sending transactions and getting chain data.
-   **Anvil**: Local Ethereum node, akin to Ganache, Hardhat Network.
-   **Chisel**: Fast, utilitarian, and verbose solidity REPL.

### Documentation

https://book.getfoundry.sh/

### Build

```shell
$ forge build
```

### Test

```shell
$ forge test
```

### Format

```shell
$ forge fmt
```

### Gas Snapshots

```shell
$ forge snapshot
```

### Anvil

```shell
$ anvil
```

### Deploy

```shell
$ forge script script/Counter.s.sol:CounterScript --rpc-url <your_rpc_url> --private-key <your_private_key>
```

### Cast

```shell
$ cast <subcommand>
```

### Help

```shell
$ forge --help
$ anvil --help
$ cast --help
```
