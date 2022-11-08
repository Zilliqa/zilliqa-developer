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
exports.randomBytes = void 0;
var tslib_1 = require("tslib");
/**
 * randomBytes
 *
 * Uses JS-native CSPRNG to generate a specified number of bytes.
 * NOTE: this method throws if no PRNG is available.
 *
 * @param {number} bytes
 * @returns {string}
 */
var sodium_randbytes_1 = (0, tslib_1.__importDefault)(require("sodium-randbytes"));
var randomBytes = function (bytes) {
    // For node enviroment, use sodium-native because we prefer kernel CSPRNG.
    // References:
    // - https://paragonie.com/blog/2016/05/how-generate-secure-random-numbers-in-various-programming-languages#nodejs-csprng
    // - https://github.com/nodejs/node/issues/5798
    var b = (0, sodium_randbytes_1.default)(bytes);
    return b.toString('hex');
};
exports.randomBytes = randomBytes;
//# sourceMappingURL=random.js.map