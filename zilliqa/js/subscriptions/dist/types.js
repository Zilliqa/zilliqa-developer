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
exports.StatusType = exports.QueryParam = exports.MessageType = exports.SocketState = exports.SocketConnect = void 0;
var SocketConnect;
(function (SocketConnect) {
    SocketConnect["READY"] = "ready";
    SocketConnect["CONNECT"] = "connect";
    SocketConnect["ERROR"] = "error";
    SocketConnect["CLOSE"] = "close";
    SocketConnect["RECONNECT"] = "reconnect";
})(SocketConnect = exports.SocketConnect || (exports.SocketConnect = {}));
var SocketState;
(function (SocketState) {
    SocketState["SOCKET_CONNECT"] = "socket_connect";
    SocketState["SOCKET_MESSAGE"] = "socket_message";
    SocketState["SOCKET_READY"] = "socket_ready";
    SocketState["SOCKET_CLOSE"] = "socket_close";
    SocketState["SOCKET_ERROR"] = "socket_error";
})(SocketState = exports.SocketState || (exports.SocketState = {}));
// message type pushed by server side
var MessageType;
(function (MessageType) {
    MessageType["NEW_BLOCK"] = "NewBlock";
    MessageType["EVENT_LOG"] = "EventLog";
    MessageType["NOTIFICATION"] = "Notification";
    MessageType["UNSUBSCRIBE"] = "Unsubscribe";
})(MessageType = exports.MessageType || (exports.MessageType = {}));
// message type that we can query with to server
var QueryParam;
(function (QueryParam) {
    QueryParam["NEW_BLOCK"] = "NewBlock";
    QueryParam["EVENT_LOG"] = "EventLog";
    QueryParam["UNSUBSCRIBE"] = "Unsubscribe";
})(QueryParam = exports.QueryParam || (exports.QueryParam = {}));
// indicate that whether we subscribe successfully
var StatusType;
(function (StatusType) {
    StatusType["SUBSCRIBE_NEW_BLOCK"] = "SubscribeNewBlock";
    StatusType["SUBSCRIBE_EVENT_LOG"] = "SubscribeEventLog";
})(StatusType = exports.StatusType || (exports.StatusType = {}));
//# sourceMappingURL=types.js.map