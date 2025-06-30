import React from 'react';
import { getEnvironment } from '../util/config-json-helper';
import { Environment, ContractState } from '../util/enum';


function WarningBanner(props: any) {
    // config.js from public folder
    const env = getEnvironment();

    return (
        <div id="banner" className="mb-4 text-center ">
            { 
                env === Environment.PROD ? 
                <>
                <div className="px-3 py-3">
                    <div>
                        <strong>Announcement</strong>: <span style={{ color: "#00D0C6" }}>Zilliqa 2.0 is live!</span> The new staking platform for the upgraded network can be found at <a rel="noopener noreferrer" target='_blank' href='https://stake.zilliqa.com'>stake.zilliqa.com</a>.
                    </div>

                    <div className='mt-2'>
                        Check out <a rel="noopener noreferrer" target='_blank' href="https://blog.zilliqa.com/how-to-restake-on-zilliqa-evm">this blog post</a> to learn how to move your stakes and benefit from high APR.
                    </div>
                </div>
                {
                    ContractState.IS_PAUSED.toString() === 'true' && 
                    <div className="px-3 py-3"><strong>Attention</strong>: We have noticed an issue with the staking contract which is causing a slowdown of the Zilliqa network. <br/>While we are investigating the issue and work on a fix, the staking contract has been paused for a week (starting from May 21, 2021).<br/>Once the staking contract is unpaused, any missed rewards for the paused period will be retroactively disbursed.<br/>All funds are SAFU and we apologize for your inconvenience. </div>
                }
                </> :
                <>
                <div className="p-3"><strong>Warning</strong>: Zillion is still in testnet. You are using this dApp at your own risk. Zilliqa cannot assume any responsibility for any loss of funds.</div>
                </>
            }
        </div>
    );
}

export default WarningBanner;