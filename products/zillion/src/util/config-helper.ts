import { getEnvironment, getNetworks, NetworkConfig } from "./config-json-helper";
import { Environment, Network } from "./enum";

export function envStringToEnv(env: string): Environment {
    const selectedEnv = Object.entries(Environment).find(([key, val]) => val === env);

    if (!selectedEnv) {
        const environmentKeys = Object.values(Environment).join(', ');
        throw new Error(`Invalid environment: ${env}. Available environments: ${environmentKeys}`);
    }

    return selectedEnv[1]
}

export function getDefaultNetworkForEnv(env: Environment): Network {
    return {
        [Environment.PROD]: Network.MAINNET,
        [Environment.STAGE]: Network.TESTNET,
        [Environment.DEV]: Network.TESTNET,
        [Environment.STAGE_ZQ2_PROTOMAINNET]: Network.ZQ2_PROTOMAINNET
    }[env];
}

export function getDefaultNetworkForCurrentEnv(): Network {
    return getDefaultNetworkForEnv(
        envStringToEnv(
            getEnvironment()
        )
    );
}

export function networkStringToNetwork(network: string): Network {
    const selectedNetwork = Object.entries(Network).find(([key, val]) => val === network);

    if (!selectedNetwork) {
        const networkKeys = Object.values(Network).join(', ');
        throw new Error(`Invalid network: ${network}. Available networks: ${networkKeys}`);
    }

    return selectedNetwork[1]
}

export function getNetworkConfigByEnv(env: Environment): NetworkConfig {
    const defaultEnvNetwork = getDefaultNetworkForEnv(env);
    return getNetworks()[defaultEnvNetwork];
}

export function getNetworkConfigForCurrentEnv(): NetworkConfig {
    return getNetworkConfigByEnv(
        envStringToEnv(
            getEnvironment()
        )
    );
}

export function networkToNetworkName(network: Network): string {
    return {
        [Network.TESTNET]: 'Testnet',
        [Network.MAINNET]: 'Mainnet',
        [Network.ISOLATED_SERVER]: 'Isolated Server',
        [Network.PRIVATE]: 'Private',
        [Network.ZQ2_PROTOMAINNET]: 'ZQ2 Protomainnet'
    }[network];
}
