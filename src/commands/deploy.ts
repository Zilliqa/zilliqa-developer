#!/usr/bin/env node

import * as chalk from 'chalk';
import * as shell from 'shelljs';
import * as Listr from 'listr';
import * as yargs from 'yargs';

const CURR_DIR = process.cwd();

async function postProcessNode() {
    const deploy = require(CURR_DIR + '/scripts/deploy.js');

    return await deploy();
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