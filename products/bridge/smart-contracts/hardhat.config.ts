import { HardhatUserConfig } from "hardhat/config";
import "dotenv/config";
import "@nomicfoundation/hardhat-toolbox";
import "@nomicfoundation/hardhat-foundry";
import "@openzeppelin/hardhat-upgrades";

const config: HardhatUserConfig = {
  solidity: {
    compilers: [
      { version: "0.5.16" },
      {
        version: "0.8.20",
        settings: {
          viaIR: true,
          optimizer: {
            enabled: true,
            runs: 10_000,
            details: {
              yulDetails: {
                optimizerSteps: "u",
              },
            },
          },
        },
      },
    ],
  },
  mocha: {
    timeout: 2000000,
  },
  networks: {
    hardhat: {
      chainId: Number(process.env.HARDHAT_CHAIN_ID ?? 31337),
    },
    "zq-testnet": {
      url: "https://dev-api.zilliqa.com",
      accounts: [process.env.PRIVATE_KEY_TEST ?? ""],
    },
    zq: {
      url: "https://api.zilliqa.com",
      accounts: [process.env.PRIVATE_KEY_TEST ?? ""],
    },
    bsc: {
      url: "https://bsc-dataseed1.binance.org",
      accounts: [process.env.PRIVATE_KEY_TEST ?? ""],
    },
    "bsc-testnet": {
      url: "https://data-seed-prebsc-1-s1.binance.org:8545/",
      accounts: [process.env.PRIVATE_KEY_TEST ?? ""],
    },
    net1: {
      url: `http://127.0.0.1:8545/`,
    },
    net2: {
      url: `http://127.0.0.1:8546/`,
    },
  },
};

export default config;
