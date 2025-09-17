# **ZRC6 to ERC712 NFT Swap** Project

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

### Testing environment

To test this page you can use already deployed NFTs contracts.

Scilla NFT: https://otterscan.testnet.zilliqa.com/address/0x9796b1e3adfb73ca354fa5920521fb7ef21f71af
EVM NFT: https://otterscan.testnet.zilliqa.com/address/0x02C5908A23Edf9bA26969E00e3d246FF77cA0706

You can mint yourself Scilla NFTs by going to https://ide.zilliqa.com, importing the contract, and calling `mint` function on it. Everybody can call `mint` on this NFT contract.