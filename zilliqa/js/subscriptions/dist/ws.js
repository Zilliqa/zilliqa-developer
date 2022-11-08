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
exports.WebSocketProvider = void 0;
var tslib_1 = require("tslib");
var mitt_1 = (0, tslib_1.__importDefault)(require("mitt"));
var websocket_1 = require("websocket");
var types_1 = require("./types");
var WebSocketProvider = /** @class */ (function () {
    // basically, options is a collection of metadata things like protocol or headers
    function WebSocketProvider(url, options) {
        this.handlers = {};
        this.url = url;
        this.options = options;
        this.emitter = new mitt_1.default(this.handlers);
        this.websocket = WebSocketProvider.NewWebSocket(url, options);
        this.subscriptions = {};
        this.registerEventListeners();
    }
    WebSocketProvider.NewWebSocket = function (url, options) {
        if (typeof window !== 'undefined' && window.WebSocket) {
            return new WebSocket(url, options !== undefined ? options.protocol : []);
        }
        else {
            var headers = options !== undefined ? options.headers || {} : undefined;
            var urlObject = new URL(url);
            if (headers !== undefined &&
                !headers.authorization &&
                urlObject.username &&
                urlObject.password) {
                var authToken = Buffer.from(urlObject.username + ":" + urlObject.password).toString('base64');
                headers.authorization = "Basic " + authToken;
            }
            return new websocket_1.w3cwebsocket(url, options !== undefined ? options.protocol : undefined, undefined, headers, undefined, options !== undefined ? options.clientConfig : undefined);
        }
    };
    WebSocketProvider.prototype.registerEventListeners = function () {
        this.websocket.onopen = this.onConnect.bind(this);
        this.websocket.onclose = this.onClose.bind(this);
        this.websocket.onmessage = this.onMessage.bind(this);
        this.websocket.onerror = this.onError.bind(this);
    };
    WebSocketProvider.prototype.removeAllSocketListeners = function () {
        this.removeEventListener(types_1.SocketState.SOCKET_MESSAGE);
        this.removeEventListener(types_1.SocketState.SOCKET_READY);
        this.removeEventListener(types_1.SocketState.SOCKET_CLOSE);
        this.removeEventListener(types_1.SocketState.SOCKET_ERROR);
        this.removeEventListener(types_1.SocketState.SOCKET_CONNECT);
    };
    WebSocketProvider.prototype.removeEventListener = function (type, handler) {
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
    WebSocketProvider.prototype.reconnect = function () {
        var _this = this;
        setTimeout(function () {
            _this.removeAllSocketListeners();
            _this.websocket = WebSocketProvider.NewWebSocket(_this.url, _this.options);
            _this.registerEventListeners();
        }, 5000);
    };
    WebSocketProvider.prototype.onClose = function (event) {
        return (0, tslib_1.__awaiter)(this, void 0, void 0, function () {
            return (0, tslib_1.__generator)(this, function (_a) {
                // reconnect
                if (this.subscriptions !== null && !event.wasClean) {
                    this.emitter.emit(types_1.SocketConnect.RECONNECT, event);
                    this.reconnect();
                    return [2 /*return*/];
                }
                // normal close
                if (this.websocket.CONNECTING) {
                    this.emitter.emit(types_1.SocketConnect.CLOSE, event);
                    this.websocket.close();
                    return [2 /*return*/];
                }
                return [2 /*return*/];
            });
        });
    };
    WebSocketProvider.prototype.onError = function (event) {
        this.emitter.emit(types_1.SocketConnect.ERROR, event);
        if (this.websocket.CONNECTING) {
            this.websocket.close();
        }
        return;
    };
    WebSocketProvider.prototype.onConnect = function () {
        return (0, tslib_1.__awaiter)(this, void 0, void 0, function () {
            var subscriptionKeys, subscriptionKeys_1, subscriptionKeys_1_1, key, id, parameters, e_1_1;
            var e_1, _a;
            return (0, tslib_1.__generator)(this, function (_b) {
                switch (_b.label) {
                    case 0:
                        if (!this.subscriptions) {
                            this.subscriptions = {};
                        }
                        subscriptionKeys = Object.keys(this.subscriptions);
                        if (!(subscriptionKeys.length > 0)) return [3 /*break*/, 8];
                        _b.label = 1;
                    case 1:
                        _b.trys.push([1, 6, 7, 8]);
                        subscriptionKeys_1 = (0, tslib_1.__values)(subscriptionKeys), subscriptionKeys_1_1 = subscriptionKeys_1.next();
                        _b.label = 2;
                    case 2:
                        if (!!subscriptionKeys_1_1.done) return [3 /*break*/, 5];
                        key = subscriptionKeys_1_1.value;
                        id = key;
                        parameters = this.subscriptions[key].parameters;
                        delete this.subscriptions[id];
                        return [4 /*yield*/, this.subscribe(parameters)];
                    case 3:
                        _b.sent();
                        _b.label = 4;
                    case 4:
                        subscriptionKeys_1_1 = subscriptionKeys_1.next();
                        return [3 /*break*/, 2];
                    case 5: return [3 /*break*/, 8];
                    case 6:
                        e_1_1 = _b.sent();
                        e_1 = { error: e_1_1 };
                        return [3 /*break*/, 8];
                    case 7:
                        try {
                            if (subscriptionKeys_1_1 && !subscriptionKeys_1_1.done && (_a = subscriptionKeys_1.return)) _a.call(subscriptionKeys_1);
                        }
                        finally { if (e_1) throw e_1.error; }
                        return [7 /*endfinally*/];
                    case 8:
                        this.emitter.emit(types_1.SocketState.SOCKET_CONNECT);
                        this.emitter.emit(types_1.SocketConnect.CONNECT);
                        return [2 /*return*/];
                }
            });
        });
    };
    WebSocketProvider.prototype.onMessage = function (msg) {
        var e_2, _a;
        if (msg.data) {
            var dataObj = JSON.parse(msg.data);
            if (dataObj.type === types_1.MessageType.NOTIFICATION) {
                this.emitter.emit(types_1.SocketState.SOCKET_MESSAGE, dataObj);
                try {
                    for (var _b = (0, tslib_1.__values)(dataObj.values), _c = _b.next(); !_c.done; _c = _b.next()) {
                        var value = _c.value;
                        if (value.query === types_1.MessageType.NEW_BLOCK) {
                            this.emitter.emit(types_1.MessageType.NEW_BLOCK, value);
                        }
                        else if (value.query === types_1.MessageType.EVENT_LOG) {
                            this.emitter.emit(types_1.MessageType.EVENT_LOG, value);
                        }
                        else if (value.query === types_1.MessageType.UNSUBSCRIBE) {
                            this.emitter.emit(types_1.MessageType.UNSUBSCRIBE, value);
                        }
                        else {
                            throw new Error('unsupported value type');
                        }
                    }
                }
                catch (e_2_1) { e_2 = { error: e_2_1 }; }
                finally {
                    try {
                        if (_c && !_c.done && (_a = _b.return)) _a.call(_b);
                    }
                    finally { if (e_2) throw e_2.error; }
                }
            }
            else if (dataObj.query === types_1.QueryParam.NEW_BLOCK) {
                // subscribe NewBlock succeed
                this.subscriptions[dataObj.query] = {
                    id: dataObj.query,
                    parameters: dataObj,
                };
                this.emitter.emit(types_1.StatusType.SUBSCRIBE_NEW_BLOCK, dataObj);
                this.emitter.emit(types_1.SocketConnect.RECONNECT);
            }
            else if (dataObj.query === types_1.QueryParam.EVENT_LOG) {
                // subscribe EventLog succeed
                this.subscriptions[dataObj.query] = {
                    id: dataObj.query,
                    parameters: dataObj,
                };
                this.emitter.emit(types_1.StatusType.SUBSCRIBE_EVENT_LOG, dataObj);
                this.emitter.emit(types_1.SocketConnect.RECONNECT);
            }
            else if (dataObj.query === types_1.QueryParam.UNSUBSCRIBE) {
                this.emitter.emit(types_1.MessageType.UNSUBSCRIBE, dataObj);
            }
            else {
                throw new Error('unsupported message type');
            }
        }
        else {
            throw new Error('message data is empty');
        }
    };
    WebSocketProvider.prototype.addEventListener = function (type, handler) {
        this.emitter.on(type, handler);
    };
    WebSocketProvider.prototype.connecting = function () {
        return this.websocket.readyState === this.websocket.CONNECTING;
    };
    WebSocketProvider.prototype.send = function (query) {
        var _this = this;
        return new Promise(function (resolve, reject) {
            if (!_this.connecting()) {
                try {
                    _this.websocket.send(JSON.stringify(query));
                }
                catch (error) {
                    throw error;
                }
                var queryParam = void 0;
                if (query.query === types_1.QueryParam.NEW_BLOCK) {
                    queryParam = types_1.StatusType.SUBSCRIBE_NEW_BLOCK;
                }
                else if (query.query === types_1.QueryParam.EVENT_LOG) {
                    queryParam = types_1.StatusType.SUBSCRIBE_EVENT_LOG;
                }
                else {
                    queryParam = query.query;
                }
                _this.emitter.on(queryParam, function (data) {
                    resolve(data);
                });
                _this.emitter.on(types_1.SocketConnect.ERROR, reject);
            }
            var connectHandler = function () {
                _this.send(query).then(resolve).catch(reject);
            };
            var offConnectHandler = function () {
                _this.emitter.off(types_1.SocketConnect.CONNECT, connectHandler);
            };
            _this.emitter.on(types_1.SocketConnect.CONNECT, connectHandler);
            _this.emitter.on(types_1.SocketConnect.RECONNECT, offConnectHandler);
        });
    };
    WebSocketProvider.prototype.subscribe = function (payload) {
        return (0, tslib_1.__awaiter)(this, void 0, void 0, function () {
            var result;
            return (0, tslib_1.__generator)(this, function (_a) {
                switch (_a.label) {
                    case 0: return [4 /*yield*/, this.send(payload)];
                    case 1:
                        result = _a.sent();
                        return [2 /*return*/, result.query === payload.query];
                }
            });
        });
    };
    WebSocketProvider.prototype.unsubscribe = function (payload) {
        return (0, tslib_1.__awaiter)(this, void 0, void 0, function () {
            var result, succeed;
            return (0, tslib_1.__generator)(this, function (_a) {
                switch (_a.label) {
                    case 0: return [4 /*yield*/, this.send(payload)];
                    case 1:
                        result = _a.sent();
                        succeed = result.query === payload.query;
                        if (succeed) {
                            this.subscriptions[payload.query] = null;
                        }
                        return [2 /*return*/, succeed];
                }
            });
        });
    };
    return WebSocketProvider;
}());
exports.WebSocketProvider = WebSocketProvider;
//# sourceMappingURL=ws.js.map