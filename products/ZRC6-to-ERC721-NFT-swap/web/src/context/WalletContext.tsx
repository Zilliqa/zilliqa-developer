import { useState, useEffect, useCallback } from "react"
import { useAccount, useBalance } from "wagmi"
import { Address } from "viem"
import { createContainer } from "../utils/context"
import { mockWallets, MockWallet } from "../config/mock-wallets"
import { formatAddress } from "../utils/formatting"

export enum ConnectedWalletType {
  None,
  MockWallet,
  RealWallet,
  ZilPayWallet,
}

declare global {
  interface Window {
    zilPay?: {
      wallet: {
        isConnect: boolean;
        defaultAccount: {
          bech32: string;
          base16: string;
        } | null;
        connect: () => Promise<boolean>;
      };
    };
  }
}

const useWalletConnector = () => {
  // Wagmi state for EVM wallets
  const walletAccount = useAccount()
  
  // Mock wallet state
  const [isDummyWalletConnected, setIsDummyWalletConnected] = useState(false)
  const [isDummyWalletConnecting, setIsDummyWalletConnecting] = useState(false)
  const [dummyWallet, setDummyWallet] = useState<MockWallet | null>(null)
  const [isMockWalletSelectorOpen, setIsMockWalletSelectorOpen] = useState(false)

  // ZilPay state
  const [zilPayAccount, setZilPayAccount] = useState<string | null>(null)
  const [isZilPayConnected, setIsZilPayConnected] = useState(false)
  const [isZilPayConnecting, setIsZilPayConnecting] = useState(false)

  // Determine connected wallet type
  const connectedWalletType = isZilPayConnected
    ? ConnectedWalletType.ZilPayWallet
    : isDummyWalletConnected
    ? ConnectedWalletType.MockWallet
    : walletAccount.isConnected
    ? ConnectedWalletType.RealWallet
    : ConnectedWalletType.None

  // Get wallet address based on connection type
  const walletAddress = connectedWalletType === ConnectedWalletType.ZilPayWallet
    ? zilPayAccount
    : connectedWalletType === ConnectedWalletType.MockWallet
    ? dummyWallet?.address
    : connectedWalletType === ConnectedWalletType.RealWallet
    ? walletAccount.address
    : null

  // Get balance for real wallets
  const { data: balanceData, refetch: refetchBalance } = useBalance({
    address: walletAddress && connectedWalletType === ConnectedWalletType.RealWallet 
      ? (walletAddress as Address) 
      : undefined,
  })

  // Get available balance
  const zilAvailable = connectedWalletType === ConnectedWalletType.MockWallet
    ? dummyWallet?.balance
    : connectedWalletType === ConnectedWalletType.RealWallet
    ? balanceData?.value
    : BigInt(0)

  // Check ZilPay connection on mount
  useEffect(() => {
    checkZilPayConnection()
  }, [])

  const checkZilPayConnection = async () => {
    if (typeof window !== 'undefined' && window.zilPay) {
      try {
        const isConnected = window.zilPay.wallet.isConnect
        if (isConnected && window.zilPay.wallet.defaultAccount) {
          const account = window.zilPay.wallet.defaultAccount.bech32.toLowerCase()
          setZilPayAccount(account)
          setIsZilPayConnected(true)
        }
      } catch (error) {
        console.error('Error checking ZilPay connection:', error)
      }
    }
  }

  const connectZilPay = async () => {
    if (typeof window === 'undefined' || typeof window.zilPay === 'undefined') {
      alert('Please install the ZilPay wallet extension.')
      return
    }
    
    setIsZilPayConnecting(true)
    try {
      const isConnect = await window.zilPay.wallet.connect()
      if (isConnect && window.zilPay.wallet.defaultAccount) {
        const account = window.zilPay.wallet.defaultAccount.bech32.toLowerCase()
        setZilPayAccount(account)
        setIsZilPayConnected(true)
      }
    } catch (error) {
      console.error("ZilPay connection error:", error)
      alert('ZilPay connection was rejected.')
    } finally {
      setIsZilPayConnecting(false)
    }
  }

  const disconnectZilPay = () => {
    setZilPayAccount(null)
    setIsZilPayConnected(false)
  }

  const connectDummyWallet = () => {
    setIsMockWalletSelectorOpen(true)
  }

  const selectMockWallet = (wallet: MockWallet) => {
    setIsDummyWalletConnecting(true)
    setTimeout(() => {
      setDummyWallet(wallet)
      setIsDummyWalletConnected(true)
      setIsDummyWalletConnecting(false)
      setIsMockWalletSelectorOpen(false)
    }, 1000) // Simulate connection delay
  }

  const disconnectDummyWallet = () => {
    setIsDummyWalletConnected(false)
    setDummyWallet(null)
  }

  const updateWalletBalance = useCallback(() => {
    if (connectedWalletType === ConnectedWalletType.RealWallet) {
      refetchBalance()
    }
  }, [connectedWalletType, refetchBalance])

  const isWalletConnected = connectedWalletType !== ConnectedWalletType.None

  return {
    // General wallet state
    isWalletConnected,
    connectedWalletType,
    walletAddress,
    zilAvailable,
    updateWalletBalance,

    // ZilPay wallet
    zilPayAccount,
    isZilPayConnected,
    isZilPayConnecting,
    connectZilPay,
    disconnectZilPay,

    // EVM wallet
    evmAccount: walletAccount.address || null,
    isEvmConnected: walletAccount.isConnected,

    // Mock wallet
    isDummyWalletConnected,
    isDummyWalletConnecting,
    dummyWallet,
    connectDummyWallet,
    selectMockWallet,
    disconnectDummyWallet,
    isMockWalletSelectorOpen,
    setIsMockWalletSelectorOpen,
    mockWallets,

    // Utility functions
    formatAddress,
  }
}

export const WalletProvider = createContainer(useWalletConnector)

// Legacy export for backward compatibility
export function useWallet() {
  return WalletProvider.useContainer()
}
