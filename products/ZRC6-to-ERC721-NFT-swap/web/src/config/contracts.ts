import { Address } from 'viem'

// Sample contract ABIs and addresses
// Replace these with your actual contract addresses and ABIs

export const ERC721_CONTRACT_ADDRESS = "0x..." // Your ERC721 contract address
export const SWAP_CONTRACT_ADDRESS = "0x..." // Your swap contract address

// Contract ABIs
export const SWAP_CONTRACT_ABI = [
  {
    "inputs": [
      {
        "internalType": "uint256[]",
        "name": "scillaNftIdsToSwap",
        "type": "uint256[]"
      }
    ],
    "name": "swapZRC6NFTForErc721NFTByByrningZRC6",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [],
    "name": "scillaNFTAddress",
    "outputs": [
      {
        "internalType": "address",
        "name": "",
        "type": "address"
      }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [],
    "name": "evmNFTAddress",
    "outputs": [
      {
        "internalType": "address",
        "name": "",
        "type": "address"
      }
    ],
    "stateMutability": "view",
    "type": "function"
  }
] as const

export const CONTRACT_ADDRESSES = {
  33101: { // Zilliqa Testnet
    ZRC6: "0x9796b1e3adfb73ca354fa5920521fb7ef21f71af" as Address, // Replace with your Scilla NFT collection address
    ERC721: "0xC768f120Ea2e1EDB9805942cF569a4eB2D4eb5fE" as Address, // Replace with your EVM NFT collection address
    SWAP: "0x8ac210608DDA334D44b35FFf9C5511AA01b9E7Dc" as Address, // Replace with actual address
  },
  // 32769: { // Zilliqa Mainnet not yet deployed
  //   ZRC6: "0x...", // Replace with actual address
  //   ERC721: "0x...", // Replace with actual address  
  //   SWAP: "0x" as Address, // Replace with actual address
  // }
} as const
