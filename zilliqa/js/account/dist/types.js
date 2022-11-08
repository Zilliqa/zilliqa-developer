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
exports.TxEventName = exports.TxStatus = void 0;
var TxStatus;
(function (TxStatus) {
    TxStatus[TxStatus["Initialised"] = 0] = "Initialised";
    TxStatus[TxStatus["Pending"] = 1] = "Pending";
    TxStatus[TxStatus["Confirmed"] = 2] = "Confirmed";
    TxStatus[TxStatus["Rejected"] = 3] = "Rejected";
})(TxStatus = exports.TxStatus || (exports.TxStatus = {}));
var TxEventName;
(function (TxEventName) {
    TxEventName["Error"] = "error";
    TxEventName["Receipt"] = "receipt";
    TxEventName["Track"] = "track";
})(TxEventName = exports.TxEventName || (exports.TxEventName = {}));
//# sourceMappingURL=types.js.map