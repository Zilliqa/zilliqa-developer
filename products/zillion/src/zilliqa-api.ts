import { Zilliqa } from "@zilliqa-js/zilliqa";
import store from "./store/store";
import { ApiRandomizer } from "./util/api-randomizer";
import { getApiMaxRetry } from "./util/config-json-helper";
import { NetworkURL, OperationStatus } from "./util/enum";
import { logger } from "./util/logger";

const API_MAX_ATTEMPT = getApiMaxRetry();
const API_RANDOMIZER = ApiRandomizer.getInstance();

export class ZilSdk {

    /**
     * query the contract state using random api via batch JSON-RPC
     * @param queryList an array of array in the form [ [impl_address, contract_field, [indices], [impl_address, contract_field2, [indices]] ] ]
     */
    static getSmartContractSubStateBatch = async (queryList: any[]): Promise<any> => {
        let result;
        for (let attempt = 0; attempt < API_MAX_ATTEMPT; attempt++) {
            result = await ZilSdk.getActualSmartContractSubStateBatch(queryList);
            if (result !== OperationStatus.ERROR) {
                break;
            }
        }
        return result;
    }

    private static getActualSmartContractSubStateBatch = async (queryList: any[]): Promise<any> => {
        try {
            const { blockchain, api_list }  = store.getState().blockchain
            const randomAPI = API_RANDOMIZER.fetchApi(blockchain as NetworkURL, api_list);
            const zilliqa = new Zilliqa(randomAPI);

            let response: any = await zilliqa.blockchain.getSmartContractSubStateBatch(queryList);

            if (!response.hasOwnProperty("batch_result") || response.batch_result === undefined) {
                return OperationStatus.ERROR;
            }

            // sort response by id in ascending order
            return response.batch_result.sort(function (a: any, b: any) {
                return (a.id - b.id);
            });

        } catch (err) {
            console.error(err);
            return OperationStatus.ERROR;
        }
    }

    /**
     * query the contract state using random api
     * retry if there is an error in the response
     */
    static getSmartContractSubState = async (impl: string, state: string, indices?: any): Promise<any> => {
        let result;
        for (let attempt = 0; attempt < API_MAX_ATTEMPT; attempt++) {
            result = await ZilSdk.getActualSmartContractSubState(impl, state, indices);
            if (result !== OperationStatus.ERROR) {
                break;
            }
        }
        return result;
    }

    /**
     * query the wallet balance
     * 
     * @param address wallet address in base16 or bech32 format
     * @returns amount in zils, returns '0' if the balance cannot be found after max attempt
     */
    static getBalance = async (address: string): Promise<string> => {
        let result;
        for (let attempt = 0; attempt < API_MAX_ATTEMPT; attempt++) {
            result = await ZilSdk.getActualBalance(address);
            if (result !== OperationStatus.ERROR) {
                break;
            }
        }
        if (result === OperationStatus.ERROR) {
            // still fail after max attempt
            return "0";
        }
        return result;
    }

    /**
     * query the current number of tx blocks
     */
    static getNumTxBlocks = async () => {
        let result;
        for (let attempt = 0; attempt < API_MAX_ATTEMPT; attempt++) {
            result = await ZilSdk.getActualNumTxBlocks();
            if (result !== OperationStatus.ERROR) {
                break;
            }
        }
        return result;
    }

    /**
     * fetch the total zil coin supply
     */
    static getTotalCoinSupply = async (): Promise<any> => {
        let result;
        for (let attempt = 0; attempt < API_MAX_ATTEMPT; attempt++) {
            result = await ZilSdk.getActualTotalCoinSupply();
            if (result !== OperationStatus.ERROR) {
                break;
            }
        }
        return result;
    }

    /**
     * checks if the connected wallet is a node operator
     * 
     * @param impl      contract implementation address
     * @param address   base16 wallet address to check
     * @returns true if the connected wallet is a node operator, false otherwise
     */
    static isOperator = async (impl: string, address: string): Promise<any> => {
        if (!impl || !address) {
            return false;
        }
        logger("check is operator: ", address);
        const response = await ZilSdk.getSmartContractSubState(impl, "ssnlist", [address]);

        if (!response || response === null || response === OperationStatus.ERROR) {
            return false;
        }
        return true;
    }

    private static getActualNumTxBlocks = async () => {
        try {
            const { blockchain, api_list }  = store.getState().blockchain
            const randomAPI = API_RANDOMIZER.fetchApi(blockchain as NetworkURL, api_list);
            const zilliqa = new Zilliqa(randomAPI);
            const response =  await zilliqa.blockchain.getBlockChainInfo();

            if (!response.hasOwnProperty("result") || response.result === undefined) {
                return OperationStatus.ERROR;
            }
            return response.result.NumTxBlocks;
        } catch (err) {
            return OperationStatus.ERROR;
        }
    }

    private static getActualTotalCoinSupply = async () => {
        try {
            const { blockchain, api_list }  = store.getState().blockchain
            const randomAPI = API_RANDOMIZER.fetchApi(blockchain as NetworkURL, api_list);
            const zilliqa = new Zilliqa(randomAPI);
            const response =  await zilliqa.blockchain.getTotalCoinSupply();

            if (!response.hasOwnProperty("result") || response.result === undefined) {
                return OperationStatus.ERROR;
            }
            return response.result;
        } catch (err) {
            return OperationStatus.ERROR;
        }
    }

    private static getActualBalance = async (address: string) => {
        try {
            const { blockchain, api_list }  = store.getState().blockchain
            const randomAPI = API_RANDOMIZER.fetchApi(blockchain as NetworkURL, api_list);
            const zilliqa = new Zilliqa(randomAPI);
            const response =  await zilliqa.blockchain.getBalance(address);

            if (!response.hasOwnProperty("result") || response.result.balance === undefined) {
                return "0";
            }
            return response.result.balance;
        } catch (err) {
            return OperationStatus.ERROR;
        }
    }
    
    /**
     * Get smart contract sub state with a new zilliqa object
     * sets the network but doesn't affect the rest of the zilliqa calls such as sending transaction
     * which depends on the main zilliqa object
     * 
     * @param impl      implementation contract in checksum format
     * @param state     name of the variable in the contract
     * @param indices   JSOn array to specify the indices if the variable is a map type
     */
    private static getActualSmartContractSubState = async (impl: string, state: string, indices?: any) => {
        if (!impl) {
            console.error("error: get contract sub state - no implementation contract found");
            return OperationStatus.ERROR;
        }

        try {
            const { blockchain, api_list }  = store.getState().blockchain
            const randomAPI = API_RANDOMIZER.fetchApi(blockchain as NetworkURL, api_list);
            const zilliqa = new Zilliqa(randomAPI);

            let response: any = null;
            if (indices !== null) {
                response = await zilliqa.blockchain.getSmartContractSubState(impl, state, indices);
            } else {
                response = await zilliqa.blockchain.getSmartContractSubState(impl, state);
            }

            if (!response.hasOwnProperty("result") || response.result === undefined) {
                return OperationStatus.ERROR;
            }
            return response.result;
            
        } catch (err) {
            console.error(err);
            return OperationStatus.ERROR;
        }
    }
}