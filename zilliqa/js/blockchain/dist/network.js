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
exports.Network = void 0;
var NetworkMethods;
(function (NetworkMethods) {
    NetworkMethods["GetClientVersion"] = "GetClientVersion";
    NetworkMethods["GetNetworkId"] = "GetNetworkId";
    NetworkMethods["GetProtocolVersion"] = "GetProtocolVersion";
})(NetworkMethods || (NetworkMethods = {}));
var Network = /** @class */ (function () {
    function Network(provider, signer) {
        this.provider = provider;
        this.signer = signer;
    }
    Network.prototype.getClientVersion = function () {
        return this.provider.send("GetClientVersion" /* GetClientVersion */);
    };
    Network.prototype.GetNetworkId = function () {
        return this.provider.send("GetNetworkId" /* GetNetworkId */);
    };
    Network.prototype.GetProtocolVersion = function (blockNum) {
        return this.provider.send("GetProtocolVersion" /* GetProtocolVersion */);
    };
    return Network;
}());
exports.Network = Network;
//# sourceMappingURL=network.js.map