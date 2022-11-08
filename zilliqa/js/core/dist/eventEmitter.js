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
exports.EventEmitter = void 0;
var tslib_1 = require("tslib");
var mitt_1 = (0, tslib_1.__importDefault)(require("mitt"));
var EventEmitter = /** @class */ (function () {
    function EventEmitter() {
        var _this = this;
        this.handlers = {};
        this.emitter = new mitt_1.default(this.handlers);
        this.off = this.emitter.off.bind(this);
        this.emit = this.emitter.emit.bind(this);
        this.promise = new Promise(function (resolve, reject) {
            _this.resolve = resolve;
            _this.reject = reject;
        });
        this.then = this.promise.then.bind(this.promise);
    }
    EventEmitter.prototype.resetHandlers = function () {
        for (var i in this.handlers) {
            delete this.handlers[i];
        }
    };
    EventEmitter.prototype.on = function (type, handler) {
        this.emitter.on(type, handler);
        return this;
    };
    EventEmitter.prototype.once = function (type, handler) {
        var _this = this;
        this.emitter.on(type, function (e) {
            handler(e);
            _this.removeEventListener(type);
        });
    };
    EventEmitter.prototype.addEventListener = function (type, handler) {
        this.emitter.on(type, handler);
    };
    EventEmitter.prototype.removeEventListener = function (type, handler) {
        if (!type) {
            this.handlers = {};
            return;
        }
        if (!handler) {
            delete this.handlers[type];
        }
        else {
            return this.emitter.off(type, handler);
        }
    };
    EventEmitter.prototype.onError = function (error) {
        this.emitter.on('error', error);
        this.removeEventListener('*');
    };
    EventEmitter.prototype.onData = function (data) {
        this.emitter.on('data', data);
        this.removeEventListener('*');
    };
    EventEmitter.prototype.listenerCount = function (listenKey) {
        var count = 0;
        Object.keys(this.handlers).forEach(function (val) {
            if (listenKey === val) {
                count += 1;
            }
        });
        return count;
    };
    return EventEmitter;
}());
exports.EventEmitter = EventEmitter;
//# sourceMappingURL=eventEmitter.js.map