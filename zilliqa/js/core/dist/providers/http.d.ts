import { BaseProvider } from './base';
import { RPCMethod, RPCRequest, RPCResponse } from '../net';
import { Provider, Subscriber } from '../types';
export declare class HTTPProvider extends BaseProvider implements Provider {
    buildPayload<T extends any[]>(method: RPCMethod, params: T): RPCRequest<T>;
    buildBatchPayload<T extends any[]>(method: RPCMethod, paramsList: T[]): RPCRequest<T>;
    send<P extends any[], R = any, E = string>(method: RPCMethod, ...params: P): Promise<RPCResponse<R, E>>;
    sendBatch<P extends any[], R = any, E = string>(method: RPCMethod, params: P[]): Promise<RPCResponse<R, E>>;
    subscribe(event: string, subscriber: Subscriber): symbol;
    unsubscribe(token: symbol): void;
}
//# sourceMappingURL=http.d.ts.map