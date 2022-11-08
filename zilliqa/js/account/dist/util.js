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
exports.sleep = exports.formatOutgoingTx = exports.isTxParams = exports.isTxReceipt = exports.encodeTransactionProto = void 0;
var tslib_1 = require("tslib");
var core_1 = require("@zilliqa-js/core");
var util_1 = require("@zilliqa-js/util");
var proto_1 = require("@zilliqa-js/proto");
var encodeTransactionProto = function (tx) {
    var msg = {
        version: tx.version,
        nonce: tx.nonce || 0,
        // core protocol Schnorr expects lowercase, non-prefixed address.
        toaddr: util_1.bytes.hexToByteArray(tx.toAddr.replace('0x', '').toLowerCase()),
        senderpubkey: proto_1.ZilliqaMessage.ByteArray.create({
            data: util_1.bytes.hexToByteArray(tx.pubKey || '00'),
        }),
        amount: proto_1.ZilliqaMessage.ByteArray.create({
            data: Uint8Array.from(tx.amount.toArrayLike(Buffer, undefined, 16)),
        }),
        gasprice: proto_1.ZilliqaMessage.ByteArray.create({
            data: Uint8Array.from(tx.gasPrice.toArrayLike(Buffer, undefined, 16)),
        }),
        gaslimit: tx.gasLimit,
        code: tx.code && tx.code.length
            ? Uint8Array.from((0, tslib_1.__spreadArray)([], (0, tslib_1.__read)(tx.code), false).map(function (c) { return c.charCodeAt(0); }))
            : null,
        data: tx.data && tx.data.length
            ? Uint8Array.from((0, tslib_1.__spreadArray)([], (0, tslib_1.__read)(tx.data), false).map(function (c) { return c.charCodeAt(0); }))
            : null,
    };
    var serialised = proto_1.ZilliqaMessage.ProtoTransactionCoreInfo.create(msg);
    return Buffer.from(proto_1.ZilliqaMessage.ProtoTransactionCoreInfo.encode(serialised).finish());
};
exports.encodeTransactionProto = encodeTransactionProto;
var isTxReceipt = function (x) {
    return util_1.validation.isPlainObject(x) && util_1.validation.matchesObject(x, {});
};
exports.isTxReceipt = isTxReceipt;
var isTxParams = function (obj) {
    var validator = {
        version: [util_1.validation.required(util_1.validation.isNumber)],
        toAddr: [util_1.validation.required(util_1.validation.isAddress)],
        amount: [util_1.validation.required(util_1.validation.isBN)],
        gasPrice: [util_1.validation.required(util_1.validation.isBN)],
        gasLimit: [util_1.validation.required(util_1.validation.isLong)],
        code: [util_1.validation.isString],
        data: [util_1.validation.isString],
        receipt: [exports.isTxReceipt],
        nonce: [util_1.validation.required(util_1.validation.isNumber)],
        signature: [util_1.validation.required(util_1.validation.isSignature)],
    };
    return util_1.validation.matchesObject(obj, validator);
};
exports.isTxParams = isTxParams;
var formatOutgoingTx = function (req) {
    var e_1, _a;
    // if batch create transaction, payload is array
    if (Array.isArray(req.payload) &&
        req.payload[0].method === core_1.RPCMethod.CreateTransaction &&
        (0, exports.isTxParams)(req.payload[0].params[0])) {
        // loop thru batch payloads and format the params
        var payloads = [];
        try {
            for (var _b = (0, tslib_1.__values)(req.payload), _c = _b.next(); !_c.done; _c = _b.next()) {
                var txPayload = _c.value;
                var txConfig = txPayload.params[0];
                payloads.push((0, tslib_1.__assign)((0, tslib_1.__assign)({}, txPayload), { params: [
                        (0, tslib_1.__assign)((0, tslib_1.__assign)({}, txConfig), { amount: txConfig.amount.toString(), gasLimit: txConfig.gasLimit.toString(), gasPrice: txConfig.gasPrice.toString() }),
                    ] }));
            }
        }
        catch (e_1_1) { e_1 = { error: e_1_1 }; }
        finally {
            try {
                if (_c && !_c.done && (_a = _b.return)) _a.call(_b);
            }
            finally { if (e_1) throw e_1.error; }
        }
        var ret = (0, tslib_1.__assign)((0, tslib_1.__assign)({}, req), { payload: payloads });
        return ret;
    }
    // non-batch create transactions
    if (!Array.isArray(req.payload) &&
        req.payload.method === core_1.RPCMethod.CreateTransaction &&
        (0, exports.isTxParams)(req.payload.params[0])) {
        var txConfig = req.payload.params[0];
        var ret = (0, tslib_1.__assign)((0, tslib_1.__assign)({}, req), { payload: (0, tslib_1.__assign)((0, tslib_1.__assign)({}, req.payload), { params: [
                    (0, tslib_1.__assign)((0, tslib_1.__assign)({}, txConfig), { amount: txConfig.amount.toString(), gasLimit: txConfig.gasLimit.toString(), gasPrice: txConfig.gasPrice.toString() }),
                ] }) });
        return ret;
    }
    return req;
};
exports.formatOutgoingTx = formatOutgoingTx;
function sleep(ms) {
    return (0, tslib_1.__awaiter)(this, void 0, void 0, function () {
        return (0, tslib_1.__generator)(this, function (_a) {
            return [2 /*return*/, new Promise(function (resolve) {
                    setTimeout(function () { return resolve(undefined); }, ms);
                })];
        });
    });
}
exports.sleep = sleep;
//# sourceMappingURL=util.js.map