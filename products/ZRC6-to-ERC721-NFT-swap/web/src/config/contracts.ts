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

// Contract addresses - these should be updated with actual deployed addresses
export const CONTRACT_ADDRESSES = {
  33101: { // Zilliqa Testnet
    ZRC6: "0x9796b1e3adfb73ca354fa5920521fb7ef21f71af", // Replace with your Scilla NFT collection address
    ERC721: "0x02C5908A23Edf9bA26969E00e3d246FF77cA0706.", // Replace with your EVM NFT collection address
    SWAP: "0x0000000000000000000000000000000000000000" as Address, // Replace with actual address
  },
  // 32769: { // Zilliqa Mainnet not yet deployed
  //   ZRC6: "0x...", // Replace with actual address
  //   ERC721: "0x...", // Replace with actual address  
  //   SWAP: "0x0000000000000000000000000000000000000000" as Address, // Replace with actual address
  // }
} as const
