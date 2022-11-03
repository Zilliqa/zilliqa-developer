var assert = require('assert');
var fs = require('fs');
var path = require('path');

// Initialize testing library
const ZilTest = require('zilliqa-testing-library').ZilTest;
const Test = new ZilTest();

// Read contract file
const contract = fs.readFileSync(path.join(__dirname, '../hello-world.scilla'), 'utf8');

describe('Run hello-world tests...', function () {
   
    it('Generate 2 Zilliqa accounts on network', async function () {
        await Test.generateAccounts(2);

        assert(Test.accounts.length === 2);
    });

    it('Load contract into Testing Suite and run scilla checker', async function () {
        await Test.loadContract(contract); // Contracts[0]

        assert(Test.contracts.length === 1);
    });

    it('Deploy HelloWorld contract', async function () {
        const preparedContract = Test.contracts[0];

        const [tx, deployed] = await preparedContract.deploy(
            Test.accounts[0].address,
            {
                owner: Test.accounts[0].address
            }
        );

        assert(tx.receipt.success === true, "Transaction failed");

        const init = await deployed.getInit();

        assert(init.owner === Test.accounts[0].address);
    });

    it('non-owner calls setHello should return event setHello.code = 1', async function () {
        const preparedContract = Test.contracts[0];

        const [tx, deployed] = await preparedContract.deploy(
            Test.accounts[0].address,
            {
                owner: Test.accounts[0].address
            }
        );

        const callTx = await deployed.setHello(Test.accounts[1].address, { msg: 'not owner' });

        assert(callTx.events.setHello.code === "1");

        const state = await deployed.getState();

        assert(state.welcome_msg === '');
    });

    it('owner calls setHello should return event setHello.code = 2 and update state', async function () {
        const preparedContract = Test.contracts[0];

        const [tx, deployed] = await preparedContract.deploy(
            Test.accounts[0].address,
            {
                owner: Test.accounts[0].address
            }
        );

        // define a new message
        const newMessage = "I am owner";

        const callTx = await deployed.setHello(Test.accounts[0].address, { msg: newMessage });

        assert(callTx.events.setHello.code === "2");

        const state = await deployed.getState();

        assert(state.welcome_msg === newMessage);
    });

    it('getHello should return event getHello.msg equal to the value provided', async function () {
        const preparedContract = Test.contracts[0];

        const [tx, deployed] = await preparedContract.deploy(
            Test.accounts[0].address,
            {
                owner: Test.accounts[0].address
            }
        );

        // set a new message
        const message = 'I am owner';
        await deployed.setHello(Test.accounts[0].address, { msg: message });

        const callTx = await deployed.call(Test.accounts[0].address, 'getHello');

        assert(callTx.events.getHello.msg === message);
    });

});
