#!/usr/bin/env node

import * as chalk from 'chalk';
import * as shell from 'shelljs';
import * as Listr from 'listr';
import * as yargs from 'yargs';
import ZilTest from "zilliqa-testing-library";
import * as fs from 'fs';
import * as path from "path";

const CURR_DIR = process.cwd();

async function postProcessNode() {
    const config = require(CURR_DIR + '/zilliqa.config.js');


    // Initialize Zilliqa RPC Provider
    const ZT = new ZilTest({ network: config.networkUrl });

    // Import the wallet from PRIVATE KEY that wa provided
    await ZT.importAccounts([config.accountPrivateKey]);

    // Read contract code from root directory of the project and store it for deployment.
    const contractCode = fs.readFileSync(
        path.join(CURR_DIR, config.contractFile),
        "utf8"
    );

    // Load contract into the Zilliqa Testing Library. 
    // This function will also run scilla-checker on the contract. 
    // More informations cn be found on the Zilliqa Testing Library documentation.
    const contract = await ZT.loadContract(contractCode);

    if (contract !== undefined && contract.deploy !== undefined) {
        // helloWorld.deploy return a tuple containing transaction object and a contract object
        return await contract.deploy(ZT.accounts[0].address, config.init);
    } else {
        throw new Error('Contract could not be imported. Maybe scilla-checker failed');
    }
}

const deploy = () => {
    const tasks = new Listr([
        {
            title: 'Run tests',
            task: async () => shell.exec('mocha --timeout=10000')
        },
        {
            title: 'Deploy contract',
            task: async (ctx) => {
                const response = await postProcessNode()
                ctx.tx = response[0];
                ctx.contract = response[1];
            }
        }
    ]);

    tasks.run().then(ctx => {
        console.log('\nTransaction receipt: ', ctx.tx.receipt, '\n');
        console.log('Contract address: ', chalk.yellow(ctx.contract.address), '\n\n');
        if (yargs.argv.verbose && yargs.argv.verbose === true) {
            console.log('Init params: ', ctx.contract.init);
        }
    }).catch(err => {
        console.error(err);
    });
}

export default deploy;