import React from 'react';

import { OperationStatus } from '../util/enum';
import SpinnerNormal from './spinner-normal';
import { useAppSelector } from '../store/hooks';
import { LandingStats } from '../util/interface';
import { convertGzilToCommaStr, convertQaToCommaStr } from '../util/utils';


function LandingStatsTable(props: any) {
    const loading : OperationStatus = useAppSelector(state => state.staking.is_landing_stats_loading);
    const landingStats: LandingStats = useAppSelector(state => state.staking.landing_stats);

    return (
        <>
        <div id="landing-stats" className="container">
            <div className="row p-4">
            <h2 className="mb-4">Statistics</h2>
            
            <div className="col-12 align-items-center">
                { loading === OperationStatus.PENDING && <SpinnerNormal class="spinner-border dashboard-spinner mb-4" /> }
                
                { loading === OperationStatus.COMPLETE && 

                <>
                <div className="row pb-3 mx-auto justify-content-center">
                    <div className="d-block landing-stats-card">
                        <h3>EST. Realtime APR</h3>
                        <span>{landingStats.estRealtimeAPY}%</span>
                    </div>
                    <div className="d-block landing-stats-card">
                        <h3>Circulating Supply Staked</h3>
                        <span>{landingStats.circulatingSupplyStake}%</span>
                    </div>
                    <div className="d-block landing-stats-card">
                        <h3>Delegators / Staked Seed Nodes</h3>
                        <span>{landingStats.delegNum} / {landingStats.nodesNum}</span>
                    </div>

                    <div className="w-100"></div>
                    
                    <div className="d-block landing-stats-card">
                        <h3>Stake Amount</h3>
                        <span>{convertQaToCommaStr(landingStats.totalDeposits)}</span>
                    </div>

                    <div className="d-block landing-stats-card">
                        <h3>Total GZIL minted</h3>
                        <span>{convertGzilToCommaStr(landingStats.gzil)}</span>
                    </div>

                    <div className="d-block landing-stats-card">
                    </div>
                </div>
                </>

                }

            </div>
            </div>
        </div>
        </>
    );
}

export default LandingStatsTable;