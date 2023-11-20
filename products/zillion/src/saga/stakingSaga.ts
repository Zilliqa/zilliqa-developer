import { call, delay, fork, put, race, select, take, takeLatest } from 'redux-saga/effects';
import { logger } from '../util/logger';
import { getBlockchain } from './selectors';
import { CONFIG_LOADED } from '../store/blockchainSlice';
import { POLL_STAKING_DATA_START, POLL_STAKING_DATA_STOP, PRELOAD_INFO_READY, QUERY_AND_UPDATE_STAKING_STATS, UPDATE_FETCH_LANDING_STATS_STATUS, UPDATE_FETCH_SSN_STATS_STATUS, UPDATE_GZIL_ADDRESS, UPDATE_GZIL_TOTAL_SUPPLY, UPDATE_LANDING_STATS, UPDATE_MIN_BNUM_REQ, UPDATE_MIN_DELEG, UPDATE_SSN_DROPDOWN_LIST, UPDATE_SSN_LIST, UPDATE_TOTAL_STAKE_AMOUNT } from '../store/stakingSlice';
import { Constants, OperationStatus, SsnStatus } from '../util/enum';
import { LandingStats, NodeOptions, SsnStats } from '../util/interface';
import { toBech32Address } from '@zilliqa-js/crypto';
import { ZilSdk } from '../zilliqa-api';
import { convertZilToQa, isRespOk } from '../util/utils';
import BigNumber from 'bignumber.js';
import { getRefreshRate } from '../util/config-json-helper';

const MAX_GZIL_SUPPLY = Constants.MAX_GZIL_SUPPLY.toString();
const REWARD_ZIL_PER_DAY = Constants.REWARD_ZIL_PER_DAY.toString();
const REFRESH_INTERVAL = getRefreshRate();

/**
 * fetch these data only once
 */
function* watchInitOnce() {
    try {
        yield put(UPDATE_FETCH_LANDING_STATS_STATUS(OperationStatus.PENDING));
        console.log("fetching preload...");
        const { impl } = yield select(getBlockchain);

        const checksumImpl = impl.replace("0x", '')
        let queries = [
            [checksumImpl, 'mindelegstake', []],
            [checksumImpl, 'totalstakeamount', []],
            [checksumImpl, 'bnum_req', []],
            [checksumImpl, 'gziladdr', []],
        ];

        const response: Object = yield call(ZilSdk.getSmartContractSubStateBatch, queries);
        const mindelegstake = (response as any)[0]["result"]['mindelegstake'];
        const totalstakeamount = (response as any)[1]["result"]['totalstakeamount'];
        const bnum_req = (response as any)[2]["result"]['bnum_req'];
        const gziladdr = (response as any)[3]["result"]['gziladdr'];

        logger("mindelegstake: %o", mindelegstake);

        logger("totalstakeamount: %o", totalstakeamount);

        logger("bnum req: ", bnum_req);

        const { total_supply } = yield call(ZilSdk.getSmartContractSubState, gziladdr, 'total_supply');
        logger("gziladdr: ", gziladdr);
        logger("gzil minted: ", total_supply);

        // populate landing stats data
        const { last_withdraw_cycle_deleg } = yield call(ZilSdk.getSmartContractSubState, impl, 'last_withdraw_cycle_deleg');
        
        if (isRespOk(last_withdraw_cycle_deleg)) {
            const { comm_for_ssn } = yield call(ZilSdk.getSmartContractSubState, impl, 'comm_for_ssn');
            const nodesNum = isRespOk(comm_for_ssn) ? Object.keys(comm_for_ssn).length.toString() : '0';
            const delegNum = Object.keys(last_withdraw_cycle_deleg).length.toString();
            
            let circulatingSupplyStake = '0';
            const totalCoinSupply: string = yield call(ZilSdk.getTotalCoinSupply);
            if (isRespOk(totalCoinSupply)) {
                const totalCoinSupplyBN = new BigNumber(convertZilToQa(totalCoinSupply));
                circulatingSupplyStake = (new BigNumber(totalstakeamount).dividedBy(totalCoinSupplyBN)).times(100).toFixed(5);
            }

            // compute remaining gzil percentage
            const maxGzilSupply = new BigNumber(MAX_GZIL_SUPPLY).shiftedBy(15);
            const remainGzil = maxGzilSupply.minus(new BigNumber(total_supply));
            const remainingGzil = (remainGzil.dividedBy(maxGzilSupply)).times(100).toFixed(2);
            
            // compute est. APY
            const temp = new BigNumber(totalstakeamount);
            const rewardZilPerDay = new BigNumber(convertZilToQa(REWARD_ZIL_PER_DAY));
            const estAPY = rewardZilPerDay.dividedBy(temp).times(36500).toFixed(2);

            const landingStats: LandingStats = {
                circulatingSupplyStake: circulatingSupplyStake,
                nodesNum: nodesNum,
                delegNum: delegNum,
                gzil: total_supply,
                remainingGzil: remainingGzil,
                totalDeposits: totalstakeamount,
                estRealtimeAPY: estAPY,
            }
            logger("landing stats: ", landingStats);
            yield put(UPDATE_LANDING_STATS({ landing_stats: landingStats }));
        }

        yield put(UPDATE_MIN_BNUM_REQ({ min_bnum_req: bnum_req }));
        yield put(UPDATE_MIN_DELEG({ min_deleg_stake: mindelegstake }));
        yield put(UPDATE_TOTAL_STAKE_AMOUNT({ total_stake_amount: totalstakeamount }));
        yield put(UPDATE_GZIL_ADDRESS({ gzil_address: gziladdr }));
        yield put(UPDATE_GZIL_TOTAL_SUPPLY({ gzil_total_supply: total_supply }));

    } catch (e) {
        console.warn("fetch home data failed");
        console.warn(e);
    } finally {
        yield put(UPDATE_FETCH_LANDING_STATS_STATUS(OperationStatus.COMPLETE));
        yield put(PRELOAD_INFO_READY()); // inform other saga that preloaded info is in store
    }
}

function* pollStakingData() {
    try {
        yield put(UPDATE_FETCH_SSN_STATS_STATUS(OperationStatus.PENDING));
        logger("fetching ssn data...");
        const { impl } = yield select(getBlockchain);

        const checksumImpl = impl.replace("0x", '')
        let queries = [
            [checksumImpl, 'ssnlist', []],
            [checksumImpl, 'ssn_deleg_amt', []],
        ];

        const response: Object = yield call(ZilSdk.getSmartContractSubStateBatch, queries);
        const ssnlist = (response as any)[0]["result"]["ssnlist"];
        const ssn_deleg_amt = (response as any)[1]["result"]["ssn_deleg_amt"];

        logger("ssnlist: ", ssnlist);
        logger("ssn_deleg_amt: ", ssn_deleg_amt);

        let dropdown_list: NodeOptions[] = [];
        let ssn_list: SsnStats[] = [];
        for (const ssnAddress in ssnlist) {
            const ssnArgs = ssnlist[ssnAddress]['arguments'];
            const status = (ssnArgs[0]['constructor'] === 'True') ? SsnStatus.ACTIVE : SsnStatus.INACTIVE;
            let delegNum = '0';

            if (ssn_deleg_amt.hasOwnProperty(ssnAddress)) {
                delegNum = Object.keys(ssn_deleg_amt[ssnAddress]).length.toString();
            }

            // for ssn table
            const ssnStats: SsnStats = {
                address: toBech32Address(ssnAddress),
                name: ssnArgs[3],
                apiUrl: ssnArgs[5],
                stakeAmt: ssnArgs[1],
                bufferedDeposits: ssnArgs[6],
                commRate: ssnArgs[7],
                commReward: ssnArgs[8],
                delegNum: delegNum,
                status: status,
            }

            // for use as dropdown options in the modals
            const dropdownOptions: NodeOptions = {
                address: toBech32Address(ssnAddress),
                name: ssnArgs[3],
                stakeAmt: ssnArgs[1],
                delegNum: delegNum,
                commRate: ssnArgs[7],
            }

            ssn_list.push(ssnStats);
            dropdown_list.push(dropdownOptions);
        }

        yield put(UPDATE_SSN_DROPDOWN_LIST({ dropdown_list: dropdown_list }));
        yield put(UPDATE_SSN_LIST({ ssn_list: ssn_list }));
    } catch (e) {
        console.warn("fetch failed");
        yield put(UPDATE_SSN_DROPDOWN_LIST({ dropdown_list: [] }));
        yield put(UPDATE_SSN_LIST({ ssn_list: [] }));
    } finally {
        yield put(UPDATE_FETCH_SSN_STATS_STATUS(OperationStatus.COMPLETE));
    }
}

/**
 * reload user data when zilpay change network
 * stop the current poll and resume later
 */
function* queryAndUpdateStats() {
    yield put(POLL_STAKING_DATA_STOP());
    yield call(pollStakingData);

    // delay before start to poll again
    yield delay(REFRESH_INTERVAL);
    yield put(POLL_STAKING_DATA_START());
}

function* pollStakingSaga() {
    while (true) {
        try {
            yield call(pollStakingData);
        } catch (e) {
            console.warn("poll staking data failed");
            console.warn(e);
        } finally {
            yield delay(REFRESH_INTERVAL);
        }
    }
}

function* watchPollStakingData() {
    while (true) {
        yield take(POLL_STAKING_DATA_START);
        yield race([
            call(pollStakingSaga),
            take(POLL_STAKING_DATA_STOP),
        ])
    }
}

function* stakingSaga() {
    yield take(CONFIG_LOADED) // wait for app to load details from config
    yield fork(watchInitOnce)
    
    yield fork(watchPollStakingData)
    yield takeLatest(QUERY_AND_UPDATE_STAKING_STATS, queryAndUpdateStats)
}

export default stakingSaga;