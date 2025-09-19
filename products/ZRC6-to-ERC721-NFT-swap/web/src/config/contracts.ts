import { Address } from 'viem'

// Sample contract ABIs and addresses
// Replace these with your actual contract addresses and ABIs

export const ZRC6_CONTRACT_ADDRESS = "0x..." // Your ZRC6 contract address
export const ERC721_CONTRACT_ADDRESS = "0x..." // Your ERC721 contract address
export const SWAP_CONTRACT_ADDRESS = "0x..." // Your swap contract address

// Contract ABIs
export const SWAP_CONTRACT_ABI = [
  {
    "inputs": [
      {
        "internalType": "string",
        "name": "scillaAddress",
        "type": "string"
      },
      {
        "internalType": "uint256[]",
        "name": "scillaNftIdsToSwap",
        "type": "uint256[]"
      },
      {
        "internalType": "bytes",
        "name": "signature",
        "type": "bytes"
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

// Contract addresses - these should be updated with actual deployed addresses
export const CONTRACT_ADDRESSES = {
  33469: { // Zilliqa Testnet
    ZRC6: "0x...", // Replace with actual address
    ERC721: "0x...", // Replace with actual address
    SWAP: "0x0000000000000000000000000000000000000000" as Address, // Replace with actual address
  },
  32769: { // Zilliqa Mainnet
    ZRC6: "0x...", // Replace with actual address
    ERC721: "0x...", // Replace with actual address  
    SWAP: "0x0000000000000000000000000000000000000000" as Address, // Replace with actual address
  }
} as const
