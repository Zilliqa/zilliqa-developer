require('dotenv').config()
var fs = require('fs');
var path = require('path');

const NETWORK_URL = process.env.NETWORK_URL;
const ACCOUNT_PRIVATEKEY = process.env.ACCOUNT_PRIVATEKEY;

const ZilTest = require('zilliqa-testing-library').ZilTest;

async function deploy() {
    const ZT = new ZilTest(NETWORK_URL);

    await ZT.importAccounts([ACCOUNT_PRIVATEKEY]);

    const contractCode = fs.readFileSync(path.join(__dirname, '../<%= projectName %>'), 'utf8');

    const init = JSON.parse(fs.readFileSync(path.join(__dirname, '../init.json'), 'utf8'));

    const helloWorld = await ZT.loadContract(contractCode);

    return await helloWorld.deploy(ZT.accounts[0].address, init);
}

deploy();