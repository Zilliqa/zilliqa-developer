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
exports.Subscription = void 0;
var tslib_1 = require("tslib");
var ws_1 = require("./ws");
var types_1 = require("./types");
var Subscription = /** @class */ (function (_super) {
    (0, tslib_1.__extends)(Subscription, _super);
    function Subscription(subject, url, options) {
        var _this = _super.call(this, url, options) || this;
        _this.subject = subject;
        return _this;
    }
    Subscription.prototype.start = function () {
        return (0, tslib_1.__awaiter)(this, void 0, void 0, function () {
            return (0, tslib_1.__generator)(this, function (_a) {
                return [2 /*return*/, _super.prototype.subscribe.call(this, this.subject)];
            });
        });
    };
    Subscription.prototype.stop = function () {
        return (0, tslib_1.__awaiter)(this, void 0, void 0, function () {
            var event;
            return (0, tslib_1.__generator)(this, function (_a) {
                event = this.subject.query === types_1.QueryParam.NEW_BLOCK
                    ? {
                        query: types_1.QueryParam.UNSUBSCRIBE,
                        type: types_1.QueryParam.NEW_BLOCK,
                    }
                    : { query: types_1.QueryParam.UNSUBSCRIBE, type: types_1.QueryParam.EVENT_LOG };
                return [2 /*return*/, _super.prototype.unsubscribe.call(this, event)];
            });
        });
    };
    return Subscription;
}(ws_1.WebSocketProvider));
exports.Subscription = Subscription;
//# sourceMappingURL=subscription.js.map