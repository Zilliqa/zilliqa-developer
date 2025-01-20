/**
 * helper file to extract info from public/config.js
 */

import { Network } from "./enum";

const {
    api_max_retry_attempt,
    environment_config, 
    networks_config,
    refresh_rate_config
} = (window as any as {
        config: {
            api_max_retry_attempt: number,
            environment_config: string,
            networks_config: Networks,
            refresh_rate_config: number
        }
    })['config'];


export interface NetworkConfig {
    proxy: string
    impl: string
    blockchain: string
    node_status: string
    api_list: string[]
}

export interface Networks {
    [Network.TESTNET]: NetworkConfig
    [Network.MAINNET]: NetworkConfig
    [Network.ISOLATED_SERVER]: NetworkConfig
    [Network.ZQ2_PROTOMAINNET]: NetworkConfig
    [Network.PRIVATE]: NetworkConfig          // not in used in config.json
}

export const getEnvironment = () => {
    return environment_config;
}

export const getRefreshRate = (): number => {
    return refresh_rate_config;
}

export const getApiMaxRetry = () => {
    return api_max_retry_attempt || 10;
}

export const getNetworks = () => {
    return networks_config;
}

export const tryGetNetworkLabelByApiUrl = (api: string): Network | undefined => {
    const availableNetworks = Object.entries(getNetworks()) as [string, NetworkConfig][];

    const formattedApiName = api.startsWith("https://") ? api : `https://${api}`;

    const networkWithApi = availableNetworks.find(
        ([label, config]) => config.api_list.includes(formattedApiName) 
    );

    return networkWithApi ? networkWithApi[0] as Network : undefined;
}
