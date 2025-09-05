import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react';
import { useAccount, useDisconnect } from 'wagmi';

interface WalletContextType {
  // ZilPay wallet state
  zilPayAccount: string | null;
  isZilPayConnected: boolean;
  connectZilPay: () => Promise<void>;
  disconnectZilPay: () => void;
  
  // EVM wallet state (from wagmi)
  evmAccount: string | null;
  isEvmConnected: boolean;
  disconnectEvm: () => void;
}

const WalletContext = createContext<WalletContextType | undefined>(undefined);

export function WalletProvider({ children }: { children: ReactNode }) {
  // ZilPay state
  const [zilPayAccount, setZilPayAccount] = useState<string | null>(null);
  const [isZilPayConnected, setIsZilPayConnected] = useState(false);

  // EVM wallet state from wagmi
  const { address: evmAddress, isConnected: isEvmConnected } = useAccount();
  const { disconnect: wagmiDisconnect } = useDisconnect();

  // Check ZilPay connection on mount
  useEffect(() => {
    checkZilPayConnection();
  }, []);

  const checkZilPayConnection = async () => {
    if (typeof window !== 'undefined' && window.zilPay) {
      try {
        const isConnected = window.zilPay.wallet.isConnect;
        if (isConnected && window.zilPay.wallet.defaultAccount) {
          const account = window.zilPay.wallet.defaultAccount.bech32.toLowerCase();
          setZilPayAccount(account);
          setIsZilPayConnected(true);
        }
      } catch (error) {
        console.error('Error checking ZilPay connection:', error);
      }
    }
  };

  const connectZilPay = async () => {
    if (typeof window.zilPay === 'undefined') {
      alert('Please install the ZilPay wallet extension.');
      return;
    }
    
    try {
      console.log('Connecting to ZilPay...');
      const isConnect = await window.zilPay.wallet.connect();
      if (isConnect && window.zilPay.wallet.defaultAccount) {
        const account = window.zilPay.wallet.defaultAccount.bech32.toLowerCase();
        setZilPayAccount(account);
        setIsZilPayConnected(true);
        console.log('ZilPay connected successfully!', { account });
      } else {
        console.log('Could not connect to ZilPay.');
      }
    } catch (error) {
      console.error("ZilPay connection error:", error);
      alert('ZilPay connection was rejected.');
    }
  };

  const disconnectZilPay = () => {
    setZilPayAccount(null);
    setIsZilPayConnected(false);
    console.log('ZilPay disconnected');
  };

  const disconnectEvm = () => {
    wagmiDisconnect();
  };

  const value: WalletContextType = {
    // ZilPay
    zilPayAccount,
    isZilPayConnected,
    connectZilPay,
    disconnectZilPay,
    
    // EVM
    evmAccount: evmAddress || null,
    isEvmConnected,
    disconnectEvm,
  };

  return (
    <WalletContext.Provider value={value}>
      {children}
    </WalletContext.Provider>
  );
}

export function useWallet() {
  const context = useContext(WalletContext);
  if (context === undefined) {
    throw new Error('useWallet must be used within a WalletProvider');
  }
  return context;
}

// Type declaration for ZilPay
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
