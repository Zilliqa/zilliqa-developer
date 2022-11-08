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
exports.Contracts = void 0;
var tslib_1 = require("tslib");
var hash_js_1 = (0, tslib_1.__importDefault)(require("hash.js"));
var account_1 = require("@zilliqa-js/account");
var crypto_1 = require("@zilliqa-js/crypto");
var core_1 = require("@zilliqa-js/core");
var util_1 = require("@zilliqa-js/util");
var contract_1 = require("./contract");
/**
 * Contracts
 *
 * Unlike most zilliqa-js modules, `Contracts` is a factory class.
 * As a result, individual `Contract` instances are instead obtained by
 * calling `Contracts.at` (for an already-deployed contract) and
 * `Contracts.new` (to deploy a new contract).
 */
var Contracts = /** @class */ (function () {
    function Contracts(provider, signer) {
        this.provider = provider;
        this.provider.middleware.request.use(account_1.util.formatOutgoingTx, core_1.RPCMethod.CreateTransaction);
        this.signer = signer;
    }
    /**
     * getAddressForContract
     *
     * @static
     * @param {Transaction} tx - transaction used to create the contract
     * @returns {string} - the contract address
     */
    Contracts.getAddressForContract = function (tx) {
        // always subtract 1 from the tx nonce, as contract addresses are computed
        // based on the nonce in the global state.
        var nonce = tx.txParams.nonce ? tx.txParams.nonce - 1 : 0;
        return (0, crypto_1.toChecksumAddress)(hash_js_1.default
            .sha256()
            .update(tx.senderAddress.replace('0x', '').toLowerCase(), 'hex')
            .update(util_1.bytes.intToHexArray(nonce, 16).join(''), 'hex')
            .digest('hex')
            .slice(24));
    };
    Contracts.prototype.at = function (address, abi, code, init, state) {
        return new contract_1.Contract(this, code, abi, address, init, state);
    };
    Contracts.prototype.atBech32 = function (address, abi, code, init, state) {
        return new contract_1.Contract(this, code, abi, address, init, state, true);
    };
    Contracts.prototype.new = function (code, init, abi) {
        return new contract_1.Contract(this, code, abi, undefined, init);
    };
    return Contracts;
}());
exports.Contracts = Contracts;
//# sourceMappingURL=factory.js.map