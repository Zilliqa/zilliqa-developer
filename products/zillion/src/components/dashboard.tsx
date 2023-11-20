import React, { useEffect, useState } from 'react';
import { ToastContainer, toast } from 'react-toastify';
import ReactTooltip from 'react-tooltip';

import { Role, NetworkURL, Network as NetworkLabel, AccountType, Environment, Constants, TransactionType, ButtonText, ContractState, OperationStatus } from '../util/enum';
import { convertQaToCommaStr, getAddressLink } from '../util/utils';
import StakingPortfolio from './staking-portfolio';
import SsnTable from './ssn-table';
import Alert from './alert';

import { toBech32Address } from '@zilliqa-js/crypto';

import WithdrawCommModal from './contract-calls/withdraw-comm';
import UpdateReceiverAddress from './contract-calls/update-receiver-address';
import UpdateCommRateModal from './contract-calls/update-commission-rate';

import DelegateStakeModal from './contract-calls/delegate-stake';
import ReDelegateStakeModal from './contract-calls/redeleg';
import WithdrawStakeModal from './contract-calls/withdraw-stake';
import WithdrawRewardModal from './contract-calls/withdraw-reward';
import CompleteWithdrawModal from './contract-calls/complete-withdraw';
import SwapDelegModal from './contract-calls/swap-deleg';

import logo from "../static/logo.png";
import DisclaimerModal from './disclaimer';
import DelegatorStatsTable from './delegator-stats-table';
import OperatorStatsTable from './operator-stats-table';
import CompleteWithdrawalTable from './complete-withdrawal-table';

import IconShuffle from './icons/shuffle';
import IconQuestionCircle from './icons/question-circle';
import IconRefresh from './icons/refresh';
import IconBell from './icons/bell';
import IconCheckboxBlankCircle from './icons/checkbox-blank-circle';
import IconSun from './icons/sun';
import IconMoon from './icons/moon';

import useDarkMode from '../util/use-dark-mode';
import { getLocalItem, storeLocalItem } from '../util/use-local-storage';

import Footer from './footer';
import RecentTxnDropdown from './recent-txn';
import Tippy from '@tippyjs/react';
import '../tippy.css';
import 'tippy.js/animations/shift-away-subtle.css';

import WarningDashboardBanner from './warning-dashboard-banner';

import { POLL_USER_DATA_STOP, QUERY_AND_UPDATE_USER_STATS, RESET_USER_STATE, UPDATE_ADDRESS } from '../store/userSlice';
import { useAppDispatch, useAppSelector } from '../store/hooks';
import { logger } from '../util/logger';
import { getEnvironment, getNetworks, NetworkConfig, Networks } from '../util/config-json-helper';
import { RESET_BLOCKCHAIN_STATE, UPDATE_CHAIN_INFO } from '../store/blockchainSlice';
import { ZilSigner } from '../zilliqa-signer';
import { QUERY_AND_UPDATE_STAKING_STATS } from '../store/stakingSlice';


function Dashboard(props: any) {
    const dispatch = useAppDispatch();

    const userState = useAppSelector(state => state.user);
    const isDelegStatsLoading = useAppSelector(state => state.user.is_deleg_stats_loading);
    const blockchainState = useAppSelector(state => state.blockchain);

    // config.js from public folder
    const env = getEnvironment();
    const networks: Networks = getNetworks();
    const proxy = useAppSelector(state => state.blockchain.proxy);
    const networkURL = useAppSelector(state => state.blockchain.blockchain);

    const [walletAddress, setWalletAddress] = useState(userState.address_base16 || '');
    const [blockchain, setBlockchain] = useState(blockchainState.blockchain || '');

    const [isRefreshDisabled, setIsRefreshDisabled] = useState(false);
    const [isTxnNotify, setIsTxnNotify] = useState(false);
    const [ariaExpanded, setAriaExpanded] = useState(false);

    const [recentTransactions, setRecentTransactions] = useState([] as any)

    const darkMode = useDarkMode(true);

    const cleanUp = () => {
        logger("directing to main");
        dispatch(POLL_USER_DATA_STOP());
        dispatch(RESET_USER_STATE());
        dispatch(RESET_BLOCKCHAIN_STATE());
        props.history.push("/");
    }

    // set recent txn indicator icon
    const handleTxnNotify = () => {
        if (!isTxnNotify) {
            return;
        }
        setIsTxnNotify(false);
    }

    const toggleTheme = () => {
        if (darkMode.value === true) {
          darkMode.disable();
        } else {
          darkMode.enable();
        }
    }

    const updateRecentTransactions = (type: TransactionType, txnId: string) => {
        let temp = JSON.parse(JSON.stringify(recentTransactions));
        if ((temp.length + 1) > 10) {
            // suppose we add a new element
            // restrict number of elements as local storage has limits
            // recent txn is always in newest to oldest
            // remove last element - last element = oldest txn
            temp.pop();
        }
        // reverse so that order is oldest to newest
        // add new item as last element
        temp = temp.reverse();
        temp.push({type: type, txnId: txnId});

        // restore order back
        setRecentTransactions([...temp].reverse());
        storeLocalItem(userState.address_bech32, proxy, networkURL, 'recent-txn', temp.reverse());

        // set recent txn indicator icon
        setIsTxnNotify(true);
    }

    // re-hydrate data from localstorage
    useEffect(() => {
        let txns = getLocalItem(userState.address_bech32, proxy, networkURL, 'recent-txn', [] as any); 
        setRecentTransactions(txns);
    }, [userState.address_bech32, proxy, networkURL]);

    const timeout = (delay: number) => {
        return new Promise(res => setTimeout(res, delay));
    }

    const pollData = async () => {
        console.log("polling data...")
        setIsRefreshDisabled(true);
        dispatch(QUERY_AND_UPDATE_USER_STATS());
        dispatch(QUERY_AND_UPDATE_STAKING_STATS());
        await timeout(Constants.MANUAL_REFRESH_DELAY);
        setIsRefreshDisabled(false);
    }

    // for zilpay to toggle different network
    const networkChanger = (net: string) => {
        let label;

        switch (net) {
            case NetworkLabel.MAINNET:
                // do nothing
                Alert("info", "Info", "You are on Mainnet.");
                label = NetworkLabel.MAINNET;
                break;
            case NetworkLabel.TESTNET:
                label = NetworkLabel.TESTNET;
                if (env === Environment.PROD) {
                    // warn users not to switch to testnet on production
                    Alert("warn", "Testnet not supported", "Please switch to Mainnet via ZilPay.");
                }
                break;
            case NetworkLabel.ISOLATED_SERVER:
            case NetworkLabel.PRIVATE:
                label = NetworkLabel.ISOLATED_SERVER;
                if (env === Environment.PROD) {
                    // warn users not to switch to testnet on production
                    Alert("warn", "Private network not supported", "Please switch to Mainnet via ZilPay.");
                }
                break;
            default:
                label = NetworkLabel.TESTNET;
                break;
        }

        const networkConfig: NetworkConfig = networks[label];
        dispatch(UPDATE_CHAIN_INFO({
            proxy: networkConfig.proxy || '',
            impl: networkConfig.impl || '',
            blockchain: networkConfig.blockchain || '',
            staking_viewer: networkConfig.node_status || '',
            api_list: networkConfig.api_list || [],
        }));
    }

    /**
     * When document has loaded, it start to observable network form zilpay.
     */
    useEffect(() => {
        if (userState.account_type === AccountType.ZILPAY) {
            const zilPay = (window as any).zilPay;

            if (zilPay) {
                // switch to the zilpay network on load
                networkChanger(zilPay.wallet.net);

                const accountStreamChanged = zilPay.wallet.observableAccount().subscribe((account: any) => {
                    console.log("zil pay account changing...");
                    const bech32 = toBech32Address(account.base16);
                    dispatch(UPDATE_ADDRESS({ address_base16: account.base16, address_bech32: bech32 }));
                });

                const networkStreamChanged = zilPay.wallet.observableNetwork().subscribe((net: string) => networkChanger(net));

                return () => {
                    accountStreamChanged.unsubscribe();
                    networkStreamChanged.unsubscribe();
                };
            }
        }
        // must only run once due to global listener
        // eslint-disable-next-line
    }, []);

    useEffect(() => {
        if (env === Environment.DEV) {
            // disable auth check for development
            return;
        }

        if (!userState.authenticated) {
            // redirect to login request
            dispatch(POLL_USER_DATA_STOP());
            dispatch(RESET_USER_STATE());
            dispatch(RESET_BLOCKCHAIN_STATE());
            props.history.push("/oops");
        }

    }, [env, userState.authenticated, props.history, dispatch]);

    // change to correct role
    useEffect(() => {
        console.log("change wallet")
        
        if (walletAddress !== userState.address_base16) {
            // address changed
            dispatch(QUERY_AND_UPDATE_USER_STATS());
        }

        setWalletAddress(userState.address_base16);
    }, [walletAddress, userState.address_base16, dispatch]);

    useEffect(() => {
        console.log("change network")
        if (blockchain !== blockchainState.blockchain) {
            // network changed
            dispatch(QUERY_AND_UPDATE_USER_STATS());
            dispatch(QUERY_AND_UPDATE_STAKING_STATS());
            ZilSigner.changeNetwork(blockchainState.blockchain);
        }
        setBlockchain(blockchainState.blockchain);

    }, [blockchain, blockchainState.blockchain, dispatch]);

    // prevent user from refreshing
    useEffect(() => {
        window.onbeforeunload = (e: any) => {
            e.preventDefault();
            e.returnValue = 'The page auto retrieves data periodically. Please do not force refresh as you will lose your wallet connection.';
            setTimeout(() => {
                toast.dismiss();
            }, 8000);
            return (
                Alert("warn", "Warning", "The app auto retrieves data periodically. Please do not force refresh as you will lose your wallet connection.")
            );
        }
    }, []);

    useEffect(() => {
        if (userState.selected_role === Role.OPERATOR && 
            userState.role === Role.DELEGATOR) {
            Alert("warn", "Warning", "You have been redirected to the delegator dashboard.");
        }
    }, [userState.selected_role, userState.role]);


    // eslint-disable-next-line
    return (
        <>
        <nav className="navbar navbar-expand-lg navbar-dark">
            <button type="button" className="btn navbar-brand shadow-none p-0 pl-2">
                <span>
                    <img className="logo mx-auto" src={logo} alt="zilliqa_logo"/>
                    <span className="navbar-title">ZILLIQA STAKING</span>
                </span>
            </button>
            <button className="navbar-toggler" type="button" data-toggle="collapse" data-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
                <span className="navbar-toggler-icon"></span>
            </button>

            <div className="collapse navbar-collapse" id="navbarSupportedContent">
                <ul className="navbar-nav mr-auto">
                </ul>
                <ul className="navbar-nav navbar-right">
                    {/* wallet address */}
                    <li className="nav-item">
                        <p className="px-1">{userState.address_bech32 ? <a href={getAddressLink(userState.address_bech32, networkURL)} className="wallet-link" target="_blank" rel="noopener noreferrer">{userState.address_bech32}</a> : 'No wallet detected'}</p>
                    </li>
                    
                    {/* balance */}
                    <li className="nav-item">
                        <p className="px-1">{userState.balance ? convertQaToCommaStr(`${userState.balance}`) : '0.000'} ZIL</p>
                    </li>

                    {/* network */}
                    <li className="nav-item">
                        { networkURL === NetworkURL.TESTNET && <p className="px-1">Testnet</p> }
                        { networkURL === NetworkURL.MAINNET && <p className="px-1">Mainnet</p> }
                        { networkURL === NetworkURL.ISOLATED_SERVER && <p className="px-1">Isolated Server</p> }
                    </li>

                    <li className="nav-item">
                        <button 
                            type="button" 
                            className="btn btn-notify-dropdown btn-theme shadow-none mx-2"  
                            data-toggle="modal" 
                            data-target="#swap-deleg-modal" 
                            data-keyboard="false" 
                            data-backdrop="static"
                            data-tip
                            data-for="swap-tip">
                                <IconShuffle width="16" height="16"/>
                                {
                                    //@ts-ignore
                                    <span badge={userState.swap_deleg_modal_data.swapRecipientAddress ? (userState.swap_deleg_modal_data.requestorList.length+1) : userState.swap_deleg_modal_data.requestorList.length}></span>
                                }
                        </button>

                        <ReactTooltip id="swap-tip" place="bottom" type="dark" effect="solid">
                            <span>Change Stake Ownership</span>
                        </ReactTooltip>
                    </li>

                    {/* txn notifications */}
                    <li className="nav-item">
                        <Tippy 
                            content={<RecentTxnDropdown data={recentTransactions} networkURL={networkURL} />} 
                            animation="shift-away-subtle"
                            trigger="click"
                            arrow={false}
                            interactive={true}
                            placement="bottom-end"
                            appendTo="parent"
                            onMount={() => setAriaExpanded(true)}
                            onHide={() => setAriaExpanded(false)}>
                                <button 
                                    type="button" 
                                    className="btn btn-notify-dropdown shadow-none" 
                                    onClick={handleTxnNotify} 
                                    aria-haspopup="true" 
                                    aria-expanded={ariaExpanded}
                                    data-tip
                                    data-for="notification-tip">
                                        <div className="dropdown-notify-wrapper">
                                            <IconBell width="16" height="16" className="dropdown-toggle-icon" />
                                            { isTxnNotify && <IconCheckboxBlankCircle width="10" height="10" className="dropdown-notify-icon" /> }
                                        </div>
                                </button>
                        </Tippy>
                        <ReactTooltip id="notification-tip" place="bottom" type="dark" effect="solid">
                            <span>Recent Transactions</span>
                        </ReactTooltip>
                    </li>

                    <li className="nav-item">
                        <button type="button" className="btn btn-notify-dropdown btn-theme shadow-none mx-2" onClick={toggleTheme} data-tip data-for="theme-toggle-tip">
                        { 
                            darkMode.value === true ? 
                            <IconSun width="16" height="16"/> : 
                            <IconMoon width="16" height="16"/>
                        }
                        </button>
                        <ReactTooltip id="theme-toggle-tip" place="bottom" type="dark" effect="solid">
                            <span>Appearance</span>
                        </ReactTooltip>
                    </li>

                    <li className="nav-item">
                        <button type="button" className="btn btn-sign-out mx-2" onClick={cleanUp}>Sign Out</button>
                    </li>
                </ul>
            </div>
        </nav>

        <WarningDashboardBanner />

        <div id="dashboard" className="container-fluid h-100">
            <div className="row h-100">
                <div id="content" className="col pt-4">
                    <div className="container-xl">
                        <div className="row">
                            <div className="col-12">
                                <div className="d-flex justify-content-end">
                                    <button 
                                        type="button" 
                                        className="btn btn-user-secondary-action shadow-none" 
                                        onClick={pollData} data-tip data-for="refresh-tip" 
                                        disabled={isRefreshDisabled || isDelegStatsLoading === OperationStatus.PENDING}>
                                            <IconRefresh width="20" height="20" />
                                    </button>
                                    <ReactTooltip id="refresh-tip" place="bottom" type="dark" effect="solid">
                                        <span>Refresh</span>
                                    </ReactTooltip>
                                </div>

                                
                                {/* delegator section */}
                                {/* complete withdrawal */}
                                {
                                    (userState.role === Role.DELEGATOR) &&

                                    <CompleteWithdrawalTable />
                                }

                                {
                                    (userState.role === Role.OPERATOR) &&

                                    <>
                                    {/* node operator section */}

                                    <div className="p-4 mt-4 dashboard-card">
                                        <h5 className="card-title mb-4">Hi {userState.operator_stats.name ? userState.operator_stats.name : 'Operator'}! What would you like to do today?</h5>
                                        <button 
                                            type="button" 
                                            className="btn btn-contract mr-4 shadow-none" 
                                            data-toggle="modal" 
                                            data-target="#update-comm-rate-modal" 
                                            data-keyboard="false" 
                                            data-backdrop="static" 
                                            disabled={ContractState.IS_PAUSED.toString() === 'true' ? true : false}>
                                                {ContractState.IS_PAUSED.toString() === 'true' ? ButtonText.NOT_AVAILABLE : 'Update Commission'}
                                        </button>
                                        <button 
                                            type="button" 
                                            className="btn btn-contract mr-4 shadow-none" 
                                            data-toggle="modal" 
                                            data-target="#update-recv-addr-modal" 
                                            data-keyboard="false" 
                                            data-backdrop="static" 
                                            disabled={ContractState.IS_PAUSED.toString() === 'true' ? true : false}>
                                                {ContractState.IS_PAUSED.toString() === 'true' ? ButtonText.NOT_AVAILABLE : 'Update Receiving Address'}
                                        </button>
                                        <button 
                                            type="button" 
                                            className="btn btn-contract mr-4 shadow-none" 
                                            data-toggle="modal" 
                                            data-target="#withdraw-comm-modal" 
                                            data-keyboard="false" 
                                            data-backdrop="static" 
                                            disabled={ContractState.IS_PAUSED.toString() === 'true' ? true : false}>
                                                {ContractState.IS_PAUSED.toString() === 'true' ? ButtonText.NOT_AVAILABLE : 'Withdraw Commission'}
                                        </button>
                                    </div>
                                    </>
                                }

                                {
                                    (userState.role === Role.DELEGATOR) &&
                                    <>
                                    {/* delegator statistics */}

                                    <div id="delegator-stats-details" className="p-4 dashboard-card container-fluid">
                                        <div className="row">
                                            <div className="col">
                                                <h5 className="card-title mb-4">Overview</h5>
                                            </div> 
                                            <div className="col-12 text-center">
                                                <DelegatorStatsTable />
                                            </div>
                                        </div>
                                    </div>
                                    </>
                                }

                                {
                                    (userState.role === Role.DELEGATOR) &&
                                    <>
                                    {/* delegator portfolio */}

                                    <div id="staking-portfolio-details" className="p-4 dashboard-card container-fluid">
                                        <div className="row">
                                            <div className="col">
                                                <h5 className="card-title mb-4">My Staking Portfolio</h5>
                                            </div>
                                            <div className="col-12 mt-2 px-4 text-center">
                                                <div className="inner-section">
                                                    <h6 className="inner-section-heading px-4 pt-4 pb-3">Deposits <span data-tip data-for="deposit-question"><IconQuestionCircle width="16" height="16" className="section-icon" /></span></h6>
                                                    <StakingPortfolio />
                                                </div>
                                            </div>
                                            <ReactTooltip id="deposit-question" place="bottom" type="dark" effect="solid">
                                                <span>This shows you the list of nodes which you have staked your deposit in.</span>
                                            </ReactTooltip>
                                        </div>
                                    </div>
                                    </>
                                }

                                {/* operator statistics */}
                                {
                                    (userState.role === Role.OPERATOR) &&

                                   <div id="operator-stats-details" className="p-4 dashboard-card container-fluid">
                                        <div className="row">
                                            <div className="col">
                                                <h5 className="card-title mb-4">My Node Performance</h5>
                                            </div> 
                                            <div className="col-12 text-center">
                                                <OperatorStatsTable />
                                            </div>
                                        </div>
                                    </div>
                                }

                                <div id="dashboard-ssn-details" className="p-4 dashboard-card container-fluid">
                                    <div className="row">
                                        <div className="col">
                                            <h5 className="card-title mb-4">Staked Seed Nodes</h5>
                                            <p className="info mt-4">Please refer to our&nbsp; 
                                                <a className="info-link" href={blockchainState.staking_viewer ? 
                                                    blockchainState.staking_viewer : 
                                                    "https://zilliqa.com/"} 
                                                        target="_blank" 
                                                        rel="noopener noreferrer">
                                                        Staking Viewer 
                                                </a> 
                                                &nbsp;for more information on the nodes' statuses.
                                            </p>
                                        </div>
                                        <div className="col-12 text-center">
                                            <SsnTable showStakeBtn={true} />
                                        </div>
                                    </div>
                                </div>

                                <div>
                                    <ToastContainer 
                                        hideProgressBar={true} 
                                        autoClose={10000} 
                                        pauseOnHover />
                                </div>

                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <Footer />
            <DisclaimerModal />

            <UpdateCommRateModal 
                updateData={pollData}
                updateRecentTransactions={updateRecentTransactions} />

            <UpdateReceiverAddress
                updateData={pollData}
                updateRecentTransactions={updateRecentTransactions} />

            <WithdrawCommModal 
                updateData={pollData}
                updateRecentTransactions={updateRecentTransactions} />

            <DelegateStakeModal 
                updateData={pollData}
                updateRecentTransactions={updateRecentTransactions} />

            <ReDelegateStakeModal
                updateData={pollData}
                updateRecentTransactions={updateRecentTransactions} />

            <WithdrawStakeModal
                updateData={pollData}
                updateRecentTransactions={updateRecentTransactions} />

            <WithdrawRewardModal 
                updateData={pollData}
                updateRecentTransactions={updateRecentTransactions} />

            <CompleteWithdrawModal 
                updateData={pollData}
                updateRecentTransactions={updateRecentTransactions} />
            
            <SwapDelegModal
                updateData={pollData}
                updateRecentTransactions={updateRecentTransactions} />
                
        </div>
        </>
    );
}

export default Dashboard;