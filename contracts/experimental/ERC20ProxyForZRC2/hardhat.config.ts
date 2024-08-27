import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";
import "dotenv/config";
import "./tasks/deployProxy.ts";
import "hardhat-scilla-plugin";


if (!process.env.PRIVATE_KEY) {
  throw new Error("PRIVATE_KEY not set");
}

const config: HardhatUserConfig = {
  solidity: "0.8.20",
  networks: {
    "zq-testnet": {
      url: "https://dev-api.zilliqa.com",
      accounts: [process.env.PRIVATE_KEY ?? ""],
    },
    "zq": {
      url: "https://api.zilliqa.com",
      accounts: [process.env.PRIVATE_KEY ?? ""],
    },
    "local-proxy": {
      url: "http://localhost:5555",
      accounts: [process.env.PRIVATE_KEY ?? ""],
    }
  },
  sourcify: {
    enabled: true,
  },
  etherscan: {
    enabled: false,
    customChains: [
      {
        network: "zilliqa-testnet",
        chainId: 33101,
        urls: {
          apiURL: "https://dev-api.zilliqa.com",
          browserURL: "https://otterscan.testnet.zilliqa.com"
        }
      },
      {
        network: "zilliqa",
        chainId: 32769,
        urls: {
          apiURL: "https://api.zilliqa.com",
          browserURL: "https://otterscan.zilliqa.com"
        }
      }
    ]
  },
};

export default config;
