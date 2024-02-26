import React, { useEffect, useRef, useState, useCallback } from 'react';
import { trackPromise } from 'react-promise-tracker';
import DisclaimerModal from './disclaimer';
import Footer from './footer';

import { Environment, Network, PromiseArea, ContractState, OperationStatus } from '../util/enum';
import { DelegStats, DelegStakingPortfolioStats, initialDelegStats, PendingWithdrawStats } from '../util/interface';

import { fromBech32Address, toBech32Address } from '@zilliqa-js/crypto';
import { validation } from "@zilliqa-js/util";
import { computeDelegRewards } from '../util/reward-calculator';
import { convertQaToCommaStr, isRespOk } from '../util/utils';
import Spinner from './spinner';
import useDarkMode from '../util/use-dark-mode';

import ZillionLogo from '../static/zillion.svg';
import ZillionLightLogo from '../static/light/zillion.svg';
import IconSun from './icons/sun';
import IconMoon from './icons/moon';
import IconSearch from './icons/search';
import ExplorerStakingPortfolio from './explorer-staking-portfolio';
import WarningBanner from './warning-banner';
import ExplorerPendingWithdrawalTable from './explorer-pending-withdrawal-table';
import RewardCountdownTable from './reward-countdown-table';
import { getEnvironment } from '../util/config-json-helper';
import { useAppSelector } from '../store/hooks';
import { ZilSdk } from '../zilliqa-api';
import { BigNumber } from 'bignumber.js';

function Explorer(props: any) {
    const address = props.match.params.address; // bech32 wallet address;
    const [walletBase16Address, setWalletBase16Address] = useState('');
    const [explorerSearchAddress, setExplorerSearchAddress] = useState('');
    const [delegStats, setDelegStats] = useState<DelegStats>(initialDelegStats);
    const [stakedNodeList, setStakedNodeList] = useState([] as DelegStakingPortfolioStats[]);

    const [pendingWithdrawalList, setPendingWithdrawlList] = useState([] as PendingWithdrawStats[]);
    const [totalWithdrawAmt, setTotalWithdrawAmt] = useState('0');

    // config.js from public folder
    const impl = useAppSelector(state => state.blockchain.impl);
    const env = getEnvironment();
    const network = env === Environment.PROD ? Network.MAINNET : Network.TESTNET;

    const mountedRef = useRef(true);

    // need this to set the correct theme
    // eslint-disable-next-line
    const darkMode = useDarkMode(true);

    const getWalletAddress = useCallback(() => {
        let wallet = '';

        if (validation.isBech32(address)) {
            // bech32
            try {
                wallet = fromBech32Address(address).toLowerCase();
            } catch (err) {
                // input address maybe of bech32 format but cannot be decoded
                console.error("No such address: %o", address);
                wallet = '';
            }
        } else if (validation.isAddress(address)) {
            // base16
            wallet = address.toLowerCase();
        } else {
            // invalid address
            wallet = '';
        }

        return wallet;
    }, [address]);
    
    useEffect(() => {
        let totalDeposits = new BigNumber(0);
        let totalRewards = new BigNumber(0);
        let wallet = getWalletAddress();
        let stakedNodesList: DelegStakingPortfolioStats[] = [];

        if (mountedRef.current) {
            setWalletBase16Address(wallet);
        }

        trackPromise(ZilSdk.getSmartContractSubState(impl, 'deposit_amt_deleg', [wallet])
            .then(async (contractState) => {
                if (contractState === undefined || contractState === null || contractState === OperationStatus.ERROR) {
                    return null;
                }

                const depositDelegList = contractState['deposit_amt_deleg'][wallet];

                // fetch the ssn information for each deposit
                const ssnContractState = await ZilSdk.getSmartContractSubState(impl, 'ssnlist');

                for (const ssnAddress in depositDelegList) {
                    if (!depositDelegList.hasOwnProperty(ssnAddress)) {
                        continue;
                    }

                    // compute total deposits
                    const delegAmtQaBN = new BigNumber(depositDelegList[ssnAddress]);
                    totalDeposits = totalDeposits.plus(delegAmtQaBN);

                    // compute zil rewards
                    const delegRewards = new BigNumber(await computeDelegRewards(impl, ssnAddress, wallet)).toString();
                    totalRewards = totalRewards.plus(delegRewards);

                    // append data to list of staked nodes
                    const data: DelegStakingPortfolioStats = {
                        ssnName: ssnContractState["ssnlist"][ssnAddress]["arguments"][3],
                        ssnAddress: toBech32Address(ssnAddress),
                        delegAmt: `${delegAmtQaBN}`,
                        rewards: `${delegRewards}`
                    }
                    stakedNodesList.push(data);
                }
            })
            .catch((err) => {
                console.error(err);
                return null;
            })
            .finally(() => {
                if (mountedRef.current) {
                    const data : DelegStats = {
                        ...initialDelegStats,
                        zilRewards: `${totalRewards}`,
                        gzilRewards: `${totalRewards}`,
                        totalDeposits: `${totalDeposits}`,
                    }
                    setDelegStats(data);
                    setStakedNodeList([...stakedNodesList]);
                }
            }), PromiseArea.PROMISE_GET_EXPLORER_STATS);

    }, [impl, address, getWalletAddress]);


    // compute pending withdrawal progress
    useEffect(() => {
        let wallet = getWalletAddress();
        let pendingWithdrawList: PendingWithdrawStats[] = [];
        let totalWithdrawAmt = new BigNumber(0);
        let progress = '0';

        trackPromise(ZilSdk.getSmartContractSubState(impl, 'withdrawal_pending', [wallet])
            .then(async (contractState) => {
                if (contractState === undefined || contractState === null || contractState === OperationStatus.ERROR) {
                    return null;
                }
    
                const blkNumPendingWithdrawal = contractState['withdrawal_pending'][wallet];

                // get min bnum req
                const blkNumReqState = await ZilSdk.getSmartContractSubState(impl, 'bnum_req');
                const blkNumReq = blkNumReqState['bnum_req'];
                const numTxBlk = await ZilSdk.getNumTxBlocks();
                const currBlkNum = isRespOk(numTxBlk) === true ? new BigNumber(numTxBlk!).minus(1) : new BigNumber(0);

                // compute each of the pending withdrawal progress
                for (const blkNum in blkNumPendingWithdrawal) {
                    if (!blkNumPendingWithdrawal.hasOwnProperty(blkNum)) {
                        continue;
                    }
        
                    // compute each pending stake withdrawal progress
                    let pendingAmt = new BigNumber(blkNumPendingWithdrawal[blkNum]);
                    let blkNumCheck = new BigNumber(blkNum).plus(blkNumReq);
                    let blkNumCountdown = blkNumCheck.minus(currBlkNum); // may be negative
                    let completed = new BigNumber(0);
        
                    // compute progress using blk num countdown ratio
                    if (blkNumCountdown.isLessThanOrEqualTo(0)) {
                        // can withdraw
                        totalWithdrawAmt = totalWithdrawAmt.plus(pendingAmt);
                        blkNumCountdown = new BigNumber(0);
                        completed = new BigNumber(1);
                    } else {
                        // still have pending blks
                        // 1 - (countdown/blk_req)
                        const processed = blkNumCountdown.dividedBy(blkNumReq);
                        completed = new BigNumber(1).minus(processed);
                    }
        
                    // convert progress to percentage
                    progress = completed.times(100).toFixed(2);
        
                    // record the stake withdrawal progress
                    pendingWithdrawList.push({
                        amount: `${pendingAmt}`,
                        blkNumCountdown: `${blkNumCountdown}`,
                        blkNumCheck: `${blkNumCheck}`,
                        progress: `${progress}`
                    } as PendingWithdrawStats)
                }
            })
            .catch((err) => {
                console.error(err);
                return null;
            })
            .finally(() => {
                if (mountedRef.current) {
                    setPendingWithdrawlList([...pendingWithdrawList]);
                    setTotalWithdrawAmt(`${totalWithdrawAmt}`)
                }
            }), PromiseArea.PROMISE_GET_EXPLORER_PENDING_WITHDRAWAL);
    }, [impl, getWalletAddress]);

    const redirectToMain = () => {
        props.history.push("/");
    };

    const toggleTheme = () => {
        if (darkMode.value === true) {
          darkMode.disable();
        } else {
          darkMode.enable();
        }
    };

    const toggleZillionLogo = () => {
        if (darkMode.value === true) {
          return <img src={ZillionLogo} alt="zillion" width="480px" className="mt-2 mb-4 zillion-logo" />;
        } else {
          return <img src={ZillionLightLogo} alt="zillion" width="480px" className="mt-2 mb-4 zillion-logo" />;
        }
    };

    const handleExplorerSearchAddress = (e: any) => {
        setExplorerSearchAddress(e.target.value);
    }
    
    const explorerCheckRewards = () => {
        const zillionExplorerUrl = "/address/" + explorerSearchAddress
        props.history.push(zillionExplorerUrl);
    };

    const handleKeyPress = (e: any) => {
        if (e.keyCode === 13) {
            // Enter key
            // proceed to search
            explorerCheckRewards();
        }
    }

    return (
        <div className="cover explorer">
            <div className="container-fluid">
                <div className="row align-items-center">
                    <div className="cover-content col-12 text-center">

                        <WarningBanner />

                        <div 
                            id="explorer-mini-navbar" 
                            className={
                                ContractState.IS_PAUSED.toString() === "true" ? 
                                'explorer-mini-navbar-disabled d-flex align-items-end mr-4' : 
                                'explorer-mini-navbar-enabled d-flex align-items-end mr-4'}>

                        <div>
                            <button type="button" className="btn btn-theme shadow-none mr-3" onClick={toggleTheme}>
                            { 
                                darkMode.value === true ? 
                                <IconSun width="20" height="20"/> : 
                                <IconMoon width="20" height="20"/>
                            }
                            </button>
                        </div>

                        { 
                            ( env === Environment.STAGE || env === Environment.PROD ) && 
                            <span className="mr-2">{network}</span>
                        }

                        </div>

                        <div className="heading">
                            <>{toggleZillionLogo()}</>
                            <p className="tagline">Staking with Zilliqa. Simplified!</p>
                        </div>

                        <div className="wallet-access">
                            <h2 className="mb-0">Zillion Explorer</h2>
                      
                            <div className="d-flex justify-content-center h-100">
                                <div className="explorer-search mb-4">
                                    <input type="text" className="explorer-search-input" value={explorerSearchAddress} onKeyDown={handleKeyPress} onChange={handleExplorerSearchAddress} placeholder="Enter wallet address to check rewards" maxLength={42}/>
                                    <button type="button" className="btn explorer-search-icon shadow-none" onClick={() => explorerCheckRewards()}><IconSearch width="18" height="18" /></button>
                                </div>
                            </div>
                           
                            <h6 className="explorer-wallet mt-4">{walletBase16Address && address}</h6>
                        </div>
                        
                        { !walletBase16Address && <p className="mb-4">No such address.</p> }

                        {/* delegator rewards */}
                        {
                            walletBase16Address &&

                            <>

                            <RewardCountdownTable />

                            <div id="delegator-stats-details" className="p-4 dashboard-card container">
                                <div className="row">
                                    <div className="col text-left">
                                        <h5 className="card-title mb-4">Overview</h5>
                                    </div>
                                    <div className="col-12 text-center">
                                        <Spinner class="spinner-border dashboard-spinner mb-4" area={PromiseArea.PROMISE_GET_EXPLORER_STATS} />
                                        <div className="row px-2 pb-3 align-items-center justify-content-center">
                                            <div className="d-block deleg-stats-card">
                                                <h3>Total Deposits</h3>
                                                <span>{convertQaToCommaStr(delegStats.totalDeposits)}</span>
                                            </div>
                                            <div className="d-block deleg-stats-card">
                                                <h3>Unclaimed ZIL Rewards</h3>
                                                <span>{convertQaToCommaStr(delegStats.zilRewards)}</span>
                                            </div>
                                            <div className="d-block deleg-stats-card">
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                            
                            </>
                        }

                        {/* each ssn staked */}

                        {
                            walletBase16Address &&

                            <>
                            <div id="staking-portfolio-details" className="p-4 dashboard-card mb-4 container">
                                <div className="row">
                                    <div className="col">
                                        <h5 className="card-title mb-4 text-left">Nodes Staked</h5>
                                    </div>
                                    <div className="col-12">
                                        <ExplorerStakingPortfolio data={stakedNodeList} />
                                    </div>
                                </div>
                            </div>

                            <ExplorerPendingWithdrawalTable
                                data={pendingWithdrawalList}
                                totalWithdrawAmt={totalWithdrawAmt} />
                            </>
                        }

                        <button type="button" className="btn btn-user-action-cancel mt-2 mx-2" onClick={redirectToMain}>Back to Main</button>

                    </div>
                    <Footer />
                    <DisclaimerModal />
                </div>
            </div>
        </div>
    );
}

export default Explorer;