# Airdrop Utility

The `Disperse` contract and script in this repo provide a handy utility to automate batched transfers to a large number of accounts. It's useful for airdrops and other events where hundreds or tousands of accounts have to be sent an individual amount of a token. The contract supports the native token as well as `ERC20` tokens, but the script can currently handle only `ERC20` tokens.

The contract in `contracts/Disperse.sol` is an updated version of the original contract implemented by the [Disperse Protocol](https://disperse.app/disperse.pdf) ported to Solidity 0.8.20. The original contract is deployed and actively used on the Ethereum mainnet: https://etherscan.io/address/0xD152f549545093347A162Dce210e7293f1452150.

## Installation

Install [Hardhat](https://hardhat.org/hardhat-runner/docs/getting-started#installation) and configure your network(s) and account(s) in `hardhat.config.js`.

There are already existing deployments on the following networks:

| Network           | `Disperse` contract address                  |
| ----------------- | -------------------------------------------- |
| Zilliqa 1 mainnet | `0x8Cc17F9eA46cD1A98EbB1Dd5495067e3095956aA` |
| Zilliqa 1 testnet | `0x38048F4B71a87a31d21C86FF373a91d1E401bea5` |

Deploy a new `Disperse` contract only if it does not yet exist on the respective network:

```
npx hardhat run scripts/deploy.js --network <one_of_the_networks_in_hardhat.config.js>
```

The script will output the address of the `Disperse` contract you can add to the table above and use in the `disperse.js` script below.

## Usage

Adjust the `disperse.js` script as follows:

- change the address in line 9 to the address at which the `Disperse` contract is deployed on the respective network,
- change the address in line 10 to the address at which your `ERC20` token is deployed on the respective network,
- change how many accounts to send tokens to per `batch` in line 11

Prepare the `input.csv` file. Each line consists of two values separated by a comma:

- the value before the comma is a hex or bech32 formatted address
- the value after the comma is an integer amount

Note that the script does not support decimal numbers as amount.

Transfer the total amount of tokens to be distributed to the account that will be used to execute the script as configured in `hardhat.config.js`.

Run the script:

```
npx hardhat run scripts/disperse.js --network <one_of_the_networks_in_hardhat.config.js>
```
