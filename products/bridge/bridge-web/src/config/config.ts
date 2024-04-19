import { Chain } from "viem";
import { bsc, bscTestnet, zilliqa, zilliqaTestnet } from "viem/chains";
import fps_token from "../assets/fps_token.png";
import hrse_token from "../assets/hrse_token.png";

export enum TokenManagerType {
  MintAndBurn,
  LockAndRelease,
}

export type Chains = "bsc-testnet" | "zq-testnet" | "bsc" | "zq";

export const chainConfigs: Partial<Record<Chains, ChainConfig>> =
  import.meta.env.MODE === "production"
    ? {
        zq: {
          chain: "zq",
          name: "Zilliqa",
          tokenManagerAddress: "0x6D61eFb60C17979816E4cE12CD5D29054E755948",
          chainGatewayAddress: "0xbA44BC29371E19117DA666B729A1c6e1b35DDb40",
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
            {
              name: "HRSE",
              address: "0x63B991C17010C21250a0eA58C6697F696a48cdf3",
              blockExplorer:
                "https://otterscan.zilliqa.com/address/0x63B991C17010C21250a0eA58C6697F696a48cdf3",
              logo: hrse_token,
            },
          ],
          chainId: 32769,
          isZilliqa: true,
          blockExplorer: "https://otterscan.zilliqa.com/tx/",
          nativeTokenSymbol: "ZIL",
        },
        bsc: {
          chain: "bsc",
          name: "BSC",
          wagmiChain: bsc,
          tokenManagerAddress: "0xF391A1Ee7b3ccad9a9451D2B7460Ac646F899f23",
          chainGatewayAddress: "0x3967f1a272Ed007e6B6471b942d655C802b42009",
          tokenManagerType: TokenManagerType.MintAndBurn,
          tokens: [
            {
              name: "FPS",
              address: "0x351dA1E7500aBA1d168b9435DCE73415718d212F",
              blockExplorer:
                "https://bscscan.com/address/0x351dA1E7500aBA1d168b9435DCE73415718d212F",
              logo: fps_token,
            },
            {
              name: "HRSE",
              address: "0x3BE0E5EDC58bd55AAa381Fa642688ADC289c05a3",
              blockExplorer:
                "https://bscscan.com/address/0x3BE0E5EDC58bd55AAa381Fa642688ADC289c05a3",
              logo: hrse_token,
            },
          ],
          chainId: 56,
          isZilliqa: false,
          blockExplorer: "https://bscscan.com/tx/",
          nativeTokenSymbol: "BNB",
        },
      }
    : {
        "zq-testnet": {
          chain: "zq-testnet",
          name: "Zilliqa Testnet",
          tokenManagerAddress: "0x1509988c41f02014aA59d455c6a0D67b5b50f129",
          tokenManagerType: TokenManagerType.LockAndRelease,
          chainGatewayAddress: "0x7370e69565BB2313C4dA12F9062C282513919230",
          wagmiChain: zilliqaTestnet,
          tokens: [
            {
              name: "TST",
              address: "0x8618d39a8276D931603c6Bc7306af6A53aD2F1F3",
              blockExplorer:
                "https://otterscan.testnet.zilliqa.com/address/0x8618d39a8276D931603c6Bc7306af6A53aD2F1F3",
            },
            {
              name: "TSLM",
              address: "0xE90Dd366D627aCc5feBEC126211191901A69f8a0",
              blockExplorer:
                "https://otterscan.testnet.zilliqa.com/address/0xE90Dd366D627aCc5feBEC126211191901A69f8a0",
            },
          ],
          chainId: 33101,
          isZilliqa: true,
          blockExplorer: "https://otterscan.testnet.zilliqa.com/tx/",
          nativeTokenSymbol: "ZIL",
        },
        "bsc-testnet": {
          chain: "bsc-testnet",
          name: "BSC Testnet",
          wagmiChain: bscTestnet,
          tokenManagerAddress: "0xA6D73210AF20a59832F264fbD991D2abf28401d0",
          tokenManagerType: TokenManagerType.MintAndBurn,
          chainGatewayAddress: "0xa9A14C90e53EdCD89dFd201A3bF94D867f8098fE",
          tokens: [
            {
              name: "TST",
              address: "0x5190e8b4Bbe8C3a732BAdB600b57fD42ACbB9F4B",
              blockExplorer:
                "https://testnet.bscscan.com/address/0x5190e8b4Bbe8C3a732BAdB600b57fD42ACbB9F4B",
            },
            {
              name: "TST",
              address: "0x7Cc585de659E8938Aa7d5709BeaF34bD108bdC03",
              blockExplorer:
                "https://testnet.bscscan.com/address/0x7Cc585de659E8938Aa7d5709BeaF34bD108bdC03",
            },
          ],
          chainId: 97,
          isZilliqa: false,
          blockExplorer: "https://testnet.bscscan.com/tx/",
          nativeTokenSymbol: "BNB",
        },
      };

export type ChainConfig = {
  name: string;
  chain: Chains;
  wagmiChain: Chain;
  tokenManagerAddress: `0x${string}`;
  chainGatewayAddress: `0x${string}`;
  tokenManagerType: TokenManagerType;
  tokens: TokenConfig[];
  chainId: number;
  isZilliqa: boolean;
  blockExplorer: string;
  nativeTokenSymbol: string;
};

export type TokenConfig = {
  name: string;
  address: `0x${string}`;
  blockExplorer: string;
  logo?: string;
};
