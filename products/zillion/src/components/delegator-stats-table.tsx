import React from 'react';
import { useAppSelector } from '../store/hooks';
import { OperationStatus } from '../util/enum';
import { DelegStats } from '../util/interface';

import { convertQaToCommaStr, convertGzilToCommaStr } from '../util/utils';
import SpinnerNormal from './spinner-normal';


function DelegatorStatsTable(props: any) {
    // const data = props.data;
    const data: DelegStats = useAppSelector(state => state.user.deleg_stats);
    const loading: OperationStatus = useAppSelector(state => state.user.is_deleg_stats_loading);
    // const totalPendingWithdrawalAmt = props.totalPendingWithdrawalAmt; // Qa
    const blockCountToReward = useAppSelector(state => state.staking.reward_blk_countdown);

    return (
        <>
        {
            loading === OperationStatus.PENDING &&
            <SpinnerNormal class="spinner-border dashboard-spinner mb-4" />
        }
        
        {
            loading === OperationStatus.COMPLETE &&

            <div className="row px-2 pb-3 align-items-center justify-content-center">
                <div className="d-block deleg-stats-card">
                    <h3>EST. Realtime APR</h3>
                    <span>{data.globalAPY}%</span>
                </div>
                <div className="d-block deleg-stats-card">
                    <h3>Total Deposits</h3>
                    <span>{convertQaToCommaStr(data.totalDeposits)}</span>
                </div>
                <div className="d-block deleg-stats-card">
                    <h3>Blocks Until Rewards</h3>
                    <span>{blockCountToReward}</span>
                </div>

                <div className="w-100"></div>

                <div className="d-block deleg-stats-card">
                    <h3>GZIL Balance</h3>
                    <span>{convertGzilToCommaStr(data.gzilBalance)}</span>
                </div>
                <div className="d-block deleg-stats-card">
                    <h3>Unclaimed ZIL Rewards</h3>
                    <span>{convertQaToCommaStr(data.zilRewards)}</span>
                </div>
                <div className="d-block deleg-stats-card">
                </div>
            </div>
        }
        </>
    );
}

export default DelegatorStatsTable;