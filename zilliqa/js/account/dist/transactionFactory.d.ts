import { Provider, ZilliqaModule } from '@zilliqa-js/core';
import { Transaction } from './transaction';
import { TxParams } from './types';
import { Wallet } from './wallet';
export declare class TransactionFactory implements ZilliqaModule {
    provider: Provider;
    signer: Wallet;
    constructor(provider: Provider, signer: Wallet);
    new(txParams: TxParams, toDs?: boolean, enableSecureAddress?: boolean): Transaction;
    /**
     * This constructor could help you to check if there is a default account to be used, and further more, if it has
     * sufficient fund to do the transfer.
     * @param txParams
     */
    payment(txParams: TxParams): Promise<Transaction>;
}
//# sourceMappingURL=transactionFactory.d.ts.map