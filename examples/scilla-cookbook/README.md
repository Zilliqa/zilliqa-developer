# Community Scilla CookBook (CSC)

The Community Scilla Cookbook (CSC) is maintained by the community and is a reference for various scilla related documents, contracts, howto's, tips and tricks. If you have found this documentation useful, consider contributing.

## How To contribute

### Required technologies

You should have these technologies installed on your machine to run the scilla cookbook.

* NodeJS
* npm
* yarn

### Contributing

Please review [this guide](https://github.com/firstcontributions/first-contributions) for further information on how to contribute to a github repository and [this guide](https://gist.github.com/joshbuchea/6f47e86d2510bce28f8e7f42ae84c716) about commit messages.

* Fork this repo
* Clone your fork of this repo to your local machine
* Create a branch from master called docs/xxxxx
* Make necessary changes
* Push changes to your fork username/scilla-cookbook
* Raise a pull request from your fork, targeting zilliqa/scilla-cookbook
* Follow the pull request read-me

## How to build the site locally

### Dependancy installation

This command installs the dependencies required to run scilla cookbook.

```bash
yarn
```

### Start a local scilla cookbook server

This command starts a local development server and opens up a browser window. Most changes are reflected live without having to restart the server.

```bash
yarn start
```

## How to deploy the site

### Local Deployment

Amend docusarus.config.js and change the organisation values to your local repo and Github username.

Build a static site from the dynamic content into /build/ directory.

```bash
yarn build
```

Deploy the /build/ directory to the branch /gh-pages/

```bash
GIT_USER=<Your GitHub username> USE_SSH=true yarn deploy
```

## How to run markdown-spellcheck 

<!-- Check markdown spelling mistakes and add exceptions to the dictionary by running -->

```\mdspell "docs/**/*.md"```
