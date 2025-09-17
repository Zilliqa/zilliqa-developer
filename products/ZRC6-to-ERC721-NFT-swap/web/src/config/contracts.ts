// Sample contract ABIs and addresses
// Replace these with your actual contract addresses and ABIs

export const ZRC6_CONTRACT_ADDRESS = "0x..." // Your ZRC6 contract address
export const ERC721_CONTRACT_ADDRESS = "0x..." // Your ERC721 contract address
export const SWAP_CONTRACT_ADDRESS = "0x..." // Your swap contract address

// Basic ERC20/ZRC6 ABI for token operations
export const ZRC6_ABI = [
  {
    inputs: [
      { name: "_spender", type: "address" },
      { name: "_amount", type: "uint256" }
    ],
    name: "approve",
    outputs: [{ name: "", type: "bool" }],
    stateMutability: "nonpayable",
    type: "function"
  },
  {
    inputs: [
      { name: "_to", type: "address" },
      { name: "_amount", type: "uint256" }
    ],
    name: "transfer",
    outputs: [{ name: "", type: "bool" }],
    stateMutability: "nonpayable",
    type: "function"
  },
  {
    inputs: [{ name: "_owner", type: "address" }],
    name: "balanceOf",
    outputs: [{ name: "", type: "uint256" }],
    stateMutability: "view",
    type: "function"
  }
] as const

// Basic ERC721 ABI
export const ERC721_ABI = [
  {
    inputs: [
      { name: "_to", type: "address" },
      { name: "_tokenId", type: "uint256" }
    ],
    name: "mint",
    outputs: [],
    stateMutability: "nonpayable",
    type: "function"
  },
  {
    inputs: [
      { name: "_from", type: "address" },
      { name: "_to", type: "address" },
      { name: "_tokenId", type: "uint256" }
    ],
    name: "transferFrom",
    outputs: [],
    stateMutability: "nonpayable",
    type: "function"
  },
  {
    inputs: [{ name: "_tokenId", type: "uint256" }],
    name: "ownerOf",
    outputs: [{ name: "", type: "address" }],
    stateMutability: "view",
    type: "function"
  }
] as const

// Sample swap contract ABI
export const SWAP_CONTRACT_ABI = [
  {
    inputs: [
      { name: "_zrc6Amount", type: "uint256" },
      { name: "_tokenURI", type: "string" },
      { name: "_recipient", type: "address" }
    ],
    name: "swapZRC6ToERC721",
    outputs: [{ name: "tokenId", type: "uint256" }],
    stateMutability: "nonpayable",
    type: "function"
  },
  {
    inputs: [
      { name: "_tokenId", type: "uint256" },
      { name: "_recipient", type: "address" }
    ],
    name: "swapERC721ToZRC6",
    outputs: [{ name: "amount", type: "uint256" }],
    stateMutability: "nonpayable",
    type: "function"
  }
] as const

// Contract addresses by network
export const CONTRACT_ADDRESSES = {
  33469: { // Zilliqa Testnet
    ZRC6: "0x...", // Replace with actual address
    ERC721: "0x...", // Replace with actual address
    SWAP: "0x...", // Replace with actual address
  },
  32769: { // Zilliqa Mainnet
    ZRC6: "0x...", // Replace with actual address
    ERC721: "0x...", // Replace with actual address  
    SWAP: "0x...", // Replace with actual address
  }
} as const
