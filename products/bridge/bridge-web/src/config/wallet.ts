import { connectorsForWallets } from "@rainbow-me/rainbowkit";
import { metaMaskWallet } from "@rainbow-me/rainbowkit/wallets";
import { configureChains, createConfig } from "wagmi";
import { publicProvider } from "wagmi/providers/public";
import { chainConfigs } from "./config";

export const { chains, publicClient } = configureChains(
  Object.values(chainConfigs).map((chain) => chain.wagmiChain),
  [publicProvider()]
);

const connectors = connectorsForWallets([
  {
    groupName: "Recommended",
    // Project ID for WalletConnect Cloud.
    wallets: [metaMaskWallet({ chains, projectId: "f4d0c28ceef8b7064e5d92457c6f283b" })],
  },
]);

export const wagmiConfig = createConfig({
  autoConnect: true,
  connectors,
  publicClient,
});
