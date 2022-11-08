import { Transaction, Wallet } from '@zilliqa-js/account';
import { BlockchainInfo, BlockList, DsBlockObj, TransactionStatusObj, Provider, RPCResponse, ShardingStructure, TransactionObj, MinerInfo, TxBlockObj, TxList, ZilliqaModule } from '@zilliqa-js/core';
export declare class Blockchain implements ZilliqaModule {
    signer: Wallet;
    provider: Provider;
    pendingErrorMap: {
        [key: number]: string;
    };
    transactionStatusMap: {
        [key: number]: {
            [key: number]: string;
        };
    };
    constructor(provider: Provider, signer: Wallet);
    getBlockChainInfo(): Promise<RPCResponse<BlockchainInfo, string>>;
    getShardingStructure(): Promise<RPCResponse<ShardingStructure, string>>;
    getDSBlock(blockNum: number): Promise<RPCResponse<DsBlockObj, string>>;
    getLatestDSBlock(): Promise<RPCResponse<DsBlockObj, string>>;
    getNumDSBlocks(): Promise<RPCResponse<string, string>>;
    getDSBlockRate(): Promise<RPCResponse<number, string>>;
    getDSBlockListing(max: number): Promise<RPCResponse<BlockList, string>>;
    getTxBlock(blockNum: number): Promise<RPCResponse<TxBlockObj, string>>;
    getLatestTxBlock(): Promise<RPCResponse<TxBlockObj, string>>;
    getNumTxBlocks(): Promise<RPCResponse<string, string>>;
    getTxBlockRate(): Promise<RPCResponse<number, string>>;
    getTxBlockListing(max: number): Promise<RPCResponse<BlockList, string>>;
    getNumTransactions(): Promise<RPCResponse<string, string>>;
    getTransactionRate(): Promise<RPCResponse<number, string>>;
    getCurrentMiniEpoch(): Promise<RPCResponse<string, string>>;
    getCurrentDSEpoch(): Promise<RPCResponse<any, string>>;
    getPrevDifficulty(): Promise<RPCResponse<number, string>>;
    getPrevDSDifficulty(): Promise<RPCResponse<number, string>>;
    getTotalCoinSupply(): Promise<RPCResponse<string, string>>;
    getMinerInfo(dsBlockNumber: string): Promise<RPCResponse<MinerInfo, any>>;
    createTransaction(tx: Transaction, maxAttempts?: number, interval?: number, blockConfirm?: boolean): Promise<Transaction>;
    createBatchTransaction(signedTxList: Transaction[], maxAttempts?: number, interval?: number, blockConfirm?: boolean): Promise<Transaction[]>;
    createTransactionRaw(payload: string): Promise<string>;
    createTransactionWithoutConfirm(tx: Transaction): Promise<Transaction>;
    createBatchTransactionWithoutConfirm(signedTxList: Transaction[]): Promise<Transaction[]>;
    getTransaction(txHash: string): Promise<Transaction>;
    getTransactionStatus(txHash: string): Promise<TransactionStatusObj>;
    getRecentTransactions(): Promise<RPCResponse<TxList, never>>;
    getTransactionsForTxBlock(txBlock: number): Promise<RPCResponse<string[][], string>>;
    getTransactionsForTxBlockEx(txBlock: number): Promise<RPCResponse<any, string>>;
    getTxnBodiesForTxBlock(txBlock: number): Promise<RPCResponse<TransactionObj[], string>>;
    getTxnBodiesForTxBlockEx(txBlock: number): Promise<RPCResponse<any, string>>;
    getNumTxnsTxEpoch(epoch: number): Promise<RPCResponse<string, string>>;
    getNumTxnsDSEpoch(epoch: number): Promise<RPCResponse<string, string>>;
    getMinimumGasPrice(): Promise<RPCResponse<string, string>>;
    getBalance(addr: string): Promise<RPCResponse<any, string>>;
    getSmartContractCode(addr: string): Promise<RPCResponse<{
        code: string;
    }, string>>;
    getSmartContractInit(addr: string): Promise<RPCResponse<any, string>>;
    getSmartContractState(addr: string): Promise<RPCResponse<any, string>>;
    getSmartContractSubState(addr: string, variableName: string, indices?: string[]): Promise<RPCResponse<any, string>>;
    getSmartContractSubStateBatch(reqs: any[]): Promise<RPCResponse<any, any>>;
    getSmartContracts(addr: string): Promise<RPCResponse<any, string>>;
    getContractAddressFromTransactionID(txHash: string): Promise<RPCResponse<string, string>>;
    getStateProof(contractAddress: string, sha256Hash: string, txBlock: number | string): Promise<RPCResponse<any, string>>;
}
//# sourceMappingURL=chain.d.ts.map