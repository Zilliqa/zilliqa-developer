import { Transaction, Wallet } from '@zilliqa-js/account';
import { Provider } from '@zilliqa-js/core';
import { Blockchain } from '@zilliqa-js/blockchain';
import { Contracts } from './factory';
import { ABI, CallParams, ContractStatus, DeployParams, Init, State, Value } from './types';
export declare class Contract {
    factory: Contracts;
    provider: Provider;
    signer: Wallet;
    blockchain: Blockchain;
    init: Init;
    abi?: ABI;
    state?: State;
    address?: string;
    code?: string;
    status: ContractStatus;
    error?: any;
    constructor(factory: Contracts, code?: string, abi?: ABI, address?: string, init?: any, state?: any, checkAddr?: boolean);
    /**
     * isInitialised
     *
     * Returns true if the contract has not been deployed
     *
     * @returns {boolean}
     */
    isInitialised(): boolean;
    /**
     * isDeployed
     *
     * Returns true if the contract is deployed
     *
     * @returns {boolean}
     */
    isDeployed(): boolean;
    /**
     * isRejected
     *
     * Returns true if an attempt to deploy the contract was made, but the
     * underlying transaction was unsuccessful.
     *
     * @returns {boolean}
     */
    isRejected(): boolean;
    prepareTx(tx: Transaction, attempts: number | undefined, interval: number | undefined, isDeploy: boolean): Promise<Transaction>;
    prepare(tx: Transaction): Promise<string | undefined>;
    /**
     * deploy smart contract with no confirm
     * @param params
     * @param toDs
     */
    deployWithoutConfirm(params: DeployParams, toDs?: boolean): Promise<[Transaction, Contract]>;
    /**
     * deploy
     *
     * @param {DeployParams} params
     * @returns {Promise<Contract>}
     */
    deploy(params: DeployParams, attempts?: number, interval?: number, toDs?: boolean): Promise<[Transaction, Contract]>;
    callWithoutConfirm(transition: string, args: Value[], params: CallParams, toDs?: boolean): Promise<Transaction>;
    /**
     * call
     *
     * @param {string} transition
     * @param {any} params
     * @returns {Promise<Transaction>}
     */
    call(transition: string, args: Value[], params: CallParams, attempts?: number, interval?: number, toDs?: boolean): Promise<Transaction>;
    getState(): Promise<State>;
    getSubState(variableName: string, indices?: string[]): Promise<State>;
    getInit(): Promise<State>;
}
//# sourceMappingURL=contract.d.ts.map