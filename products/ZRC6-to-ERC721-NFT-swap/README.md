# ZRC6 to ERC721 NFT Swap

## Intro

This project is a generic template that enables users to swap Scilla NFTs for EVM NFTs.

## Setup requirements

1. Address of Scilla NFTs collection to be burned
2. Address of EVM NFTs collection to be swapped for

## Structure

### Web

Contains the implementation of the web application that user interacts with to swap NFTs.

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

### Testing environment

To test this page you can use already deployed NFTs contracts.

Scilla NFT: https://otterscan.testnet.zilliqa.com/address/0x9796b1e3adfb73ca354fa5920521fb7ef21f71af
EVM NFT: https://otterscan.testnet.zilliqa.com/address/0x02C5908A23Edf9bA26969E00e3d246FF77cA0706

You can mint yourself Scilla NFTs by going to https://ide.zilliqa.com, importing the contract, and calling `mint` function on it. Everybody can call `mint` on this NFT contract.