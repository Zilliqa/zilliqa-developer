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
    ],
    {
      appName,
      projectId,
      appUrl,
    }
  )
}
