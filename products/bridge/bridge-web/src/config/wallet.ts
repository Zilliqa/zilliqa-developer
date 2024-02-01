import { getDefaultWallets } from "@rainbow-me/rainbowkit";
import { bsc, zilliqa } from "viem/chains";
import { configureChains, createConfig } from "wagmi";
import { publicProvider } from "wagmi/providers/public";

export const { chains, publicClient } = configureChains(
  [bsc, zilliqa],
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
