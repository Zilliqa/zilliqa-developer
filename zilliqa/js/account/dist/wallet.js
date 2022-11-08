"use strict";
//  Copyright (C) 2018 Zilliqa
//
//  This file is part of zilliqa-js
//
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with this program.  If not, see <https://www.gnu.org/licenses/>.
Object.defineProperty(exports, "__esModule", { value: true });
exports.Wallet = void 0;
var tslib_1 = require("tslib");
var bip39_1 = (0, tslib_1.__importDefault)(require("bip39"));
var hdkey_1 = (0, tslib_1.__importDefault)(require("hdkey"));
var core_1 = require("@zilliqa-js/core");
var zcrypto = (0, tslib_1.__importStar)(require("@zilliqa-js/crypto"));
var account_1 = require("./account");
var util_1 = require("@zilliqa-js/util");
var Wallet = /** @class */ (function (_super) {
    (0, tslib_1.__extends)(Wallet, _super);
    /**
     * constructor
     *
     * Takes an array of Account objects and instantiates a Wallet instance.
     *
     * @param {Account[]} accounts
     */
    function Wallet(provider, accounts) {
        if (accounts === void 0) { accounts = []; }
        var _this = _super.call(this) || this;
        _this.accounts = {};
        if (accounts.length) {
            _this.accounts = accounts.reduce(function (acc, account) {
                var _a;
                return (0, tslib_1.__assign)((0, tslib_1.__assign)({}, acc), (_a = {}, _a[account.address] = account, _a));
            }, {});
        }
        _this.provider = provider;
        _this.defaultAccount = accounts[0];
        return _this;
    }
    /**
     * create
     *
     * Creates a new keypair with a randomly-generated private key. The new
     * account is accessible by address.
     *
     * @returns {string} - address of the new account
     */
    Wallet.prototype.create = function () {
        var _a;
        var privateKey = zcrypto.schnorr.generatePrivateKey();
        var newAccount = new account_1.Account(privateKey);
        this.accounts = (0, tslib_1.__assign)((0, tslib_1.__assign)({}, this.accounts), (_a = {}, _a[newAccount.address] = newAccount, _a));
        if (!this.defaultAccount) {
            this.defaultAccount = newAccount;
        }
        return newAccount.address;
    };
    /**
     * addByPrivateKey
     *
     * Adds an account to the wallet by private key.
     *
     * @param {string} privateKey - hex-encoded private key
     * @returns {string} - the corresponing address, computer from the private
     * key.
     */
    Wallet.prototype.addByPrivateKey = function (privateKey) {
        var _a;
        var newAccount = new account_1.Account(privateKey);
        this.accounts = (0, tslib_1.__assign)((0, tslib_1.__assign)({}, this.accounts), (_a = {}, _a[newAccount.address] = newAccount, _a));
        if (!this.defaultAccount) {
            this.defaultAccount = newAccount;
        }
        return newAccount.address;
    };
    /**
     * addByKeystore
     *
     * Adds an account by keystore. This method is asynchronous and returns
     * a Promise<string>, in order not to block on the underlying decryption
     * operation.
     *
     * @param {string} keystore
     * @param {string} passphrase
     * @returns {Promise<string>}
     */
    Wallet.prototype.addByKeystore = function (keystore, passphrase) {
        return (0, tslib_1.__awaiter)(this, void 0, void 0, function () {
            var newAccount;
            var _a;
            return (0, tslib_1.__generator)(this, function (_b) {
                switch (_b.label) {
                    case 0: return [4 /*yield*/, account_1.Account.fromFile(keystore, passphrase)];
                    case 1:
                        newAccount = _b.sent();
                        this.accounts = (0, tslib_1.__assign)((0, tslib_1.__assign)({}, this.accounts), (_a = {}, _a[newAccount.address] = newAccount, _a));
                        if (!this.defaultAccount) {
                            this.defaultAccount = newAccount;
                        }
                        return [2 /*return*/, newAccount.address];
                }
            });
        });
    };
    /**
     * addByMnemonic
     *
     * Adds an `Account` by use of a mnemonic as specified in BIP-32 and BIP-39
     *
     * @param {string} phrase - 12-word mnemonic phrase
     * @param {number} index=0 - the number of the child key to add
     * @returns {string} - the corresponding address
     */
    Wallet.prototype.addByMnemonic = function (phrase, index) {
        if (index === void 0) { index = 0; }
        if (!this.isValidMnemonic(phrase)) {
            throw new Error("Invalid mnemonic phrase: " + phrase);
        }
        var seed = bip39_1.default.mnemonicToSeed(phrase);
        var hdKey = hdkey_1.default.fromMasterSeed(seed);
        var childKey = hdKey.derive("m/44'/313'/0'/0/" + index);
        var privateKey = childKey.privateKey.toString('hex');
        return this.addByPrivateKey(privateKey);
    };
    /**
     * addByMnemonicLedger
     *
     * Adds an `Account` by use of a mnemonic as specified in BIP-32 and BIP-39
     * The key derivation path used in Ledger is different from that of
     * addByMnemonic.
     *
     * @param {string} phrase - 12-word mnemonic phrase
     * @param {number} index=0 - the number of the child key to add
     * @returns {string} - the corresponding address
     */
    Wallet.prototype.addByMnemonicLedger = function (phrase, index) {
        if (index === void 0) { index = 0; }
        if (!this.isValidMnemonic(phrase)) {
            throw new Error("Invalid mnemonic phrase: " + phrase);
        }
        var seed = bip39_1.default.mnemonicToSeed(phrase);
        var hdKey = hdkey_1.default.fromMasterSeed(seed);
        var childKey = hdKey.derive("m/44'/313'/" + index + "'/0'/0'");
        var privateKey = childKey.privateKey.toString('hex');
        return this.addByPrivateKey(privateKey);
    };
    /**
     * export
     *
     * Exports the specified account as a keystore file.
     *
     * @param {string} address
     * @param {string} passphrase
     * @param {KDF} kdf='scrypt'
     * @returns {Promise<string>}
     */
    Wallet.prototype.export = function (address, passphrase, kdf) {
        if (kdf === void 0) { kdf = 'scrypt'; }
        if (!this.accounts[address]) {
            throw new Error("No account with address " + address + " exists");
        }
        return this.accounts[address].toFile(passphrase, kdf);
    };
    /**
     * remove
     *
     * Removes an account from the wallet and returns boolean to indicate
     * failure or success.
     *
     * @param {string} address
     * @returns {boolean}
     */
    Wallet.prototype.remove = function (address) {
        if (this.accounts[address]) {
            var _a = this.accounts, _b = address, toRemove = _a[_b], rest = (0, tslib_1.__rest)(_a, [typeof _b === "symbol" ? _b : _b + ""]);
            this.accounts = rest;
            return true;
        }
        return false;
    };
    /**
     * setDefault
     *
     * Sets the default account of the wallet.
     *
     * @param {string} address
     */
    Wallet.prototype.setDefault = function (address) {
        this.defaultAccount = this.accounts[address];
    };
    /**
     * sign
     *
     * signs an unsigned transaction with the default account.
     *
     * @param {Transaction} tx
     * @param {boolean} offlineSign
     * @returns {Transaction}
     */
    Wallet.prototype.sign = function (tx, offlineSign) {
        if (tx.txParams && tx.txParams.pubKey) {
            // attempt to find the address
            var senderAddress = zcrypto.getAddressFromPublicKey(tx.txParams.pubKey);
            if (!this.accounts[senderAddress]) {
                throw new Error("Could not sign the transaction with " + senderAddress + " as it does not exist");
            }
            return this.signWith(tx, senderAddress, offlineSign);
        }
        if (!this.defaultAccount) {
            throw new Error('This wallet has no default account.');
        }
        return this.signWith(tx, this.defaultAccount.address, offlineSign);
    };
    Wallet.prototype.signBatch = function (txList) {
        return (0, tslib_1.__awaiter)(this, void 0, void 0, function () {
            var batchResults, signer_1, balance, nextNonce, _loop_1, this_1, index, err_1;
            return (0, tslib_1.__generator)(this, function (_a) {
                switch (_a.label) {
                    case 0:
                        batchResults = [];
                        if (!this.defaultAccount) {
                            throw new Error('This wallet has no default account.');
                        }
                        _a.label = 1;
                    case 1:
                        _a.trys.push([1, 7, , 8]);
                        signer_1 = this.accounts[this.defaultAccount.address];
                        return [4 /*yield*/, this.provider.send(core_1.RPCMethod.GetBalance, signer_1.address.replace('0x', '').toLowerCase())];
                    case 2:
                        balance = _a.sent();
                        if (balance.result === undefined) {
                            throw new Error('Could not get balance');
                        }
                        if (typeof balance.result.nonce !== 'number') {
                            throw new Error('Could not get nonce');
                        }
                        nextNonce = balance.result.nonce + 1;
                        _loop_1 = function (index) {
                            var currentNonce, withNonceTx, signedTx;
                            return (0, tslib_1.__generator)(this, function (_b) {
                                switch (_b.label) {
                                    case 0:
                                        currentNonce = index + nextNonce;
                                        withNonceTx = txList[index].map(function (txObj) {
                                            return (0, tslib_1.__assign)((0, tslib_1.__assign)({}, txObj), { nonce: currentNonce, pubKey: signer_1.publicKey });
                                        });
                                        return [4 /*yield*/, this_1.sign(withNonceTx)];
                                    case 1:
                                        signedTx = _b.sent();
                                        batchResults.push(signedTx);
                                        return [2 /*return*/];
                                }
                            });
                        };
                        this_1 = this;
                        index = 0;
                        _a.label = 3;
                    case 3:
                        if (!(index < txList.length)) return [3 /*break*/, 6];
                        return [5 /*yield**/, _loop_1(index)];
                    case 4:
                        _a.sent();
                        _a.label = 5;
                    case 5:
                        index++;
                        return [3 /*break*/, 3];
                    case 6: return [3 /*break*/, 8];
                    case 7:
                        err_1 = _a.sent();
                        throw err_1;
                    case 8: return [2 /*return*/, batchResults];
                }
            });
        });
    };
    /**
     * signWith
     *
     * @param {Transaction} tx
     * @param {string} account
     * @param {boolean} offlineSign
     * @returns {Transaction}
     */
    Wallet.prototype.signWith = function (tx, account, offlineSign) {
        return (0, tslib_1.__awaiter)(this, void 0, void 0, function () {
            var signer, gasPrice, gasLimit, debt, currNonce, balance, bal, withNonce_1, withPublicKey, err_2;
            return (0, tslib_1.__generator)(this, function (_a) {
                switch (_a.label) {
                    case 0:
                        if (!this.accounts[account]) {
                            throw new Error('The selected account does not exist on this Wallet instance.');
                        }
                        signer = this.accounts[account];
                        gasPrice = tx.txParams.gasPrice;
                        gasLimit = new util_1.BN(tx.txParams.gasLimit.toString());
                        debt = gasPrice.mul(gasLimit).add(tx.txParams.amount);
                        currNonce = 0;
                        _a.label = 1;
                    case 1:
                        _a.trys.push([1, 5, , 6]);
                        if (!!tx.txParams.nonce) return [3 /*break*/, 4];
                        if (offlineSign) {
                            throw new Error('No nonce detected in tx params when signing in offline mode');
                        }
                        if (!(typeof offlineSign === 'undefined' || !offlineSign)) return [3 /*break*/, 3];
                        return [4 /*yield*/, this.provider.send(core_1.RPCMethod.GetBalance, signer.address.replace('0x', '').toLowerCase())];
                    case 2:
                        balance = _a.sent();
                        if (balance.result === undefined) {
                            throw new Error('Could not get balance');
                        }
                        bal = new util_1.BN(balance.result.balance);
                        if (debt.gt(bal)) {
                            throw new Error('You do not have enough funds, need ' +
                                debt.toString() +
                                ' but only have ' +
                                bal.toString());
                        }
                        if (typeof balance.result.nonce !== 'number') {
                            throw new Error('Could not get nonce');
                        }
                        currNonce = balance.result.nonce;
                        _a.label = 3;
                    case 3:
                        withNonce_1 = tx.map(function (txObj) {
                            return (0, tslib_1.__assign)((0, tslib_1.__assign)({}, txObj), { nonce: txObj.nonce || currNonce + 1, pubKey: signer.publicKey });
                        });
                        return [2 /*return*/, withNonce_1.map(function (txObj) {
                                // @ts-ignore
                                return (0, tslib_1.__assign)((0, tslib_1.__assign)({}, txObj), { signature: signer.signTransaction(withNonce_1.bytes) });
                            })];
                    case 4:
                        withPublicKey = tx.map(function (txObj) {
                            return (0, tslib_1.__assign)((0, tslib_1.__assign)({}, txObj), { pubKey: signer.publicKey });
                        });
                        return [2 /*return*/, withPublicKey.map(function (txObj) {
                                return (0, tslib_1.__assign)((0, tslib_1.__assign)({}, txObj), { signature: signer.signTransaction(tx.bytes) });
                            })];
                    case 5:
                        err_2 = _a.sent();
                        throw err_2;
                    case 6: return [2 /*return*/];
                }
            });
        });
    };
    Wallet.prototype.isValidMnemonic = function (phrase) {
        if (phrase.trim().split(/\s+/g).length < 12) {
            return false;
        }
        return bip39_1.default.validateMnemonic(phrase);
    };
    return Wallet;
}(core_1.Signer));
exports.Wallet = Wallet;
//# sourceMappingURL=wallet.js.map