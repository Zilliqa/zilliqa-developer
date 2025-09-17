import { defineChain } from "viem"
import { createConfig } from "wagmi"
import { createClient, http } from "viem"
import { getWalletConnectors } from "./wallets"

// Zilliqa Mainnet (EVM-compatible)
export const ZILLIQA_MAINNET = defineChain({
  id: 32769,
  name: "Zilliqa EVM Mainnet",
  nativeCurrency: { name: "ZIL", symbol: "ZIL", decimals: 18 },
  rpcUrls: {
    default: {
      http: ["https://api.zilliqa.com"],
    },
  },
  blockExplorers: {
    default: {
      name: "Otterscan",
      url: "https://otterscan.zilliqa.com",
    },
  },
})

// Zilliqa Testnet (EVM-compatible)
export const ZILLIQA_TESTNET = defineChain({
  id: 33469,
  name: "Zilliqa EVM Testnet",
  nativeCurrency: { name: "ZIL", symbol: "ZIL", decimals: 18 },
  rpcUrls: {
    default: {
      http: ["https://api.zq2-devnet.zilliqa.com"],
    },
  },
  blockExplorers: {
    default: {
      name: "Otterscan",
      url: "https://explorer.zq2-devnet.zilliqa.com",
    },
  },
})

// Zilliqa Devnet
export const ZILLIQA_DEVNET = defineChain({
  id: 33469,
  name: "Zilliqa Devnet",
  nativeCurrency: { name: "ZIL", symbol: "ZIL", decimals: 18 },
  rpcUrls: {
    default: {
      http: ["https://api.zq2-devnet.zilliqa.com"],
    },
  },
  blockExplorers: {
    default: {
      name: "Otterscan",
      url: "https://otterscan.zq2-devnet.zilliqa.com",
    },
  },
})

// Local development chain
export const ZILLIQA_LOCAL = defineChain({
  id: 32768,
  name: "Zilliqa Local",
  nativeCurrency: { name: "ZIL", symbol: "ZIL", decimals: 18 },
  rpcUrls: {
    default: {
      http: ["http://localhost:4201"],
    },
  },
  blockExplorers: {
    default: {
      name: "Local Explorer",
      url: "http://localhost:5100",
    },
  },
})

// Mock chain for development
export const MOCK_CHAIN = defineChain({
  id: 31337,
  name: "Mock Chain",
  nativeCurrency: { name: "ZIL", symbol: "ZIL", decimals: 18 },
  rpcUrls: {
    default: {
      http: ["http://localhost:8545"],
    },
  },
  blockExplorers: {
    default: {
      name: "Mock Explorer",
      url: "http://localhost:3000",
    },
  },
})

export function getChain(chainId: number) {
  const chains = [
    ZILLIQA_MAINNET,
    ZILLIQA_TESTNET,
    ZILLIQA_DEVNET,
    ZILLIQA_LOCAL,
    MOCK_CHAIN
  ]
  const chain = chains.find((chain) => chain.id === chainId)

  if (!chain) {
    throw new Error(`Unsupported chain ID: ${chainId}`)
  }

  return chain
}

// Get all available Zilliqa chains
export function getAllZilliqaChains() {
  return [
    ZILLIQA_MAINNET,
    ZILLIQA_TESTNET,
    ZILLIQA_DEVNET,
    ZILLIQA_LOCAL
  ]
}

export function createWagmiConfig(
  chainId: number,
  projectId: string,
  appName: string,
  appUrl: string
) {
  const chain = getChain(chainId)
  
  return createConfig({
    chains: [chain],
    client({ chain }) {
      return createClient({ chain, transport: http() })
    },
    connectors: getWalletConnectors(projectId, appName, appUrl),
    ssr: true,
  })
}
