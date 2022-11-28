// @ts-check
// Note: type annotations allow type checking and IDEs autocompletion

const lightCodeTheme = require("prism-react-renderer/themes/github");
const darkCodeTheme = require("prism-react-renderer/themes/dracula");

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: "Zilliqa Developer Portal",
  tagline: "Technical documentation for participating in the Zilliqa network.",
  url: "https://dev.zilliqa.com",
  baseUrl:
    !process.env.BASE_URL || process.env.BASE_URL == ""
      ? "/"
      : process.env.BASE_URL,
  onBrokenLinks: "warn", // TODO: "throw",
  onBrokenMarkdownLinks: "warn",
  favicon: "img/favicon.ico",
  staticDirectories: ["assets", "static"],

  // GitHub pages deployment config.
  // If you aren't using GitHub pages, you don't need these.
  organizationName: "zilliqa", // Usually your GitHub org/user name.
  projectName: "zilliqa-developer", // Usually your repo name.

  // Even if you don't use internalization, you can use this field to set useful
  // metadata like html lang. For example, if your site is Chinese, you may want
  // to replace "en" with "zh-Hans".
  i18n: {
    defaultLocale: "en",
    locales: ["en"],
  },

  presets: [
    [
      "classic",
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        docs: {
          sidebarPath: require.resolve("./sidebars.js"),
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
          editUrl:
            "https://github.com/facebook/docusaurus/tree/main/packages/create-docusaurus/templates/shared/",
        },
        /*
        blog: {
          showReadingTime: true,
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
          editUrl:
            "https://github.com/facebook/docusaurus/tree/main/packages/create-docusaurus/templates/shared/",
        },
        */
        theme: {
          customCss: require.resolve("./src/css/custom.css"),
        },
      }),
    ],
  ],

  themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
    ({
      navbar: {
        title: "Zilliqa Developer Portal",
        logo: {
          alt: "Zilliqa Logo",
          src: "img/logo.png",
        },
        items: [
          {
            type: "doc",
            docId: "intro",
            position: "left",
            label: "Tutorial",
          },
          {
            to: "docs/basics/basics-intro-blockchain",
            activeBasePath: "docs/basics",
            label: "Basics",
            position: "right",
          },
          {
            to: "docs/dev-dapps/dev-started-introduction",
            activeBasePath: "docs/dev",
            label: "Developers",
            position: "right",
          },
          {
            to: "docs/apis/api-introduction",
            activeBasePath: "docs/apis",
            label: "APIs",
            position: "right",
          },
          {
            to: "docs/miners/mining-getting-started",
            activeBasePath: "docs/miners",
            label: "Miners",
            position: "right",
          },
          {
            to: "docs/exchanges/exchange-getting-started",
            activeBasePath: "docs/exchanges",
            label: "Exchanges",
            position: "right",
          },
          {
            to: "docs/staking/staking-overview",
            activeBasePath: "docs/staking",
            label: "Staking",
            position: "right",
          },
          {
            to: "docs/contributors/contribute-buildzil",
            activeBasePath: "docs/contributors",
            label: "Contributors",
            position: "right",
          },

          /*{ to: "/blog", label: "Blog", position: "left" },*/
          {
            href: "https://github.com/Zilliqa/Zilliqa",
            label: "GitHub",
            position: "right",
          },
        ],
      },
      footer: {
        style: "dark",
        links: [
          {
            title: "Docs",
            items: [
              {
                label: "Tutorial",
                to: "/docs/intro",
              },
            ],
          },
          {
            title: "Community",
            items: [
              {
                label: "Stack Overflow",
                href: "https://stackoverflow.com/questions/tagged/docusaurus",
              },
              {
                label: "Discord",
                href: "https://discordapp.com/invite/docusaurus",
              },
              {
                label: "Twitter",
                href: "https://twitter.com/docusaurus",
              },
            ],
          },
          {
            title: "More",
            items: [
              /*
                                {
                                  label: "Blog",
                                  to: "/blog",
                                },
                                */
              {
                label: "GitHub",
                href: "https://github.com/facebook/docusaurus",
              },
            ],
          },
        ],
        copyright: `Copyright © ${new Date().getFullYear()} My Project, Inc. Built with Docusaurus.`,
      },
      prism: {
        theme: lightCodeTheme,
        darkTheme: darkCodeTheme,
      },
    }),
};

module.exports = config;
