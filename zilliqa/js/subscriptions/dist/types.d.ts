import { IClientConfig } from 'websocket';
export declare enum SocketConnect {
    READY = "ready",
    CONNECT = "connect",
    ERROR = "error",
    CLOSE = "close",
    RECONNECT = "reconnect"
}
export declare enum SocketState {
    SOCKET_CONNECT = "socket_connect",
    SOCKET_MESSAGE = "socket_message",
    SOCKET_READY = "socket_ready",
    SOCKET_CLOSE = "socket_close",
    SOCKET_ERROR = "socket_error"
}
export declare enum MessageType {
    NEW_BLOCK = "NewBlock",
    EVENT_LOG = "EventLog",
    NOTIFICATION = "Notification",
    UNSUBSCRIBE = "Unsubscribe"
}
export declare enum QueryParam {
    NEW_BLOCK = "NewBlock",
    EVENT_LOG = "EventLog",
    UNSUBSCRIBE = "Unsubscribe"
}
export declare enum StatusType {
    SUBSCRIBE_NEW_BLOCK = "SubscribeNewBlock",
    SUBSCRIBE_EVENT_LOG = "SubscribeEventLog"
}
export interface NewBlockQuery {
    query: string;
}
export interface NewEventQuery {
    query: string;
    addresses: string[];
}
export interface Unsubscribe {
    query: string;
    type: string;
}
export interface SubscriptionOption {
    addresses?: string[];
    clientConfig?: IClientConfig;
    headers?: {
        authorization?: string;
    };
    protocol?: string;
    protocols?: string | string[];
}
//# sourceMappingURL=types.d.ts.map