#!/usr/bin/env node
"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var init_1 = require("./commands/init");
var deploy_1 = require("./commands/deploy");
require('yargs')
    .scriptName("scilla-boilerplate")
    .usage('$0 <cmd> [args]')
    .command('init [name] [template]', 'Scaffold new project', function (yargs) {
    yargs.positional('name', {
        type: 'string',
        describe: 'Scilla contract name you want to generate'
    });
    yargs.positional('template', {
        type: 'string',
        describe: 'Template used for the contract'
    });
}, function (argv) {
    init_1.default();
})
    .command('deploy [pk] [network]', 'Deploy contract', function (yargs) {
    yargs.positional('pk', {
        type: 'string',
        default: undefined,
        describe: 'Private Key of the account you want to deploy from'
    });
    yargs.positional('network', {
        type: 'string',
        default: undefined,
        describe: 'Network API where you want to deploy'
    });
}, function (argv) {
    deploy_1.default();
})
    .option('verbose', {
    alias: 'v',
    type: 'boolean',
    default: false,
    description: 'Run with verbose logging'
})
    .help()
    .argv;
//# sourceMappingURL=index.js.map