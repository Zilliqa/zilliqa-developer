import store from "../store/store";
import { ApiRandomizer } from "./api-randomizer";
import { getApiMaxRetry } from "./config-json-helper";
import { NetworkURL } from "./enum";

const apiRandomizer = ApiRandomizer.getInstance();
const { RewardCalculator } = require('./calculator');

// config.js from public folder
const API_MAX_ATTEMPT = getApiMaxRetry();

let rewardCalculator: typeof RewardCalculator;

export const computeDelegRewardsExec = async (impl: string, networkURL: string, ssn: string, delegator: string) => {
    if (!rewardCalculator) {
        rewardCalculator = new RewardCalculator(networkURL, impl);
        try {
            await rewardCalculator.compute_maps();
        } catch (err) {
            // error with fetching; api error
            // set to null to re-declare a new object with a new api
            rewardCalculator = null;
            throw err;
        }
    }

    return await rewardCalculator.get_rewards(ssn, delegator);
};

export const computeDelegRewards = async (impl: string, ssn: string, delegator: string) => {
    let result;

    for (let attempt = 0; attempt < API_MAX_ATTEMPT; attempt++) {
        try {
            const { blockchain, api_list }  = store.getState().blockchain
            const randomAPI = apiRandomizer.fetchApi(blockchain as NetworkURL, api_list);
            result = await computeDelegRewardsExec(impl, randomAPI, ssn, delegator);
            break;
        } catch (err) {
            // error with querying api
            // retry
            continue;
        }
    }
    return result;
};