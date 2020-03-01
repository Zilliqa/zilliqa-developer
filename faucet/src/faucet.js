import 'babel-polyfill';
import { BN, Long, bytes, units } from '@zilliqa-js/util';
import { Zilliqa } from '@zilliqa-js/zilliqa';
import {
    toBech32Address,
    getAddressFromPrivateKey,
} from '@zilliqa-js/crypto';
import fs from 'fs-extra';

require('dotenv').config();

const ZILS_PER_ACCOUNT = process.env.ZILS_PER_ACCOUNT;
const DEPOSIT_AMOUNT = process.env.DEPOSIT_AMOUNT;
const BLOCKS_TO_WAIT = process.env.BLOCKS_TO_WAIT;
const PRIVATE_KEY = process.env.OWNER_PRIVATEKEY;

const chainId = 1; // chainId of the developer testnet
const msgVersion = 1; // current msgVersion
const VERSION = bytes.pack(chainId, msgVersion);

const registerUser = async (user_address) => {
    try {
        const faucetFile = fs.readJSONSync('./faucet-state.json');

        const zilliqa = new Zilliqa(process.env.ISOLATED_URL);

        zilliqa.wallet.addByPrivateKey(PRIVATE_KEY);

        const myGasPrice = units.toQa('1000', units.Units.Li);

        const tx = zilliqa.transactions.new({
            version: VERSION,
            toAddr: faucetFile.contractAddress,
            amount: new BN(0),
            gasPrice: myGasPrice, // in Qa
            gasLimit: Long.fromNumber(8000),
            code: '',
            data: JSON.stringify({
                _tag: "register_user",
                params: [
                    {
                        vname: "user_address",
                        type: "ByStr20",
                        value: user_address
                    }
                ]
            }),
            priority: true
        });

        const callTx = await zilliqa.blockchain.createTransaction(tx);
        // Retrieving the transaction receipt (See note 2)
        return callTx.receipt;
    } catch (error) {
        throw error;
    }
}

const deployFaucet = async () => {
    try {
        const zilliqa = new Zilliqa(process.env.ISOLATED_URL);
        zilliqa.wallet.addByPrivateKey(PRIVATE_KEY);

        const address = getAddressFromPrivateKey(PRIVATE_KEY);
        console.log(`account address is: ${address}`);
        // Get Balance
        const balance = await zilliqa.blockchain.getBalance(address);
        // Get Minimum Gas Price from blockchain
        const minGasPrice = await zilliqa.blockchain.getMinimumGasPrice();

        // Account balance (See note 1)
        console.log(`Your account balance is:`);
        console.log(balance.result);
        console.log(`Current Minimum Gas Price: ${minGasPrice.result}`);
        const myGasPrice = units.toQa('1000', units.Units.Li); // Gas Price that will be used by all transactions
        console.log(`My Gas Price ${myGasPrice.toString()}`);
        const isGasSufficient = myGasPrice.gte(new BN(minGasPrice.result)); // Checks if your gas price is less than the minimum gas price
        console.log(`Is the gas price sufficient? ${isGasSufficient}`);
        // Deploy a contract
        console.log(`Deploying the contract....`);
        const code = fs.readFileSync('./contract.scilla', 'utf-8');

        const init = [
            // this parameter is mandatory for all init arrays
            {
                vname: '_scilla_version',
                type: 'Uint32',
                value: '0',
            },
            {
                vname: 'owner',
                type: 'ByStr20',
                value: `${address}`,
            },
            {
                vname: 'zils_per_account',
                type: 'Uint128',
                value: `${ZILS_PER_ACCOUNT}`
            },
            {
                vname: 'blocks_to_wait',
                type: 'Uint128',
                value: `${BLOCKS_TO_WAIT}`
            }
        ];

        const tx = zilliqa.transactions.new({
            version: VERSION,
            toAddr: "0x0000000000000000000000000000000000000000",
            amount: new BN(0),
            gasPrice: myGasPrice, // in Qa
            gasLimit: Long.fromNumber(8000),
            code: code,
            data: JSON.stringify(init).replace(/\\"/g, '"'),
            priority: true
        });

        const deployTx = await zilliqa.blockchain.createTransaction(tx);

        const contractId = await zilliqa.blockchain.getContractAddressFromTransactionID(
            deployTx.id
        );

        // Introspect the state of the underlying transaction
        console.log(`Deployment Transaction ID: ${deployTx.id}`);
        console.log(`Deployment Transaction Receipt:`);
        console.log(deployTx.txParams.receipt);

        // Get the deployed contract address
        console.log('The contract address is:');
        const contractAddress = toBech32Address(contractId.result);
        console.log(contractAddress);

        console.log('Calling deposit transaction...');

        const newtx = zilliqa.transactions.new({
            version: VERSION,
            toAddr: contractAddress,
            amount: new BN(DEPOSIT_AMOUNT),
            gasPrice: myGasPrice, // in Qa
            gasLimit: Long.fromNumber(8000),
            code: '',
            data: JSON.stringify({
                _tag: "deposit",
                params: []
            }),
            priority: true
        });

        const callTx = await zilliqa.blockchain.createTransaction(newtx);

        // Retrieving the transaction receipt (See note 2)
        console.log(JSON.stringify(callTx.receipt, null, 4));

        //Get the contract state
        console.log('Getting contract state...');
        const state = await zilliqa.blockchain.getSmartContractState(contractAddress);
        console.log('The state of the contract is:');
        console.log(JSON.stringify(state.result, null, 4));

        fs.writeJSONSync('./faucet-state.json', {
            contractAddress: contractAddress,
            depositState: state.result
        });
        console.log('Faucet contract successfully deployed.');
    } catch (err) {
        console.log(err);
    }
}

const getState = () => {
    return fs.readJSONSync('./faucet-state.json');
}

export { deployFaucet, registerUser, getState };