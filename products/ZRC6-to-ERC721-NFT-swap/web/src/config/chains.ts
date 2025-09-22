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
  id: 33101,
  name: "Zilliqa EVM Testnet",
  nativeCurrency: { name: "ZIL", symbol: "ZIL", decimals: 18 },
  rpcUrls: {
    default: {
      http: ["https://api.testnet.zilliqa.com"],
    },
  },
  blockExplorers: {
    default: {
      name: "Otterscan",
      url: "https://otterscan.testnet.zilliqa.com",
    },
  },
})

export function getChain(chainId: number) {
  const chains = [
    ZILLIQA_MAINNET,
    ZILLIQA_TESTNET,
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
