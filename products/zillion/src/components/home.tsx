import React, { useEffect, useState } from 'react';
import { withRouter } from "react-router-dom";
import ReactTooltip from "react-tooltip";

import { AccountType, ContractState, Role } from '../util/enum';
import DisclaimerModal from './disclaimer';
import Footer from './footer';
import SsnTable from './ssn-table';

import WalletKeystore from './wallet-keystore';
import WalletLedger from './wallet-ledger';
import WalletZilPay from './wallet-zilpay';
import WalletTorch from "./wallet-torch";

import IconKeystoreLine from './icons/keystore-line';
import IconLedgerLine from './icons/ledger-line';
import IconZilPayLine from './icons/zil-pay-line';
import IconTorchLine from "./icons/torch-line";

import ZillionLightLogo from '../static/light/zillion.svg';
import ZillionLogo from '../static/zillion.svg';
import LandingStatsTable from './landing-stats-table';

import AvelyLogo from '../static/avely.svg';
import PlunderswapLogo from '../static/plunderswap_dao.png';
import TorchWalletLogo from '../static/torch_wallet.png';

import { ToastContainer } from 'react-toastify';
import useDarkMode from '../util/use-dark-mode';
import IconSearch from './icons/search';
import WarningBanner from './warning-banner';

import { useAppDispatch, useAppSelector } from '../store/hooks';
import { QUERY_AND_UPDATE_STAKING_STATS } from '../store/stakingSlice';
import { INIT_USER, QUERY_AND_UPDATE_USER_STATS, UPDATE_LEDGER_INDEX } from '../store/userSlice';
import { envStringToEnv, getDefaultNetworkForEnv, networkToNetworkName } from '../util/config-helper';
import { getEnvironment } from '../util/config-json-helper';
import { logger } from '../util/logger';
import { ZilSigner } from '../zilliqa-signer';
import RewardCountdownTable from './reward-countdown-table';


function Home(props: any) {
  const dispatch = useAppDispatch();
  const chainInfo = useAppSelector(state => state.blockchain);

  // config.js from public folder
  const env = getEnvironment();

  const [isDirectDashboard, setIsDirectDashboard] = useState(false);
  const [isShowAccessMethod, setShowAccessMethod] = useState(false);
  const [explorerSearchAddress, setExplorerSearchAddress] = useState('');
  const [role, setRole] = useState('');
  const [accessMethod, setAccessMethod] = useState('');
  const [selectedNetwork, setSelectedNetwork] = useState(
    () => getDefaultNetworkForEnv(envStringToEnv(env))
  );

  const darkMode = useDarkMode(true);

  // trigger show wallets to choose
  const resetWalletsClicked = () => {
    setAccessMethod('');
    setIsDirectDashboard(false);
  }

  const timeout = (delay: number) => {
    return new Promise(res => setTimeout(res, delay));
  }

  const redirectToDashboard = async (addressBase16: string, addressBech32: string, accountType: AccountType, ledgerIndex?: number) => {
    // login success
    // update store and signer network
    dispatch(INIT_USER({ address_base16: addressBase16, address_bech32: addressBech32, account_type: accountType, authenticated: true, selected_role: role }));
    dispatch(QUERY_AND_UPDATE_USER_STATS());
    await ZilSigner.changeNetwork(chainInfo.blockchain);

    if (accountType === AccountType.LEDGER && typeof (ledgerIndex) !== 'undefined') {
      // update ledger index to store if using ledger
      dispatch(UPDATE_LEDGER_INDEX({ ledger_index: ledgerIndex }));
    }

    // add some delay
    await timeout(1000);
    logger("directing to dashboard");
    props.history.push("/dashboard");
  }

  const handleAccessMethod = (access: string) => {
    setAccessMethod(access);
  }

  const handleShowAccessMethod = (selectedRole: string) => {
    setRole(selectedRole);
    setShowAccessMethod(true);
  }

  const toggleDirectToDashboard = () => {
    setIsDirectDashboard(true);
  }

  const resetView = () => {
    setRole('');
    setShowAccessMethod(false);
    setAccessMethod('');
  }

  const DisplayAccessMethod = () => {
    switch (accessMethod) {
      case AccountType.KEYSTORE:
        return <WalletKeystore
          onReturnCallback={resetWalletsClicked}
          onWalletLoadingCallback={toggleDirectToDashboard}
          onSuccessCallback={redirectToDashboard}
          role={role} />;
      case AccountType.ZILPAY:
        return <WalletZilPay
          onReturnCallback={resetWalletsClicked}
          onWalletLoadingCallback={toggleDirectToDashboard}
          onSuccessCallback={redirectToDashboard}
          role={role} />;
      case AccountType.TORCH:
        return (
          <WalletTorch
            onReturnCallback={resetWalletsClicked}
            onWalletLoadingCallback={toggleDirectToDashboard}
            onSuccessCallback={redirectToDashboard}
            role={role}
          />
        );
      case AccountType.LEDGER:
        return <WalletLedger
          onReturnCallback={resetWalletsClicked}
          onWalletLoadingCallback={toggleDirectToDashboard}
          onSuccessCallback={redirectToDashboard}
          role={role} />;
      default:
        return null;
    }
  }

  const DisplayLoader = () => {
    logger("retrieving wallet info...");
    return (
      <div className="wallet-access">
        <h2>Retrieving wallet info...</h2>
        <div className="spinner-border dashboard-spinner" role="status">
          <span className="sr-only">Connecting...</span>
        </div>
      </div>
    );
  }

  // const toggleTheme = () => {
  //   if (darkMode.value === true) {
  //     darkMode.disable();
  //   } else {
  //     darkMode.enable();
  //   }
  // }

  const toggleZillionLogo = () => {
    if (darkMode.value === true) {
      return <img src={ZillionLogo} alt="zillion" width="480px" className="mt-2 mb-4 zillion-logo" />;
    } else {
      return <img src={ZillionLightLogo} alt="zillion" width="480px" className="mt-2 mb-4 zillion-logo" />;
    }
  }

  const handleExplorerSearchAddress = (e: any) => {
    setExplorerSearchAddress(e.target.value);
  }

  const handleExplorerKeyPress = (e: any) => {
    if (e.keyCode === 13) {
      // Enter key
      // proceed to search
      explorerCheckRewards();
    }
  }

  const explorerCheckRewards = () => {
    const zillionExplorerUrl = "/address/" + explorerSearchAddress
    props.history.push(zillionExplorerUrl);
  };

  useEffect(() => {
    setSelectedNetwork(getDefaultNetworkForEnv(envStringToEnv(env)));
    dispatch(QUERY_AND_UPDATE_STAKING_STATS());
  }, [env]); // eslint-disable-line react-hooks/exhaustive-deps

  useEffect(() => {
    window.onbeforeunload = null;
  }, []);


  return (
    <div className="cover">
      <div className="container-fluid">
        <div className="row align-items-center">
          <div className="cover-content col-12 text-center">

            <WarningBanner />

            <div
              id="home-mini-navbar"
              className={
                ContractState.IS_PAUSED.toString() === "true" ?
                  'home-mini-navbar-disabled d-flex align-items-end mr-4' :
                  'home-mini-navbar-enabled d-flex align-items-end mr-4'}>

              {/* <div>
                <button type="button" className="btn btn-theme shadow-none mr-3" onClick={toggleTheme}>
                  {
                    darkMode.value === true ?
                      <IconSun width="20" height="20" /> :
                      <IconMoon width="20" height="20" />
                  }
                </button>
              </div> */}

              {
                selectedNetwork &&
                  <span className="mr-2">{networkToNetworkName(selectedNetwork)}</span>
              }

            </div>

            <div className="heading">
              <>{toggleZillionLogo()}</>
              <p className="tagline">Staking with Zilliqa. Simplified!</p>
            </div>
            <div className='d-sm-flex justify-content-center pt-5'>
              <div
                className='btn-logos d-flex justify-content-center align-items-center mx-2'
                onClick={() => window.location.href = 'https://dapp.avely.fi/'}
              >
                <img className='mr-1' src={AvelyLogo} alt="Avely logo"></img>
                <span className='ml-1'>Avely</span>
              </div>
              <div
                className='btn-logos d-flex justify-content-center align-items-center mx-2 mt-3 mt-sm-0'
                onClick={() => window.location.href = 'https://instantunstaking.torchwallet.io/'}
              >
                <img className='mr-1 cover-img' src={TorchWalletLogo} width={26} height={26} alt="Torch wallet logo"></img>
                <span className='ml-1'>Torch Wallet</span>
              </div>
                <div
                className='btn-logos d-flex justify-content-center align-items-center mx-2 mt-3 mt-sm-0'
                onClick={() => window.location.href = 'https://stake.plunderswap.com/'}
              >
                <img className='mr-1 cover-img' src={PlunderswapLogo} width={26} height={26} alt="Plunderswap logo"></img>
                <span className='ml-1'>PlunderSwap</span>
              </div>
            </div>

            <div className="d-flex justify-content-center h-100 py-3">
              <div className="explorer-search">
                <input type="text" className="explorer-search-input" value={explorerSearchAddress} onKeyDown={handleExplorerKeyPress} onChange={handleExplorerSearchAddress} placeholder="Enter wallet address to check rewards" maxLength={42} />
                <button type="button" className="btn explorer-search-icon shadow-none" onClick={() => explorerCheckRewards()}><IconSearch width="18" height="18" /></button>
              </div>
            </div>

            {
              !isShowAccessMethod ?


                <div className="initial-load">
                  { /* sign in and seed node table */}
                  <div className="btn btn-sign-in mt-4 mx-3" style={{ width: '250px', textAlign: 'center' }} onClick={() => handleShowAccessMethod(Role.DELEGATOR.toString())}>Sign in for Delegators</div>
                  <div className="btn btn-sign-in mt-4 mx-3" style={{ width: '250px', textAlign: 'center' }} onClick={() => handleShowAccessMethod(Role.OPERATOR.toString())}>Sign in for Operators</div>
                  {/* <div 
                    className="btn btn-sign-in mt-4 mx-3" 
                    onClick={() => window.location.href = 'https://dapp.avely.fi/'} 
                    style={{ width: '250px', textAlign: 'center', backgroundColor: '#FF6FAC', color: 'white' }}
                >
                    Liquid Staking
                </div> */}



                  <RewardCountdownTable />
                  <LandingStatsTable />

                  <div id="home-ssn-details" className="container">
                    <div className="row pl-2 pt-4">
                      <div className="col text-left">
                        <h2>Staked Seed Nodes</h2>
                        <p className="info mt-4 mb-0">Please refer to our&nbsp;
                          <a className="info-link" href={chainInfo.staking_viewer ?
                            chainInfo.staking_viewer :
                            "https://zilliqa.com/"}
                            target="_blank"
                            rel="noopener noreferrer">
                            Staking Viewer
                          </a>
                          &nbsp;for more information on the nodes' statuses.
                        </p>
                      </div>
                    </div>
                    <div className="row">
                      <div className="col-12 content">
                        <SsnTable />
                      </div>
                    </div>
                  </div>
                </div>

                :

                !accessMethod ?

                  <>
                    { /* no wallets selected - show wallets to connect */}
                    <p className="wallet-connect-text animate__animated animate__fadeIn"><strong>Connect your wallet to start</strong></p>
                    <div id="wallet-access" className="row align-items-center justify-content-center animate__animated animate__fadeIn mb-4">

                      <div
                        className="btn-wallet-access d-block"
                        onClick={() => handleAccessMethod(AccountType.KEYSTORE)}>
                        <IconKeystoreLine className="home-icon my-4" height="42px" /><span className="d-block mt-0.5">Keystore</span>
                      </div>

                      <div
                        className="btn-wallet-access d-block"
                        onClick={() => handleAccessMethod(AccountType.LEDGER)}>
                        <IconLedgerLine className="home-icon icon-ledger-line my-4" /><span className="d-block mt-0.5">Ledger</span>
                      </div>

                      <div
                        className="btn-wallet-access d-block"
                        onClick={() => handleAccessMethod(AccountType.ZILPAY)}
                        data-tip={`Ensure your ZilPay is on ${getDefaultNetworkForEnv(envStringToEnv(env))} network`}
                      >
                        <IconZilPayLine className="home-icon icon-zilpay-line my-4" /><span className="d-block mt-0.5">ZilPay</span>
                      </div>

                      <div
                        className="btn-wallet-access d-block"
                        onClick={() => handleAccessMethod(AccountType.TORCH)}
                        data-tip={`Ensure your Torch is on ${getDefaultNetworkForEnv(envStringToEnv(env))} network`}
                      >
                        <IconTorchLine className="home-icon icon-torch-line my-4" width={40} height={40} /><span className="d-block mt-0.5">Torch</span>
                      </div>

                      <ReactTooltip place="bottom" type="light" effect="float" />
                    </div>
                    <button type="button" className="btn btn-user-action-cancel mt-5 animate__animated animate__fadeIn" onClick={() => resetView()}>Back to Main</button>
                  </>

                  :

                  <>
                    {/* wallet selected - show chosen wallet component */}
                    {isDirectDashboard ?

                      <>{DisplayLoader()}</>

                      :

                      <>{DisplayAccessMethod()}</>}
                  </>
            }
          </div>
          <ToastContainer hideProgressBar={true} />
          <Footer networkLabel={selectedNetwork} />
          <DisclaimerModal />
        </div>
      </div>
    </div>
  );
}

export default withRouter(Home);
