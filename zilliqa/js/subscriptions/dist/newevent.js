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
exports.NewEventSubscription = void 0;
var tslib_1 = require("tslib");
var subscription_1 = require("./subscription");
var types_1 = require("./types");
var NewEventSubscription = /** @class */ (function (_super) {
    (0, tslib_1.__extends)(NewEventSubscription, _super);
    function NewEventSubscription(url, options) {
        var _this = _super.call(this, { query: types_1.QueryParam.EVENT_LOG }, url, options) || this;
        _this.subject = {
            query: 'EventLog',
            addresses: options !== undefined ? options.addresses : [],
        };
        return _this;
    }
    return NewEventSubscription;
}(subscription_1.Subscription));
exports.NewEventSubscription = NewEventSubscription;
//# sourceMappingURL=newevent.js.map