/// <reference types="long" />
import { BN, Long } from '@zilliqa-js/util';
import { TransactionReceiptObj } from '@zilliqa-js/core';
export declare enum TxStatus {
    Initialised = 0,
    Pending = 1,
    Confirmed = 2,
    Rejected = 3
}
export interface TxCreated {
    Info: string;
    TranID: string;
    ContractAddress?: string;
}
export interface TxRejected {
    Error: string;
}
export declare type TxReceipt = TransactionReceiptObj<number>;
export interface TxIncluded {
    ID: string;
    receipt: TransactionReceiptObj;
}
export interface TxParams {
    version: number;
    toAddr: string;
    amount: BN;
    gasPrice: BN;
    gasLimit: Long;
    code?: string;
    data?: string;
    receipt?: TxReceipt;
    nonce?: number;
    pubKey?: string;
    signature?: string;
}
export declare enum TxEventName {
    Error = "error",
    Receipt = "receipt",
    Track = "track"
}
//# sourceMappingURL=types.d.ts.map