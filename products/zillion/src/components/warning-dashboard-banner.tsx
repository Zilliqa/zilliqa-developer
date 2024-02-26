import React from 'react';
import { getEnvironment } from '../util/config-json-helper';
import { Environment, ContractState } from '../util/enum';


function WarningDashboardBanner(props: any) {
    // config.js from public folder
    const env = getEnvironment();

    return (
        <div id="banner" className="mb-4 text-center">
            { 
                env === Environment.PROD ? 
                <>
                <div className="px-3 py-3">
                    <div><strong>Important</strong>: Please note that gZIL minting has concluded as of <span className="final-gzil-mint-block ml-2">Block 1483713</span>. <span className="mx-2">No further gZIL will be minted.</span> The Zilliqa staking program will still continue to distribute ZIL for staking rewards.</div>
                </div>
                {
                    ContractState.IS_PAUSED.toString() === 'true' && 
                    <div className="px-3 pt-3 py-3"><strong>Attention</strong>: We have noticed an issue with the staking contract which is causing a slowdown of the Zilliqa network. <br/>While we are investigating the issue and work on a fix, the staking contract has been paused for a week (starting from May 21, 2021).<br/>Once the staking contract is unpaused, any missed rewards for the paused period will be retroactively disbursed.<br/>All funds are SAFU and we apologize for your inconvenience. </div>
                }
                </> : null
            }
        </div>
    );
}

export default WarningDashboardBanner;