# Zilliqa Wallet Integration Guide

A comprehensive guide to implementing wallet connectivity Zilliqa network applications using RainbowKit, Wagmi, and WalletConnect. This guide focuses on EVM-compatible wallets (MetaMask, WalletConnect, Coinbase, etc.) as the primary integration method, with optional native Zilliqa wallet support like ZilPay for enhanced functionality.

## Table of Contents

This guide is organized into progressive sections that will take you from initial setup to production-ready wallet integration:

1. **[Technology Stack](#technology-stack)** - Core libraries and their roles
2. **[Project Structure](#project-structure)** - Recommended file organization
3. **[Installation & Dependencies](#installation--dependencies)** - Package installation and setup
4. **[Configuration Setup](#configuration-setup)** - Environment variables and API configuration
5. **[Chain Configuration](#chain-configuration)** - Zilliqa network definitions and RPC setup
6. **[Wallet Integration](#wallet-integration)** - EVM wallet connectors and ZilPay (optional)
7. **[Provider Setup](#provider-setup)** - React context and provider hierarchy
8. **[Transaction Management](#transaction-management)** - Hooks for contract interactions
9. **[Mock Wallet System](#mock-wallet-system)** - Development and testing tools
10. **[UI Components](#ui-components)** - Wallet connection buttons and status displays
11. **[Advanced Features](#advanced-features)** - Multi-chain, deep linking, and performance
12. **[Best Practices](#best-practices)** - Security, error handling, and optimization
13. **[Troubleshooting](#troubleshooting)** - Common issues and solutions

## Technology Stack

This section outlines the core technologies used in this wallet integration approach. Understanding these dependencies will help you make informed decisions about implementation.

### Core Dependencies
- **RainbowKit**: Primary wallet connection UI and management
- **Wagmi**: React hooks for EVM wallet interactions
- **Viem**: TypeScript interface for EVM chains
- **WalletConnect**: Protocol for mobile wallet connections
- **React/Next.js**: Frontend framework
- **UI Library**: Any React component library (Ant Design, Chakra UI, etc.)
- **TanStack Query**: Data fetching and caching

### Why EVM Wallets for Zilliqa?
Zilliqa's EVM-compatible network allows standard Ethereum wallets to work seamlessly. This approach offers significant advantages:

- **Broader Adoption**: Most Web3 users already have MetaMask, Coinbase Wallet, or other EVM wallets installed
- **Familiar UX**: Users don't need to learn new wallet connection flows - they use the same process as other dApps
- **Mobile Support**: WalletConnect protocol enables seamless mobile wallet integration across dozens of wallets
- **Developer Experience**: Leverage the mature Ethereum tooling ecosystem (Wagmi, Viem, RainbowKit)
- **Future-Proof**: As Zilliqa EVM adoption grows, wallet support will improve automatically

### Additional Dependencies
```json
{
  "@rainbow-me/rainbowkit": "^2.0.0",
  "wagmi": "^2.0.0",
  "viem": "^2.0.0",
  "@tanstack/react-query": "^5.0.0"
}
```

## Project Structure

A well-organized project structure makes wallet integration maintainable and scalable. This recommended structure separates concerns and follows React/Next.js best practices.

```
src/
├── components/
│   ├── WalletConnect.tsx           # Main wallet connection UI
│   └── MockWalletSelector.tsx      # Mock wallet selector (optional)
├── contexts/
│   ├── WalletProvider.tsx          # Wallet state management
│   ├── TransactionProvider.tsx     # Transaction operations
│   └── AppConfig.tsx               # App configuration
├── config/
│   ├── chains.ts                   # Zilliqa chain configurations
│   ├── wallets.ts                  # Supported wallet configurations
│   └── contracts.ts                # Contract ABIs and addresses
├── utils/
│   ├── connector.ts                # Provider detection utilities
│   └── formatting.ts               # Utility functions
└── pages/
    ├── _app.tsx                    # Provider hierarchy setup
    └── api/
        └── config.ts               # Environment configuration
```

## Installation & Dependencies

This section covers installing the required packages for wallet connectivity. We'll start with core dependencies and then add UI libraries based on your project needs.

### 1. Install Core Packages

These are the essential packages for wallet connectivity on Zilliqa:

```bash
npm install @rainbow-me/rainbowkit wagmi viem @tanstack/react-query
```

### 2. Install UI Dependencies

Choose a UI library that fits your project. Examples shown use basic styling, but you can adapt to any framework:

```bash
npm install antd @ant-design/icons
```

### 3. Install Additional Utilities

These packages provide helpful utilities for date/time handling and formatting:

```bash
npm install luxon
npm install -D @types/luxon
```

## Configuration Setup

Proper configuration is crucial for wallet connectivity. This section sets up environment variables and API endpoints that your wallet integration will use.

### 1. Environment Variables

Environment variables keep sensitive data secure and allow different configurations for development, staging, and production:

Create your environment configuration in `.env.local`:

```env
NEXT_PUBLIC_CHAIN_ID=32769
NEXT_PUBLIC_WALLETCONNECT_PROJECT_ID=your_walletconnect_project_id
NEXT_PUBLIC_APP_URL=http://localhost:3000
NEXT_PUBLIC_APP_NAME=Your_Zilliqa_App
```

### 2. API Configuration Endpoint

This API endpoint provides configuration data to your frontend application, keeping sensitive keys on the server side:

Create `src/pages/api/config.ts`:

```typescript
import { NextApiRequest, NextApiResponse } from "next"

export interface AppConfig {
  chainId: number
  walletConnectProjectId: string
  appUrl: string
  appName: string
}

export default function handler(
  req: NextApiRequest,
  res: NextApiResponse<AppConfig>
) {
  const config: AppConfig = {
    chainId: parseInt(process.env.NEXT_PUBLIC_CHAIN_ID || "32769"),
    walletConnectProjectId: process.env.NEXT_PUBLIC_WALLETCONNECT_PROJECT_ID || "",
    appUrl: process.env.NEXT_PUBLIC_APP_URL || "http://localhost:3000",
    appName: process.env.NEXT_PUBLIC_APP_NAME || "Zilliqa DApp",
  }

  res.status(200).json(config)
}
```

## Chain Configuration

Defining chain configurations properly ensures your application can connect to the correct Zilliqa networks and switch between them as needed.

### 1. Define Zilliqa Chain Configurations

Zilliqa operates multiple networks for different purposes. Here's how to configure each one:

Create `src/config/chains.ts`:

```typescript
import { defineChain } from "viem"

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
```

### 2. Chain Selection Utilities

These utility functions help manage chain selection and validation throughout your application:

```typescript
export function getChain(chainId: number) {
  const chains = [
    ZILLIQA_MAINNET,
    ZILLIQA_TESTNET,
    ZILLIQA_PROTO_TESTNET,
    ZILLIQA_LOCAL
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
    ZILLIQA_PROTO_TESTNET,
    ZILLIQA_LOCAL
  ]
}
```

## Wallet Integration

This is the core of the guide - setting up wallet connectors with an EVM-first approach. We prioritize widely-adopted wallets while keeping ZilPay as an optional enhancement.

### 1. Wallet Connector Configuration

This configuration defines which wallets appear in your connection modal and their priority order. The order matters for user experience:

Create `src/config/wallets.ts`:

```typescript
import { connectorsForWallets } from "@rainbow-me/rainbowkit"
import {
  coinbaseWallet,
  ledgerWallet,
  metaMaskWallet,
  rabbyWallet,
  rainbowWallet,
  trustWallet,
  walletConnectWallet,
} from "@rainbow-me/rainbowkit/wallets"
// Optional: import { zilPayWallet } from "./zilpay-wallet"

export function getWalletConnectors(projectId: string, appName: string, appUrl: string) {
  return connectorsForWallets(
    [
      {
        groupName: "Popular Wallets",
        wallets: [
          metaMaskWallet,         // Most popular EVM wallet
          walletConnectWallet,    // Mobile wallet support
          coinbaseWallet,         // Major exchange wallet
          rabbyWallet,           // Advanced features
        ],
      },
      {
        groupName: "Other Wallets",
        wallets: [
          trustWallet,
          ledgerWallet,
          rainbowWallet,
        ],
      },
      {
        groupName: "Zilliqa Native (Optional)",
        wallets: [
          zilPayWallet,           // Enhanced Zilliqa features
        ],
      },
    ],
    {
      appName,
      projectId,
      appUrl,
    }
  )
}
```

### 2. ZilPay Wallet Integration (Optional Enhancement)

ZilPay provides enhanced Zilliqa-specific features but should be treated as an optional add-on. Most users will connect via standard EVM wallets, so ensure your app works fully without ZilPay.

**Note**: ZilPay provides enhanced Zilliqa-specific features but is optional. Most users will connect via standard EVM wallets.

Create `src/config/zilpay-wallet.ts`:

```typescript
import {
  getWalletConnectConnector,
  RainbowKitWalletConnectParameters,
  Wallet,
} from "@rainbow-me/rainbowkit"
import { hasInjectedProvider, getInjectedConnector } from "./connector"

export const zilPayWallet = ({
  projectId,
  walletConnectParameters,
}: {
  projectId: string
  walletConnectParameters?: RainbowKitWalletConnectParameters
}): Wallet => {
  const isZilPayInjected = hasInjectedProvider({
    flag: "isZilPay",
  })
  const shouldUseWalletConnect = !isZilPayInjected

  const getUri = (uri: string) => {
    return `zilpay://wc?uri=${encodeURIComponent(uri)}`
  }

  return {
    id: "zilpay",
    name: "ZilPay",
    rdns: "io.zilpay",
    iconUrl: async () => "/path/to/zilpay-icon.svg",
    iconBackground: "#ffffff",
    installed: isZilPayInjected,
    downloadUrls: {
      android: "https://play.google.com/store/apps/details?id=com.zilpaymobile",
      ios: "https://apps.apple.com/app/zilpay/id1547105860",
      mobile: "https://zilpay.io/",
      qrCode: "https://zilpay.io/",
    },
    mobile: {
      getUri: shouldUseWalletConnect ? getUri : undefined,
    },
    qrCode: shouldUseWalletConnect
      ? {
          getUri: (uri) => uri,
          instructions: {
            learnMoreUrl: "https://learn.zilpay.io/",
            steps: [
              {
                description: "Install the ZilPay app on your mobile device.",
                step: "install",
                title: "Install ZilPay",
              },
              {
                description: "Create a new wallet or import an existing one.",
                step: "create",
                title: "Create or Import Wallet",
              },
              {
                description: "Tap the scan button and scan this QR code.",
                step: "scan",
                title: "Scan QR Code",
              },
            ],
          },
        }
      : undefined,
    createConnector: shouldUseWalletConnect
      ? getWalletConnectConnector({
          projectId,
          walletConnectParameters,
        })
      : getInjectedConnector({
          flag: "isZilPay",
        }),
  }
}
```

### 3. Provider Detection Utility

This utility detects which wallets are installed in the user's browser, enabling smart wallet recommendations:

Create `src/utils/connector.ts`:

```typescript
import { getInjectedConnector as getRainbowKitInjectedConnector } from "@rainbow-me/rainbowkit"

declare global {
  interface Window {
    ethereum?: any
  }
}

export function hasInjectedProvider({ flag }: { flag: string }): boolean {
  if (typeof window === "undefined") return false

  const provider = window.ethereum
  return !!(provider && provider[flag])
}

export function getInjectedConnector({ flag }: { flag: string }) {
  return getRainbowKitInjectedConnector({
    flag,
    shimDisconnect: true,
  })
}
```

### 4. Wagmi Configuration

Wagmi manages the connection between your app and the blockchain. This configuration ties together your chain definitions and wallet connectors:

Update `src/config/chains.ts` to include the Wagmi config:

```typescript
import { createConfig } from "wagmi"
import { createClient, http } from "viem"

export function createWagmiConfig(
  chainId: number,
  projectId: string,
  appName: string,
  appUrl: string
) {
  return createConfig({
    chains: [getChain(chainId)],
    client({ chain }) {
      return createClient({ chain, transport: http() })
    },
    connectors: getWalletConnectors(projectId, appName, appUrl),
    ssr: true,
  })
}

// Viem client for direct blockchain interactions
const chainIdClientMap: Record<number, ReturnType<typeof createPublicClient>> = {}

export function getViemClient(chainId: number) {
  if (!chainIdClientMap[chainId]) {
    chainIdClientMap[chainId] = createPublicClient({
      chain: getChain(chainId),
      transport: http(),
    })
  }
  return chainIdClientMap[chainId]
}
```

## Provider Setup

### 1. App Provider Hierarchy

Update `src/pages/_app.tsx`:

```typescript
import { QueryClient, QueryClientProvider } from "@tanstack/react-query"
import { WagmiProvider } from "wagmi"
import { RainbowKitProvider } from "@rainbow-me/rainbowkit"
import { createWagmiConfig } from "@/config/chains"
import { WalletProvider } from "@/contexts/WalletProvider"
import type { AppProps } from "next/app"
import { useEffect, useState } from "react"
import type { AppConfig } from "./api/config"

const queryClient = new QueryClient()

export default function App({ Component, pageProps }: AppProps) {
  const [appConfig, setAppConfig] = useState<AppConfig | null>(null)

  useEffect(() => {
    const fetchConfig = async () => {
      try {
        const res = await fetch("/api/config")
        const data = await res.json()
        setAppConfig(data)
      } catch (error) {
        console.error("Failed to load app config:", error)
      }
    }
    fetchConfig()
  }, [])

  if (!appConfig) {
    return (
      <div className="flex items-center justify-center min-h-screen">
        <div>Loading...</div>
      </div>
    )
  }

  const wagmiConfig = createWagmiConfig(
    appConfig.chainId,
    appConfig.walletConnectProjectId,
    appConfig.appName,
    appConfig.appUrl
  )

  return (
    <WagmiProvider config={wagmiConfig} reconnectOnMount={true}>
      <QueryClientProvider client={queryClient}>
        <RainbowKitProvider showRecentTransactions={true}>
          <WalletProvider>
            <Component {...pageProps} />
          </WalletProvider>
        </RainbowKitProvider>
      </QueryClientProvider>
    </WagmiProvider>
  )
}
```

### 2. Wallet State Management Context

This context provides a unified interface for wallet operations, abstracting away the complexity of different wallet types and connection states:

Create `src/contexts/WalletProvider.tsx`:

```typescript
import { useState } from "react"
import { useAccount, useBalance } from "wagmi"
import { Address } from "viem"
import { createContainer } from "./context"

export enum ConnectedWalletType {
  None,
  MockWallet,
  RealWallet,
}

const useWalletConnector = () => {
  const walletAccount = useAccount()
  const [isDummyWalletConnected, setIsDummyWalletConnected] = useState(false)
  const [dummyWallet, setDummyWallet] = useState<any>(null)

  const connectedWalletType = isDummyWalletConnected
    ? ConnectedWalletType.MockWallet
    : walletAccount.isConnected
      ? ConnectedWalletType.RealWallet
      : ConnectedWalletType.None

  const walletAddress = connectedWalletType === ConnectedWalletType.MockWallet
    ? dummyWallet?.address
    : connectedWalletType === ConnectedWalletType.RealWallet
      ? walletAccount.address
      : null

  const { data: balanceData, refetch: refetchBalance } = useBalance({
    address: walletAddress ? (walletAddress as Address) : undefined,
  })

  const connectDummyWallet = () => {
    // Mock wallet connection logic
  }

  const disconnectDummyWallet = () => {
    setIsDummyWalletConnected(false)
    setDummyWallet(null)
  }

  const updateWalletBalance = () => {
    if (!isDummyWalletConnected) {
      refetchBalance()
    }
  }

  return {
    isWalletConnected: walletAccount.isConnected || isDummyWalletConnected,
    connectedWalletType,
    walletAddress,
    zilAvailable: balanceData?.value || dummyWallet?.balance,
    connectDummyWallet,
    disconnectDummyWallet,
    updateWalletBalance,
  }
}

export const WalletProvider = createContainer(useWalletConnector)

// Create a simple context helper if you don't have one
export function createContainer<T>(useHook: () => T) {
  const Context = createContext<T | null>(null)

  function Provider({ children, ...props }: any) {
    const value = useHook()
    return <Context.Provider value={value}>{children}</Context.Provider>
  }

  function useContainer(): T {
    const context = useContext(Context)
    if (!context) {
      throw new Error("useContainer must be used within Provider")
    }
    return context
  }

  return { Provider, useContainer }
}
```

## Transaction Management

Transaction management handles the complex flow of preparing, submitting, and monitoring blockchain transactions. This section provides reusable hooks and patterns for reliable transaction handling.

### 1. Transaction Management Hook

This hook abstracts the common transaction flow patterns, providing consistent error handling and state management across your application:

Create `src/contexts/TransactionProvider.tsx`:

```typescript
import { useWriteContract, useWaitForTransactionReceipt, useGasPrice } from "wagmi"
import { Address, WriteContractParameters } from "viem"

const useTransaction = (
  successMessage: string,
  errorMessage: string,
  onSuccess?: () => void
) => {
  const [txHash, setTxHash] = useState<Address | undefined>(undefined)
  const [isTxInPreparation, setIsTxInPreparation] = useState(false)

  const {
    isLoading: isTxProcessedByChain,
    error: txContractError,
    status: txReceiptStatus,
  } = useWaitForTransactionReceipt({ hash: txHash })

  const {
    writeContract,
    status: txSubmissionStatus,
    error: txSubmissionError,
    data: currentTxData,
  } = useWriteContract()

  const callContract = (txCallParams: WriteContractParameters) => {
    setIsTxInPreparation(true)
    setTxHash(undefined)

    if (isDummyWalletConnected) {
      // Mock transaction simulation
      setTimeout(() => {
        setTxHash("0x1234567890234567890234567890234567890" as Address)
        setIsTxInPreparation(false)
      }, 2000)
    } else {
      try {
        writeContract(txCallParams)
      } catch (error) {
        console.error("Transaction failed:", error)
        setIsTxInPreparation(false)
      }
    }
  }

  // Handle transaction status updates
  useEffect(() => {
    if (txReceiptStatus === "success") {
      console.log(successMessage)
      // Trigger data refresh
    }
  }, [txReceiptStatus])

  useEffect(() => {
    if (txSubmissionStatus === "success") {
      setTxHash(currentTxData)
      setIsTxInPreparation(false)
    }
  }, [txSubmissionStatus, currentTxData])

  useEffect(() => {
    if (txSubmissionStatus === "error") {
      console.error(errorMessage, txSubmissionError)
      setIsTxInPreparation(false)
    }
  }, [txSubmissionStatus])

  return {
    isTxInPreparation: isTxInPreparation || txSubmissionStatus === "pending",
    isTxProcessedByChain: isTxProcessedByChain && !isDummyWalletConnected,
    txHash,
    txContractError,
    callContract,
  }
}
```

### 2. Gas Price Calculation

Proper gas estimation prevents failed transactions and provides users with accurate fee estimates:

```typescript
const { data: reportedGasPrice } = useGasPrice()

// Apply 25% buffer to reported gas price
const adjustedGasPrice = ((reportedGasPrice || 0n) * 125n) / 100n

const getGasCostInZil = (estimatedGas: bigint) => {
  return Math.ceil(parseFloat(formatUnits(estimatedGas * adjustedGasPrice, 18)))
}
```

### 3. Generic Contract Interaction Examples

These examples show common patterns for interacting with smart contracts on Zilliqa. Adapt these patterns for your specific use cases:

```typescript
// ERC-20 Token Transfer
const transferTokens = (tokenAddress: string, to: string, amount: bigint) => {
  const wagmiConfig = useConfig()
  const { walletAddress } = useWallet()

  callContract({
    address: tokenAddress as Address,
    abi: erc20Abi,
    functionName: "transfer",
    args: [to as Address, amount],
    chain: wagmiConfig.chains[0],
    account: walletAddress as Address,
  })
}

// Generic contract function call
const callContractFunction = (
  contractAddress: string,
  abi: any[],
  functionName: string,
  args: any[] = [],
  value?: bigint
) => {
  const wagmiConfig = useConfig()
  const { walletAddress } = useWallet()

  callContract({
    address: contractAddress as Address,
    abi,
    functionName,
    args,
    value,
    chain: wagmiConfig.chains[0],
    account: walletAddress as Address,
  })
}

// ZIL transfer (native token)
const sendZIL = (to: string, amount: bigint) => {
  const wagmiConfig = useConfig()
  const { walletAddress } = useWallet()

  callContract({
    address: to as Address,
    abi: [],
    functionName: "",
    args: [],
    value: amount,
    chain: wagmiConfig.chains[0],
    account: walletAddress as Address,
  })
}
```

## Mock Wallet System

The mock wallet system enables development and testing without requiring real wallets or testnet tokens. This is essential for rapid development and automated testing.

### 1. Mock Wallet Definitions (Optional)

Define various wallet scenarios for testing different user conditions:

Create `src/config/mock-wallets.ts`:

```typescript
export interface MockWallet {
  id: string
  name: string
  address: string
  balance: bigint
  description: string
}

export const mockWallets: MockWallet[] = [
  {
    id: "wallet-1",
    name: "Developer Wallet",
    address: "0x1234567890123456789012345678901234567890",
    balance: BigInt("1000000000000000000000"), // 1000 ZIL
    description: "High balance for testing large transactions",
  },
  {
    id: "wallet-2",
    name: "User Wallet",
    address: "0x0987654321098765432109876543210987654321",
    balance: BigInt("100000000000000000000"), // 100 ZIL
    description: "Medium balance for typical user scenarios",
  },
  {
    id: "wallet-3",
    name: "Low Balance Wallet",
    address: "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd",
    balance: BigInt("1000000000000000000"), // 1 ZIL
    description: "Low balance for testing insufficient funds",
  },
]
```

### 2. Mock Wallet Selector Component

This component allows developers to quickly switch between different mock wallet scenarios during development:

```typescript
import { mockWallets } from "@/config/mock-wallets"

const MockWalletSelector = () => {
  const {
    isMockWalletSelectorOpen,
    selectMockWallet,
    setIsMockWalletSelectorOpen
  } = WalletProvider.useContainer()

  return (
    <div className="modal" style={{ display: isMockWalletSelectorOpen ? 'block' : 'none' }}>
      <div className="modal-content">
        <h3>Select Mock Wallet</h3>
        <div className="wallet-list">
          {mockWallets.map((wallet) => (
            <div
              key={wallet.id}
              className="wallet-item"
              onClick={() => selectMockWallet(wallet)}
              style={{ cursor: "pointer", padding: "10px", border: "1px solid #ccc", margin: "5px" }}
            >
              <div className="wallet-name">{wallet.name}</div>
              <div className="wallet-balance">
                {(Number(wallet.balance) / 1e18).toFixed(2)} ZIL
              </div>
              <div className="wallet-description">{wallet.description}</div>
            </div>
          ))}
        </div>
        <button onClick={() => setIsMockWalletSelectorOpen(false)}>
          Cancel
        </button>
      </div>
    </div>
  )
}
```

## UI Components

UI components provide the user interface for wallet interactions. These components handle the complex logic of wallet states while presenting a clean interface to users.

### 1. Wallet Connect Component

The main wallet connection component handles different wallet states and provides appropriate UI for each:

Create `src/components/customWalletConnect.tsx`:

```typescript
import { ConnectButton } from "@rainbow-me/rainbowkit"
import { Button } from "antd"
import { WalletOutlined } from "@ant-design/icons"

interface CustomWalletConnectProps {
  children: React.ReactNode
  notConnectedClassName: string
}

const CustomWalletConnect: React.FC<CustomWalletConnectProps> = ({
  children,
  notConnectedClassName,
}) => {
  const { appConfig } = AppConfigStorage.useContainer()
  const {
    isDummyWalletConnecting,
    connectDummyWallet,
    disconnectDummyWallet,
    isDummyWalletConnected,
    walletAddress,
    zilAvailable,
  } = WalletConnector.useContainer()

  // Mock chain handling
  if (appConfig.chainId === MOCK_CHAIN.id) {
    if (!isDummyWalletConnected) {
      return (
        <Button
          type="primary"
          onClick={connectDummyWallet}
          loading={isDummyWalletConnecting}
          className={notConnectedClassName}
        >
          {children}
        </Button>
      )
    } else {
      return (
        <Button
          type="primary"
          className="group relative"
          onClick={disconnectDummyWallet}
        >
          <div className="group-hover:hidden flex items-center">
            <WalletOutlined className="mr-2" />
            {formatAddress(walletAddress || "")} |{" "}
            {formatUnits(zilAvailable || 0n, 18)} ZIL
          </div>
          <span className="hidden group-hover:block">
            Disconnect
          </span>
        </Button>
      )
    }
  }

  // Real wallet handling
  return (
    <ConnectButton.Custom>
      {({ account, chain, openConnectModal, mounted }) => {
        if (!mounted) {
          return <Button className={notConnectedClassName}>Loading...</Button>
        }

        if (!account || !chain) {
          return (
            <Button
              onClick={openConnectModal}
              className={notConnectedClassName}
            >
              {children}
            </Button>
          )
        }

        return (
          <div className="flex justify-end items-center">
            <ConnectButton />
          </div>
        )
      }}
    </ConnectButton.Custom>
  )
}
```

### 2. Transaction Status Display

This component provides real-time feedback on transaction status, keeping users informed during the transaction lifecycle:

```typescript
const TransactionStatus = ({
  isPreparation,
  isProcessing,
  txHash,
  error
}) => {
  if (isPreparation) {
    return <span>⏳ Preparing transaction...</span>
  }

  if (isProcessing) {
    return <span>🔄 Processing transaction...</span>
  }

  if (error) {
    return <span>❌ Transaction failed: {error.message}</span>
  }

  if (txHash) {
    return <span>✅ Transaction successful!</span>
  }

  return null
}
```

## Advanced Features

Advanced features enhance the user experience and provide additional functionality for sophisticated applications. Implement these based on your specific requirements.

### 1. Multiple Chain Support

Enable users to switch between different Zilliqa networks (mainnet, testnet, etc.):

```typescript
const chains = [
  CHAIN_MAINNET,
  CHAIN_DEVNET,
  CHAIN_TESTNET,
  MOCK_CHAIN
]

export function getWagmiConfig(chainId: number, ...) {
  const selectedChain = getChain(chainId)

  return createConfig({
    chains: [selectedChain],
    // ... other config
  })
}
```

### 2. Network Switching

Provide UI for users to switch between different networks:

```typescript
import { useSwitchChain } from "wagmi"

const NetworkSwitcher = () => {
  const { switchChain } = useSwitchChain()

  const handleNetworkSwitch = (chainId: number) => {
    switchChain({ chainId })
  }

  return (
    <Select onChange={handleNetworkSwitch}>
      <Option value={32769}>Mainnet</Option>
      <Option value={33469}>Devnet</Option>
    </Select>
  )
}
```

### 3. Transaction History

Track and display user transaction history for better UX:

```typescript
const useTransactionHistory = () => {
  const [transactions, setTransactions] = useState([])

  const addTransaction = (tx: Transaction) => {
    setTransactions(prev => [tx, ...prev])
  }

  const updateTransaction = (txHash: string, update: Partial<Transaction>) => {
    setTransactions(prev =>
      prev.map(tx =>
        tx.hash === txHash ? { ...tx, ...update } : tx
      )
    )
  }

  return { transactions, addTransaction, updateTransaction }
}
```

### 4. Deep Linking for Mobile Wallets

Enable direct mobile wallet integration through deep links:

```typescript
const getDeepLink = (uri: string, walletType: string) => {
  const links = {
    zilpay: `zilpay://wc?uri=${encodeURIComponent(uri)}`,
    metamask: `metamask://wc?uri=${encodeURIComponent(uri)}`,
    trust: `trust://wc?uri=${encodeURIComponent(uri)}`,
  }

  return links[walletType] || uri
}
```

## Best Practices

Following these best practices ensures your wallet integration is secure, performant, and provides excellent user experience.

### 1. Error Handling

Comprehensive error handling prevents user confusion and provides actionable feedback:

```typescript
const handleTransactionError = (error: any) => {
  if (error.code === 4001) {
    // User rejected transaction
    notification.warning({
      message: "Transaction Cancelled",
      description: "You cancelled the transaction.",
    })
  } else if (error.code === -32000) {
    // Insufficient funds
    notification.error({
      message: "Insufficient Funds",
      description: "You don't have enough balance for this transaction.",
    })
  } else {
    // Generic error
    notification.error({
      message: "Transaction Failed",
      description: "An unexpected error occurred. Please try again.",
    })
  }
}
```

### 2. Loading States

Proper loading states keep users informed and prevent multiple transaction submissions:

```typescript
const WalletButton = () => {
  const [isConnecting, setIsConnecting] = useState(false)

  const handleConnect = async () => {
    setIsConnecting(true)
    try {
      await connectWallet()
    } finally {
      setIsConnecting(false)
    }
  }

  return (
    <Button loading={isConnecting} onClick={handleConnect}>
      {isConnecting ? "Connecting..." : "Connect Wallet"}
    </Button>
  )
}
```

### 3. Type Safety

TypeScript types prevent runtime errors and improve developer experience:

```typescript
// Define proper types for your contracts
interface StakingContract {
  stake: (amount: bigint) => Promise<void>
  unstake: (amount: bigint) => Promise<void>
  claimRewards: () => Promise<void>
}

// Use typed contract interactions
const useStakingContract = (address: Address): StakingContract => {
  const { writeContract } = useWriteContract()

  return {
    stake: async (amount: bigint) => {
      return writeContract({
        address,
        abi: stakingAbi,
        functionName: "stake",
        args: [],
        value: amount,
      })
    },
    // ... other methods
  }
}
```

### 4. Performance Optimization

Optimize performance to ensure smooth user experience:

```typescript
// Memoize expensive computations
const gasEstimate = useMemo(() => {
  return calculateGasCost(estimatedGas, gasPrice)
}, [estimatedGas, gasPrice])

// Debounce frequent updates
const debouncedUpdateBalance = useCallback(
  debounce(updateWalletBalance, 1000),
  [updateWalletBalance]
)
```

### 5. Security Considerations

Security is paramount in Web3 applications. Follow these practices to protect users:

```typescript
// Validate addresses
const isValidAddress = (address: string): boolean => {
  return /^0x[a-fA-F0-9]{40}$/.test(address)
}

// Sanitize user inputs
const sanitizeAmount = (amount: string): bigint => {
  const cleaned = amount.replace(/[^0-9.]/g, "")
  return parseUnits(cleaned, 18)
}

// Verify transaction parameters
const verifyTransaction = (params: TransactionParams): boolean => {
  return (
    isValidAddress(params.to) &&
    params.value > 0n &&
    params.gasLimit > 21000n
  )
}
```

## Troubleshooting

This section covers common issues you might encounter during wallet integration and their solutions.

### Common Issues

These are the most frequently encountered problems and their solutions:

#### 1. RainbowKit Styles Not Loading
```typescript
// Ensure CSS import order in _app.tsx
import "@rainbow-me/rainbowkit/styles.css"
import "tailwindcss/tailwind.css" // After RainbowKit
```

#### 2. WalletConnect Connection Issues
```typescript
// Check project ID configuration
const projectId = process.env.NEXT_PUBLIC_WALLETCONNECT_PROJECT_ID
if (!projectId) {
  throw new Error("WalletConnect project ID not found")
}
```

#### 3. Chain Configuration Errors
```typescript
// Ensure chain is properly defined
const validateChain = (chain: Chain) => {
  if (!chain.id || !chain.rpcUrls.default.http[0]) {
    throw new Error("Invalid chain configuration")
  }
}
```

#### 4. Transaction Failures
```typescript
// Add proper error boundaries
const TransactionWrapper = ({ children }) => {
  return (
    <ErrorBoundary
      fallback={<div>Transaction error occurred</div>}
      onError={(error) => console.error("Transaction error:", error)}
    >
      {children}
    </ErrorBoundary>
  )
}
```

### Debugging Tips

Use these techniques to diagnose and resolve integration issues:

1. **Enable Wagmi Debug Mode**:
```typescript
const config = createConfig({
  // ... other config
  batch: { multicall: true },
  pollingInterval: 4000,
})
```

2. **Log Transaction States**:
```typescript
useEffect(() => {
  console.log("Transaction state:", {
    isPreparation,
    isProcessing,
    txHash,
    error
  })
}, [isPreparation, isProcessing, txHash, error])
```

3. **Monitor Wallet Events**:
```typescript
useEffect(() => {
  const handleAccountsChanged = (accounts: string[]) => {
    console.log("Accounts changed:", accounts)
  }

  if (window.ethereum) {
    window.ethereum.on("accountsChanged", handleAccountsChanged)
    return () => {
      window.ethereum.removeListener("accountsChanged", handleAccountsChanged)
    }
  }
}, [])
```

### Performance Monitoring

Monitor your wallet integration performance to identify bottlenecks:

```typescript
const usePerformanceMonitor = () => {
  useEffect(() => {
    const observer = new PerformanceObserver((list) => {
      list.getEntries().forEach((entry) => {
        if (entry.name.includes("wallet")) {
          console.log(`${entry.name}: ${entry.duration}ms`)
        }
      })
    })

    observer.observe({ entryTypes: ["measure"] })
    return () => observer.disconnect()
  }, [])
}
```

## Quick Start Checklist

### 1. Environment Setup
- [ ] Install dependencies: RainbowKit, Wagmi, Viem
- [ ] Set up environment variables
- [ ] Configure WalletConnect project ID

### 2. Chain Configuration
- [ ] Define Zilliqa chain configurations
- [ ] Set up Wagmi config with chain selection
- [ ] Configure RPC endpoints

### 3. Wallet Integration
- [ ] Configure primary EVM wallets (MetaMask, WalletConnect, Coinbase)
- [ ] Test wallet connection flow with standard wallets
- [ ] Optionally add ZilPay for enhanced Zilliqa features

### 4. Transaction Management
- [ ] Implement transaction hooks
- [ ] Add error handling
- [ ] Set up gas estimation

### 5. UI Components
- [ ] Create wallet connect button
- [ ] Add transaction status display
- [ ] Implement responsive design

## Example Zilliqa DApp Use Cases

### DeFi Applications
```typescript
// Token swapping
const swapTokens = (tokenA: string, tokenB: string, amount: bigint) => {
  callContractFunction(
    DEX_CONTRACT_ADDRESS,
    dexAbi,
    "swapExactTokensForTokens",
    [amount, 0, [tokenA, tokenB], walletAddress, deadline]
  )
}

// Liquidity provision
const addLiquidity = (tokenA: string, tokenB: string, amountA: bigint, amountB: bigint) => {
  callContractFunction(
    DEX_CONTRACT_ADDRESS,
    dexAbi,
    "addLiquidity",
    [tokenA, tokenB, amountA, amountB, 0, 0, walletAddress, deadline]
  )
}
```

### NFT Marketplace
```typescript
// Mint NFT
const mintNFT = (to: string, tokenURI: string) => {
  callContractFunction(
    NFT_CONTRACT_ADDRESS,
    nftAbi,
    "mint",
    [to, tokenURI]
  )
}

// Transfer NFT
const transferNFT = (from: string, to: string, tokenId: bigint) => {
  callContractFunction(
    NFT_CONTRACT_ADDRESS,
    nftAbi,
    "transferFrom",
    [from, to, tokenId]
  )
}
```

### Gaming Applications
```typescript
// Purchase in-game item
const buyItem = (itemId: number, price: bigint) => {
  callContractFunction(
    GAME_CONTRACT_ADDRESS,
    gameAbi,
    "purchaseItem",
    [itemId],
    price
  )
}

// Claim rewards
const claimRewards = () => {
  callContractFunction(
    GAME_CONTRACT_ADDRESS,
    gameAbi,
    "claimRewards",
    []
  )
}
```

## Advanced Integration Patterns

### Multi-Signature Wallet Support
```typescript
const createMultisigTransaction = (to: string, value: bigint, data: string) => {
  callContractFunction(
    MULTISIG_WALLET_ADDRESS,
    multisigAbi,
    "submitTransaction",
    [to, value, data]
  )
}
```

### Cross-Chain Bridge Integration
```typescript
const bridgeTokens = (targetChain: number, amount: bigint) => {
  callContractFunction(
    BRIDGE_CONTRACT_ADDRESS,
    bridgeAbi,
    "lock",
    [targetChain, amount]
  )
}
```

## Recommended Wallet Integration Strategy

### Primary Approach: EVM Wallets
1. **Start with MetaMask**: Most widely adopted Web3 wallet
2. **Add WalletConnect**: Essential for mobile wallet support
3. **Include Major Wallets**: Coinbase, Rabby, Trust for broader reach
4. **Leverage Existing UX**: Users are familiar with standard Web3 flows

### Optional Enhancement: ZilPay
- Add ZilPay only if you need specific Zilliqa features
- Position as "enhanced experience" rather than requirement
- Ensure your app works fully without it

## Conclusion

This guide provides a complete foundation for implementing wallet connectivity for Zilliqa EVM applications. The architecture prioritizes:

- **EVM-First Approach**: Leverage the broader Web3 wallet ecosystem
- **Standard Web3 UX**: Familiar wallet connection flows for users
- **Mobile Support**: WalletConnect enables mobile wallet integration
- **Production Ready**: Type-safe transactions with comprehensive error handling
- **Broad Compatibility**: Support for all major EVM wallets
- **Optional Enhancement**: ZilPay for Zilliqa-specific features

### Key Benefits of EVM-First Strategy:
- **Larger User Base**: Most Web3 users already have MetaMask/Coinbase Wallet
- **Reduced Friction**: No need to install new wallets
- **Better Mobile Support**: WalletConnect works with many mobile wallets
- **Future-Proof**: As Zilliqa EVM grows, standard wallets will add native support

The EVM-first implementation patterns shown here can be adapted for any Zilliqa application, from simple token transfers to complex DeFi protocols, NFT marketplaces, and gaming platforms.

### Next Steps
1. Start with basic MetaMask + WalletConnect integration
2. Test thoroughly on Zilliqa testnet
3. Add additional EVM wallets based on user demand
4. Consider ZilPay only if you need Zilliqa-specific functionality
5. Monitor wallet usage analytics to optimize your wallet selection

### Resources
- [Zilliqa EVM Documentation](https://dev.zilliqa.com/)
- [RainbowKit Documentation](https://rainbowkit.com/)
- [Wagmi Documentation](https://wagmi.sh/)
- [WalletConnect Documentation](https://docs.walletconnect.com/)