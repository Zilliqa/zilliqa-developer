/// <reference types="node" />
import { ReqMiddlewareFn } from '@zilliqa-js/core';
import { TxReceipt, TxParams } from './types';
export declare const encodeTransactionProto: (tx: TxParams) => Buffer;
export declare const isTxReceipt: (x: unknown) => x is TxReceipt;
export declare const isTxParams: (obj: unknown) => obj is TxParams;
export declare const formatOutgoingTx: ReqMiddlewareFn<[TxParams]>;
export declare function sleep(ms: number): Promise<unknown>;
//# sourceMappingURL=util.d.ts.map