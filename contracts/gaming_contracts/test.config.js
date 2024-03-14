const config = {
  defaultNetwork: "local_isolated_server",
  networks: {
    local_isolated_server: {
      url: "http://localhost:5555/",
      websocketUrl: "ws://localhost:5555",
      accounts: [
        "e53d1c3edaffc7a7bab5418eb836cf75819a82872b4a1a0f1c7fcf5c3e020b89",
        "d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba",
        "e7f59a4beb997a02a13e0d5e025b39a6f0adc64d37bb1e6a849a4863b4680411",
        "589417286a3213dceb37f8f89bd164c3505a4cec9200c61f7c6db13a30a71b45",
        "5430365143ce0154b682301d0ab731897221906a7054bbf5bd83c7663a6cbc40",
        "1080d2cca18ace8225354ac021f9977404cee46f1d12e9981af8c36322eac1a4",
      ],
      chainId: 222,
      version: 1,
      zilliqaNetwork: true,
      miningState: false,
      metaData: {
        contractDir: "contracts",
        pubKey:
          "0x03273a1480e75bd5bf3486d3d7072153c6d5f415eb7b751fa1b8c33caa4170bd69",
        privKey:
          "503d7f9758221766de828b4001d8187a804e24107ab7442e647bec90feb238ef",
      },
    },
    isolated_server: {
      url: "https://zilliqa-isolated-server.zilliqa.com",
      websocketUrl: "ws://zilliqa-isolated-server.zilliqa.com/",
      accounts: [
        "e53d1c3edaffc7a7bab5418eb836cf75819a82872b4a1a0f1c7fcf5c3e020b89",
        "938f763ec23f092a18c0a04acd4f97c89b2cd9021f6f578286b1ee428ae4650d",
        "dd28caefee06f07dbdd1ba1bd2b69a1f8b3ae67a91bf184be2784cd18aac191b",
        "3dcf5c47da5596cbf46823840c93faea1a7ef1e8b59b94f9857b091a4c0ad95c",
      ],
      chainId: 222,
      version: 1,
      zilliqaNetwork: true,
      miningState: false,
      metaData: {
        contractDir: "contracts",
        pubKey:
          "0x03273a1480e75bd5bf3486d3d7072153c6d5f415eb7b751fa1b8c33caa4170bd69",
        privKey:
          "503d7f9758221766de828b4001d8187a804e24107ab7442e647bec90feb238ef",
      },
    },
    testnet: {
      url: "https://dev-api.zilliqa.com",
      websocketUrl: "wss://dev-ws.zilliqa.com",
      accounts: [
        "e53d1c3edaffc7a7bab5418eb836cf75819a82872b4a1a0f1c7fcf5c3e020b89",
        "938f763ec23f092a18c0a04acd4f97c89b2cd9021f6f578286b1ee428ae4650d",
        "dd28caefee06f07dbdd1ba1bd2b69a1f8b3ae67a91bf184be2784cd18aac191b",
        "3dcf5c47da5596cbf46823840c93faea1a7ef1e8b59b94f9857b091a4c0ad95c",
      ],
      chainId: 333,
      version: 1,
      zilliqaNetwork: true,
      miningState: false,
      metaData: {
        contractDir: "contracts",
        pubKey:
          "0x03273a1480e75bd5bf3486d3d7072153c6d5f415eb7b751fa1b8c33caa4170bd69",
        privKey:
          "503d7f9758221766de828b4001d8187a804e24107ab7442e647bec90feb238ef",
      },
    },
  },
};

exports.config = config;
