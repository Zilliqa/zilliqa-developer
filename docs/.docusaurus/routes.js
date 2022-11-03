import React from 'react';
import ComponentCreator from '@docusaurus/ComponentCreator';

export default [
  {
    path: '/__docusaurus/debug',
    component: ComponentCreator('/__docusaurus/debug', 'b5e'),
    exact: true
  },
  {
    path: '/__docusaurus/debug/config',
    component: ComponentCreator('/__docusaurus/debug/config', '00e'),
    exact: true
  },
  {
    path: '/__docusaurus/debug/content',
    component: ComponentCreator('/__docusaurus/debug/content', '2da'),
    exact: true
  },
  {
    path: '/__docusaurus/debug/globalData',
    component: ComponentCreator('/__docusaurus/debug/globalData', '037'),
    exact: true
  },
  {
    path: '/__docusaurus/debug/metadata',
    component: ComponentCreator('/__docusaurus/debug/metadata', '85f'),
    exact: true
  },
  {
    path: '/__docusaurus/debug/registry',
    component: ComponentCreator('/__docusaurus/debug/registry', '65c'),
    exact: true
  },
  {
    path: '/__docusaurus/debug/routes',
    component: ComponentCreator('/__docusaurus/debug/routes', '75f'),
    exact: true
  },
  {
    path: '/components/cards_img01',
    component: ComponentCreator('/components/cards_img01', '352'),
    exact: true
  },
  {
    path: '/components/cards_img02',
    component: ComponentCreator('/components/cards_img02', '551'),
    exact: true
  },
  {
    path: '/components/cards_img03',
    component: ComponentCreator('/components/cards_img03', '148'),
    exact: true
  },
  {
    path: '/components/cards_img04',
    component: ComponentCreator('/components/cards_img04', '246'),
    exact: true
  },
  {
    path: '/components/cards_img05',
    component: ComponentCreator('/components/cards_img05', '867'),
    exact: true
  },
  {
    path: '/docs',
    component: ComponentCreator('/docs', '2f0'),
    routes: [
      {
        path: '/docs/apis/api-account-get-balance',
        component: ComponentCreator('/docs/apis/api-account-get-balance', '7ce'),
        exact: true,
        sidebar: "APIsSideBar"
      },
      {
        path: '/docs/apis/api-blockchain-ds-block-listing',
        component: ComponentCreator('/docs/apis/api-blockchain-ds-block-listing', '239'),
        exact: true,
        sidebar: "APIsSideBar"
      },
      {
        path: '/docs/apis/api-blockchain-get-blockchain-info',
        component: ComponentCreator('/docs/apis/api-blockchain-get-blockchain-info', '742'),
        exact: true,
        sidebar: "APIsSideBar"
      },
      {
        path: '/docs/apis/api-blockchain-get-current-ds-epoch',
        component: ComponentCreator('/docs/apis/api-blockchain-get-current-ds-epoch', 'ba0'),
        exact: true,
        sidebar: "APIsSideBar"
      },
      {
        path: '/docs/apis/api-blockchain-get-current-mini-epoch',
        component: ComponentCreator('/docs/apis/api-blockchain-get-current-mini-epoch', 'ad3'),
        exact: true,
        sidebar: "APIsSideBar"
      },
      {
        path: '/docs/apis/api-blockchain-get-ds-block',
        component: ComponentCreator('/docs/apis/api-blockchain-get-ds-block', '6ee'),
        exact: true,
        sidebar: "APIsSideBar"
      },
      {
        path: '/docs/apis/api-blockchain-get-ds-block-rate',
        component: ComponentCreator('/docs/apis/api-blockchain-get-ds-block-rate', '21a'),
        exact: true,
        sidebar: "APIsSideBar"
      },
      {
        path: '/docs/apis/api-blockchain-get-latest-ds-block',
        component: ComponentCreator('/docs/apis/api-blockchain-get-latest-ds-block', 'e5f'),
        exact: true,
        sidebar: "APIsSideBar"
      },
      {
        path: '/docs/apis/api-blockchain-get-latest-tx-block',
        component: ComponentCreator('/docs/apis/api-blockchain-get-latest-tx-block', 'ca9'),
        exact: true,
        sidebar: "APIsSideBar"
      },
      {
        path: '/docs/apis/api-blockchain-get-miner-info',
        component: ComponentCreator('/docs/apis/api-blockchain-get-miner-info', '137'),
        exact: true,
        sidebar: "APIsSideBar"
      },
      {
        path: '/docs/apis/api-blockchain-get-network-id',
        component: ComponentCreator('/docs/apis/api-blockchain-get-network-id', '4ff'),
        exact: true,
        sidebar: "APIsSideBar"
      },
      {
        path: '/docs/apis/api-blockchain-get-num-tx',
        component: ComponentCreator('/docs/apis/api-blockchain-get-num-tx', '550'),
        exact: true,
        sidebar: "APIsSideBar"
      },
      {
        path: '/docs/apis/api-blockchain-get-num-tx-blocks',
        component: ComponentCreator('/docs/apis/api-blockchain-get-num-tx-blocks', '88d'),
        exact: true,
        sidebar: "APIsSideBar"
      },
      {
        path: '/docs/apis/api-blockchain-get-prev-difficulty',
        component: ComponentCreator('/docs/apis/api-blockchain-get-prev-difficulty', 'f60'),
        exact: true,
        sidebar: "APIsSideBar"
      },
      {
        path: '/docs/apis/api-blockchain-get-prev-ds-difficulty',
        component: ComponentCreator('/docs/apis/api-blockchain-get-prev-ds-difficulty', '10e'),
        exact: true,
        sidebar: "APIsSideBar"
      },
      {
        path: '/docs/apis/api-blockchain-get-total-coin-supply',
        component: ComponentCreator('/docs/apis/api-blockchain-get-total-coin-supply', '3d3'),
        exact: true,
        sidebar: "APIsSideBar"
      },
      {
        path: '/docs/apis/api-blockchain-get-tx-block',
        component: ComponentCreator('/docs/apis/api-blockchain-get-tx-block', '881'),
        exact: true,
        sidebar: "APIsSideBar"
      },
      {
        path: '/docs/apis/api-blockchain-get-tx-block-rate',
        component: ComponentCreator('/docs/apis/api-blockchain-get-tx-block-rate', '8d1'),
        exact: true,
        sidebar: "APIsSideBar"
      },
      {
        path: '/docs/apis/api-blockchain-get-tx-rate',
        component: ComponentCreator('/docs/apis/api-blockchain-get-tx-rate', '21c'),
        exact: true,
        sidebar: "APIsSideBar"
      },
      {
        path: '/docs/apis/api-blockchain-num-ds-blocks',
        component: ComponentCreator('/docs/apis/api-blockchain-num-ds-blocks', 'afb'),
        exact: true,
        sidebar: "APIsSideBar"
      },
      {
        path: '/docs/apis/api-blockchain-tx-block-listing',
        component: ComponentCreator('/docs/apis/api-blockchain-tx-block-listing', 'e44'),
        exact: true,
        sidebar: "APIsSideBar"
      },
      {
        path: '/docs/apis/api-contract-get-contractaddress-from-txid',
        component: ComponentCreator('/docs/apis/api-contract-get-contractaddress-from-txid', '7d5'),
        exact: true,
        sidebar: "APIsSideBar"
      },
      {
        path: '/docs/apis/api-contract-get-smartcontract-code',
        component: ComponentCreator('/docs/apis/api-contract-get-smartcontract-code', '6fa'),
        exact: true,
        sidebar: "APIsSideBar"
      },
      {
        path: '/docs/apis/api-contract-get-smartcontract-init',
        component: ComponentCreator('/docs/apis/api-contract-get-smartcontract-init', 'aaa'),
        exact: true,
        sidebar: "APIsSideBar"
      },
      {
        path: '/docs/apis/api-contract-get-smartcontract-state',
        component: ComponentCreator('/docs/apis/api-contract-get-smartcontract-state', '978'),
        exact: true,
        sidebar: "APIsSideBar"
      },
      {
        path: '/docs/apis/api-contract-get-smartcontract-substate',
        component: ComponentCreator('/docs/apis/api-contract-get-smartcontract-substate', 'cbf'),
        exact: true,
        sidebar: "APIsSideBar"
      },
      {
        path: '/docs/apis/api-contract-get-smartcontracts',
        component: ComponentCreator('/docs/apis/api-contract-get-smartcontracts', '448'),
        exact: true,
        sidebar: "APIsSideBar"
      },
      {
        path: '/docs/apis/api-contract-get-state-proof',
        component: ComponentCreator('/docs/apis/api-contract-get-state-proof', '64c'),
        exact: true,
        sidebar: "APIsSideBar"
      },
      {
        path: '/docs/apis/api-introduction',
        component: ComponentCreator('/docs/apis/api-introduction', 'dd9'),
        exact: true,
        sidebar: "APIsSideBar"
      },
      {
        path: '/docs/apis/api-transaction-create-tx',
        component: ComponentCreator('/docs/apis/api-transaction-create-tx', 'dcb'),
        exact: true,
        sidebar: "APIsSideBar"
      },
      {
        path: '/docs/apis/api-transaction-get-minimum-gas-price',
        component: ComponentCreator('/docs/apis/api-transaction-get-minimum-gas-price', '919'),
        exact: true,
        sidebar: "APIsSideBar"
      },
      {
        path: '/docs/apis/api-transaction-get-num-txns-dsepoch',
        component: ComponentCreator('/docs/apis/api-transaction-get-num-txns-dsepoch', '5e3'),
        exact: true,
        sidebar: "APIsSideBar"
      },
      {
        path: '/docs/apis/api-transaction-get-num-txns-txepoch',
        component: ComponentCreator('/docs/apis/api-transaction-get-num-txns-txepoch', '979'),
        exact: true,
        sidebar: "APIsSideBar"
      },
      {
        path: '/docs/apis/api-transaction-get-pending-tx',
        component: ComponentCreator('/docs/apis/api-transaction-get-pending-tx', '025'),
        exact: true
      },
      {
        path: '/docs/apis/api-transaction-get-pending-txs',
        component: ComponentCreator('/docs/apis/api-transaction-get-pending-txs', 'efb'),
        exact: true
      },
      {
        path: '/docs/apis/api-transaction-get-recent-txs',
        component: ComponentCreator('/docs/apis/api-transaction-get-recent-txs', 'a00'),
        exact: true,
        sidebar: "APIsSideBar"
      },
      {
        path: '/docs/apis/api-transaction-get-transaction-status',
        component: ComponentCreator('/docs/apis/api-transaction-get-transaction-status', '435'),
        exact: true,
        sidebar: "APIsSideBar"
      },
      {
        path: '/docs/apis/api-transaction-get-tx',
        component: ComponentCreator('/docs/apis/api-transaction-get-tx', '8ea'),
        exact: true,
        sidebar: "APIsSideBar"
      },
      {
        path: '/docs/apis/api-transaction-get-txbodies-for-txblock',
        component: ComponentCreator('/docs/apis/api-transaction-get-txbodies-for-txblock', '515'),
        exact: true,
        sidebar: "APIsSideBar"
      },
      {
        path: '/docs/apis/api-transaction-get-txbodies-for-txblock-ex',
        component: ComponentCreator('/docs/apis/api-transaction-get-txbodies-for-txblock-ex', '01c'),
        exact: true,
        sidebar: "APIsSideBar"
      },
      {
        path: '/docs/apis/api-transaction-get-txs-for-txblock',
        component: ComponentCreator('/docs/apis/api-transaction-get-txs-for-txblock', 'b91'),
        exact: true,
        sidebar: "APIsSideBar"
      },
      {
        path: '/docs/apis/api-transaction-get-txs-for-txblock-ex',
        component: ComponentCreator('/docs/apis/api-transaction-get-txs-for-txblock-ex', 'a7a'),
        exact: true,
        sidebar: "APIsSideBar"
      },
      {
        path: '/docs/basics/basics-intro-accounts',
        component: ComponentCreator('/docs/basics/basics-intro-accounts', '6c5'),
        exact: true,
        sidebar: "BasicsSideBar"
      },
      {
        path: '/docs/basics/basics-intro-blockchain',
        component: ComponentCreator('/docs/basics/basics-intro-blockchain', 'c12'),
        exact: true,
        sidebar: "BasicsSideBar"
      },
      {
        path: '/docs/basics/basics-intro-consensus',
        component: ComponentCreator('/docs/basics/basics-intro-consensus', '833'),
        exact: true,
        sidebar: "BasicsSideBar"
      },
      {
        path: '/docs/basics/basics-intro-gas',
        component: ComponentCreator('/docs/basics/basics-intro-gas', '0aa'),
        exact: true,
        sidebar: "BasicsSideBar"
      },
      {
        path: '/docs/basics/basics-intro-txns',
        component: ComponentCreator('/docs/basics/basics-intro-txns', '94c'),
        exact: true,
        sidebar: "BasicsSideBar"
      },
      {
        path: '/docs/basics/basics-zil-consensus',
        component: ComponentCreator('/docs/basics/basics-zil-consensus', '617'),
        exact: true,
        sidebar: "BasicsSideBar"
      },
      {
        path: '/docs/basics/basics-zil-contract',
        component: ComponentCreator('/docs/basics/basics-zil-contract', '401'),
        exact: true,
        sidebar: "BasicsSideBar"
      },
      {
        path: '/docs/basics/basics-zil-gas',
        component: ComponentCreator('/docs/basics/basics-zil-gas', '856'),
        exact: true,
        sidebar: "BasicsSideBar"
      },
      {
        path: '/docs/basics/basics-zil-nodes',
        component: ComponentCreator('/docs/basics/basics-zil-nodes', '749'),
        exact: true,
        sidebar: "BasicsSideBar"
      },
      {
        path: '/docs/basics/basics-zil-reward',
        component: ComponentCreator('/docs/basics/basics-zil-reward', '5ca'),
        exact: true,
        sidebar: "BasicsSideBar"
      },
      {
        path: '/docs/basics/basics-zil-schnorr-signatures',
        component: ComponentCreator('/docs/basics/basics-zil-schnorr-signatures', '707'),
        exact: true,
        sidebar: "BasicsSideBar"
      },
      {
        path: '/docs/basics/basics-zil-sharding',
        component: ComponentCreator('/docs/basics/basics-zil-sharding', 'bcf'),
        exact: true,
        sidebar: "BasicsSideBar"
      },
      {
        path: '/docs/contributors/contribute-bug-bounty',
        component: ComponentCreator('/docs/contributors/contribute-bug-bounty', '916'),
        exact: true,
        sidebar: "ContributorsSidebar"
      },
      {
        path: '/docs/contributors/contribute-buildzil',
        component: ComponentCreator('/docs/contributors/contribute-buildzil', 'a5a'),
        exact: true,
        sidebar: "ContributorsSidebar"
      },
      {
        path: '/docs/contributors/contribute-guidelines',
        component: ComponentCreator('/docs/contributors/contribute-guidelines', 'b00'),
        exact: true,
        sidebar: "ContributorsSidebar"
      },
      {
        path: '/docs/contributors/contribute-standards',
        component: ComponentCreator('/docs/contributors/contribute-standards', 'a9a'),
        exact: true,
        sidebar: "ContributorsSidebar"
      },
      {
        path: '/docs/contributors/core-accounts',
        component: ComponentCreator('/docs/contributors/core-accounts', 'dcb'),
        exact: true,
        sidebar: "ContributorsSidebar"
      },
      {
        path: '/docs/contributors/core-blacklist',
        component: ComponentCreator('/docs/contributors/core-blacklist', '78f'),
        exact: true,
        sidebar: "ContributorsSidebar"
      },
      {
        path: '/docs/contributors/core-broadcasting',
        component: ComponentCreator('/docs/contributors/core-broadcasting', 'e29'),
        exact: true,
        sidebar: "ContributorsSidebar"
      },
      {
        path: '/docs/contributors/core-coinbase',
        component: ComponentCreator('/docs/contributors/core-coinbase', '1f5'),
        exact: true,
        sidebar: "ContributorsSidebar"
      },
      {
        path: '/docs/contributors/core-consensus',
        component: ComponentCreator('/docs/contributors/core-consensus', 'f31'),
        exact: true,
        sidebar: "ContributorsSidebar"
      },
      {
        path: '/docs/contributors/core-diagnostic-data',
        component: ComponentCreator('/docs/contributors/core-diagnostic-data', 'b42'),
        exact: true,
        sidebar: "ContributorsSidebar"
      },
      {
        path: '/docs/contributors/core-difficulty-adjustment',
        component: ComponentCreator('/docs/contributors/core-difficulty-adjustment', 'dd1'),
        exact: true,
        sidebar: "ContributorsSidebar"
      },
      {
        path: '/docs/contributors/core-ds-mimo',
        component: ComponentCreator('/docs/contributors/core-ds-mimo', 'e34'),
        exact: true,
        sidebar: "ContributorsSidebar"
      },
      {
        path: '/docs/contributors/core-ds-reputation',
        component: ComponentCreator('/docs/contributors/core-ds-reputation', 'f39'),
        exact: true,
        sidebar: "ContributorsSidebar"
      },
      {
        path: '/docs/contributors/core-global-gas-price',
        component: ComponentCreator('/docs/contributors/core-global-gas-price', 'c80'),
        exact: true,
        sidebar: "ContributorsSidebar"
      },
      {
        path: '/docs/contributors/core-gossip',
        component: ComponentCreator('/docs/contributors/core-gossip', 'e60'),
        exact: true,
        sidebar: "ContributorsSidebar"
      },
      {
        path: '/docs/contributors/core-guard-mode',
        component: ComponentCreator('/docs/contributors/core-guard-mode', '1e3'),
        exact: true,
        sidebar: "ContributorsSidebar"
      },
      {
        path: '/docs/contributors/core-incremental-db',
        component: ComponentCreator('/docs/contributors/core-incremental-db', '7d7'),
        exact: true,
        sidebar: "ContributorsSidebar"
      },
      {
        path: '/docs/contributors/core-intro',
        component: ComponentCreator('/docs/contributors/core-intro', '642'),
        exact: true
      },
      {
        path: '/docs/contributors/core-isolated-server',
        component: ComponentCreator('/docs/contributors/core-isolated-server', 'd2e'),
        exact: true,
        sidebar: "ContributorsSidebar"
      },
      {
        path: '/docs/contributors/core-lookup',
        component: ComponentCreator('/docs/contributors/core-lookup', 'b31'),
        exact: true
      },
      {
        path: '/docs/contributors/core-message-dispatch',
        component: ComponentCreator('/docs/contributors/core-message-dispatch', '548'),
        exact: true,
        sidebar: "ContributorsSidebar"
      },
      {
        path: '/docs/contributors/core-message-queues',
        component: ComponentCreator('/docs/contributors/core-message-queues', '357'),
        exact: true,
        sidebar: "ContributorsSidebar"
      },
      {
        path: '/docs/contributors/core-messaging-limits',
        component: ComponentCreator('/docs/contributors/core-messaging-limits', 'eaa'),
        exact: true,
        sidebar: "ContributorsSidebar"
      },
      {
        path: '/docs/contributors/core-multipliers',
        component: ComponentCreator('/docs/contributors/core-multipliers', '04a'),
        exact: true,
        sidebar: "ContributorsSidebar"
      },
      {
        path: '/docs/contributors/core-multisignatures',
        component: ComponentCreator('/docs/contributors/core-multisignatures', '060'),
        exact: true,
        sidebar: "ContributorsSidebar"
      },
      {
        path: '/docs/contributors/core-node-operation',
        component: ComponentCreator('/docs/contributors/core-node-operation', '6e5'),
        exact: true,
        sidebar: "ContributorsSidebar"
      },
      {
        path: '/docs/contributors/core-por',
        component: ComponentCreator('/docs/contributors/core-por', '999'),
        exact: true,
        sidebar: "ContributorsSidebar"
      },
      {
        path: '/docs/contributors/core-pow',
        component: ComponentCreator('/docs/contributors/core-pow', 'd1d'),
        exact: true,
        sidebar: "ContributorsSidebar"
      },
      {
        path: '/docs/contributors/core-rejoin-mechanism',
        component: ComponentCreator('/docs/contributors/core-rejoin-mechanism', '35f'),
        exact: true,
        sidebar: "ContributorsSidebar"
      },
      {
        path: '/docs/contributors/core-schnorr',
        component: ComponentCreator('/docs/contributors/core-schnorr', '452'),
        exact: true
      },
      {
        path: '/docs/contributors/core-scilla-operation',
        component: ComponentCreator('/docs/contributors/core-scilla-operation', '3ae'),
        exact: true,
        sidebar: "ContributorsSidebar"
      },
      {
        path: '/docs/contributors/core-status-server',
        component: ComponentCreator('/docs/contributors/core-status-server', '3c2'),
        exact: true,
        sidebar: "ContributorsSidebar"
      },
      {
        path: '/docs/contributors/core-transaction-dispatch',
        component: ComponentCreator('/docs/contributors/core-transaction-dispatch', '3ef'),
        exact: true,
        sidebar: "ContributorsSidebar"
      },
      {
        path: '/docs/contributors/core-transaction-lifecycle',
        component: ComponentCreator('/docs/contributors/core-transaction-lifecycle', 'cc7'),
        exact: true,
        sidebar: "ContributorsSidebar"
      },
      {
        path: '/docs/contributors/core-view-change',
        component: ComponentCreator('/docs/contributors/core-view-change', 'bdc'),
        exact: true,
        sidebar: "ContributorsSidebar"
      },
      {
        path: '/docs/contributors/core-websocket-server',
        component: ComponentCreator('/docs/contributors/core-websocket-server', '2d9'),
        exact: true,
        sidebar: "ContributorsSidebar"
      },
      {
        path: '/docs/dev-dapps/dev-education-videos',
        component: ComponentCreator('/docs/dev-dapps/dev-education-videos', '655'),
        exact: true,
        sidebar: "DevelopersSidebar"
      },
      {
        path: '/docs/dev-dapps/dev-keys-introduction',
        component: ComponentCreator('/docs/dev-dapps/dev-keys-introduction', '2ac'),
        exact: true,
        sidebar: "DevelopersSidebar"
      },
      {
        path: '/docs/dev-dapps/dev-keys-ledger',
        component: ComponentCreator('/docs/dev-dapps/dev-keys-ledger', '1b5'),
        exact: true
      },
      {
        path: '/docs/dev-dapps/dev-keys-magic',
        component: ComponentCreator('/docs/dev-dapps/dev-keys-magic', 'c74'),
        exact: true,
        sidebar: "DevelopersSidebar"
      },
      {
        path: '/docs/dev-dapps/dev-keys-moonlet',
        component: ComponentCreator('/docs/dev-dapps/dev-keys-moonlet', '181'),
        exact: true
      },
      {
        path: '/docs/dev-dapps/dev-keys-pkey',
        component: ComponentCreator('/docs/dev-dapps/dev-keys-pkey', 'ed2'),
        exact: true,
        sidebar: "DevelopersSidebar"
      },
      {
        path: '/docs/dev-dapps/dev-keys-zilpay',
        component: ComponentCreator('/docs/dev-dapps/dev-keys-zilpay', '33d'),
        exact: true,
        sidebar: "DevelopersSidebar"
      },
      {
        path: '/docs/dev-dapps/dev-keys-zrc2-wallet-support',
        component: ComponentCreator('/docs/dev-dapps/dev-keys-zrc2-wallet-support', '659'),
        exact: true,
        sidebar: "DevelopersSidebar"
      },
      {
        path: '/docs/dev-dapps/dev-rentonzilliqa-components',
        component: ComponentCreator('/docs/dev-dapps/dev-rentonzilliqa-components', 'e54'),
        exact: true,
        sidebar: "DevelopersSidebar"
      },
      {
        path: '/docs/dev-dapps/dev-rentonzilliqa-contract',
        component: ComponentCreator('/docs/dev-dapps/dev-rentonzilliqa-contract', 'd39'),
        exact: true,
        sidebar: "DevelopersSidebar"
      },
      {
        path: '/docs/dev-dapps/dev-rentonzilliqa-frontend',
        component: ComponentCreator('/docs/dev-dapps/dev-rentonzilliqa-frontend', '2aa'),
        exact: true,
        sidebar: "DevelopersSidebar"
      },
      {
        path: '/docs/dev-dapps/dev-rentonzilliqa-introduction',
        component: ComponentCreator('/docs/dev-dapps/dev-rentonzilliqa-introduction', '8b7'),
        exact: true,
        sidebar: "DevelopersSidebar"
      },
      {
        path: '/docs/dev-dapps/dev-rentonzilliqa-library',
        component: ComponentCreator('/docs/dev-dapps/dev-rentonzilliqa-library', 'ec3'),
        exact: true,
        sidebar: "DevelopersSidebar"
      },
      {
        path: '/docs/dev-dapps/dev-rentonzilliqa-modals',
        component: ComponentCreator('/docs/dev-dapps/dev-rentonzilliqa-modals', 'e94'),
        exact: true,
        sidebar: "DevelopersSidebar"
      },
      {
        path: '/docs/dev-dapps/dev-rentonzilliqa-mutable-variables',
        component: ComponentCreator('/docs/dev-dapps/dev-rentonzilliqa-mutable-variables', '726'),
        exact: true,
        sidebar: "DevelopersSidebar"
      },
      {
        path: '/docs/dev-dapps/dev-rentonzilliqa-pages',
        component: ComponentCreator('/docs/dev-dapps/dev-rentonzilliqa-pages', 'da1'),
        exact: true,
        sidebar: "DevelopersSidebar"
      },
      {
        path: '/docs/dev-dapps/dev-rentonzilliqa-procedures',
        component: ComponentCreator('/docs/dev-dapps/dev-rentonzilliqa-procedures', 'e1a'),
        exact: true,
        sidebar: "DevelopersSidebar"
      },
      {
        path: '/docs/dev-dapps/dev-rentonzilliqa-scripting',
        component: ComponentCreator('/docs/dev-dapps/dev-rentonzilliqa-scripting', 'caf'),
        exact: true,
        sidebar: "DevelopersSidebar"
      },
      {
        path: '/docs/dev-dapps/dev-rentonzilliqa-transitions',
        component: ComponentCreator('/docs/dev-dapps/dev-rentonzilliqa-transitions', 'c01'),
        exact: true,
        sidebar: "DevelopersSidebar"
      },
      {
        path: '/docs/dev-dapps/dev-section-navigation',
        component: ComponentCreator('/docs/dev-dapps/dev-section-navigation', '6d0'),
        exact: true
      },
      {
        path: '/docs/dev-dapps/dev-started-env',
        component: ComponentCreator('/docs/dev-dapps/dev-started-env', 'd57'),
        exact: true,
        sidebar: "DevelopersSidebar"
      },
      {
        path: '/docs/dev-dapps/dev-started-helloworld',
        component: ComponentCreator('/docs/dev-dapps/dev-started-helloworld', '9af'),
        exact: true,
        sidebar: "DevelopersSidebar"
      },
      {
        path: '/docs/dev-dapps/dev-started-introduction',
        component: ComponentCreator('/docs/dev-dapps/dev-started-introduction', 'f58'),
        exact: true,
        sidebar: "DevelopersSidebar"
      },
      {
        path: '/docs/dev-dapps/dev-tools-ceres',
        component: ComponentCreator('/docs/dev-dapps/dev-tools-ceres', '1f7'),
        exact: true,
        sidebar: "DevelopersSidebar"
      },
      {
        path: '/docs/dev-dapps/dev-tools-cli',
        component: ComponentCreator('/docs/dev-dapps/dev-tools-cli', '10c'),
        exact: true,
        sidebar: "DevelopersSidebar"
      },
      {
        path: '/docs/dev-dapps/dev-tools-explorer',
        component: ComponentCreator('/docs/dev-dapps/dev-tools-explorer', 'dac'),
        exact: true,
        sidebar: "DevelopersSidebar"
      },
      {
        path: '/docs/dev-dapps/dev-tools-faucet',
        component: ComponentCreator('/docs/dev-dapps/dev-tools-faucet', 'deb'),
        exact: true,
        sidebar: "DevelopersSidebar"
      },
      {
        path: '/docs/dev-dapps/dev-tools-gozilliqa',
        component: ComponentCreator('/docs/dev-dapps/dev-tools-gozilliqa', '825'),
        exact: true,
        sidebar: "DevelopersSidebar"
      },
      {
        path: '/docs/dev-dapps/dev-tools-ide',
        component: ComponentCreator('/docs/dev-dapps/dev-tools-ide', '8fe'),
        exact: true,
        sidebar: "DevelopersSidebar"
      },
      {
        path: '/docs/dev-dapps/dev-tools-java',
        component: ComponentCreator('/docs/dev-dapps/dev-tools-java', 'd70'),
        exact: true,
        sidebar: "DevelopersSidebar"
      },
      {
        path: '/docs/dev-dapps/dev-tools-sdks',
        component: ComponentCreator('/docs/dev-dapps/dev-tools-sdks', 'f64'),
        exact: true,
        sidebar: "DevelopersSidebar"
      },
      {
        path: '/docs/dev-dapps/dev-tools-websockets',
        component: ComponentCreator('/docs/dev-dapps/dev-tools-websockets', '27e'),
        exact: true,
        sidebar: "DevelopersSidebar"
      },
      {
        path: '/docs/dev-dapps/dev-tools-zilliqajs',
        component: ComponentCreator('/docs/dev-dapps/dev-tools-zilliqajs', '6e7'),
        exact: true,
        sidebar: "DevelopersSidebar"
      },
      {
        path: '/docs/dev-dapps/dev-txn-broadcasting',
        component: ComponentCreator('/docs/dev-dapps/dev-txn-broadcasting', 'ec1'),
        exact: true,
        sidebar: "DevelopersSidebar"
      },
      {
        path: '/docs/dev-dapps/dev-txn-confirmation',
        component: ComponentCreator('/docs/dev-dapps/dev-txn-confirmation', 'f95'),
        exact: true,
        sidebar: "DevelopersSidebar"
      },
      {
        path: '/docs/dev-dapps/dev-txn-polling',
        component: ComponentCreator('/docs/dev-dapps/dev-txn-polling', 'a3d'),
        exact: true,
        sidebar: "DevelopersSidebar"
      },
      {
        path: '/docs/dev-dapps/dev-txn-receipt',
        component: ComponentCreator('/docs/dev-dapps/dev-txn-receipt', 'e0b'),
        exact: true,
        sidebar: "DevelopersSidebar"
      },
      {
        path: '/docs/dev-dapps/dev-txn-signing',
        component: ComponentCreator('/docs/dev-dapps/dev-txn-signing', '368'),
        exact: true,
        sidebar: "DevelopersSidebar"
      },
      {
        path: '/docs/dev-dapps/dev-upgrade-v8',
        component: ComponentCreator('/docs/dev-dapps/dev-upgrade-v8', '6bd'),
        exact: true,
        sidebar: "DevelopersSidebar"
      },
      {
        path: '/docs/dev-dapps/dev-wrapped-zil',
        component: ComponentCreator('/docs/dev-dapps/dev-wrapped-zil', '956'),
        exact: true,
        sidebar: "DevelopersSidebar"
      },
      {
        path: '/docs/exchanges/exchange-account-management',
        component: ComponentCreator('/docs/exchanges/exchange-account-management', '490'),
        exact: true,
        sidebar: "ExchangesSidebar"
      },
      {
        path: '/docs/exchanges/exchange-getting-started',
        component: ComponentCreator('/docs/exchanges/exchange-getting-started', '5cc'),
        exact: true,
        sidebar: "ExchangesSidebar"
      },
      {
        path: '/docs/exchanges/exchange-ip-whitelisting',
        component: ComponentCreator('/docs/exchanges/exchange-ip-whitelisting', 'c36'),
        exact: true,
        sidebar: "ExchangesSidebar"
      },
      {
        path: '/docs/exchanges/exchange-key-whitelisting-1',
        component: ComponentCreator('/docs/exchanges/exchange-key-whitelisting-1', '334'),
        exact: true,
        sidebar: "ExchangesSidebar"
      },
      {
        path: '/docs/exchanges/exchange-key-whitelisting-2',
        component: ComponentCreator('/docs/exchanges/exchange-key-whitelisting-2', '374'),
        exact: true,
        sidebar: "ExchangesSidebar"
      },
      {
        path: '/docs/exchanges/exchange-managing-zrc2-tokens',
        component: ComponentCreator('/docs/exchanges/exchange-managing-zrc2-tokens', 'a79'),
        exact: true,
        sidebar: "ExchangesSidebar"
      },
      {
        path: '/docs/exchanges/exchange-sending-transactions',
        component: ComponentCreator('/docs/exchanges/exchange-sending-transactions', 'f6a'),
        exact: true,
        sidebar: "ExchangesSidebar"
      },
      {
        path: '/docs/exchanges/exchange-tracking-deposits',
        component: ComponentCreator('/docs/exchanges/exchange-tracking-deposits', '8d5'),
        exact: true,
        sidebar: "ExchangesSidebar"
      },
      {
        path: '/docs/exchanges/exchange-transaction-receipts',
        component: ComponentCreator('/docs/exchanges/exchange-transaction-receipts', 'bb8'),
        exact: true,
        sidebar: "ExchangesSidebar"
      },
      {
        path: '/docs/exchanges/rosetta-construction',
        component: ComponentCreator('/docs/exchanges/rosetta-construction', '1e5'),
        exact: true,
        sidebar: "ExchangesSidebar"
      },
      {
        path: '/docs/exchanges/rosetta-construction-combine',
        component: ComponentCreator('/docs/exchanges/rosetta-construction-combine', '56d'),
        exact: true,
        sidebar: "ExchangesSidebar"
      },
      {
        path: '/docs/exchanges/rosetta-construction-derive',
        component: ComponentCreator('/docs/exchanges/rosetta-construction-derive', '8c2'),
        exact: true,
        sidebar: "ExchangesSidebar"
      },
      {
        path: '/docs/exchanges/rosetta-construction-hash',
        component: ComponentCreator('/docs/exchanges/rosetta-construction-hash', '943'),
        exact: true,
        sidebar: "ExchangesSidebar"
      },
      {
        path: '/docs/exchanges/rosetta-construction-mempool-transaction',
        component: ComponentCreator('/docs/exchanges/rosetta-construction-mempool-transaction', 'f16'),
        exact: true
      },
      {
        path: '/docs/exchanges/rosetta-construction-metadata',
        component: ComponentCreator('/docs/exchanges/rosetta-construction-metadata', '5ea'),
        exact: true,
        sidebar: "ExchangesSidebar"
      },
      {
        path: '/docs/exchanges/rosetta-construction-parse',
        component: ComponentCreator('/docs/exchanges/rosetta-construction-parse', '051'),
        exact: true,
        sidebar: "ExchangesSidebar"
      },
      {
        path: '/docs/exchanges/rosetta-construction-payloads',
        component: ComponentCreator('/docs/exchanges/rosetta-construction-payloads', '5be'),
        exact: true,
        sidebar: "ExchangesSidebar"
      },
      {
        path: '/docs/exchanges/rosetta-construction-preprocess',
        component: ComponentCreator('/docs/exchanges/rosetta-construction-preprocess', '220'),
        exact: true,
        sidebar: "ExchangesSidebar"
      },
      {
        path: '/docs/exchanges/rosetta-construction-submit',
        component: ComponentCreator('/docs/exchanges/rosetta-construction-submit', '99b'),
        exact: true,
        sidebar: "ExchangesSidebar"
      },
      {
        path: '/docs/exchanges/rosetta-data-account-balance',
        component: ComponentCreator('/docs/exchanges/rosetta-data-account-balance', '524'),
        exact: true,
        sidebar: "ExchangesSidebar"
      },
      {
        path: '/docs/exchanges/rosetta-data-block',
        component: ComponentCreator('/docs/exchanges/rosetta-data-block', '3a8'),
        exact: true,
        sidebar: "ExchangesSidebar"
      },
      {
        path: '/docs/exchanges/rosetta-data-block-transaction',
        component: ComponentCreator('/docs/exchanges/rosetta-data-block-transaction', '7ae'),
        exact: true,
        sidebar: "ExchangesSidebar"
      },
      {
        path: '/docs/exchanges/rosetta-data-network-list',
        component: ComponentCreator('/docs/exchanges/rosetta-data-network-list', '7fa'),
        exact: true,
        sidebar: "ExchangesSidebar"
      },
      {
        path: '/docs/exchanges/rosetta-data-network-options',
        component: ComponentCreator('/docs/exchanges/rosetta-data-network-options', 'a4e'),
        exact: true,
        sidebar: "ExchangesSidebar"
      },
      {
        path: '/docs/exchanges/rosetta-data-network-status',
        component: ComponentCreator('/docs/exchanges/rosetta-data-network-status', '50c'),
        exact: true,
        sidebar: "ExchangesSidebar"
      },
      {
        path: '/docs/exchanges/rosetta-introduction',
        component: ComponentCreator('/docs/exchanges/rosetta-introduction', '130'),
        exact: true,
        sidebar: "ExchangesSidebar"
      },
      {
        path: '/docs/exchanges/rosetta-setting-up-no-seed-node',
        component: ComponentCreator('/docs/exchanges/rosetta-setting-up-no-seed-node', '4ab'),
        exact: true,
        sidebar: "ExchangesSidebar"
      },
      {
        path: '/docs/exchanges/rosetta-setting-up-seed-node',
        component: ComponentCreator('/docs/exchanges/rosetta-setting-up-seed-node', '72b'),
        exact: true,
        sidebar: "ExchangesSidebar"
      },
      {
        path: '/docs/exchanges/rosetta-unsupported-api',
        component: ComponentCreator('/docs/exchanges/rosetta-unsupported-api', '050'),
        exact: true,
        sidebar: "ExchangesSidebar"
      },
      {
        path: '/docs/miners/mining-additional-info',
        component: ComponentCreator('/docs/miners/mining-additional-info', '4e7'),
        exact: true,
        sidebar: "MinersSidebar"
      },
      {
        path: '/docs/miners/mining-getting-started',
        component: ComponentCreator('/docs/miners/mining-getting-started', '4bf'),
        exact: true,
        sidebar: "MinersSidebar"
      },
      {
        path: '/docs/miners/mining-proxy',
        component: ComponentCreator('/docs/miners/mining-proxy', 'd5a'),
        exact: true,
        sidebar: "MinersSidebar"
      },
      {
        path: '/docs/miners/mining-zilclient',
        component: ComponentCreator('/docs/miners/mining-zilclient', 'ed7'),
        exact: true,
        sidebar: "MinersSidebar"
      },
      {
        path: '/docs/miners/mining-zilminer',
        component: ComponentCreator('/docs/miners/mining-zilminer', '692'),
        exact: true,
        sidebar: "MinersSidebar"
      },
      {
        path: '/docs/quickstart/projects-using-zilliqa',
        component: ComponentCreator('/docs/quickstart/projects-using-zilliqa', '2bf'),
        exact: true
      },
      {
        path: '/docs/staking/phase1/delegator/staking-delegator-gzil',
        component: ComponentCreator('/docs/staking/phase1/delegator/staking-delegator-gzil', '656'),
        exact: true,
        sidebar: "StakingSidebar"
      },
      {
        path: '/docs/staking/phase1/delegator/staking-delegator-operations',
        component: ComponentCreator('/docs/staking/phase1/delegator/staking-delegator-operations', '60b'),
        exact: true,
        sidebar: "StakingSidebar"
      },
      {
        path: '/docs/staking/phase1/delegator/staking-delegator-overview',
        component: ComponentCreator('/docs/staking/phase1/delegator/staking-delegator-overview', '9dd'),
        exact: true,
        sidebar: "StakingSidebar"
      },
      {
        path: '/docs/staking/phase1/delegator/staking-delegator-reading-contract-states',
        component: ComponentCreator('/docs/staking/phase1/delegator/staking-delegator-reading-contract-states', '854'),
        exact: true,
        sidebar: "StakingSidebar"
      },
      {
        path: '/docs/staking/phase1/ssn-operator/staking-commission-management',
        component: ComponentCreator('/docs/staking/phase1/ssn-operator/staking-commission-management', '39f'),
        exact: true,
        sidebar: "StakingSidebar"
      },
      {
        path: '/docs/staking/phase1/ssn-operator/staking-ssn-before-you-start',
        component: ComponentCreator('/docs/staking/phase1/ssn-operator/staking-ssn-before-you-start', '875'),
        exact: true,
        sidebar: "StakingSidebar"
      },
      {
        path: '/docs/staking/phase1/ssn-operator/staking-ssn-enrollment',
        component: ComponentCreator('/docs/staking/phase1/ssn-operator/staking-ssn-enrollment', '516'),
        exact: true,
        sidebar: "StakingSidebar"
      },
      {
        path: '/docs/staking/phase1/ssn-operator/staking-ssn-maintenance',
        component: ComponentCreator('/docs/staking/phase1/ssn-operator/staking-ssn-maintenance', '14d'),
        exact: true,
        sidebar: "StakingSidebar"
      },
      {
        path: '/docs/staking/phase1/ssn-operator/staking-ssn-setup',
        component: ComponentCreator('/docs/staking/phase1/ssn-operator/staking-ssn-setup', '672'),
        exact: true,
        sidebar: "StakingSidebar"
      },
      {
        path: '/docs/staking/phase1/ssn-operator/staking-ssn-upgrading',
        component: ComponentCreator('/docs/staking/phase1/ssn-operator/staking-ssn-upgrading', '969'),
        exact: true,
        sidebar: "StakingSidebar"
      },
      {
        path: '/docs/staking/phase1/staking-error-codes',
        component: ComponentCreator('/docs/staking/phase1/staking-error-codes', 'da1'),
        exact: true,
        sidebar: "StakingSidebar"
      },
      {
        path: '/docs/staking/phase1/staking-general-information',
        component: ComponentCreator('/docs/staking/phase1/staking-general-information', '440'),
        exact: true,
        sidebar: "StakingSidebar"
      },
      {
        path: '/docs/staking/phase1/staking-phase1-overview',
        component: ComponentCreator('/docs/staking/phase1/staking-phase1-overview', '82f'),
        exact: true,
        sidebar: "StakingSidebar"
      },
      {
        path: '/docs/staking/phase1/staking-phase11-notice',
        component: ComponentCreator('/docs/staking/phase1/staking-phase11-notice', 'dbc'),
        exact: true,
        sidebar: "StakingSidebar"
      },
      {
        path: '/docs/staking/staking-disclaimer',
        component: ComponentCreator('/docs/staking/staking-disclaimer', '8db'),
        exact: true,
        sidebar: "StakingSidebar"
      },
      {
        path: '/docs/staking/staking-overview',
        component: ComponentCreator('/docs/staking/staking-overview', '26f'),
        exact: true,
        sidebar: "StakingSidebar"
      }
    ]
  },
  {
    path: '/',
    component: ComponentCreator('/', '37d'),
    exact: true
  },
  {
    path: '*',
    component: ComponentCreator('*'),
  },
];
