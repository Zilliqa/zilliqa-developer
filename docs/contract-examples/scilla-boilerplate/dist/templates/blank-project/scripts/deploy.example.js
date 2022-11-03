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
    const ZT = new ZilTest({ network: NETWORK_URL });

    // Import the wallet from PRIVATE KEY that wa provided
    await ZT.importAccounts([ACCOUNT_PRIVATEKEY]);

    // Read contract code from root directory of the project and store it for deployment.
    const contractCode = fs.readFileSync(
        path.join(__dirname, "../<%= projectName %>.scilla"),
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