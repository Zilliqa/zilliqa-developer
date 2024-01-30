import { getDefaultWallets } from "@rainbow-me/rainbowkit";
import { polygon, optimism, arbitrum, base, zora } from "viem/chains";
import { configureChains, mainnet, createConfig } from "wagmi";
import { publicProvider } from "wagmi/providers/public";

export const { chains, publicClient } = configureChains(
  [mainnet, polygon, optimism, arbitrum, base, zora],
  [publicProvider()]
);

const { connectors } = getDefaultWallets({
  appName: "Bridge Frontend",
  projectId: "Project ID",
  chains,
});

export const wagmiConfig = createConfig({
  autoConnect: true,
  connectors,
  publicClient,
});
