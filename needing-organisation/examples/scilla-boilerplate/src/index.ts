#!/usr/bin/env node
import init from "./commands/init";
import deploy from "./commands/deploy";

require("yargs")
  .scriptName("scilla-boilerplate")
  .usage("$0 <cmd> [args]")
  .command(
    "init [name] [template]",
    "Scaffold new project",
    (yargs: any) => {
      yargs.positional("name", {
        type: "string",
        describe: "Scilla contract name you want to generate",
      });
      yargs.positional("template", {
        type: "string",
        describe: "Template used for the contract",
      });
    },
    function (argv: any) {
      init();
    }
  )
  .command(
    "deploy [pk] [network]",
    "Deploy contract",
    (yargs: any) => {
      yargs.positional("pk", {
        type: "string",
        default: undefined,
        describe: "Private Key of the account you want to deploy from",
      });
      yargs.positional("network", {
        type: "string",
        default: undefined,
        describe: "Network API where you want to deploy",
      });
    },
    function (argv: any) {
      deploy();
    }
  )
  .option("verbose", {
    alias: "v",
    type: "boolean",
    default: false,
    description: "Run with verbose logging",
  })
  .help().argv;
