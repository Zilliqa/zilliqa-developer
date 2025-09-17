import "@/styles/globals.css";
import '@rainbow-me/rainbowkit/styles.css';
import type { AppProps } from "next/app";
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { WagmiProvider } from 'wagmi';
import { RainbowKitProvider } from '@rainbow-me/rainbowkit';
import { createWagmiConfig } from '@/config/chains';
import { WalletProvider } from '@/context/WalletContext';
import { TransactionProvider } from '@/contexts/TransactionProvider';
import { useEffect, useState } from "react";
import type { AppConfig } from "./api/config";

const queryClient = new QueryClient();

export default function App({ Component, pageProps }: AppProps) {
  const [appConfig, setAppConfig] = useState<AppConfig | null>(null);

  useEffect(() => {
    const fetchConfig = async () => {
      try {
        const res = await fetch("/api/config");
        const data = await res.json();
        setAppConfig(data);
      } catch (error) {
        console.error("Failed to load app config:", error);
        // Use default config if fetch fails
        setAppConfig({
          chainId: 33469,
          walletConnectProjectId: process.env.NEXT_PUBLIC_WALLETCONNECT_PROJECT_ID || '',
          appUrl: 'http://localhost:3000',
          appName: 'ZRC6 to ERC721 NFT Swap',
        });
      }
    };
    fetchConfig();
  }, []);

  if (!appConfig) {
    return (
      <div className="flex items-center justify-center min-h-screen">
        <div className="text-lg">Loading...</div>
      </div>
    );
  }

  const wagmiConfig = createWagmiConfig(
    appConfig.chainId,
    appConfig.walletConnectProjectId,
    appConfig.appName,
    appConfig.appUrl
  );

  return (
    <WagmiProvider config={wagmiConfig} reconnectOnMount={true}>
      <QueryClientProvider client={queryClient}>
        <RainbowKitProvider showRecentTransactions={true}>
          <WalletProvider.Provider>
            <TransactionProvider.Provider>
              <Component {...pageProps} />
            </TransactionProvider.Provider>
          </WalletProvider.Provider>
        </RainbowKitProvider>
      </QueryClientProvider>
    </WagmiProvider>
  );
}
