// @ts-check
// Note: type annotations allow type checking and IDEs autocompletion

const lightCodeTheme = require('prism-react-renderer/themes/github');
const darkCodeTheme = require('prism-react-renderer/themes/dracula');

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: 'Zilliqa Developer Portal',
  tagline: 'Technical documentation for participating in the Zilliqa network.',
  url: 'https://dev.zilliqa.com',
  baseUrl:
    !process.env.BASE_URL || process.env.BASE_URL == ''
      ? '/'
      : process.env.BASE_URL,
  favicon: 'img/favicon.png',
  organizationName: 'Zilliqa', // Usually your GitHub org/user name.
  projectName: 'Developer Portal', // Usually your repo name.
  staticDirectories: ['docs/static/'],

  presets: [
    [
      'classic',
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        docs: {
          sidebarPath: require.resolve('./sidebars.js'),
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
          editUrl:
            'https://github.com/facebook/docusaurus/tree/main/packages/create-docusaurus/templates/shared/',
        },
        blog: {
          showReadingTime: true,
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
          editUrl:
            'https://github.com/facebook/docusaurus/tree/main/packages/create-docusaurus/templates/shared/',
        },
        theme: {
          customCss: require.resolve('./src/css/custom.css'),
        },
      }),
    ],
  ],

  themeConfig: {
    colorMode: {
      defaultMode: 'light',
      respectPrefersColorScheme: false,
    },
    navbar: {
      title: 'Zilliqa Developer Portal',
      logo: {
        alt: 'Zilliqa Logo',
        src: 'img/logo.png',
      },
      items: [
        {
          to: 'docs/basics/basics-intro-blockchain',
          activeBasePath: 'docs/basics',
          label: 'Basics',
          position: 'right',
        },
        {
          to: 'docs/dev-dapps/dev-started-introduction',
          activeBasePath: 'docs/dev',
          label: 'Developers',
          position: 'right',
        },
        {
          to: 'docs/apis/api-introduction',
          activeBasePath: 'docs/apis',
          label: 'APIs',
          position: 'right',
        },
        {
          to: 'docs/miners/mining-getting-started',
          activeBasePath: 'docs/miners',
          label: 'Miners',
          position: 'right',
        },
        {
          to: 'docs/exchanges/exchange-getting-started',
          activeBasePath: 'docs/exchanges',
          label: 'Exchanges',
          position: 'right',
        },
        {
          to: 'docs/staking/staking-overview',
          activeBasePath: 'docs/staking',
          label: 'Staking',
          position: 'right',
        },
        {
          to: 'docs/contributors/contribute-buildzil',
          activeBasePath: 'docs/contributors',
          label: 'Contributors',
          position: 'right',
        },
      ],
    },
    footer: {
      links: [
        {
          title: 'Developer Groups',
          items: [
            {
              label: 'Discord',
              href: 'https://discord.com/invite/XMRE9tt',
            },
            {
              label: 'Telegram',
              href: 'https://t.me/ZilliqaDevs',
            },
          ],
        },
        {
          title: 'Social',
          items: [
            {
              label: 'Twitter',
              href: 'https://twitter.com/zilliqa',
            },
            {
              label: 'Youtube',
              href: 'https://www.youtube.com/channel/UCvinnFbf0u71cajoxKcfZIQ',
            },
          ],
        },
        {
          title: 'Other Links',
          items: [
            {
              label: 'Blog',
              href: 'https://blog.zilliqa.com/',
            },
            {
              label: 'GitHub',
              href: 'https://github.com/zilliqa',
            },
          ],
        },
      ],
      copyright: `Copyright © 2022 Zilliqa Research Pte. Ltd.`,
    },
    // algolia: {
    //   apiKey: 'f1777493b2d9d1824a5daf3be87092db',
    //   indexName: 'zilliqa_developer',
    // },
  },
};

module.exports = config;
