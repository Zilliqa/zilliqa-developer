/**
 * helper file to extract info from public/config.js
 */

import { Environment, Network } from "./enum";

const {
    api_max_retry_attempt,
    blockchain_explorer_config, 
    environment_config, 
    networks_config,
    refresh_rate_config
} = (window as { [key: string]: any })['config'];


export interface NetworkConfig {
    proxy: string
    impl: string
    blockchain: string
    node_status: string
    api_list: []
}

export interface Networks {
    [Network.TESTNET]: NetworkConfig
    [Network.MAINNET]: NetworkConfig
    [Network.ISOLATED_SERVER]: NetworkConfig
    [Network.PRIVATE]: NetworkConfig          // not in used in config.json
}

export const getEnvironment = () => {
    return environment_config;
}

export const getBlockchainExplorer = () => {
    return blockchain_explorer_config;
}

export const getRefreshRate = (): number => {
    return refresh_rate_config;
}

export const getApiMaxRetry = () => {
    return api_max_retry_attempt || 10;
}

// returns entire networks_config json
// e.g. networks_config : { testnet: { ... } , mainnet: { ... } }
export const getNetworks = (): Networks => {
    return networks_config;
}

// return only specfic networks_config section
// according to environment_config
// e.g. mainnet : { proxy: "", impl: "" }
export const getNetworkConfigByEnv = (): NetworkConfig => {
    if (environment_config === Environment.PROD) {
        return networks_config[Network.MAINNET]
    }
    // defaults to testnet
    return networks_config[Network.TESTNET]
}