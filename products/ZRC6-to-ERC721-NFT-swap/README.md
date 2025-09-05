# ZRC6 to ERC721 NFT Swap

## Intro

This project is a generic template that enables users to swap Scilla NFTs for EVM NFTs.

## Setup requirements

1. Address of Scilla NFTs collection to be burned
2. Address of EVM NFTs collection to be swapped for

## Structure

### Web

Contains the NextJS web application. The application flow is as follows:
1. User connects ZilPay
2. Application checks what Scilla NFTs user has
3. Application displays a list of user owned Scilla collection NFTs
4. User selects the Scilla collection NFTs to be swapped for EVM collection NFTs
4. User connect EVM wallet
5. User approves the swap contract as a spender of Scilla NFT using ZilPay
6. User calls `burnAndReceive` using EVM wallet

### Contracts

Contains solidity smart contract deployed with Forge.

#### Swap Contract

This contract handles one specific pair of Scilla NFTs collection and its corresponding EVM NFTs collection.
Addresses of those collections are specified through constructor.
The contract is using Zilliqa interop to call the Scilla NFTs collection contract. 
This contract allows user to call the `burnAndReceive` method using EVM wallet. This method takes the owner' ZilPay wallet address (the current address that owns Scilla NFTs), signature, and a `list of NFT ids to be burned and swapped`. The signature data is a signed owner' ZilPay wallet address signed with ZilPay. The signature is provided as a proof that EVM wallet that calls the contract also owns the the ZilPay address.
Successful `burnAndReceive` call:
1. makes all the NFTs on Scilla NFTs collection among `list of Scilla NFT ids to be burned and swapped` owned by zero address
2. makes all the NFTs on EVM NFTs collection among `list of NFT ids to be burned and swapped` owned by the EVM wallet address of the caller