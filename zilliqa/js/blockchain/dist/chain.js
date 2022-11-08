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
exports.Blockchain = void 0;
var tslib_1 = require("tslib");
var account_1 = require("@zilliqa-js/account");
var crypto_1 = require("@zilliqa-js/crypto");
var util_1 = require("@zilliqa-js/util");
var core_1 = require("@zilliqa-js/core");
var util_2 = require("./util");
var isBlockNumber = function (blockNum) {
    return Number.isFinite(blockNum) && Number.isInteger(blockNum) && blockNum >= 0;
};
var Blockchain = /** @class */ (function () {
    function Blockchain(provider, signer) {
        this.pendingErrorMap = {
            0: 'Transaction not found',
            1: 'Pending - Dispatched',
            2: 'Pending - Soft-confirmed (awaiting Tx block generation)',
            4: 'Pending - Nonce is higher than expected',
            5: 'Pending - Microblock gas limit exceeded',
            6: 'Pending - Consensus failure in network',
            3: 'Confirmed',
            10: 'Rejected - Transaction caused math error',
            11: 'Rejected - Scilla invocation error',
            12: 'Rejected - Contract account initialization error',
            13: 'Rejected - Invalid source account',
            14: 'Rejected - Gas limit higher than shard gas limit',
            15: 'Rejected - Unknown transaction type',
            16: 'Rejected - Transaction sent to wrong shard',
            17: 'Rejected - Contract & source account cross-shard issue',
            18: 'Rejected - Code size exceeded limit',
            19: 'Rejected - Transaction verification failed',
            20: 'Rejected - Gas limit too low',
            21: 'Rejected - Insufficient balance',
            22: 'Rejected - Insufficient gas to invoke Scilla checker',
            23: 'Rejected - Duplicate transaction exists',
            24: 'Rejected - Transaction with same nonce but same/higher gas price exists',
            25: 'Rejected - Invalid destination address',
            26: 'Rejected - Failed to add contract account to state',
            27: 'Rejected - Nonce is lower than expected',
            255: 'Rejected - Internal error',
        };
        this.transactionStatusMap = {
            0: { 0: 'Transaction not found', 1: ' Pending - Dispatched' },
            1: {
                2: 'Pending - Soft-confirmed (awaiting Tx block generation)',
                4: 'Pending - Nonce is higher than expected',
                5: 'Pending - Microblock gas limit exceeded',
                6: 'Pending - Consensus failure in network',
            },
            2: {
                3: 'Confirmed',
                10: 'Rejected - Transaction caused math error',
                11: 'Rejected - Scilla invocation error',
                12: 'Rejected - Contract account initialization error',
                13: 'Rejected - Invalid source account',
                14: 'Rejected - Gas limit higher than shard gas limit',
                15: 'Rejected - Unknown transaction type',
                16: 'Rejected - Transaction sent to wrong shard',
                17: 'Rejected - Contract & source account cross-shard issue',
                18: 'Rejected - Code size exceeded limit',
                19: 'Rejected - Transaction verification failed',
                20: 'Rejected - Gas limit too low',
                21: 'Rejected - Insufficient balance',
                22: 'Rejected - Insufficient gas to invoke Scilla checker',
                23: 'Rejected - Duplicate transaction exists',
                24: 'Rejected - Transaction with higher gas price exists',
                25: 'Rejected - Invalid destination address',
                26: 'Rejected - Failed to add contract account to state',
                27: 'Rejected - Nonce is lower than expected',
                255: 'Rejected - Internal error',
            },
        };
        this.provider = provider;
        this.provider.middleware.request.use(account_1.util.formatOutgoingTx, core_1.RPCMethod.CreateTransaction);
        this.signer = signer;
    }
    Blockchain.prototype.getBlockChainInfo = function () {
        return this.provider.send(core_1.RPCMethod.GetBlockchainInfo);
    };
    Blockchain.prototype.getShardingStructure = function () {
        return this.provider.send(core_1.RPCMethod.GetShardingStructure);
    };
    // Gets details of a Directory Service block by block number.
    Blockchain.prototype.getDSBlock = function (blockNum) {
        return this.provider.send(core_1.RPCMethod.GetDSBlock, blockNum.toString());
    };
    // Gets details of the most recent Directory Service block.
    Blockchain.prototype.getLatestDSBlock = function () {
        return this.provider.send(core_1.RPCMethod.GetLatestDSBlock);
    };
    // Gets the number of DS blocks that the network has processed.
    Blockchain.prototype.getNumDSBlocks = function () {
        return this.provider.send(core_1.RPCMethod.GetNumDSBlocks);
    };
    // Gets the average rate of DS blocks processed per second
    Blockchain.prototype.getDSBlockRate = function () {
        return this.provider.send(core_1.RPCMethod.GetDSBlockRate);
    };
    // Gets a paginated list of up to 10 Directory Service (DS) blocks
    // and their block hashes for a specified page.
    Blockchain.prototype.getDSBlockListing = function (max) {
        return this.provider.send(core_1.RPCMethod.DSBlockListing, max);
    };
    // Gets details of a Transaction block by block number.
    Blockchain.prototype.getTxBlock = function (blockNum) {
        return this.provider.send(core_1.RPCMethod.GetTxBlock, blockNum.toString());
    };
    // Gets details of the most recent Transaction block.
    Blockchain.prototype.getLatestTxBlock = function () {
        return this.provider.send(core_1.RPCMethod.GetLatestTxBlock);
    };
    // Gets the total number of TxBlocks.
    Blockchain.prototype.getNumTxBlocks = function () {
        return this.provider.send(core_1.RPCMethod.GetNumTxBlocks);
    };
    // Gets the average number of Tx blocks per second.
    Blockchain.prototype.getTxBlockRate = function () {
        return this.provider.send(core_1.RPCMethod.GetTxBlockRate);
    };
    // Get a paginated list of Transaction blocks.
    Blockchain.prototype.getTxBlockListing = function (max) {
        return this.provider.send(core_1.RPCMethod.TxBlockListing, max);
    };
    // Gets the number of transactions processed by the network so far.
    Blockchain.prototype.getNumTransactions = function () {
        return this.provider.send(core_1.RPCMethod.GetNumTransactions);
    };
    // Gets the number of transactions processed per second
    Blockchain.prototype.getTransactionRate = function () {
        return this.provider.send(core_1.RPCMethod.GetTransactionRate);
    };
    // Gets the current Tx Epoch.
    Blockchain.prototype.getCurrentMiniEpoch = function () {
        return this.provider.send(core_1.RPCMethod.GetCurrentMiniEpoch);
    };
    // Gets the current DS Epoch.
    Blockchain.prototype.getCurrentDSEpoch = function () {
        return this.provider.send(core_1.RPCMethod.GetCurrentDSEpoch);
    };
    // Gets shard difficulty for previous PoW round
    Blockchain.prototype.getPrevDifficulty = function () {
        return this.provider.send(core_1.RPCMethod.GetPrevDifficulty);
    };
    // Gets DS difficulty for previous PoW round
    Blockchain.prototype.getPrevDSDifficulty = function () {
        return this.provider.send(core_1.RPCMethod.GetPrevDSDifficulty);
    };
    // Returns the total supply (ZIL) of coins in the network.
    Blockchain.prototype.getTotalCoinSupply = function () {
        return this.provider.send(core_1.RPCMethod.GetTotalCoinSupply);
    };
    // Returns the mining nodes (i.e., the members of the DS committee and shards)
    // at the specified DS block.
    Blockchain.prototype.getMinerInfo = function (dsBlockNumber) {
        return this.provider.send(core_1.RPCMethod.GetMinerInfo, dsBlockNumber);
    };
    // Creates a transaction and polls the lookup node for a transaction receipt.
    Blockchain.prototype.createTransaction = function (tx, maxAttempts, interval, blockConfirm) {
        if (maxAttempts === void 0) { maxAttempts = core_1.GET_TX_ATTEMPTS; }
        if (interval === void 0) { interval = 1000; }
        if (blockConfirm === void 0) { blockConfirm = false; }
        return (0, tslib_1.__awaiter)(this, void 0, void 0, function () {
            var response, err_1;
            return (0, tslib_1.__generator)(this, function (_a) {
                switch (_a.label) {
                    case 0:
                        _a.trys.push([0, 2, , 3]);
                        return [4 /*yield*/, this.provider.send(core_1.RPCMethod.CreateTransaction, (0, tslib_1.__assign)((0, tslib_1.__assign)({}, tx.txParams), { priority: tx.toDS }))];
                    case 1:
                        response = _a.sent();
                        if (response.error) {
                            throw response.error;
                        }
                        if (blockConfirm) {
                            return [2 /*return*/, tx.blockConfirm(response.result.TranID, maxAttempts, interval)];
                        }
                        return [2 /*return*/, tx.confirm(response.result.TranID, maxAttempts, interval)];
                    case 2:
                        err_1 = _a.sent();
                        throw err_1;
                    case 3: return [2 /*return*/];
                }
            });
        });
    };
    // used together with signed batch
    // this method waits for each txn to confirm
    // see @createBatchTransactionWithoutConfirm for transactions without confirmation
    Blockchain.prototype.createBatchTransaction = function (signedTxList, maxAttempts, interval, blockConfirm) {
        if (maxAttempts === void 0) { maxAttempts = core_1.GET_TX_ATTEMPTS; }
        if (interval === void 0) { interval = 1000; }
        if (blockConfirm === void 0) { blockConfirm = false; }
        return (0, tslib_1.__awaiter)(this, void 0, void 0, function () {
            var txParamsList, signedTxList_1, signedTxList_1_1, signedTx, response, batchResults, i, tx, txRes, _a, _b, _c, _d, err_2;
            var e_1, _e;
            return (0, tslib_1.__generator)(this, function (_f) {
                switch (_f.label) {
                    case 0:
                        _f.trys.push([0, 8, , 9]);
                        txParamsList = [];
                        try {
                            for (signedTxList_1 = (0, tslib_1.__values)(signedTxList), signedTxList_1_1 = signedTxList_1.next(); !signedTxList_1_1.done; signedTxList_1_1 = signedTxList_1.next()) {
                                signedTx = signedTxList_1_1.value;
                                if (signedTx.txParams.signature === undefined) {
                                    throw new Error('The transaction is not signed.');
                                }
                                txParamsList.push((0, tslib_1.__assign)((0, tslib_1.__assign)({}, signedTx.txParams), { priority: signedTx.toDS }));
                            }
                        }
                        catch (e_1_1) { e_1 = { error: e_1_1 }; }
                        finally {
                            try {
                                if (signedTxList_1_1 && !signedTxList_1_1.done && (_e = signedTxList_1.return)) _e.call(signedTxList_1);
                            }
                            finally { if (e_1) throw e_1.error; }
                        }
                        return [4 /*yield*/, this.provider.sendBatch(core_1.RPCMethod.CreateTransaction, txParamsList)];
                    case 1:
                        response = _f.sent();
                        if (response.error) {
                            throw response.error;
                        }
                        batchResults = [];
                        i = 0;
                        _f.label = 2;
                    case 2:
                        if (!(i < signedTxList.length)) return [3 /*break*/, 7];
                        tx = signedTxList[i];
                        txRes = response.batch_result[i];
                        if (!blockConfirm) return [3 /*break*/, 4];
                        _b = (_a = batchResults).push;
                        return [4 /*yield*/, tx.blockConfirm(txRes.result.TranID, maxAttempts, interval)];
                    case 3:
                        _b.apply(_a, [_f.sent()]);
                        return [3 /*break*/, 6];
                    case 4:
                        _d = (_c = batchResults).push;
                        return [4 /*yield*/, tx.confirm(txRes.result.TranID, maxAttempts, interval)];
                    case 5:
                        _d.apply(_c, [_f.sent()]);
                        _f.label = 6;
                    case 6:
                        i++;
                        return [3 /*break*/, 2];
                    case 7: return [2 /*return*/, batchResults];
                    case 8:
                        err_2 = _f.sent();
                        throw err_2;
                    case 9: return [2 /*return*/];
                }
            });
        });
    };
    // Create a transaction by using a exist signed transaction payload
    // This payload may come form some offline signing software like ledger
    // Currently we haven't supported convert a singed transaction back to transaction param, so we won't perform
    // confirm logic here.
    Blockchain.prototype.createTransactionRaw = function (payload) {
        return (0, tslib_1.__awaiter)(this, void 0, void 0, function () {
            var tx, response, err_3;
            return (0, tslib_1.__generator)(this, function (_a) {
                switch (_a.label) {
                    case 0:
                        _a.trys.push([0, 2, , 3]);
                        tx = JSON.parse(payload);
                        return [4 /*yield*/, this.provider.send(core_1.RPCMethod.CreateTransaction, tx)];
                    case 1:
                        response = _a.sent();
                        if (response.error) {
                            throw response.error;
                        }
                        return [2 /*return*/, response.result.TranID];
                    case 2:
                        err_3 = _a.sent();
                        throw err_3;
                    case 3: return [2 /*return*/];
                }
            });
        });
    };
    Blockchain.prototype.createTransactionWithoutConfirm = function (tx) {
        return (0, tslib_1.__awaiter)(this, void 0, void 0, function () {
            var response, err_4;
            return (0, tslib_1.__generator)(this, function (_a) {
                switch (_a.label) {
                    case 0:
                        _a.trys.push([0, 2, , 3]);
                        return [4 /*yield*/, this.provider.send(core_1.RPCMethod.CreateTransaction, (0, tslib_1.__assign)((0, tslib_1.__assign)({}, tx.txParams), { priority: tx.toDS }))];
                    case 1:
                        response = _a.sent();
                        if (response.error) {
                            throw response.error;
                        }
                        tx.id = response.result.TranID;
                        return [2 /*return*/, tx];
                    case 2:
                        err_4 = _a.sent();
                        throw err_4;
                    case 3: return [2 /*return*/];
                }
            });
        });
    };
    // used together with signed batch
    Blockchain.prototype.createBatchTransactionWithoutConfirm = function (signedTxList) {
        return (0, tslib_1.__awaiter)(this, void 0, void 0, function () {
            var txParamsList, signedTxList_2, signedTxList_2_1, signedTx, response, batchResults, i, tx, txRes, err_5;
            var e_2, _a;
            return (0, tslib_1.__generator)(this, function (_b) {
                switch (_b.label) {
                    case 0:
                        _b.trys.push([0, 2, , 3]);
                        txParamsList = [];
                        try {
                            for (signedTxList_2 = (0, tslib_1.__values)(signedTxList), signedTxList_2_1 = signedTxList_2.next(); !signedTxList_2_1.done; signedTxList_2_1 = signedTxList_2.next()) {
                                signedTx = signedTxList_2_1.value;
                                if (signedTx.txParams.signature === undefined) {
                                    throw new Error('The transaction is not signed.');
                                }
                                txParamsList.push((0, tslib_1.__assign)((0, tslib_1.__assign)({}, signedTx.txParams), { priority: signedTx.toDS }));
                            }
                        }
                        catch (e_2_1) { e_2 = { error: e_2_1 }; }
                        finally {
                            try {
                                if (signedTxList_2_1 && !signedTxList_2_1.done && (_a = signedTxList_2.return)) _a.call(signedTxList_2);
                            }
                            finally { if (e_2) throw e_2.error; }
                        }
                        return [4 /*yield*/, this.provider.sendBatch(core_1.RPCMethod.CreateTransaction, txParamsList)];
                    case 1:
                        response = _b.sent();
                        if (response.error) {
                            throw response.error;
                        }
                        batchResults = [];
                        for (i = 0; i < signedTxList.length; i++) {
                            tx = signedTxList[i];
                            txRes = response.batch_result[i];
                            tx.id = txRes.result.TranID;
                            batchResults.push(tx);
                        }
                        return [2 /*return*/, batchResults];
                    case 2:
                        err_5 = _b.sent();
                        throw err_5;
                    case 3: return [2 /*return*/];
                }
            });
        });
    };
    // Returns the details of a specified Transaction.
    Blockchain.prototype.getTransaction = function (txHash) {
        return (0, tslib_1.__awaiter)(this, void 0, void 0, function () {
            var response, err_6;
            return (0, tslib_1.__generator)(this, function (_a) {
                switch (_a.label) {
                    case 0:
                        _a.trys.push([0, 2, , 3]);
                        return [4 /*yield*/, this.provider.send(core_1.RPCMethod.GetTransaction, txHash.replace('0x', ''))];
                    case 1:
                        response = _a.sent();
                        if (response.error) {
                            return [2 /*return*/, Promise.reject(response.error)];
                        }
                        return [2 /*return*/, response.result.receipt.success
                                ? account_1.Transaction.confirm((0, util_2.toTxParams)(response), this.provider)
                                : account_1.Transaction.reject((0, util_2.toTxParams)(response), this.provider)];
                    case 2:
                        err_6 = _a.sent();
                        throw err_6;
                    case 3: return [2 /*return*/];
                }
            });
        });
    };
    // Returns the status of a specified transaction.
    Blockchain.prototype.getTransactionStatus = function (txHash) {
        return (0, tslib_1.__awaiter)(this, void 0, void 0, function () {
            var response, modificationState, status_1, err_7;
            return (0, tslib_1.__generator)(this, function (_a) {
                switch (_a.label) {
                    case 0:
                        _a.trys.push([0, 2, , 3]);
                        return [4 /*yield*/, this.provider.send(core_1.RPCMethod.GetTransactionStatus, txHash.replace('0x', ''))];
                    case 1:
                        response = _a.sent();
                        if (response.error) {
                            return [2 /*return*/, Promise.reject(response.error)];
                        }
                        modificationState = response.result.modificationState;
                        status_1 = response.result.status;
                        response.result.statusMessage =
                            this.transactionStatusMap[modificationState][status_1];
                        return [2 /*return*/, response.result];
                    case 2:
                        err_7 = _a.sent();
                        throw err_7;
                    case 3: return [2 /*return*/];
                }
            });
        });
    };
    // Gets a list of recent transactions
    Blockchain.prototype.getRecentTransactions = function () {
        return this.provider.send(core_1.RPCMethod.GetRecentTransactions);
    };
    // Returns the validated transactions included
    // within a specified final transaction block as an array of
    // length i, where i is the number of shards plus the DS committee.
    Blockchain.prototype.getTransactionsForTxBlock = function (txBlock) {
        return this.provider.send(core_1.RPCMethod.GetTransactionsForTxBlock, txBlock.toString());
    };
    // returns the transactions in batches (or pages) of 2,500.
    // This API behaves similar to GetTransactionsForTxBlock
    Blockchain.prototype.getTransactionsForTxBlockEx = function (txBlock) {
        if (!isBlockNumber(txBlock)) {
            throw new Error('invalid txBlock');
        }
        return this.provider.send(core_1.RPCMethod.GetTransactionsForTxBlockEx, txBlock.toString());
    };
    // Returns the validated transactions (in verbose form)
    // included within a specified final transaction block.
    Blockchain.prototype.getTxnBodiesForTxBlock = function (txBlock) {
        return this.provider.send(core_1.RPCMethod.GetTxnBodiesForTxBlock, txBlock.toString());
    };
    // Returns the transactions in batches (or pages) of 2,500
    // This API behaves similar to GetTxBodiesForTxBlock
    Blockchain.prototype.getTxnBodiesForTxBlockEx = function (txBlock) {
        if (!isBlockNumber(txBlock)) {
            throw new Error('invalid txBlock');
        }
        return this.provider.send(core_1.RPCMethod.GetTxnBodiesForTxBlockEx, txBlock.toString());
    };
    // Gets the number of transactions procesed for a given Tx Epoch.
    Blockchain.prototype.getNumTxnsTxEpoch = function (epoch) {
        return this.provider.send(core_1.RPCMethod.GetNumTxnsTxEpoch, epoch);
    };
    // Gets the number of transactions procesed for a given DS Epoch.
    Blockchain.prototype.getNumTxnsDSEpoch = function (epoch) {
        return this.provider.send(core_1.RPCMethod.GetNumTxnsDSEpoch, epoch);
    };
    // Gets the numeric minimum gas price.
    Blockchain.prototype.getMinimumGasPrice = function () {
        return this.provider.send(core_1.RPCMethod.GetMinimumGasPrice);
    };
    // Gets the balance of an account by address.
    Blockchain.prototype.getBalance = function (addr) {
        var address = util_1.validation.isBech32(addr) ? (0, crypto_1.fromBech32Address)(addr) : addr;
        return this.provider.send(core_1.RPCMethod.GetBalance, address.replace('0x', '').toLowerCase());
    };
    // Returns the Scilla code associated with a smart contract address
    Blockchain.prototype.getSmartContractCode = function (addr) {
        var address = util_1.validation.isBech32(addr) ? (0, crypto_1.fromBech32Address)(addr) : addr;
        return this.provider.send(core_1.RPCMethod.GetSmartContractCode, address.replace('0x', '').toLowerCase());
    };
    // Returns the initialization (immutable) parameters of
    // a given smart contract, represented in a JSON format.
    Blockchain.prototype.getSmartContractInit = function (addr) {
        var address = util_1.validation.isBech32(addr) ? (0, crypto_1.fromBech32Address)(addr) : addr;
        return this.provider.send(core_1.RPCMethod.GetSmartContractInit, address.replace('0x', '').toLowerCase());
    };
    // Retrieves the entire state of a smart contract.
    Blockchain.prototype.getSmartContractState = function (addr) {
        var address = util_1.validation.isBech32(addr) ? (0, crypto_1.fromBech32Address)(addr) : addr;
        return this.provider.send(core_1.RPCMethod.GetSmartContractState, address.replace('0x', '').toLowerCase());
    };
    // Queries the contract state, filtered by the variable names.
    Blockchain.prototype.getSmartContractSubState = function (addr, variableName, indices) {
        var address = util_1.validation.isBech32(addr) ? (0, crypto_1.fromBech32Address)(addr) : addr;
        if (!variableName) {
            throw new Error('Variable name required');
        }
        return this.provider.send(core_1.RPCMethod.GetSmartContractSubState, address.replace('0x', '').toLowerCase(), variableName, indices === undefined ? [] : indices);
    };
    // Queries the contract state using batch rpc.
    Blockchain.prototype.getSmartContractSubStateBatch = function (reqs) {
        return this.provider.sendBatch(core_1.RPCMethod.GetSmartContractSubState, reqs);
    };
    Blockchain.prototype.getSmartContracts = function (addr) {
        var address = util_1.validation.isBech32(addr) ? (0, crypto_1.fromBech32Address)(addr) : addr;
        return this.provider.send(core_1.RPCMethod.GetSmartContracts, address.replace('0x', '').toLowerCase());
    };
    Blockchain.prototype.getContractAddressFromTransactionID = function (txHash) {
        return this.provider.send(core_1.RPCMethod.GetContractAddressFromTransactionID, txHash);
    };
    // Returns the state proof for the corresponding TxBlock for a smart contract.
    Blockchain.prototype.getStateProof = function (contractAddress, sha256Hash, txBlock) {
        var address = util_1.validation.isBech32(contractAddress)
            ? (0, crypto_1.fromBech32Address)(contractAddress)
            : contractAddress;
        var isLatestStr = txBlock === 'latest';
        var isValid = isLatestStr || isBlockNumber(Number(txBlock));
        if (!isValid) {
            throw new Error('invalid txBlock');
        }
        return this.provider.send(core_1.RPCMethod.GetStateProof, address.replace('0x', '').toLowerCase(), sha256Hash, txBlock.toString());
    };
    (0, tslib_1.__decorate)([
        core_1.sign,
        (0, tslib_1.__metadata)("design:type", Function),
        (0, tslib_1.__metadata)("design:paramtypes", [account_1.Transaction, Number, Number, Boolean]),
        (0, tslib_1.__metadata)("design:returntype", Promise)
    ], Blockchain.prototype, "createTransaction", null);
    (0, tslib_1.__decorate)([
        core_1.sign,
        (0, tslib_1.__metadata)("design:type", Function),
        (0, tslib_1.__metadata)("design:paramtypes", [account_1.Transaction]),
        (0, tslib_1.__metadata)("design:returntype", Promise)
    ], Blockchain.prototype, "createTransactionWithoutConfirm", null);
    return Blockchain;
}());
exports.Blockchain = Blockchain;
//# sourceMappingURL=chain.js.map