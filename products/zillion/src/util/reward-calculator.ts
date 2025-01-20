import { Zilliqa } from "@zilliqa-js/zilliqa";
import { getApiMaxRetry } from "./config-json-helper";
import { RewardCalculator } from "./calculator";
import { ZilSdk } from "../zilliqa-api";

// config.js from public folder
const API_MAX_ATTEMPT = getApiMaxRetry();

let rewardCalculator: RewardCalculator | null = null;

export const computeDelegRewardsExec = async (impl: string, zilliqa: Zilliqa, ssn: string, delegator: string) => {
    if (!rewardCalculator) {
        rewardCalculator = new RewardCalculator(zilliqa, impl);
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
            const zilliqa = ZilSdk.getZilliqaApi();
            result = await computeDelegRewardsExec(impl, zilliqa, ssn, delegator);
            break;
        } catch (err) {
            // error with querying api
            // retry
            continue;
        }
    }
    return result;
};