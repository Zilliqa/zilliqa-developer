import React from 'react';
import { convertQaToCommaStr, convertToProperCommRate } from '../util/utils';

import { OperatorStats } from '../util/interface';
import { OperationStatus } from '../util/enum';
import { useAppSelector } from '../store/hooks';
import SpinnerNormal from './spinner-normal';


function OperatorStatsTable(props: any) {
    const data: OperatorStats = useAppSelector(state => state.user.operator_stats);
    const loading: OperationStatus = useAppSelector(state => state.user.is_operator_stats_loading);

    return (
        <>
        {
            loading === OperationStatus.PENDING &&
            <SpinnerNormal class="spinner-border dashboard-spinner mb-4" />
        }
        {
            loading === OperationStatus.COMPLETE &&
            <>
            <div className="row px-2 align-items-center justify-content-center">
                <div className="d-block operator-stats-card">
                    <h3>Stake Amount</h3>
                    <span>{convertQaToCommaStr(data.stakeAmt)}</span>
                </div>
                <div className="d-block operator-stats-card">
                    <h3>Buffered Deposit</h3>
                    <span>{convertQaToCommaStr(data.bufferedDeposits)}</span>
                </div>
                <div className="d-block operator-stats-card">
                    <h3>Delegators</h3>
                    <span>{data.delegNum}</span>
                </div>
            </div>

            <div className="row px-2 pb-2 align-items-center justify-content-center">
                <div className="d-block operator-stats-card">
                    <h3>Commission Rate</h3>
                    <span>{convertToProperCommRate(data.commRate).toFixed(2)}%</span>
                </div>
                <div className="d-block operator-stats-card">
                    <h3>Commission Rewards</h3>
                    <span>{convertQaToCommaStr(data.commReward)}</span>
                </div>
                <div className="d-block operator-stats-card"></div>
            </div>
            </>
        }
        </>
        
    );
}

export default OperatorStatsTable;