import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";
import "@nomicfoundation/hardhat-ethers";
import "hardhat-scilla-plugin";
// import "./utils/deploy-task.ts";
import "hardhat-deploy";
import * as configUtils from "./utils/config.ts";

const chai = require("chai");
const { scillaChaiEventMatcher } = require("hardhat-scilla-plugin");
chai.use(scillaChaiEventMatcher);

// 33101 / 814D - testnet  (333 for ZIL)
// 32769 / 8001 - main net ( 1 for ZIL)
// 32990 / 80DE - isolated local server (222 for ZIL)
// 0x82BC - zblockchain localdev
//

const config: HardhatUserConfig = {
  solidity: "0.8.19",
  defaultNetwork: "isolated_server",
  networks: {
    devnet: {
      url: "https://api.devnet.zilliqa.com",
      websocketUrl: "ws://api.devnet.zilliqa.com/",
      accounts: [
        "db11cfa086b92497c8ed5a4cc6edb3a5bfe3a640c43ffb9fc6aa0873c56f2ee3",
        "e53d1c3edaffc7a7bab5418eb836cf75819a82872b4a1a0f1c7fcf5c3e020b89",
        "e7f59a4beb997a02a13e0d5e025b39a6f0adc64d37bb1e6a849a4863b4680411",
        "589417286a3213dceb37f8f89bd164c3505a4cec9200c61f7c6db13a30a71b45",
      ],
      chainId: 0x8269,
      web3ClientVersion: "Zilliqa/v8.2",
      protocolVersion: 0x41,
      zilliqaNetwork: true,
      miningState: false,
    },
    zblockchain_isolated: {
      url: "http://localhost:12005",
      websocketUrl: "ws://localhost:12005/",
      accounts: [
        "d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba",
        "e53d1c3edaffc7a7bab5418eb836cf75819a82872b4a1a0f1c7fcf5c3e020b89",
        "e7f59a4beb997a02a13e0d5e025b39a6f0adc64d37bb1e6a849a4863b4680411",
        "589417286a3213dceb37f8f89bd164c3505a4cec9200c61f7c6db13a30a71b45",
      ],
      chainId: 0x80de,
      web3ClientVersion: "Zilliqa/v8.2",
      protocolVersion: 0x41,
      zilliqaNetwork: true,
      miningState: false,
    },
    zblockchain: {
      url: "http://localhost:12003",
      websocketUrl: "ws://localhost:12003/",
      accounts: [
        "db11cfa086b92497c8ed5a4cc6edb3a5bfe3a640c43ffb9fc6aa0873c56f2ee3",
        "e53d1c3edaffc7a7bab5418eb836cf75819a82872b4a1a0f1c7fcf5c3e020b89",
        "e7f59a4beb997a02a13e0d5e025b39a6f0adc64d37bb1e6a849a4863b4680411",
        "589417286a3213dceb37f8f89bd164c3505a4cec9200c61f7c6db13a30a71b45",
      ],
      chainId: 0x82bc,
      web3ClientVersion: "Zilliqa/v8.2",
      protocolVersion: 0x41,
      zilliqaNetwork: true,
      miningState: false,
    },
    isolated_server: {
      url: "http://localhost:5555/",
      websocketUrl: "ws://localhost:5555/",
      accounts: [
        "d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba",
        "e53d1c3edaffc7a7bab5418eb836cf75819a82872b4a1a0f1c7fcf5c3e020b89",
        "e7f59a4beb997a02a13e0d5e025b39a6f0adc64d37bb1e6a849a4863b4680411",
        "589417286a3213dceb37f8f89bd164c3505a4cec9200c61f7c6db13a30a71b45",
      ],
      chainId: 0x8001,
      web3ClientVersion: "Zilliqa/v8.2",
      protocolVersion: 0x41,
      zilliqaNetwork: true,
      miningState: false,
    },
    ganache: {
      url: "http://localhost:7545",
      websocketUrl: "ws://localhost:7545",
      chainId: 1337,
      web3ClientVersion: "Ganache/v7.4.1/EthereumJS TestRPC/v7.4.1/ethereum-js",
      protocolVersion: 0x3f,
      accounts: [
        // memonic: guard same cactus near figure photo remove letter target alien initial remove
        "67545ce31f5ca86719cf3743730435768515ebf014f84811463edcf7dcfaf91e",
        "9be4f8840833f64d4881027f4a53961d75bc649ac4801b33f746487ca8873f14",
        "32a75b674cc41405c914de1fe7b031b832dfd9203e1a287d09122bab689519e3",
        "dd8ce58f8cecd59fde7000fff9944908e89364b2ef36921c35725957617ddd32",
      ],
      zilliqaNetwork: false,
      miningState: true,
    },
    public_testnet: {
      url: "https://dev-api.zilliqa.com",
      websocketUrl: "https://dev-api.zilliqa.com",
      accounts: [
        "d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba",
        "e53d1c3edaffc7a7bab5418eb836cf75819a82872b4a1a0f1c7fcf5c3e020b89",
        "e7f59a4beb997a02a13e0d5e025b39a6f0adc64d37bb1e6a849a4863b4680411",
        "589417286a3213dceb37f8f89bd164c3505a4cec9200c61f7c6db13a30a71b45",
      ],
      chainId: 33101,
      zilliqaNetwork: true,
      web3ClientVersion: "Zilliqa/v8.2",
      protocolVersion: 0x41,
      miningState: false,
    },
    localdev: {
      url: "http://localhost:5556",
      websocketUrl: "ws://localdev-l2api.localdomain",
      accounts: [
        "d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba",
        "e53d1c3edaffc7a7bab5418eb836cf75819a82872b4a1a0f1c7fcf5c3e020b89",
        "e7f59a4beb997a02a13e0d5e025b39a6f0adc64d37bb1e6a849a4863b4680411",
        "589417286a3213dceb37f8f89bd164c3505a4cec9200c61f7c6db13a30a71b45",
      ],
      chainId: 0x8001,
      web3ClientVersion: "Zilliqa/v8.2",
      protocolVersion: 0x41,
      zilliqaNetwork: true,
      miningState: false,
    },
  },
  mocha: {
    timeout: 500000,
  },
};

export default config;
