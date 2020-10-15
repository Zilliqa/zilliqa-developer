## hello-world.scilla
This contract was created using scilla-boilerplate and implements testing and deploying features.

## How to test
This project comes with MochaJS installed and one test file already written.

* You can check the test file under ./test directory

To run tests you only have to execute `npm run test` command.

Testing Scilla contracts can be easily done with the help of Zilliqa Testing Library which provides high level functions for wallets and contracts management. Read more here.

## How to deploy
1. Create .env file containing network and the account you want to deploy from (see .env.example file).
2. Add your initialization parameters into the `init.json` file.
2. Execute `npm run deploy` command.