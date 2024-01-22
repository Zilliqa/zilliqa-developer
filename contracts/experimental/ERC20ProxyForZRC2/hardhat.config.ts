import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";

const config: HardhatUserConfig = {
  solidity: "0.8.20",
  networks: {
    "zq-testnet": {
      url: "https://dev-api.zilliqa.com",
      accounts: [process.env.PRIVATE_KEY_TEST ?? ""],
    },
    zq: {
      url: "https://api.zilliqa.com",
      accounts: [process.env.PRIVATE_KEY_TEST ?? ""],
    },
  },
};

export default config;
