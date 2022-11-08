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
exports.Signature = exports.schnorr = exports.sign = void 0;
var tslib_1 = require("tslib");
var schnorr = (0, tslib_1.__importStar)(require("./schnorr"));
exports.schnorr = schnorr;
var signature_1 = require("./signature");
Object.defineProperty(exports, "Signature", { enumerable: true, get: function () { return signature_1.Signature; } });
/**
 * sign
 *
 * @param {string} hash - hex-encoded hash of the data to be signed
 *
 * @returns {string} the signature
 */
var sign = function (msg, privateKey, pubKey) {
    var sig = schnorr.sign(msg, Buffer.from(privateKey, 'hex'), Buffer.from(pubKey, 'hex'));
    var r = sig.r.toString('hex');
    var s = sig.s.toString('hex');
    while (r.length < 64) {
        r = '0' + r;
    }
    while (s.length < 64) {
        s = '0' + s;
    }
    return r + s;
};
exports.sign = sign;
(0, tslib_1.__exportStar)(require("./util"), exports);
(0, tslib_1.__exportStar)(require("./keystore"), exports);
(0, tslib_1.__exportStar)(require("./random"), exports);
(0, tslib_1.__exportStar)(require("./types"), exports);
(0, tslib_1.__exportStar)(require("./bech32"), exports);
//# sourceMappingURL=index.js.map