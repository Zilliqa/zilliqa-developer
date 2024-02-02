export enum TokenManagerType {
  MintAndBurn,
  LockAndRelease,
}

export type Chains = "bsc-testnet" | "zq-testnet";

export const chainConfigs: Record<Chains, ChainConfig> = {
  "bsc-testnet": {
    chain: "bsc-testnet",
    name: "BSC Testnet",
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
  },
  "zq-testnet": {
    chain: "zq-testnet",
    name: "Zilliqa Testnet",
    tokenManagerAddress: "0x1509988c41f02014aA59d455c6a0D67b5b50f129",
    tokenManagerType: TokenManagerType.LockAndRelease,
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
  },
};

export type ChainConfig = {
  name: string;
  chain: Chains;
  tokenManagerAddress: `0x${string}`;
  tokenManagerType: TokenManagerType;
  tokens: TokenConfig[];
  chainId: number;
  isZilliqa: boolean;
};

export type TokenConfig = {
  name: string;
  address: `0x${string}`;
  blockExplorer: string;
};
