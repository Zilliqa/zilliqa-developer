import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";
import "dotenv/config";

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
    zq: {
      url: "https://api.zilliqa.com",
      accounts: [process.env.PRIVATE_KEY ?? ""],
    },
  },
};

export default config;
