# **ZRC6 to ERC721 NFT Swap** Project

## Intro

This project is a generic template that enables users to swap Scilla NFTs (ZRC6) for EVM NFTs (ERC721) across Zilliqa networks. The web application provides a complete user interface for the swap process, including wallet connections, token selection, and transaction execution.

## Setup requirements

1. Address of Scilla NFTs collection to be burned
2. Address of EVM NFTs collection to be swapped for
3. Deployed swap contract address

## Structure

### Web

Contains the implementation of the Next.js web application that users interact with to swap NFTs. The application features:

- **Multi-wallet support**: ZilPay for Zilliqa non-EVM network, EVM wallets for Zilliqa EVM network
- **Token selection interface**: Users can select specific ZRC6 token IDs to swap
- **Signature creation**: Automatic signing of EVM wallet address using ZilPay
- **Contract interaction**: Direct interaction with the swap smart contract
- **Transaction status tracking**: Real-time updates on swap progress

### Contracts

Contains solidity smart contract deployed with Forge that handles the actual NFT swap logic.

### Testing environment

To test this page you can use already deployed NFTs contracts.

Scilla NFT: https://otterscan.testnet.zilliqa.com/address/0x9796b1e3adfb73ca354fa5920521fb7ef21f71af
EVM NFT: https://otterscan.testnet.zilliqa.com/address/0x02C5908A23Edf9bA26969E00e3d246FF77cA0706

You can mint yourself Scilla NFTs by going to https://ide.zilliqa.com, importing the contract, and calling `mint` function on it. Everybody can call `mint` on this NFT contract.

## Swap Process

1. **Connect Wallets**: User connects both ZilPay (for ZRC6) and EVM wallet (for ERC721)
2. **Select Tokens**: User chooses which ZRC6 token IDs to swap
3. **Create Signature**: Application signs the EVM wallet address using ZilPay
4. **Execute Swap**: Smart contract burns ZRC6 tokens and mints ERC721 tokens
5. **Confirmation**: User receives transaction confirmation
