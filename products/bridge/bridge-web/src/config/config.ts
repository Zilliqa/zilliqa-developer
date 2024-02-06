import { Chain } from "viem";
import { bsc, bscTestnet, zilliqa, zilliqaTestnet } from "viem/chains";
import fps_token from "../assets/fps_token.png";

export enum TokenManagerType {
  MintAndBurn,
  LockAndRelease,
}

export type Chains = "bsc-testnet" | "zq-testnet" | "bsc" | "zq";

export const chainConfigs: Partial<Record<Chains, ChainConfig>> =
  //import.meta.env.MODE === "production"
  true
    ? {
        zq: {
          chain: "zq",
          name: "Zilliqa",
          tokenManagerAddress: "0x6D61eFb60C17979816E4cE12CD5D29054E755948",
          tokenManagerType: TokenManagerType.LockAndRelease,
          wagmiChain: zilliqa,
          tokens: [
            {
              name: "FPS",
              address: "0x241c677D9969419800402521ae87C411897A029f",
              blockExplorer:
                "https://otterscan.zilliqa.com/address/0x241c677D9969419800402521ae87C411897A029f",
              logo: fps_token,
            },
          ],
          chainId: 32769,
          isZilliqa: true,
          blockExplorer: "https://otterscan.zilliqa.com/tx/",
        },
        bsc: {
          chain: "bsc",
          name: "BSC",
          wagmiChain: bsc,
          tokenManagerAddress: "0xF391A1Ee7b3ccad9a9451D2B7460Ac646F899f23",
          tokenManagerType: TokenManagerType.MintAndBurn,
          tokens: [
            {
              name: "FPS",
              address: "0x351dA1E7500aBA1d168b9435DCE73415718d212F",
              blockExplorer:
                "https://bscscan.com/address/0x351dA1E7500aBA1d168b9435DCE73415718d212F",
              logo: fps_token,
            },
          ],
          chainId: 56,
          isZilliqa: false,
          blockExplorer: "https://bscscan.com/tx/",
        },
      }
    : {
        "zq-testnet": {
          chain: "zq-testnet",
          name: "Zilliqa Testnet",
          tokenManagerAddress: "0x1509988c41f02014aA59d455c6a0D67b5b50f129",
          tokenManagerType: TokenManagerType.LockAndRelease,
          wagmiChain: zilliqaTestnet,
          tokens: [
            {
              name: "TST",
              address: "0x8618d39a8276D931603c6Bc7306af6A53aD2F1F3",
              blockExplorer:
                "https://otterscan.testnet.zilliqa.com/address/0x8618d39a8276D931603c6Bc7306af6A53aD2F1F3",
            },
          ],
          chainId: 33101,
          isZilliqa: true,
          blockExplorer: "https://otterscan.testnet.zilliqa.com/tx/",
        },
        "bsc-testnet": {
          chain: "bsc-testnet",
          name: "BSC Testnet",
          wagmiChain: bscTestnet,
          tokenManagerAddress: "0xA6D73210AF20a59832F264fbD991D2abf28401d0",
          tokenManagerType: TokenManagerType.MintAndBurn,
          tokens: [
            {
              name: "TST",
              address: "0x5190e8b4Bbe8C3a732BAdB600b57fD42ACbB9F4B",
              blockExplorer:
                "https://testnet.bscscan.com/address/0x5190e8b4Bbe8C3a732BAdB600b57fD42ACbB9F4B",
            },
          ],
          chainId: 97,
          isZilliqa: false,
          blockExplorer: "https://testnet.bscscan.com/tx/",
        },
      };

export type ChainConfig = {
  name: string;
  chain: Chains;
  wagmiChain: Chain;
  tokenManagerAddress: `0x${string}`;
  tokenManagerType: TokenManagerType;
  tokens: TokenConfig[];
  chainId: number;
  isZilliqa: boolean;
  blockExplorer: string;
};

export type TokenConfig = {
  name: string;
  address: `0x${string}`;
  blockExplorer: string;
  logo: string | undefined;
};
