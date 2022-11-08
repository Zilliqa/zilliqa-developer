"use strict";
/*!
 * schnorr.js - schnorr signatures for bcoin
 * Copyright (c) 2017, Christopher Jeffrey (MIT License).
 * https://github.com/bcoin-org/bcoin
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.toSignature = exports.verify = exports.trySign = exports.sign = exports.hash = exports.generatePrivateKey = void 0;
var tslib_1 = require("tslib");
/**
 * This software is licensed under the MIT License.
 *
 * Copyright (c) 2017, Christopher Jeffrey (https://github.com/chjj)
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 */
var elliptic_1 = require("elliptic");
var hash_js_1 = (0, tslib_1.__importDefault)(require("hash.js"));
var hmac_drbg_1 = (0, tslib_1.__importDefault)(require("hmac-drbg"));
var util_1 = require("@zilliqa-js/util");
var random_1 = require("./random");
var _1 = require(".");
var secp256k1 = new elliptic_1.ec('secp256k1');
var curve = secp256k1.curve;
var PRIVKEY_SIZE_BYTES = 32;
// Public key is a point (x, y) on the curve.
// Each coordinate requires 32 bytes.
// In its compressed form it suffices to store the x co-ordinate
// and the sign for y.
// Hence a total of 33 bytes.
var PUBKEY_COMPRESSED_SIZE_BYTES = 33;
// Personalization string used for HMAC-DRBG instantiation.
var ALG = Buffer.from('Schnorr+SHA256  ', 'ascii');
// The length in bytes of the string above.
var ALG_LEN = 16;
// The length in bytes of entropy inputs to HMAC-DRBG
var ENT_LEN = 32;
var HEX_ENC = 'hex';
/**
 * generatePrivateKey
 *
 * @returns {string} - the hex-encoded private key
 */
var generatePrivateKey = function () {
    return secp256k1
        .genKeyPair({
        entropy: (0, random_1.randomBytes)(secp256k1.curve.n.byteLength()),
        entropyEnc: HEX_ENC,
        pers: 'zilliqajs+secp256k1+SHA256',
    })
        .getPrivate()
        .toString(16, PRIVKEY_SIZE_BYTES * 2);
};
exports.generatePrivateKey = generatePrivateKey;
/**
 * Hash (r | M).
 * @param {Buffer} msg
 * @param {BN} r
 *
 * @returns {Buffer}
 */
var hash = function (q, pubkey, msg) {
    var sha256 = hash_js_1.default.sha256();
    var totalLength = PUBKEY_COMPRESSED_SIZE_BYTES * 2 + msg.byteLength; // 33 q + 33 pubkey + variable msgLen
    var Q = q.toArrayLike(Buffer, 'be', 33);
    var B = Buffer.allocUnsafe(totalLength);
    Q.copy(B, 0);
    pubkey.copy(B, 33);
    msg.copy(B, 66);
    return new util_1.BN(sha256.update(B).digest('hex'), 16);
};
exports.hash = hash;
/**
 * sign
 *
 * @param {Buffer} msg
 * @param {Buffer} key
 * @param {Buffer} pubkey
 *
 * @returns {Signature}
 */
var sign = function (msg, privKey, pubKey) {
    var prv = new util_1.BN(privKey);
    var drbg = getDRBG(msg);
    var len = curve.n.byteLength();
    var sig;
    while (!sig) {
        var k = new util_1.BN(drbg.generate(len));
        sig = (0, exports.trySign)(msg, k, prv, pubKey);
    }
    return sig;
};
exports.sign = sign;
/**
 * trySign
 *
 * @param {Buffer} msg - the message to sign over
 * @param {BN} k - output of the HMAC-DRBG
 * @param {BN} privateKey - the private key
 * @param {Buffer} pubKey - the public key
 *
 * @returns {Signature | null =>}
 */
var trySign = function (msg, k, privKey, pubKey) {
    if (privKey.isZero()) {
        throw new Error('Bad private key.');
    }
    if (privKey.gte(curve.n)) {
        throw new Error('Bad private key.');
    }
    // 1a. check that k is not 0
    if (k.isZero()) {
        return null;
    }
    // 1b. check that k is < the order of the group
    if (k.gte(curve.n)) {
        return null;
    }
    // 2. Compute commitment Q = kG, where g is the base point
    var Q = curve.g.mul(k);
    // convert the commitment to octets first
    var compressedQ = new util_1.BN(Q.encodeCompressed());
    // 3. Compute the challenge r = H(Q || pubKey || msg)
    // mod reduce the r value by the order of secp256k1, n
    var r = (0, exports.hash)(compressedQ, pubKey, msg).umod(curve.n);
    var h = r.clone();
    if (h.isZero()) {
        return null;
    }
    // 4. Compute s = k - r * prv
    // 4a. Compute r * prv
    var s = h.imul(privKey).umod(curve.n);
    // 4b. Compute s = k - r * prv mod n
    s = k.isub(s).umod(curve.n);
    if (s.isZero()) {
        return null;
    }
    return new _1.Signature({ r: r, s: s });
};
exports.trySign = trySign;
/**
 * Verify signature.
 *
 * @param {Buffer} msg
 * @param {Buffer} signature
 * @param {Buffer} key
 *
 * @returns {boolean}
 *
 * 1. Check if r,s is in [1, ..., order-1]
 * 2. Compute Q = sG + r*kpub
 * 3. If Q = O (the neutral point), return 0;
 * 4. r' = H(Q, kpub, m)
 * 5. return r' == r
 */
var verify = function (msg, signature, key) {
    var sig = new _1.Signature(signature);
    if (sig.s.isZero() || sig.r.isZero()) {
        throw new Error('Invalid signature');
    }
    if (sig.s.isNeg() || sig.r.isNeg()) {
        throw new Error('Invalid signature');
    }
    if (sig.s.gte(curve.n) || sig.r.gte(curve.n)) {
        throw new Error('Invalid signature');
    }
    var kpub = curve.decodePoint(key);
    if (!curve.validate(kpub)) {
        throw new Error('Invalid public key');
    }
    var l = kpub.mul(sig.r);
    var r = curve.g.mul(sig.s);
    var Q = l.add(r);
    if (Q.isInfinity()) {
        throw new Error('Invalid intermediate point.');
    }
    var compressedQ = new util_1.BN(Q.encodeCompressed());
    var r1 = (0, exports.hash)(compressedQ, key, msg).umod(curve.n);
    if (r1.isZero()) {
        throw new Error('Invalid hash.');
    }
    return r1.eq(sig.r);
};
exports.verify = verify;
var toSignature = function (serialised) {
    var r = serialised.slice(0, 64);
    var s = serialised.slice(64);
    return new _1.Signature({ r: r, s: s });
};
exports.toSignature = toSignature;
/**
 * Instantiate an HMAC-DRBG.
 *
 * @param {Buffer} msg - used as nonce
 *
 * @returns {DRBG}
 */
var getDRBG = function (msg) {
    var entropy = (0, random_1.randomBytes)(ENT_LEN);
    var pers = Buffer.allocUnsafe(ALG_LEN + ENT_LEN);
    Buffer.from((0, random_1.randomBytes)(ENT_LEN)).copy(pers, 0);
    ALG.copy(pers, ENT_LEN);
    return new hmac_drbg_1.default({
        hash: hash_js_1.default.sha256,
        entropy: entropy,
        nonce: msg,
        pers: pers,
    });
};
//# sourceMappingURL=schnorr.js.map