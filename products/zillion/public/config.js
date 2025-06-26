/* config file
 * 
 * node_status
 *  - link to staking viewer
 * 
 * api_list
 *   - seed node url
 *   - Testnet excluded network due to cors:
 *     - https://seed-dev-api.zillet.io 
 *     - https://ssntestnet.zillacracy.com/api
 *    - Mainnet excluded
 *     - https://ssn-zilliqa.moonlet.network/api
 *     - https://zilliqa-api.staked.cloud
 *     - https://ssn-api-mainnet.viewblock.io
 * 
 * refresh_rate_config: [time in milliseconds]
 * - interval at which contract data and wallet's info are updated
 * 
 * environment_config: [dev (default) | stage | prod]
 *  - when set to dev, allows users to change network on home page
 *    and disables authentication checks
 * 
 *  - when set to stage, blockchain is set to testnet
 * 
 *  - when set to prod, blockchain is set to mainnet
 * 
 * api_max_retry_attempt
 * - maximum attempt to retry fetching contract data before giving up
 * 
*/

window['config'] = {
    networks_config: {
        testnet: {
            proxy: "0x05d7e121E205A84Bf1da2D60aC8A2484800FfFB3",
            impl: "0x05C2DdeC2E4449160436130CB4F9b84dE9f7eE5b",
            blockchain: "https://dev-api.zilliqa.com",
            node_status: "https://testnet-viewer.zilliqa.com",
            api_list: [
                "https://bumblebee-api.zilliqa.network",
                "https://dev-api.zilliqa.com",
            ]
        },
        mainnet: {
            proxy: "0x62A9d5D611CDCaE8D78005F31635898330e06B93",
            impl: "0xa7C67D49C82c7dc1B73D231640B2e4d0661D37c1",
            blockchain: "https://api.zilliqa.com",
            node_status: "https://staking-viewer.zilliqa.com",
            api_list : [
                "https://api.zilliqa.com",
                "https://ssn-zilliqa.cex.io/api",
                "https://ssn.ignitedao.io/api",
                "https://ssn.zillet.io",
                "https://zil-staking.ezil.me/api",
                "https://staking-zil.kucoin.com/api",
            ]
        },
        zq2_protomainnet: {
            proxy: "0x62A9d5D611CDCaE8D78005F31635898330e06B93",
            impl: "0xa7C67D49C82c7dc1B73D231640B2e4d0661D37c1",
            blockchain: "https://api.zq2-protomainnet.zilliqa.com",
            node_status: "https://staking-viewer.zilliqa.com",
            api_list : [
                "https://api.zq2-protomainnet.zilliqa.com",
            ]
        },
        isolated_server: {
            proxy: "0x0578B8e9D9c2493D4a2E98f364c7ed311F7a0d71",
            impl: "",
            blockchain: "https://zilliqa-isolated-server.zilliqa.com",
            node_status: "",
            api_list : [
                "https://zilliqa-isolated-server.zilliqa.com"
            ]
        }
    },
    refresh_rate_config: 300000,
    api_max_retry_attempt: 10,
    environment_config: "stage_zq2_protomainnet"
}
