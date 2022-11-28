/**
 * Creating a sidebar enables you to:
 - create an ordered group of docs
 - render a sidebar for each doc of that group
 - provide next/previous navigation

 The sidebars can be generated from the filesystem, or explicitly defined here.

 Create as many sidebars as you want.
 */

// @ts-check

/** @type {import('@docusaurus/plugin-content-docs').SidebarsConfig} */
const sidebars = {
  BasicsSideBar: {
    "Blockchain Basics": [
      "basics/basics-intro-blockchain",
      "basics/basics-intro-consensus",
      "basics/basics-intro-accounts",
      "basics/basics-intro-txns",
      "basics/basics-intro-gas",
    ],

    "Zilliqa Architecture": [
      "basics/basics-zil-nodes",
      "basics/basics-zil-sharding",
      "basics/basics-zil-consensus",
      "basics/basics-zil-schnorr-signatures",
      "basics/basics-zil-reward",
      "basics/basics-zil-contract",
      "basics/basics-zil-gas",
    ],
  },
  APIsSideBar: {
    Introduction: ["apis/api-introduction"],
    "Blockchain-related Methods": [
      "apis/api-blockchain-ds-block-listing",
      "apis/api-blockchain-get-blockchain-info",
      "apis/api-blockchain-get-current-ds-epoch",
      "apis/api-blockchain-get-current-mini-epoch",
      "apis/api-blockchain-get-ds-block",
      "apis/api-blockchain-get-ds-block-rate",
      "apis/api-blockchain-get-latest-ds-block",
      "apis/api-blockchain-get-latest-tx-block",
      "apis/api-blockchain-get-miner-info",
      "apis/api-blockchain-get-network-id",
      "apis/api-blockchain-num-ds-blocks",
      "apis/api-blockchain-get-num-tx",
      "apis/api-blockchain-get-num-tx-blocks",
      "apis/api-blockchain-get-prev-difficulty",
      "apis/api-blockchain-get-prev-ds-difficulty",
      "apis/api-blockchain-get-total-coin-supply",
      "apis/api-blockchain-get-tx-rate",
      "apis/api-blockchain-get-tx-block",
      "apis/api-blockchain-get-tx-block-rate",
      "apis/api-blockchain-tx-block-listing",
    ],

    "Transaction-related Methods": [
      "apis/api-transaction-create-tx",
      "apis/api-transaction-get-minimum-gas-price",
      "apis/api-transaction-get-num-txns-dsepoch",
      "apis/api-transaction-get-num-txns-txepoch",
      "apis/api-transaction-get-recent-txs",
      "apis/api-transaction-get-tx",
      "apis/api-transaction-get-transaction-status",
      "apis/api-transaction-get-txs-for-txblock",
      "apis/api-transaction-get-txs-for-txblock-ex",
      "apis/api-transaction-get-txbodies-for-txblock",
      "apis/api-transaction-get-txbodies-for-txblock-ex",
    ],

    "Contract-related Methods": [
      "apis/api-contract-get-contractaddress-from-txid",
      "apis/api-contract-get-smartcontract-code",
      "apis/api-contract-get-smartcontract-init",
      "apis/api-contract-get-smartcontracts",
      "apis/api-contract-get-smartcontract-state",
      "apis/api-contract-get-smartcontract-substate",
      "apis/api-contract-get-state-proof",
    ],

    "Account-related Methods": ["apis/api-account-get-balance"],
  },
  DevelopersSidebar: {
    "Getting Started": [
      "dev-dapps/dev-started-introduction",
      "dev-dapps/dev-started-helloworld",
      "dev-dapps/dev-started-env",
    ],
    "Upgrade Notices": ["dev-dapps/dev-upgrade-v8"],

    "User Key Management": [
      "dev-dapps/dev-keys-introduction",
      "dev-dapps/dev-keys-zilpay",
      "dev-dapps/dev-keys-pkey",
      "dev-dapps/dev-keys-zrc2-wallet-support",
      "dev-dapps/dev-keys-magic",
    ],

    "Developer Toolings": [
      "dev-dapps/dev-tools-ceres",
      {
        SDKs: [
          "dev-dapps/dev-tools-zilliqajs",
          "dev-dapps/dev-tools-gozilliqa",
          "dev-dapps/dev-tools-java",
          "dev-dapps/dev-tools-sdks",
        ],
      },
      "dev-dapps/dev-tools-websockets",
      "dev-dapps/dev-tools-cli",
      "dev-dapps/dev-tools-ide",
      "dev-dapps/dev-tools-explorer",
      "dev-dapps/dev-tools-faucet",
    ],

    "Other Developer Information": ["dev-dapps/dev-wrapped-zil"],

    "Transaction Lifecycle": [
      "dev-dapps/dev-txn-signing",
      "dev-dapps/dev-txn-broadcasting",
      "dev-dapps/dev-txn-polling",
      "dev-dapps/dev-txn-confirmation",
      "dev-dapps/dev-txn-receipt",
    ],

    "Educational Resources": [
      "dev-dapps/dev-education-videos",
      {
        "Sample app – RentOnZilliqa": [
          "dev-dapps/dev-rentonzilliqa-introduction",
          {
            "Scilla Contract": [
              "dev-dapps/dev-rentonzilliqa-contract",
              "dev-dapps/dev-rentonzilliqa-library",
              "dev-dapps/dev-rentonzilliqa-mutable-variables",
              "dev-dapps/dev-rentonzilliqa-procedures",
              "dev-dapps/dev-rentonzilliqa-transitions",
            ],
          },
          {
            "Frontend Application": [
              "dev-dapps/dev-rentonzilliqa-frontend",
              "dev-dapps/dev-rentonzilliqa-components",
              "dev-dapps/dev-rentonzilliqa-scripting",
              "dev-dapps/dev-rentonzilliqa-modals",
              "dev-dapps/dev-rentonzilliqa-pages",
            ],
          },
        ],
      },
    ],
  },
  MinersSidebar: {
    Miners: [
      "miners/mining-getting-started",
      "miners/mining-zilclient",
      "miners/mining-zilminer",
      "miners/mining-proxy",
      "miners/mining-additional-info",
    ],
  },
  ExchangesSidebar: {
    "Exchange Integration": [
      {
        "Getting Started": [
          "exchanges/exchange-getting-started",
          "exchanges/exchange-ip-whitelisting",
          "exchanges/exchange-key-whitelisting-1",
          "exchanges/exchange-key-whitelisting-2",
        ],
      },
      "exchanges/exchange-account-management",
      "exchanges/exchange-sending-transactions",
      "exchanges/exchange-tracking-deposits",
      "exchanges/exchange-transaction-receipts",
      "exchanges/exchange-managing-zrc2-tokens",
    ],
    Rosetta: [
      {
        Introduction: [
          "exchanges/rosetta-introduction",
          "exchanges/rosetta-unsupported-api",
          "exchanges/rosetta-setting-up-seed-node",
          "exchanges/rosetta-setting-up-no-seed-node",
        ],
      },
      {
        "Data API": [
          {
            Network: [
              "exchanges/rosetta-data-network-list",
              "exchanges/rosetta-data-network-options",
              "exchanges/rosetta-data-network-status",
            ],
            Account: ["exchanges/rosetta-data-account-balance"],
            Block: [
              "exchanges/rosetta-data-block",
              "exchanges/rosetta-data-block-transaction",
            ],
          },
        ],
      },
      {
        "Construction API": [
          "exchanges/rosetta-construction",
          "exchanges/rosetta-construction-derive",
          "exchanges/rosetta-construction-preprocess",
          "exchanges/rosetta-construction-metadata",
          "exchanges/rosetta-construction-payloads",
          "exchanges/rosetta-construction-parse",
          "exchanges/rosetta-construction-combine",
          "exchanges/rosetta-construction-hash",
          "exchanges/rosetta-construction-submit",
        ],
      },
    ],
  },
  StakingSidebar: {
    "Zilliqa Seed Node Staking": [
      "staking/staking-overview",
      "staking/staking-disclaimer",
    ],
    "Staking Phase 1.1": [
      "staking/phase1/staking-phase11-notice",
      "staking/phase1/staking-phase1-overview",
      "staking/phase1/staking-general-information",
      {
        Delegators: [
          "staking/phase1/delegator/staking-delegator-overview",
          "staking/phase1/delegator/staking-delegator-reading-contract-states",
          "staking/phase1/delegator/staking-delegator-operations",
          "staking/phase1/delegator/staking-delegator-gzil",
        ],
      },
      {
        "SSN Operators": [
          "staking/phase1/ssn-operator/staking-ssn-before-you-start",
          "staking/phase1/ssn-operator/staking-ssn-setup",
          "staking/phase1/ssn-operator/staking-ssn-enrollment",
          "staking/phase1/ssn-operator/staking-commission-management",
          "staking/phase1/ssn-operator/staking-ssn-maintenance",
          "staking/phase1/ssn-operator/staking-ssn-upgrading",
        ],
      },

      "staking/phase1/staking-error-codes",
    ],
  },
  ContributorsSidebar: {
    Contributors: [
      "contributors/contribute-buildzil",
      "contributors/contribute-guidelines",
      "contributors/contribute-standards",
      "contributors/contribute-bug-bounty",
    ],

    "Core Protocol Design": [
      { "Design Overview": ["contributors/core-node-operation"] },
      {
        "Consensus Layer": [
          "contributors/core-consensus",
          "contributors/core-multisignatures",
        ],
      },
      {
        "Network Layer": [
          "contributors/core-gossip",
          "contributors/core-broadcasting",
          "contributors/core-blacklist",
          "contributors/core-messaging-limits",
        ],
      },
      {
        "Messaging Layer": [
          "contributors/core-message-dispatch",
          "contributors/core-message-queues",
        ],
      },
      {
        "Data Layer": [
          "contributors/core-accounts",
          "contributors/core-transaction-lifecycle",
          "contributors/core-incremental-db",
          "contributors/core-scilla-operation",
        ],
      },
      {
        "Directory Service": [
          "contributors/core-ds-mimo",
          "contributors/core-ds-reputation",
        ],
      },
      {
        Lookup: [
          "contributors/core-isolated-server",
          "contributors/core-websocket-server",
          "contributors/core-transaction-dispatch",
          "contributors/core-multipliers",
        ],
      },
      {
        Mining: [
          "contributors/core-pow",
          "contributors/core-difficulty-adjustment",
          "contributors/core-por",
          "contributors/core-coinbase",
          "contributors/core-global-gas-price",
        ],
      },
      {
        "Mitigation Measures": [
          "contributors/core-guard-mode",
          "contributors/core-rejoin-mechanism",
          "contributors/core-view-change",
          "contributors/core-diagnostic-data",
          "contributors/core-status-server",
        ],
      },
    ],
  },
};

module.exports = sidebars;
