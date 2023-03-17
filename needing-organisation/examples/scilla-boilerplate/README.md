# scilla-boilerplate

**Start developing your next Scilla contract in seconds**

A highly scalable foundation to create, test and deploy Scilla contracts with ease.

## Installation

`npm install scilla-boilerplate`

## Project structure

You can scaffold a new Scilla project using `scilla-boilerplate init` command. This command will let you select from predefined examples of contracts or you can start an empty project by selecting `blank-project` from the list.

```
.
+-- scripts
|   --- deploy.js
+-- test
|   --- test.js
--- .env.example
--- .gitignore
--- contract-name.scilla
--- init.json
--- package.json
--- readme.md
```

### scripts/deploy.js

**This file is NOT required by the `scilla-boilerplate deploy` command.**

You can use it to replace the default scilla deploy script, if you need more advanced commands before deployment.

It should always export a function called `deploy` that returns a tuple containing `transactionObject` with a receipt property and `contractObject` with an address property. (eg. result from `contract.deploy` method provided by Zilliqa Testing Library).

`deploy.js` provided by the scilla-boilerplate init command explained:

```js
require("dotenv").config();
var fs = require("fs");
var path = require("path");

// Reads NETWORK_URL and ACCOUNT_PRIVATEKEY from .env file. Those constants are needed to initialize Zilliqa API Provider and deploy the contract.

// ACCOUNT_PRIVATEKEY should never be hardcoded in any file.
const NETWORK_URL = process.env.NETWORK_URL;
const ACCOUNT_PRIVATEKEY = process.env.ACCOUNT_PRIVATEKEY;

// Import zilliqa-testing-library
const ZilTest = require("zilliqa-testing-library").ZilTest;

async function deploy() {
  // Initialize Zilliqa RPC Provider
  const ZT = new ZilTest(NETWORK_URL);

  // Import the wallet from PRIVATE KEY that wa provided
  await ZT.importAccounts([ACCOUNT_PRIVATEKEY]);

  // Read contract code from root directory of the project and store it for deployment.
  const contractCode = fs.readFileSync(
    path.join(__dirname, "../contract-name.scilla"),
    "utf8"
  );

  // Read and store init.json that contains all initialization parameters that are needed for deployment.
  const init = JSON.parse(
    fs.readFileSync(path.join(__dirname, "../init.json"), "utf8")
  );

  // Load contract into the Zilliqa Testing Library. This function will also run scilla-checker on the contract. More informations cn be found on the Zilliqa Testing Library documentation.
  const helloWorld = await ZT.loadContract(contractCode);

  // helloWorld.deploy return a tuple containing transaction object and a contract object
  return await helloWorld.deploy(ZT.accounts[0].address, init);
}

// This file should always return a function called deploy
module.exports = deploy;
```

### test/test.js

This is a test file used by mocha. It already includes Zilliqa Testing Library and an example test so you could start right away.

### .env.example

This file contains the secrets needed to deploy a contract: NETWORK_URL, ACCOUNT_PRIVATEKEY

You should never use any PRIVATE KEYS hardcoded in your files.

### contract-name.scilla

This file is always named after the project and will contain a basic Scilla contract structure.

```scilla
(* <%= projectName %> contract *)

(***************************************************)
(*                 Scilla version                  *)
(***************************************************)

scilla_version 0

(***************************************************)
(*               Associated library                *)
(***************************************************)
library <%= projectName %>


(***************************************************)
(*             The contract definition             *)
(***************************************************)

contract <%= projectName %>
(owner: ByStr20)
```

### init.json

This file is used when deploying a contract using `scilla-boilerplate deploy` command. The default provided file contains an `owner` key.

```json
{
  "owner": "0xc154822999Db8BCbFC79062ddF80d501c4aA4da8"
}
```

## Features

- Project scaffolding from Scilla example contracts or blank projects
- Testing Suite integrated in projects, you can write tests in Mocha and easily deploy and run contracts in a simulated env. Read more about this on Zilliqa Testing Library
- Deploy contracts directly on any network.
