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
exports.normalizePrivateKey = exports.verifyPrivateKey = exports.decodeBase58 = exports.encodeBase58 = exports.normaliseAddress = exports.isValidChecksumAddress = exports.toChecksumAddress = exports.getAddressFromPublicKey = exports.compressPublicKey = exports.getAccountFrom0xPrivateKey = exports.getPubKeyFromPrivateKey = exports.getAddressFromPrivateKey = void 0;
var tslib_1 = require("tslib");
var elliptic_1 = (0, tslib_1.__importDefault)(require("elliptic"));
var hash_js_1 = (0, tslib_1.__importDefault)(require("hash.js"));
var util_1 = require("@zilliqa-js/util");
var bech32_1 = require("./bech32");
var secp256k1 = new elliptic_1.default.ec('secp256k1');
/**
 * getAddressFromPrivateKey
 *
 * takes a hex-encoded string (private key) and returns its corresponding
 * 20-byte hex-encoded address.
 *
 * @param {string} privateKey
 * @returns {string}
 */
var getAddressFromPrivateKey = function (privateKey) {
    var normalizedPrviateKey = (0, exports.normalizePrivateKey)(privateKey);
    var keyPair = secp256k1.keyFromPrivate(normalizedPrviateKey, 'hex');
    var pub = keyPair.getPublic(true, 'hex');
    return (0, exports.toChecksumAddress)(hash_js_1.default.sha256().update(pub, 'hex').digest('hex').slice(24));
};
exports.getAddressFromPrivateKey = getAddressFromPrivateKey;
/**
 * getPubKeyFromPrivateKey
 *
 * takes a hex-encoded string (private key) and returns its corresponding
 * hex-encoded 33-byte public key.
 *
 * @param {string} privateKey
 * @returns {string}
 */
var getPubKeyFromPrivateKey = function (privateKey) {
    var normalizedPrviateKey = (0, exports.normalizePrivateKey)(privateKey);
    var keyPair = secp256k1.keyFromPrivate(normalizedPrviateKey, 'hex');
    return keyPair.getPublic(true, 'hex');
};
exports.getPubKeyFromPrivateKey = getPubKeyFromPrivateKey;
/**
 * getAccountFrom0xPrivateKey
 *
 * Utility method for recovering account from 0x private key.
 * See https://github.com/Zilliqa/zilliqa-js/pull/159
 * @param privateKeyWith0x : private key with 0x prefix
 */
var getAccountFrom0xPrivateKey = function (privateKeyWith0x) {
    var privateKeyWithout0x = (0, exports.normalizePrivateKey)(privateKeyWith0x);
    var keyPair = secp256k1.keyFromPrivate(privateKeyWith0x, 'hex');
    var publicKeyWith0x = keyPair.getPublic(true, 'hex');
    var addressWith0x = (0, exports.getAddressFromPublicKey)(publicKeyWith0x);
    var bech32With0x = (0, bech32_1.toBech32Address)(addressWith0x);
    var with0x = {
        prv: privateKeyWith0x,
        pub: publicKeyWith0x,
        addr: addressWith0x,
        bech32: bech32With0x,
    };
    var keyPair2 = secp256k1.keyFromPrivate(privateKeyWithout0x, 'hex');
    var publicKeyWithout0x = keyPair2.getPublic(true, 'hex');
    var addressWithout0x = (0, exports.getAddressFromPublicKey)(publicKeyWithout0x);
    var bech32Without0x = (0, bech32_1.toBech32Address)(addressWithout0x);
    var without0x = {
        prv: privateKeyWithout0x,
        pub: publicKeyWithout0x,
        addr: addressWithout0x,
        bech32: bech32Without0x,
    };
    var privateKeyAfterChange = keyPair.getPrivate('hex');
    var publicKeyAfterChange = keyPair.getPublic(true, 'hex');
    var addressAfterChange = (0, exports.getAddressFromPublicKey)(publicKeyAfterChange);
    var bech32AfterChange = (0, bech32_1.toBech32Address)(addressAfterChange);
    var changed = {
        prv: privateKeyAfterChange,
        pub: publicKeyAfterChange,
        addr: addressAfterChange,
        bech32: bech32AfterChange,
    };
    return {
        with0x: with0x,
        without0x: without0x,
        changed: changed,
    };
};
exports.getAccountFrom0xPrivateKey = getAccountFrom0xPrivateKey;
/**
 * compressPublicKey
 *
 * @param {string} publicKey - 65-byte public key, a point (x, y)
 *
 * @returns {string}
 */
var compressPublicKey = function (publicKey) {
    return secp256k1.keyFromPublic(publicKey, 'hex').getPublic(true, 'hex');
};
exports.compressPublicKey = compressPublicKey;
/**
 * getAddressFromPublicKey
 *
 * takes hex-encoded string and returns the corresponding address
 *
 * @param {string} pubKey
 * @returns {string}
 */
var getAddressFromPublicKey = function (publicKey) {
    var normalized = publicKey.toLowerCase().replace('0x', '');
    return (0, exports.toChecksumAddress)(hash_js_1.default.sha256().update(normalized, 'hex').digest('hex').slice(24));
};
exports.getAddressFromPublicKey = getAddressFromPublicKey;
/**
 * toChecksumAddress
 *
 * takes hex-encoded string and returns the corresponding address
 *
 * @param {string} address
 * @returns {string}
 */
var toChecksumAddress = function (address) {
    if (!util_1.validation.isAddress(address)) {
        throw new Error(address + " is not a valid base 16 address");
    }
    address = address.toLowerCase().replace('0x', '');
    var hash = hash_js_1.default.sha256().update(address, 'hex').digest('hex');
    var v = new util_1.BN(hash, 'hex', 'be');
    var ret = '0x';
    for (var i = 0; i < address.length; i++) {
        if ('0123456789'.indexOf(address[i]) !== -1) {
            ret += address[i];
        }
        else {
            ret += v.and(new util_1.BN(2).pow(new util_1.BN(255 - 6 * i))).gte(new util_1.BN(1))
                ? address[i].toUpperCase()
                : address[i].toLowerCase();
        }
    }
    return ret;
};
exports.toChecksumAddress = toChecksumAddress;
/**
 * isValidChecksumAddress
 *
 * takes hex-encoded string and returns boolean if address is checksumed
 *
 * @param {string} address
 * @returns {boolean}
 */
var isValidChecksumAddress = function (address) {
    return (util_1.validation.isAddress(address.replace('0x', '')) &&
        (0, exports.toChecksumAddress)(address) === address);
};
exports.isValidChecksumAddress = isValidChecksumAddress;
/**
 * normaliseAddress
 *
 * takes in a base16 address or a zilliqa bech32 encoded address
 * and returns a checksum base16 address. If the address is neither a base16
 * nor bech32 address, the code will return an error
 * @param {string)} address
 * @returns {string}
 */
var normaliseAddress = function (address) {
    if (util_1.validation.isBech32(address)) {
        return (0, bech32_1.fromBech32Address)(address);
    }
    if (!(0, exports.isValidChecksumAddress)(address)) {
        throw Error('Wrong address format, should be either bech32 or checksummed address');
    }
    return address;
};
exports.normaliseAddress = normaliseAddress;
/**
 * encodeBase58 - may be required for DID public key
 * undeprecating this function after version 2.0.0
 *
 * @param {string} hex - base 16 encoded string
 * @returns {string} - big endian base 58 encoded string
 */
var encodeBase58 = function (hex) {
    var clean = hex.toLowerCase().replace('0x', '');
    var tbl = '123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz';
    var base = new util_1.BN(58);
    var zero = new util_1.BN(0);
    var x = new util_1.BN(clean, 16);
    var res = '';
    while (x.gt(zero)) {
        var rem = x.mod(base).toNumber(); // safe, always < 58
        // big endian
        res = tbl[rem] + res;
        // quotient, remainders thrown away in integer division
        x = x.div(base);
    }
    // convert to big endian in case the input hex is little endian
    var hexBE = x.toString('hex', clean.length);
    for (var i = 0; i < hexBE.length; i += 2) {
        if (hex[i] === '0' && hex[i + 1] === '0') {
            res = tbl[0] + res;
        }
        else {
            break;
        }
    }
    return res;
};
exports.encodeBase58 = encodeBase58;
/**
 * decodeBase58 - may be required for DID public key
 * undeprecating this function after version 2.0.0
 *
 * @param {string} raw - base 58 string
 * @returns {string} - big endian base 16 string
 */
var decodeBase58 = function (raw) {
    var tbl = '123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz';
    var base = new util_1.BN(58);
    var zero = new util_1.BN(0);
    var isBreak = false;
    var n = new util_1.BN(0);
    var leader = '';
    for (var i = 0; i < raw.length; i++) {
        var char = raw.charAt(i);
        var weight = new util_1.BN(tbl.indexOf(char));
        n = n.mul(base).add(weight);
        // check if padding required
        if (!isBreak) {
            if (i - 1 > 0 && raw[i - 1] !== '1') {
                isBreak = true;
                continue;
            }
            if (char === '1') {
                leader += '00';
            }
        }
    }
    if (n.eq(zero)) {
        return leader;
    }
    var res = leader + n.toString('hex');
    if (res.length % 2 !== 0) {
        res = '0' + res;
    }
    return res;
};
exports.decodeBase58 = decodeBase58;
/**
 * verifyPrivateKey
 *
 * @param {string|Buffer} privateKey
 * @returns {boolean}
 */
var verifyPrivateKey = function (privateKey) {
    var keyPair = secp256k1.keyFromPrivate(privateKey, 'hex');
    var result = keyPair.validate().result;
    return result;
};
exports.verifyPrivateKey = verifyPrivateKey;
/**
 * normalizePrivateKey : normalise private key from 0x or without 0x prefix
 *
 * @param {string} privateKey
 * @returns {string}
 */
var normalizePrivateKey = function (privateKey) {
    try {
        if (!util_1.validation.isPrivateKey(privateKey)) {
            throw new Error('Private key is not correct');
        }
        var normalized = privateKey.toLowerCase().replace('0x', '');
        if (!(0, exports.verifyPrivateKey)(normalized)) {
            throw new Error('Private key is not correct');
        }
        return normalized;
    }
    catch (error) {
        throw error;
    }
};
exports.normalizePrivateKey = normalizePrivateKey;
//# sourceMappingURL=util.js.map