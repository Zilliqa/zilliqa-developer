# Zilliqa Wallet Integration Guide

A comprehensive guide to **adding Zilliqa wallet connectivity to your existing React/Next.js application** using RainbowKit, Wagmi, and WalletConnect. This guide focuses on EVM-compatible wallets (MetaMask, WalletConnect, Coinbase, etc.) as the primary integration method, with optional native Zilliqa wallet support like ZilPay for enhanced functionality.

## Table of Contents

This guide shows you how to integrate Zilliqa wallet functionality into your existing application with minimal changes to your current architecture:

1. **[Prerequisites](#prerequisites)** - What your existing app needs
2. **[Installation & Dependencies](#installation--dependencies)** - Adding required packages
3. **[Configuration Setup](#configuration-setup)** - Environment variables and API configuration
4. **[Chain Configuration](#chain-configuration)** - Zilliqa network definitions
5. **[Wallet Integration](#wallet-integration)** - EVM wallet connectors setup
6. **[Provider Integration](#provider-integration)** - Adding providers to your existing app
7. **[Development and Testing](#development-and-testing)** - Using testnet for development
8. **[Transaction Management](#transaction-management)** - Hooks for contract interactions
9. **[Integration Examples](#integration-examples)** - Adding wallet features to existing components
10. **[Best Practices](#best-practices)** - Security, error handling, and optimization
11. **[Troubleshooting](#troubleshooting)** - Common issues and solutions

## Prerequisites

This guide assumes you have an existing React or Next.js application that you want to extend with Zilliqa wallet functionality. Your application should have:

### Required Foundation
- **React 18+** or **Next.js 13+** application
- **TypeScript** support (recommended for type safety)
- **Package manager**: npm, yarn, or pnpm
- **Build system**: Webpack, Vite, or Next.js built-in

### What This Guide Adds
- **RainbowKit**: Wallet connection UI and management
- **Wagmi**: React hooks for EVM wallet interactions
- **Viem**: TypeScript interface for EVM chains
- **WalletConnect**: Protocol for mobile wallet connections
- **TanStack Query**: Data fetching and caching (if not already present)

### Integration Approach
This guide uses EVM-compatible wallets for Zilliqa integration, which means:

- **Minimal App Changes**: Integrates cleanly with existing React patterns
- **Familiar User Experience**: Users can use their existing MetaMask, Coinbase Wallet, etc.
- **No Architecture Overhaul**: Adds wallet functionality without changing your app structure
- **Incremental Adoption**: Add wallet features to specific components as needed
- **Testnet Development**: Test with real wallets on Zilliqa testnet

### Files You'll Add
This integration adds these files to your existing project structure:

- `config/zilliqa-chains.ts` - Network configurations
- `config/wallet-config.ts` - Wallet connector setup
- `contexts/WalletProvider.tsx` - Wallet state management (optional)
- `utils/wallet-helpers.ts` - Helper functions
- Updates to your existing `_app.tsx` or main app file

## Installation & Dependencies

Add the required packages to your existing React/Next.js application. These packages integrate seamlessly with most existing setups.

### Install Required Packages

Add these wallet connectivity packages to your project:

```bash
npm install @rainbow-me/rainbowkit wagmi viem @tanstack/react-query
```

**Compatibility Notes:**
- Works with React 18+ and Next.js 13+
- Compatible with existing UI libraries (no conflicts)
- TanStack Query may already be in your project (that's fine)
- TypeScript types are included

### CSS Imports (Required)

Add RainbowKit styles to your main CSS file or `_app.tsx`:

```css
@import '@rainbow-me/rainbowkit/styles.css';
```

## Configuration Setup

Add these configuration variables to your existing environment setup. This integrates with your current configuration approach.

### Add Environment Variables

Add these variables to your existing `.env.local` file:

```env
# Add these to your existing environment variables
NEXT_PUBLIC_CHAIN_ID=33469
NEXT_PUBLIC_WALLETCONNECT_PROJECT_ID=your_walletconnect_project_id
```

**Get WalletConnect Project ID:**
1. Visit [WalletConnect Cloud](https://cloud.walletconnect.com/)
2. Create a free account
3. Create a new project
4. Copy the Project ID

### Configuration in Your App

You can access these variables in your components like any other environment variable:

```typescript
const chainId = parseInt(process.env.NEXT_PUBLIC_CHAIN_ID || "33469")
const projectId = process.env.NEXT_PUBLIC_WALLETCONNECT_PROJECT_ID || ""
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

```

### 2. Chain Selection Utilities

These utility functions help manage chain selection and validation throughout your application:

```typescript
export function getChain(chainId: number) {
  const chains = [
    ZILLIQA_MAINNET,
    ZILLIQA_TESTNET,
    ZILLIQA_DEVNET
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
    ZILLIQA_DEVNET
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

## Provider Integration

Integrate wallet providers into your existing app structure. This works with your current provider setup.

### Update Your Main App File

Modify your existing `_app.tsx` (Next.js) or main app component (React) to add wallet providers:

```typescript
// Add these imports to your existing _app.tsx
import { QueryClient, QueryClientProvider } from "@tanstack/react-query"
import { WagmiProvider } from "wagmi"
import { RainbowKitProvider } from "@rainbow-me/rainbowkit"
import { createWagmiConfig } from "@/config/zilliqa-chains" // You'll create this
import '@rainbow-me/rainbowkit/styles.css'

// Create QueryClient (or use existing one)
const queryClient = new QueryClient()

export default function App({ Component, pageProps }: AppProps) {
  // Your existing app logic here...

  const wagmiConfig = createWagmiConfig(
    parseInt(process.env.NEXT_PUBLIC_CHAIN_ID || "33469"),
    process.env.NEXT_PUBLIC_WALLETCONNECT_PROJECT_ID || ""
  )

  return (
    // Wrap your existing providers with wallet providers
    <WagmiProvider config={wagmiConfig}>
      <QueryClientProvider client={queryClient}>
        <RainbowKitProvider showRecentTransactions={true}>
          {/* Your existing providers go here */}
          <Component {...pageProps} />
        </RainbowKitProvider>
      </QueryClientProvider>
    </WagmiProvider>
  )
}
```

**Integration Notes:**
- Add wallet providers **outside** your existing providers
- Keep your existing routing, themes, and state management
- QueryClient can be shared if you already use TanStack Query

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
  RealWallet,
}

const useWalletConnector = () => {
  const walletAccount = useAccount()

  const connectedWalletType = walletAccount.isConnected
    ? ConnectedWalletType.RealWallet
    : ConnectedWalletType.None

  const walletAddress = connectedWalletType === ConnectedWalletType.RealWallet
    ? walletAccount.address
    : null

  const { data: balanceData, refetch: refetchBalance } = useBalance({
    address: walletAddress ? (walletAddress as Address) : undefined,
  })

  const updateWalletBalance = () => {
    refetchBalance()
  }

  return {
    isWalletConnected: walletAccount.isConnected,
    connectedWalletType,
    walletAddress,
    zilAvailable: balanceData?.value,
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

## Development and Testing

For development and testing, use the Zilliqa testnet instead of mock wallet systems. This approach provides:

- **Real Blockchain Interaction**: Test with actual blockchain behavior
- **Authentic User Experience**: Users interact with real wallets and transactions
- **Simplified Development**: No need to maintain complex mock systems
- **Better Integration Testing**: Catch integration issues early

### Getting Testnet ZIL

1. **Faucet**: Use the [Zilliqa testnet faucet](https://dev.zilliqa.com/faucet) to get testnet ZIL
2. **MetaMask Setup**: Add Zilliqa testnet to MetaMask using the chain configuration provided in this guide
3. **Multiple Accounts**: Create multiple MetaMask accounts to test different scenarios

### Testnet Development Benefits

- **Real Gas Estimation**: Accurate gas calculations
- **Actual Network Latency**: Test with real network conditions
- **Wallet Integration**: Test with actual wallet implementations
- **Error Handling**: Experience real blockchain errors

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

    try {
      writeContract(txCallParams)
    } catch (error) {
      console.error("Transaction failed:", error)
      setIsTxInPreparation(false)
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
    isTxProcessedByChain,
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



## Integration Examples

Here's how to add wallet functionality to your existing components and features.

### Adding Wallet Connection to Existing Components

Integrate wallet connection into your existing navigation or header:

```typescript
// In your existing Header/Navigation component
import { ConnectButton } from '@rainbow-me/rainbowkit'

function YourExistingHeader() {
  return (
    <header className="your-existing-classes">
      {/* Your existing navigation */}
      <nav>...</nav>

      {/* Add wallet connection */}
      <ConnectButton />
    </header>
  )
}
```

### Adding Wallet State to Existing Features

Use wallet data in your existing components:

```typescript
// In any existing component
import { useAccount, useBalance } from 'wagmi'

function YourExistingComponent() {
  const { address, isConnected } = useAccount()
  const { data: balance } = useBalance({ address })

  // Your existing component logic...

  return (
    <div className="your-existing-component">
      {/* Your existing content */}

      {/* Add wallet-aware features */}
      {isConnected && (
        <div className="wallet-info">
          <p>Connected: {address}</p>
          <p>Balance: {balance?.formatted} ZIL</p>
        </div>
      )}
    </div>
  )
}
```

### Adding Transaction Features to Existing Actions

Extend existing user actions with blockchain transactions:

```typescript
// In your existing component with user actions
import { useWriteContract } from 'wagmi'

function YourExistingActionComponent() {
  const { writeContract } = useWriteContract()

  const handleExistingAction = async () => {
    // Your existing logic...

    // Add blockchain transaction
    if (shouldInteractWithBlockchain) {
      writeContract({
        address: '0x...', // Your contract address
        abi: yourContractAbi,
        functionName: 'yourFunction',
        args: [/* your args */]
      })
    }
  }

  return (
    <div className="your-existing-component">
      {/* Your existing UI */}
      <button onClick={handleExistingAction}>
        Your Existing Action
      </button>
    </div>
  )
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
    console.warn("Transaction Cancelled: You cancelled the transaction.")
  } else if (error.code === -32000) {
    // Insufficient funds
    console.error("Insufficient Funds: You don't have enough balance for this transaction.")
  } else {
    // Generic error
    console.error("Transaction Failed: An unexpected error occurred. Please try again.")
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

## Quick Integration Checklist

### 1. Prerequisites Check
- [ ] React 18+ or Next.js 13+ app running
- [ ] TypeScript configured (recommended)
- [ ] Access to modify your main app file

### 2. Package Installation
- [ ] Install RainbowKit, Wagmi, Viem packages
- [ ] Add RainbowKit CSS import
- [ ] Get WalletConnect Project ID

### 3. Configuration
- [ ] Add environment variables to existing `.env.local`
- [ ] Create Zilliqa chain configuration file
- [ ] Set up wallet connectors

### 4. Provider Integration
- [ ] Add wallet providers to your existing `_app.tsx`
- [ ] Test basic wallet connection
- [ ] Verify integration with existing app flow

### 5. Feature Integration
- [ ] Add ConnectButton to existing components
- [ ] Use wallet hooks in existing features
- [ ] Test on Zilliqa testnet


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

This guide shows you how to **extend your existing React/Next.js application** with Zilliqa wallet functionality using minimal changes to your current architecture.

### Integration Benefits:
- **Non-Disruptive**: Works with your existing app structure
- **Incremental**: Add wallet features where needed
- **Familiar Patterns**: Uses standard React hooks and components
- **Testnet Ready**: Test with real wallets immediately
- **Production Safe**: Type-safe with comprehensive error handling

### What You've Added:
- Zilliqa network connectivity
- EVM wallet support (MetaMask, WalletConnect, etc.)
- Transaction capabilities
- Balance checking
- Network switching

### Next Steps:
1. **Start Small**: Add ConnectButton to one component
2. **Test Integration**: Verify wallet connection works with your app
3. **Expand Features**: Add wallet functionality to existing user flows
4. **Deploy to Testnet**: Test with real users before mainnet
5. **Monitor Usage**: Track which wallet features users prefer

Your existing application now has full Zilliqa wallet integration without architectural changes.

### Resources
- [Zilliqa EVM Documentation](https://dev.zilliqa.com/)
- [RainbowKit Documentation](https://rainbowkit.com/)
- [Wagmi Documentation](https://wagmi.sh/)
- [WalletConnect Documentation](https://docs.walletconnect.com/)