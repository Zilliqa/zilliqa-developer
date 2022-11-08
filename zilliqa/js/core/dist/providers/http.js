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
exports.HTTPProvider = void 0;
var tslib_1 = require("tslib");
var base_1 = require("./base");
var net_1 = require("../net");
var util_1 = require("../util");
var HTTPProvider = /** @class */ (function (_super) {
    (0, tslib_1.__extends)(HTTPProvider, _super);
    function HTTPProvider() {
        return _super !== null && _super.apply(this, arguments) || this;
    }
    HTTPProvider.prototype.buildPayload = function (method, params) {
        return {
            url: this.nodeURL,
            payload: { id: 1, jsonrpc: '2.0', method: method, params: params },
        };
    };
    HTTPProvider.prototype.buildBatchPayload = function (method, paramsList) {
        var payloads = [];
        for (var i = 0; i < paramsList.length; i++) {
            // most of the payloads should be a single param, e.g. GetTransaction
            // however, there are special cases e.g. GetSmartContractSubState & GetTransactionsForTxBlockEx
            // where the param field is a list
            var payloadParams = paramsList[i];
            var params = void 0;
            if (Array.isArray(payloadParams)) {
                // for those param field that is already a list
                params = payloadParams;
            }
            else {
                params = [payloadParams];
            }
            // id start from index 1
            payloads.push({
                id: i + 1,
                jsonrpc: '2.0',
                method: method,
                params: params,
            });
        }
        return {
            url: this.nodeURL,
            payload: payloads,
        };
    };
    HTTPProvider.prototype.send = function (method) {
        var params = [];
        for (var _i = 1; _i < arguments.length; _i++) {
            params[_i - 1] = arguments[_i];
        }
        var _a = (0, tslib_1.__read)(this.getMiddleware(method), 2), tReq = _a[0], tRes = _a[1];
        var reqMiddleware = util_1.composeMiddleware.apply(void 0, (0, tslib_1.__spreadArray)([], (0, tslib_1.__read)(tReq), false));
        var resMiddleware = util_1.composeMiddleware.apply(void 0, (0, tslib_1.__spreadArray)([], (0, tslib_1.__read)(tRes), false));
        var req = reqMiddleware(this.buildPayload(method, params));
        return (0, net_1.performRPC)(req, resMiddleware);
    };
    HTTPProvider.prototype.sendBatch = function (method, params) {
        var _a = (0, tslib_1.__read)(this.getMiddleware(method), 2), tReq = _a[0], tRes = _a[1];
        var reqMiddleware = util_1.composeMiddleware.apply(void 0, (0, tslib_1.__spreadArray)([], (0, tslib_1.__read)(tReq), false));
        var resMiddleware = util_1.composeMiddleware.apply(void 0, (0, tslib_1.__spreadArray)([], (0, tslib_1.__read)(tRes), false));
        var batchPayload = this.buildBatchPayload(method, params);
        var req = reqMiddleware(batchPayload);
        return (0, net_1.performBatchRPC)(req, resMiddleware);
    };
    HTTPProvider.prototype.subscribe = function (event, subscriber) {
        throw new Error('HTTPProvider does not support subscriptions.');
    };
    HTTPProvider.prototype.unsubscribe = function (token) {
        throw new Error('HTTPProvider does not support subscriptions.');
    };
    return HTTPProvider;
}(base_1.BaseProvider));
exports.HTTPProvider = HTTPProvider;
//# sourceMappingURL=http.js.map