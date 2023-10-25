import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";

const config: HardhatUserConfig = {
  solidity: {
    version: "0.8.19",
    settings: {
      optimizer: {
        enabled: true,
        runs: 200,
      },
    },
  },
  mocha: {
    timeout: 2000000,
  },
  networks: {
    net1: {
      url: `http://127.0.0.1:8545/`,
    },
    net2: {
      url: `http://127.0.0.1:8546/`,
    },
  },
};

export default config;
