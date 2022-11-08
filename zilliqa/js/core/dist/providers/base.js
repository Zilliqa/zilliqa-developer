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
exports.BaseProvider = void 0;
var tslib_1 = require("tslib");
var MiddlewareType;
(function (MiddlewareType) {
    MiddlewareType[MiddlewareType["REQ"] = 0] = "REQ";
    MiddlewareType[MiddlewareType["RES"] = 1] = "RES";
})(MiddlewareType || (MiddlewareType = {}));
var BaseProvider = /** @class */ (function () {
    function BaseProvider(nodeURL, reqMiddleware, resMiddleware) {
        var _this = this;
        if (reqMiddleware === void 0) { reqMiddleware = new Map(); }
        if (resMiddleware === void 0) { resMiddleware = new Map(); }
        this.middleware = {
            request: {
                use: function (fn, match) {
                    if (match === void 0) { match = '*'; }
                    _this.pushMiddleware(fn, 0 /* REQ */, match);
                },
            },
            response: {
                use: function (fn, match) {
                    if (match === void 0) { match = '*'; }
                    _this.pushMiddleware(fn, 1 /* RES */, match);
                },
            },
        };
        this.nodeURL = nodeURL;
        this.reqMiddleware = reqMiddleware;
        this.resMiddleware = resMiddleware;
    }
    /**
     * pushMiddleware
     *
     * Adds the middleware to the appropriate middleware map.
     *
     * @param {ResMiddlewareFn}
     * @param {T} type
     * @param {Matcher} match
     * @returns {void}
     */
    BaseProvider.prototype.pushMiddleware = function (fn, type, match) {
        if (type !== 0 /* REQ */ && type !== 1 /* RES */) {
            throw new Error('Please specify the type of middleware being added');
        }
        if (type === 0 /* REQ */) {
            var current = this.reqMiddleware.get(match) || [];
            this.reqMiddleware.set(match, (0, tslib_1.__spreadArray)((0, tslib_1.__spreadArray)([], (0, tslib_1.__read)(current), false), [fn], false));
        }
        else {
            var current = this.resMiddleware.get(match) || [];
            this.resMiddleware.set(match, (0, tslib_1.__spreadArray)((0, tslib_1.__spreadArray)([], (0, tslib_1.__read)(current), false), [fn], false));
        }
    };
    /**
     * getMiddleware
     *
     * Returns the middleware that matches the matcher provided. Note that
     * middleware are called in order of specificity: string -> regexp ->
     * wildcard.
     *
     * @param {Matcher} match
     * @returns {[ReqMiddlewareFn[], ResMiddlewareFn[]]}
     */
    BaseProvider.prototype.getMiddleware = function (method) {
        var e_1, _a, e_2, _b;
        var reqFns = [];
        var resFns = [];
        try {
            for (var _c = (0, tslib_1.__values)(this.reqMiddleware.entries()), _d = _c.next(); !_d.done; _d = _c.next()) {
                var _e = (0, tslib_1.__read)(_d.value, 2), key = _e[0], transformers = _e[1];
                if (typeof key === 'string' && key !== '*' && key === method) {
                    reqFns.push.apply(reqFns, (0, tslib_1.__spreadArray)([], (0, tslib_1.__read)(transformers), false));
                }
                if (key instanceof RegExp && key.test(method)) {
                    reqFns.push.apply(reqFns, (0, tslib_1.__spreadArray)([], (0, tslib_1.__read)(transformers), false));
                }
                if (key === '*') {
                    reqFns.push.apply(reqFns, (0, tslib_1.__spreadArray)([], (0, tslib_1.__read)(transformers), false));
                }
            }
        }
        catch (e_1_1) { e_1 = { error: e_1_1 }; }
        finally {
            try {
                if (_d && !_d.done && (_a = _c.return)) _a.call(_c);
            }
            finally { if (e_1) throw e_1.error; }
        }
        try {
            for (var _f = (0, tslib_1.__values)(this.resMiddleware.entries()), _g = _f.next(); !_g.done; _g = _f.next()) {
                var _h = (0, tslib_1.__read)(_g.value, 2), key = _h[0], transformers = _h[1];
                if (typeof key === 'string' && key !== '*' && key === method) {
                    resFns.push.apply(resFns, (0, tslib_1.__spreadArray)([], (0, tslib_1.__read)(transformers), false));
                }
                if (key instanceof RegExp && key.test(method)) {
                    resFns.push.apply(resFns, (0, tslib_1.__spreadArray)([], (0, tslib_1.__read)(transformers), false));
                }
                if (key === '*') {
                    resFns.push.apply(resFns, (0, tslib_1.__spreadArray)([], (0, tslib_1.__read)(transformers), false));
                }
            }
        }
        catch (e_2_1) { e_2 = { error: e_2_1 }; }
        finally {
            try {
                if (_g && !_g.done && (_b = _f.return)) _b.call(_f);
            }
            finally { if (e_2) throw e_2.error; }
        }
        return [reqFns, resFns];
    };
    return BaseProvider;
}());
exports.BaseProvider = BaseProvider;
//# sourceMappingURL=base.js.map