'use strict';

Object.defineProperty(exports, "__esModule", {
    value: true
});
exports.getState = exports.registerUser = exports.deployFaucet = undefined;

require('babel-polyfill');

var _util = require('@zilliqa-js/util');

var _zilliqa = require('@zilliqa-js/zilliqa');

var _crypto = require('@zilliqa-js/crypto');

var _fsExtra = require('fs-extra');

var _fsExtra2 = _interopRequireDefault(_fsExtra);

function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { default: obj }; }

function _asyncToGenerator(fn) { return function () { var gen = fn.apply(this, arguments); return new Promise(function (resolve, reject) { function step(key, arg) { try { var info = gen[key](arg); var value = info.value; } catch (error) { reject(error); return; } if (info.done) { resolve(value); } else { return Promise.resolve(value).then(function (value) { step("next", value); }, function (err) { step("throw", err); }); } } return step("next"); }); }; }

require('dotenv').config();

var ZILS_PER_ACCOUNT = process.env.ZILS_PER_ACCOUNT;
var DEPOSIT_AMOUNT = process.env.DEPOSIT_AMOUNT;
var BLOCKS_TO_WAIT = process.env.BLOCKS_TO_WAIT;
var PRIVATE_KEY = process.env.OWNER_PRIVATEKEY;

var chainId = 1; // chainId of the developer testnet
var msgVersion = 1; // current msgVersion
var VERSION = _util.bytes.pack(chainId, msgVersion);

var registerUser = function () {
    var _ref = _asyncToGenerator( /*#__PURE__*/regeneratorRuntime.mark(function _callee(user_address) {
        var faucetFile, zilliqa, myGasPrice, tx, callTx;
        return regeneratorRuntime.wrap(function _callee$(_context) {
            while (1) {
                switch (_context.prev = _context.next) {
                    case 0:
                        _context.prev = 0;
                        faucetFile = _fsExtra2.default.readJSONSync('./faucet-state.json');
                        zilliqa = new _zilliqa.Zilliqa(process.env.ISOLATED_URL);


                        zilliqa.wallet.addByPrivateKey(PRIVATE_KEY);

                        myGasPrice = _util.units.toQa('1000', _util.units.Units.Li);
                        tx = zilliqa.transactions.new({
                            version: VERSION,
                            toAddr: faucetFile.contractAddress,
                            amount: new _util.BN(0),
                            gasPrice: myGasPrice, // in Qa
                            gasLimit: _util.Long.fromNumber(8000),
                            code: '',
                            data: JSON.stringify({
                                _tag: "register_user",
                                params: [{
                                    vname: "user_address",
                                    type: "ByStr20",
                                    value: user_address
                                }]
                            }),
                            priority: true
                        });
                        _context.next = 8;
                        return zilliqa.blockchain.createTransaction(tx);

                    case 8:
                        callTx = _context.sent;

                        console.log(callTx);
                        // Retrieving the transaction receipt (See note 2)
                        return _context.abrupt('return', callTx.receipt);

                    case 13:
                        _context.prev = 13;
                        _context.t0 = _context['catch'](0);
                        throw _context.t0;

                    case 16:
                    case 'end':
                        return _context.stop();
                }
            }
        }, _callee, undefined, [[0, 13]]);
    }));

    return function registerUser(_x) {
        return _ref.apply(this, arguments);
    };
}();

var deployFaucet = function () {
    var _ref2 = _asyncToGenerator( /*#__PURE__*/regeneratorRuntime.mark(function _callee2() {
        var zilliqa, address, balance, minGasPrice, myGasPrice, isGasSufficient, code, init, tx, deployTx, contractId, contractAddress, newtx, callTx, state;
        return regeneratorRuntime.wrap(function _callee2$(_context2) {
            while (1) {
                switch (_context2.prev = _context2.next) {
                    case 0:
                        _context2.prev = 0;
                        zilliqa = new _zilliqa.Zilliqa(process.env.ISOLATED_URL);

                        zilliqa.wallet.addByPrivateKey(PRIVATE_KEY);

                        address = (0, _crypto.getAddressFromPrivateKey)(PRIVATE_KEY);

                        console.log('account address is: ' + address);
                        // Get Balance
                        _context2.next = 7;
                        return zilliqa.blockchain.getBalance(address);

                    case 7:
                        balance = _context2.sent;
                        _context2.next = 10;
                        return zilliqa.blockchain.getMinimumGasPrice();

                    case 10:
                        minGasPrice = _context2.sent;


                        // Account balance (See note 1)
                        console.log('Your account balance is:');
                        console.log(balance.result);
                        console.log('Current Minimum Gas Price: ' + minGasPrice.result);
                        myGasPrice = _util.units.toQa('1000', _util.units.Units.Li); // Gas Price that will be used by all transactions

                        console.log('My Gas Price ' + myGasPrice.toString());
                        isGasSufficient = myGasPrice.gte(new _util.BN(minGasPrice.result)); // Checks if your gas price is less than the minimum gas price

                        console.log('Is the gas price sufficient? ' + isGasSufficient);
                        // Deploy a contract
                        console.log('Deploying the contract....');
                        code = _fsExtra2.default.readFileSync('./contract.scilla', 'utf-8');
                        init = [
                        // this parameter is mandatory for all init arrays
                        {
                            vname: '_scilla_version',
                            type: 'Uint32',
                            value: '0'
                        }, {
                            vname: 'owner',
                            type: 'ByStr20',
                            value: '' + address
                        }, {
                            vname: 'zils_per_account',
                            type: 'Uint128',
                            value: '' + ZILS_PER_ACCOUNT
                        }, {
                            vname: 'blocks_to_wait',
                            type: 'Uint128',
                            value: '' + BLOCKS_TO_WAIT
                        }];
                        tx = zilliqa.transactions.new({
                            version: VERSION,
                            toAddr: "0x0000000000000000000000000000000000000000",
                            amount: new _util.BN(0),
                            gasPrice: myGasPrice, // in Qa
                            gasLimit: _util.Long.fromNumber(8000),
                            code: code,
                            data: JSON.stringify(init).replace(/\\"/g, '"'),
                            priority: true
                        });
                        _context2.next = 24;
                        return zilliqa.blockchain.createTransaction(tx);

                    case 24:
                        deployTx = _context2.sent;
                        _context2.next = 27;
                        return zilliqa.blockchain.getContractAddressFromTransactionID(deployTx.id);

                    case 27:
                        contractId = _context2.sent;


                        // Introspect the state of the underlying transaction
                        console.log('Deployment Transaction ID: ' + deployTx.id);
                        console.log('Deployment Transaction Receipt:');
                        console.log(deployTx.txParams.receipt);

                        // Get the deployed contract address
                        console.log('The contract address is:');
                        contractAddress = (0, _crypto.toBech32Address)(contractId.result);

                        console.log(contractAddress);

                        console.log('Calling deposit transaction...');

                        newtx = zilliqa.transactions.new({
                            version: VERSION,
                            toAddr: contractAddress,
                            amount: new _util.BN(DEPOSIT_AMOUNT),
                            gasPrice: myGasPrice, // in Qa
                            gasLimit: _util.Long.fromNumber(8000),
                            code: '',
                            data: JSON.stringify({
                                _tag: "deposit",
                                params: []
                            }),
                            priority: true
                        });
                        _context2.next = 38;
                        return zilliqa.blockchain.createTransaction(newtx);

                    case 38:
                        callTx = _context2.sent;


                        // Retrieving the transaction receipt (See note 2)
                        console.log(JSON.stringify(callTx.receipt, null, 4));

                        //Get the contract state
                        console.log('Getting contract state...');
                        _context2.next = 43;
                        return zilliqa.blockchain.getSmartContractState(contractAddress);

                    case 43:
                        state = _context2.sent;

                        console.log('The state of the contract is:');
                        console.log(JSON.stringify(state.result, null, 4));

                        _fsExtra2.default.writeJSONSync('./faucet-state.json', {
                            contractAddress: contractAddress,
                            depositState: state.result
                        });
                        console.log('Faucet contract successfully deployed.');
                        _context2.next = 53;
                        break;

                    case 50:
                        _context2.prev = 50;
                        _context2.t0 = _context2['catch'](0);

                        console.log(_context2.t0);

                    case 53:
                    case 'end':
                        return _context2.stop();
                }
            }
        }, _callee2, undefined, [[0, 50]]);
    }));

    return function deployFaucet() {
        return _ref2.apply(this, arguments);
    };
}();

var getState = function getState() {
    return _fsExtra2.default.readJSONSync('./faucet-state.json');
};

exports.deployFaucet = deployFaucet;
exports.registerUser = registerUser;
exports.getState = getState;