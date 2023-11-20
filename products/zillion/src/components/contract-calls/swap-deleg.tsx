import React, { useState, useEffect } from 'react';
import { trackPromise } from 'react-promise-tracker';
import { toast } from 'react-toastify';
import { AccountType, OperationStatus, ProxyCalls, TransactionType } from '../../util/enum';
import { bech32ToChecksum, computeGasFees, convertBase16ToBech32, getTruncatedAddress, getZillionExplorerLink, isDigits, isRespOk, showWalletsPrompt, validateBalance } from '../../util/utils';
import Alert from '../alert';
import { toBech32Address, fromBech32Address } from '@zilliqa-js/crypto';
import { Tab, Tabs, TabList, TabPanel } from 'react-tabs';

import IconWalletTransferLong from '../icons/wallet-transfer-long';
import IconQuestionCircle from '../icons/question-circle';
import IconArrowDown from '../icons/arrow-down';
import IconEditBox from '../icons/edit-box-line';
import ModalPending from '../contract-calls-modal/modal-pending';
import ModalSent from '../contract-calls-modal/modal-sent';
import ReactTooltip from 'react-tooltip';
import { computeDelegRewards } from '../../util/reward-calculator';

import SwapImg from "../../static/swap_img0.png";
import SwapImg1 from "../../static/swap_img1.png";
import SwapImg2 from "../../static/swap_img2.png";
import SwapImg3 from "../../static/swap_img3.png";
import SwapImg4 from "../../static/swap_img4.png";
import SwapImg5 from "../../static/swap_img5.png";
import { isStorageAvailable } from '../../util/use-local-storage';
import { useAppSelector } from '../../store/hooks';
import { SwapDelegModalData } from '../../util/interface';
import { ZilSigner } from '../../zilliqa-signer';
import { ZilSdk } from '../../zilliqa-api';
import { units } from '@zilliqa-js/zilliqa';
import BigNumber from 'bignumber.js';
import GasSettings from './gas-settings';


const { BN, validation } = require('@zilliqa-js/util');

function SwapDelegModal(props: any) {
    const {updateData, updateRecentTransactions} = props;
    const swapDelegModalData: SwapDelegModalData = useAppSelector(state => state.user.swap_deleg_modal_data);
    const proxy = useAppSelector(state => state.blockchain.proxy);
    const impl = useAppSelector(state => state.blockchain.impl);
    const networkURL = useAppSelector(state => state.blockchain.blockchain);
    const wallet = useAppSelector(state => state.user.address_base16);
    const accountType = useAppSelector(state => state.user.account_type);
    const ledgerIndex = useAppSelector(state => state.user.ledger_index);
    const userAddress = useAppSelector(state => state.user.address_bech32);
    const ssnstatsList = useAppSelector(state => state.staking.ssn_list);
    const proxyChecksum = bech32ToChecksum(proxy);

    // transfer all stakes to this new deleg
    const [newDelegAddr, setNewDelegAddr] = useState(''); // bech32
    const [selectedDelegAddr, setSelectedDelegAddr] = useState(''); // base16; used in incoming requests tab to track the address being accepted or rejected
    const [txnId, setTxnId] = useState('');
    const [txnType, setTxnType] = useState('');
    const [tabIndex, setTabIndex] = useState(0);
    const [isPending, setIsPending] = useState('');
    const [isEdit, setIsEdit] = useState(false);
    
    const [showHelpBox, setShowHelpBox] = useState(false);
    const [showConfirmSendRequestBox, setShowConfirmSendRequestBox] = useState(false);
    const [showConfirmRevokeBox, setShowConfirmRevokeBox] = useState(false);
    const [showConfirmSwapBox, setShowConfirmSwapBox] = useState(false);
    const [showConfirmRejectBox, setShowConfirmRejectBox] = useState(false);

    const [tutorialStep, setTutorialStep] = useState(0); // for the help guide

    const defaultGasPrice = ZilSigner.getDefaultGasPrice();
    const defaultGasLimit = ZilSigner.getDefaultGasLimit();
    const [gasPrice, setGasPrice] = useState<string>(defaultGasPrice);
    const [gasLimit, setGasLimit] = useState<string>(defaultGasLimit);
    const [gasOption, setGasOption] = useState(false);

    const cleanUp = () => {
        setNewDelegAddr('');
        setSelectedDelegAddr('');
        setTabIndex(0);
        setIsPending('');
        setIsEdit(false);
        setTxnId('');
        setTxnType('');
        setShowHelpBox(false);
        setShowConfirmSendRequestBox(false);
        setShowConfirmRevokeBox(false);
        setShowConfirmSwapBox(false);
        setShowConfirmRejectBox(false);
        setTutorialStep(0);
        setGasOption(false);
        setGasPrice(defaultGasPrice);
        setGasLimit(defaultGasLimit);
    }

    const validateAddress = (address: string) => {
        if (!address || address === "" || (!validation.isAddress(address) && !validation.isBech32(address)) ) {
            return false;
        }
        return true;
    }

    // checks if user entered ssn address
    // address zil format
    const isSsnAddress = (address: string) => {
        for (let ssn of ssnstatsList) {
            if (address === ssn.address) {
                return true;
            }
        }
        return false;
    }

    const handleNewDelegAddr = (e : any) => {
        let address = e.target.value;
        if (!address || address === null) {
            address = '';
        } else if (address && (validation.isAddress(address) || validation.isBech32(address)) ) {
            address = toBech32Address(bech32ToChecksum(address));
            if (isSsnAddress(address)) {
                Alert('error', "Invalid Address", `Do not enter a node address. If you want to transfer your stake from one node to another node, go to "Staking Portfolio" > "Manage" > "Transfer Stake" `);
                address = '';
            } 
        }
        setNewDelegAddr(address);
    }

    const decreaseTutorialStep = () => {
        let newTutorialStep = tutorialStep - 1;
        setTutorialStep(newTutorialStep);
    }

    const incrementTutorialStep = () => {
        let newTutorialStep = tutorialStep + 1;
        setTutorialStep(newTutorialStep);
    }

    // validate the input target recipient before showing the confirm send box
    const toggleConfirmSendRequestBox = () => {
        let targetRecipientAddr = newDelegAddr;
        if (!validateAddress(targetRecipientAddr)) {
            Alert('error', "Invalid Address", "Wallet address should be bech32 or checksum format.");
            return null;
        }
        setShowConfirmSendRequestBox(true);
    }

    const toggleConfirmSwapBox = (address : string) => {
        setSelectedDelegAddr(address);
        setShowConfirmSwapBox(true);
    }

    const toggleRejectSwapBox = (address : string) => {
        setSelectedDelegAddr(address);
        setShowConfirmRejectBox(true);
    }

    const toggleRevokeSwapBox = () => {
        // hide the new owner address form when clicked on revoke
        setIsEdit(false);
        setShowConfirmRevokeBox(true);
    }

    const handleClose = () => {
        // txn success
        // invoke dashboard methods
        if (txnId) {
            let convertedTxnType;
            switch (txnType) {
                case TransactionType.REQUEST_DELEG_SWAP.toString():
                    convertedTxnType = TransactionType.REQUEST_DELEG_SWAP;
                    break;
                case TransactionType.REVOKE_DELEG_SWAP.toString():
                    convertedTxnType = TransactionType.REVOKE_DELEG_SWAP;
                    break;
                case TransactionType.CONFIRM_DELEG_SWAP.toString():
                    convertedTxnType = TransactionType.CONFIRM_DELEG_SWAP;
                    break;
                case TransactionType.REJECT_DELEG_SWAP.toString():
                    convertedTxnType = TransactionType.REJECT_DELEG_SWAP;
                    break;
            }
            updateRecentTransactions(convertedTxnType, txnId);
            updateData();
        }
        
        // reset state
        // timeout to wait for modal to fade out before clearing
        // so that the animation is smoother
        toast.dismiss();
        setTimeout(() => {
            cleanUp();
        }, 150);
    }

    // reset the tutorial
    const handleCloseTutorial = () => {
        setShowHelpBox(false);
        setTutorialStep(0);
    }

    const sendTxn = async (txnType: TransactionType, txParams: any) => {
        if (await validateBalance(wallet) === false) {
            const gasFees = computeGasFees(gasPrice, gasLimit);
            Alert('error', "Insufficient Balance", "Insufficient balance in wallet to pay for the gas fee.");
            Alert('error', "Gas Fee Estimation", "Current gas fee is around " + units.fromQa(gasFees, units.Units.Zil) + " ZIL.");
            setIsPending('');
            return null;
        }

        // set txn type to store in cookie
        setTxnType(txnType.toString());
        setIsPending(OperationStatus.PENDING);

        showWalletsPrompt(accountType);

        trackPromise(ZilSigner.sign(accountType as AccountType, txParams, ledgerIndex)
            .then((result) => {
                if (result === OperationStatus.ERROR) {
                    Alert('error', "Transaction Error", "Please try again.");
                } else {
                    setTxnId(result)
                }
            }).finally(() => {
                setIsPending('');
            })
        );
    }

    const confirmDelegSwap = async (requestorAddr: string) => {

        setIsPending(OperationStatus.PENDING);

        const requestorHasBuffOrRewards = await hasBufferedOrRewards("ConfirmSwap", requestorAddr);
        if (requestorHasBuffOrRewards) {
            // requestor has buffered deposits or rewards
            setIsPending('');
            return null;
        }

        // userAddress here is the new delegator waiting to receive
        const newDelegHasBuffOrRewards = await hasBufferedOrRewards("ConfirmSwap", userAddress);
        if (newDelegHasBuffOrRewards) {
            setIsPending('');
            return null;
        }

        let txParams = {
            toAddr: proxyChecksum,
            amount: new BN(0),
            code: "",
            data: JSON.stringify({
                _tag: ProxyCalls.CONFIRM_DELEG_SWAP,
                params: [
                    {
                        vname: 'requestor',
                        type: 'ByStr20',
                        value: `${requestorAddr}`,
                    }
                ]
            }),
            gasPrice: gasPrice,
            gasLimit: gasLimit,
        };

        await sendTxn(TransactionType.CONFIRM_DELEG_SWAP, txParams);
    }

    const rejectDelegSwap = async (requestorAddr: string) => {

        let txParams = {
            toAddr: proxyChecksum,
            amount: new BN(0),
            code: "",
            data: JSON.stringify({
                _tag: ProxyCalls.REJECT_DELEG_SWAP,
                params: [
                    {
                        vname: 'requestor',
                        type: 'ByStr20',
                        value: `${requestorAddr}`,
                    }
                ]
            }),
            gasPrice: gasPrice,
            gasLimit: gasLimit,
        };

        await sendTxn(TransactionType.REJECT_DELEG_SWAP, txParams);
    }

    // returns true if address is ok
    // otherwise returns false
    // @param swapAddr: bech32 format
    const isValidSwap = (swapAddr: string) => {
        // check if it is self swap
        if (userAddress === swapAddr) {
            Alert('error', "Invalid New Owner", "Please enter another wallet address other than the connected wallet.");
            return false;
        }

        // check if it is cyclic, i.e. if B -> X, where X == A exists
        let byStr20SwapAddr = fromBech32Address(swapAddr).toLowerCase();

        if (swapDelegModalData.requestorList.includes(byStr20SwapAddr)) {
            let msg = `There is an existing request from ${getTruncatedAddress(swapAddr)}. Please accept or reject the incoming request first.`;
            Alert('error', "Invalid New Owner", msg);
            return false;
        }

        return true;
    }

    // check if address has buffered deposits or unwithdrawn rewards
    // returns true if the address has buffered deposits or rewards, otherwise returns false
    // @param invokerMethod: a string to determine if it is coming from RequestSwap or ConfirmSwap to change the message display
    // @param address: bech32 format
    const hasBufferedOrRewards = async (invokerMethod: string = "", address: string) => {
        let wallet = bech32ToChecksum(address).toLowerCase();
        let displayAddr = getTruncatedAddress(address);
        let targetName = address === userAddress ? "Your wallet" : invokerMethod === "RequestSwap" ? "The recipient" : "The requestor"

        const lrc = await ZilSdk.getSmartContractSubState(impl, "lastrewardcycle");
        const lbdc = await  ZilSdk.getSmartContractSubState(impl, "last_buf_deposit_cycle_deleg", [wallet]);
        const deposit_amt_deleg_map = await ZilSdk.getSmartContractSubState(impl, "deposit_amt_deleg", [wallet]);
        const buff_deposit_deleg_map = await ZilSdk.getSmartContractSubState(impl, "buff_deposit_deleg", [wallet]);
        
        if (!isRespOk(lrc)) {
            return false;
        }

        if (!isRespOk(lbdc)) {
            return false;
        }

        if (!isRespOk(deposit_amt_deleg_map)) {
            // delegator has not delegate with any ssn
            // no need to perform further checks
            return false;
        }

        let ssnlist = deposit_amt_deleg_map["deposit_amt_deleg"][wallet];

        for (let ssnAddress in ssnlist) {
            // check rewards
            const rewards = new BN(await computeDelegRewards(impl, ssnAddress, wallet));

            if (rewards.gt(new BN(0))) {
                let msg = `${targetName} ${displayAddr} has unwithdrawn rewards. Please withdraw or wait until the user has withdrawn the rewards before continuing.`
                Alert('info', "Unwithdrawn Rewards Found", msg);
                return true;
            }

            const lrc_o = parseInt(lrc["lastrewardcycle"]);

            // check buffered deposits
            if (lbdc["last_buf_deposit_cycle_deleg"][wallet].hasOwnProperty(ssnAddress)) {
                const ldcd = parseInt(lbdc["last_buf_deposit_cycle_deleg"][wallet][ssnAddress]);
                if (lrc_o <= ldcd) {
                    let msg = `${targetName} ${displayAddr} has buffered deposits. Please wait for the next cycle before continuing.`
                    Alert('info', "Buffered Deposits Found", msg);
                    return true;
                }
            }

            // check buffered deposits (lrc-1)
            let lrc_o_minus = lrc_o - 1
            if (lrc_o_minus >= 0) {
                if (isRespOk(buff_deposit_deleg_map) &&  
                    buff_deposit_deleg_map["buff_deposit_deleg"][wallet].hasOwnProperty(ssnAddress)) {
                    if (buff_deposit_deleg_map["buff_deposit_deleg"][wallet][ssnAddress].hasOwnProperty(lrc_o_minus)) {
                        let msg = `${targetName} ${displayAddr} has buffered deposits. Please wait for the next cycle before continuing.`
                        Alert('info', "Buffered Deposits Found", msg);
                        return true;
                    }
                }
            }

        }
        return false;
    }

    // check if address has staked with some ssn
    // returns false if address has not stake; otherwise returns true
    const hasStaked = async (address: string) => {
        let wallet = fromBech32Address(address).toLowerCase();

        const deposit_amt_deleg_map = await ZilSdk.getSmartContractSubState(impl, "deposit_amt_deleg", [wallet]);

        if (!isRespOk(deposit_amt_deleg_map)) {
            return false;
        }

        return true;
    }

    // check if address has pending withdrawal
    // returns false if address has no pending withdrawal
    const hasPendingWithdrawal = async (address: string) => {
        let wallet = fromBech32Address(address).toLowerCase();

        const pending_withdrawal_map = await ZilSdk.getSmartContractSubState(impl, "withdrawal_pending", [wallet]);

        if (!isRespOk(pending_withdrawal_map)) {
            return false;
        }

        return true;
    }

    const requestDelegSwap = async () => {

        if (!validateAddress(newDelegAddr)) {
            Alert('error', "Invalid Address", "Wallet address should be bech32 or checksum format.");
            return null;
        }

        if (!isValidSwap(newDelegAddr)) {
            return null;
        }

        setIsPending(OperationStatus.PENDING);

        const userHasStaked = await hasStaked(userAddress);
        const userHasPendingWithdrawal = await hasPendingWithdrawal(userAddress);
        if (!userHasStaked && !userHasPendingWithdrawal) {
            setIsPending('');
            Alert('info', "User Has No Stake", `You have not staked with any operators.`);
            return null;
        }

        const userHasBuffOrRewards = await hasBufferedOrRewards("RequestSwap", userAddress);
        if (userHasBuffOrRewards) {
            // user has buffered deposits or rewards
            setIsPending('');
            return null;
        }

        const newDelegHasBuffOrRewards = await hasBufferedOrRewards("RequestSwap", newDelegAddr);
        if (newDelegHasBuffOrRewards) {
            setIsPending('');
            return null;
        }

        // gas price, gas limit declared in account.ts
        let txParams = {
            toAddr: proxyChecksum,
            amount: new BN(0),
            code: "",
            data: JSON.stringify({
                _tag: ProxyCalls.REQUEST_DELEG_SWAP,
                params: [
                    {
                        vname: 'new_deleg_addr',
                        type: 'ByStr20',
                        value: `${fromBech32Address(newDelegAddr)}`,
                    }
                ]
            }),
            gasPrice: gasPrice,
            gasLimit: gasLimit,
        };

        await sendTxn(TransactionType.REQUEST_DELEG_SWAP, txParams);
    }

    const revokeDelegSwap = async () => {
        // gas price, gas limit declared in account.ts
        let txParams = {
            toAddr: proxyChecksum,
            amount: new BN(0),
            code: "",
            data: JSON.stringify({
                _tag: ProxyCalls.REVOKE_DELEG_SWAP,
                params: []
            }),
            gasPrice: gasPrice,
            gasLimit: gasLimit,
        };

        await sendTxn(TransactionType.REVOKE_DELEG_SWAP, txParams);
    }

    const onBlurGasPrice = () => {
        if (gasPrice === '' || new BigNumber(gasPrice).lt(new BigNumber(defaultGasPrice))) {
            setGasPrice(defaultGasPrice);
            Alert("Info", "Minimum Gas Price Required", "Gas price should not be lowered than default blockchain requirement.");
        }
    }

    const onGasPriceChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        let input = e.target.value;
        if (input === '' || isDigits(input)) {
            setGasPrice(input);
        }
    }

    const onBlurGasLimit = () => {
        if (gasLimit === '' || new BigNumber(gasLimit).lt(50)) {
            setGasLimit(defaultGasLimit);
        }
    }

    const onGasLimitChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        let input = e.target.value;
        if (input === '' || isDigits(input)) {
            setGasLimit(input);
        }
    }

    const onSelectTabs = (index: number) => {
        // reset the advanced gas settings display on change tabs
        setTabIndex(index);
        setGasOption(false);
        setGasPrice(defaultGasPrice);
        setGasLimit(defaultGasLimit);
    }

    useEffect(() => {
        // show the tutorial if this is the first time
        // user clicks on the change stake ownership button
        if (isStorageAvailable('localStorage')) {
            const storedValue: any = window.localStorage.getItem("show-swap-help");
            if (storedValue === null) {
                setShowHelpBox(true);
                window.localStorage.setItem("show-swap-help", JSON.stringify(false));
            }
        }
    }, [])

    return (
        <div id="swap-deleg-modal" className="modal fade" tabIndex={-1} role="dialog" aria-labelledby="swapDelegModalLabel" aria-hidden="true">
            <div className="contract-calls-modal modal-dialog modal-dialog-centered modal-dialog-scrollable modal-lg" role="document">
                <div className="modal-content">
                    {
                        isPending ?
                        
                        <ModalPending/>

                        :

                        txnId ?

                        <ModalSent txnId={txnId} networkURL={networkURL} handleClose={handleClose}/>

                        :

                        showHelpBox ?

                        <>
                        <div className="modal-header">
                            {
                                tutorialStep === 0 ?
                                <h5 className="modal-title" id="swap-deleg-modal">What is this?</h5>
                                :
                                tutorialStep === 1 ?
                                <h5 className="modal-title" id="swap-deleg-modal">How do I transfer my stakes to another wallet?</h5>
                                :
                                tutorialStep === 2 ?
                                <h5 className="modal-title" id="swap-deleg-modal">How do I modify the recipient or revoke the request?</h5>
                                :
                                tutorialStep === 3 ?
                                <h5 className="modal-title" id="swap-deleg-modal">I received an incoming transfer request. What should I do with it?</h5>
                                :
                                tutorialStep === 4 ?
                                <h5 className="modal-title" id="swap-deleg-modal">I accepted a request. How do I know if it is successful?</h5>
                                :
                                tutorialStep === 5 ?
                                <h5 className="modal-title" id="swap-deleg-modal">Final Notes</h5>
                                :
                                null
                            }
                            
                            <button type="button" className="close btn shadow-none" aria-label="CloseTutorial" onClick={handleCloseTutorial}>
                                <span aria-hidden="true">&times;</span>
                            </button>
                        </div>
                        <div className="modal-body">
                            {
                                tutorialStep === 0 ?
                                <div className="div-fade text-center">
                                    <IconWalletTransferLong width="400" className="mb-4 icon-fill"/>
                                    <p>You can now transfer your stakes from one wallet to another!</p>
                                    <button type="button" className="btn btn-user-action mx-auto shadow-none" onClick={() => incrementTutorialStep()}>Continue</button>
                                </div>

                                :
                                
                                tutorialStep === 1 ?
                                <div>
                                    <p>Under the "Change Stake Ownership" tab, enter the recipient wallet address.</p>
                                    <div className="d-flex mb-4"><img className="mx-auto img-fluid" src={SwapImg} alt="change_ownership" /></div>
                                    <p>Wait for the transaction to be processed on the blockchain.</p>
                                    <div className="d-flex mt-4">
                                        <div className="mx-auto">
                                            <button type="button" className="btn btn-user-action-cancel mx-2 shadow-none" onClick={() => decreaseTutorialStep()}>Prev</button>
                                            <button type="button" className="btn btn-user-action mx-2 shadow-none" onClick={() => incrementTutorialStep()}>Next</button>
                                        </div>
                                    </div>
                                </div>
                                
                                :

                                tutorialStep === 2 ?
                                <div>
                                    <p>After setting a recipient, should you change your mind, you may edit the recipient to another address or revoke the request entirely.</p>
                                    <div className="d-flex mb-4"><img className="mx-auto img-fluid" src={SwapImg1} alt="edit_revoke_request" /></div>
                                    <p>This can only be done if the recipient has not accepted your request.</p>
                                    <div className="d-flex mt-4">
                                        <div className="mx-auto">
                                            <button type="button" className="btn btn-user-action-cancel mx-2 shadow-none" onClick={() => decreaseTutorialStep()}>Prev</button>
                                            <button type="button" className="btn btn-user-action mx-2 shadow-none" onClick={() => incrementTutorialStep()}>Next</button>
                                        </div>
                                    </div>
                                </div>

                                :

                                tutorialStep === 3 ?
                                <div>
                                    <p>Under the "Incoming Requests", if you are the <strong>recipient</strong>, you will see the list of wallet addresses / users who wish to transfer their stakes to you.</p>
                                    <div className="d-flex mb-4"><img className="mx-auto img-fluid" src={SwapImg2} alt="incoming_requests" /></div>
                                    <p>If you <strong>accept</strong> the requests, all the stakes would be transferred from the requestors' wallet to you.</p>
                                    <p>If you <strong>reject</strong> the requests, nothing would happen and the requestor has to send a new request if he or she wishes to transfer the stakes to you.</p>
                                    <p>Please wait for the transaction to be processed completely on the blockchain before accepting or rejecting another request.</p>
                                    <div className="d-flex mt-4">
                                        <div className="mx-auto">
                                            <button type="button" className="btn btn-user-action-cancel mx-2 shadow-none" onClick={() => decreaseTutorialStep()}>Prev</button>
                                            <button type="button" className="btn btn-user-action mx-2 shadow-none" onClick={() => incrementTutorialStep()}>Next</button>
                                        </div>
                                    </div>
                                </div>

                                :

                                tutorialStep === 4 ?
                                <div>
                                    <p>If you are the <strong>recipient</strong>, you should see a transaction ID after accepting the request as shown below.</p>
                                    <div className="d-flex mb-4"><img className="mx-auto img-fluid" src={SwapImg3} alt="txn_id" /></div>
                                    <p>Click on the transaction ID and it would bring you to ViewBlock.</p>
                                    <p>Refresh the ViewBlock page once every few minutes; the transaction should be successful in less than 10 minutes.</p>
                                    <p>Head back to <strong>Zillion Dashboard</strong> and hit the manual refresh button.</p>
                                    <div className="d-flex mb-4"><img className="mx-auto img-fluid" src={SwapImg4} alt="manual_refresh" /></div>
                                    <p>You should observe the new amounts under <strong>My Staking Portfolio</strong>.</p>
                                    <div className="d-flex mb-4"><img className="mx-auto img-fluid" src={SwapImg5} alt="dashboard_staking_portfolio" /></div>
                                    <div className="d-flex mt-4">
                                        <div className="mx-auto">
                                            <button type="button" className="btn btn-user-action-cancel mx-2 shadow-none" onClick={() => decreaseTutorialStep()}>Prev</button>
                                            <button type="button" className="btn btn-user-action mx-2 shadow-none" onClick={() => incrementTutorialStep()}>Next</button>
                                        </div>
                                    </div>
                                </div>

                                :

                                tutorialStep === 5 ?
                                <div>
                                    <ul>
                                        <li className="mb-3"><strong>Once the recipient has accepted a request, Zilliqa is not able to undo the transfer process.</strong></li>
                                        <li className="mb-3">Beware of scams! Do not send a transfer request if you are unsure of who the recipient is!</li>
                                        <li className="mb-3">Do not enter any node operator's address.</li>
                                        <li className="mb-3">If you are a recipient, once you accept the transfer request, all the stakes would be transferred to your wallet. If the requestor has staked on the same node operator as you, the amount would be tabulated together.</li>
                                        <li className="mb-3">If you are a requestor, please check the recipient address carefully before sending the request. The transfer request is <strong>irreversible</strong> once the recipient has accepted.</li>
                                        <li>Ensure that you have no buffered deposits or unwithdrawn rewards before sending a transfer request or accepting a transfer. This is to facilitate the stakes tabulation when the recipient chooses to accept the request.</li>
                                    </ul>
                                    <div className="d-flex mt-4">
                                        <div className="mx-auto">
                                            <button type="button" className="btn btn-user-action-cancel mx-2 shadow-none" onClick={() => decreaseTutorialStep()}>Prev</button>
                                            <button type="button" className="btn btn-user-action mx-2 shadow-none" onClick={() => handleCloseTutorial()}>Got It!</button>
                                        </div>
                                    </div>
                                </div>

                                :

                                null

                            }
                        </div>
                        </>

                        :

                        showConfirmSendRequestBox ?

                        <div className="modal-body animate__animated animate__fadeIn">
                            <h5 className="modal-title mb-4">Send Request Confirmation</h5>
                            <p>Are you sure you want to transfer <strong><u>ALL</u></strong> your stakes to this address?</p>
                            <div className="row node-details-wrapper mb-4">
                                <div className="col node-details-panel">
                                    <h3>Target Recipient</h3>
                                    <span>{newDelegAddr}</span>
                                </div>
                            </div>
                            <GasSettings
                                gasOption={gasOption}
                                gasPrice={gasPrice}
                                gasLimit={gasLimit}
                                setGasOption={setGasOption}
                                onBlurGasPrice={onBlurGasPrice}
                                onBlurGasLimit={onBlurGasLimit}
                                onGasPriceChange={onGasPriceChange}
                                onGasLimitChange={onGasLimitChange}
                            />
                            <div className="mb-4">
                                <small><strong>Notes</strong></small>
                                <ul>
                                    <li><small>By clicking on <em>'Yes'</em>, you are sending a request to transfer all your existing stakes to this address.</small></li>
                                    <li><small>Beware of scams! Ensure that you know who the recipient is before sending the request.</small></li>
                                    <li><small>Process is <strong>irreversible</strong> once the recipient has accepted your request.</small></li>
                                </ul>
                            </div>
                            <div className="d-flex mt-4">
                                <div className="mx-auto">
                                    <button type="button" className="btn btn-user-action-cancel mx-2 shadow-none" onClick={() => setShowConfirmSendRequestBox(false)}>
                                        Cancel
                                    </button>
                                    <button type="button" className="btn btn-user-action mx-2 shadow-none" onClick={requestDelegSwap}>
                                        Yes
                                    </button>
                                </div>
                            </div>
                        </div>


                        :

                        showConfirmRevokeBox ?

                        <div className="modal-body animate__animated animate__fadeIn">
                            <h5 className="modal-title mb-4">Revoke Confirmation</h5>
                            <p>Are you sure you want to revoke the existing transfer ownership request?</p>
                            <div className="row node-details-wrapper mb-4">
                                <div className="col node-details-panel">
                                    <h3>Pending New Owner To Accept</h3>
                                    <span>{convertBase16ToBech32(swapDelegModalData.swapRecipientAddress)}</span>
                                </div>
                            </div>
                            <div className="mx-1">
                                <GasSettings
                                    gasOption={gasOption}
                                    gasPrice={gasPrice}
                                    gasLimit={gasLimit}
                                    setGasOption={setGasOption}
                                    onBlurGasPrice={onBlurGasPrice}
                                    onBlurGasLimit={onBlurGasLimit}
                                    onGasPriceChange={onGasPriceChange}
                                    onGasLimitChange={onGasLimitChange}
                                />
                            </div>
                            <div className="d-flex mt-4">
                                <div className="mx-auto">
                                    <button type="button" className="btn btn-user-action-cancel mx-2 shadow-none" onClick={() => setShowConfirmRevokeBox(false)}>
                                        Cancel
                                    </button>
                                    <button type="button" className="btn btn-user-action mx-2 shadow-none" onClick={revokeDelegSwap}>
                                        Yes
                                    </button>
                                </div>
                            </div>
                        </div>

                        :

                        showConfirmRejectBox ?

                        <div className="modal-body animate__animated animate__fadeIn">
                            <h5 className="modal-title mb-4">Reject Confirmation</h5>
                            <p>Are you sure you wish to <u><em>reject</em></u> the transfer request?</p>

                            <div className="row node-details-wrapper mb-4">
                                <div className="col node-details-panel">
                                    <h3>Requestor's Wallet</h3>
                                    <span>{convertBase16ToBech32(selectedDelegAddr)}</span>
                                    <a href={getZillionExplorerLink(convertBase16ToBech32(selectedDelegAddr))} target="_blank" rel="noopener noreferrer" data-tip data-for="explorer-tip"><IconEditBox width="16" height="16" className="zillion-explorer-link"/></a>
                                    <ReactTooltip id="explorer-tip" place="bottom" type="dark" effect="solid">
                                        <span>View Wallet on Zillion Explorer</span>
                                    </ReactTooltip>
                                </div>
                            </div>

                            <div className="mx-1">
                                <GasSettings
                                    gasOption={gasOption}
                                    gasPrice={gasPrice}
                                    gasLimit={gasLimit}
                                    setGasOption={setGasOption}
                                    onBlurGasPrice={onBlurGasPrice}
                                    onBlurGasLimit={onBlurGasLimit}
                                    onGasPriceChange={onGasPriceChange}
                                    onGasLimitChange={onGasLimitChange}
                                />
                            </div>

                            <div className="mb-4">
                                <small><strong>Notes</strong></small>
                                <ul>
                                    <li><small>By clicking on <em>'Yes'</em>, you are rejecting the transfer request and would not receive the stakes from the requestor's wallet.</small></li>
                                </ul>
                            </div>

                            <div className="d-flex mt-4">
                                <div className="mx-auto">
                                    <button type="button" className="btn btn-user-action-cancel mx-2 shadow-none" onClick={() => setShowConfirmRejectBox(false)}>
                                        Cancel
                                    </button>
                                    <button type="button" className="btn btn-user-action mx-2 shadow-none" onClick={() => rejectDelegSwap(selectedDelegAddr)}>
                                        Yes
                                    </button>
                                </div>
                            </div>
                        </div>

                        :

                        showConfirmSwapBox ?

                        <div className="modal-body animate__animated animate__fadeIn">
                            <h5 className="modal-title mb-4">Accept Confirmation</h5>
                            <p>Are you sure you wish to <u><em>accept</em></u> all the stakes from this wallet?</p>

                            <div className="row node-details-wrapper mb-4">
                                <div className="col node-details-panel">
                                    <h3>Requestor's Wallet</h3>
                                    <span>{convertBase16ToBech32(selectedDelegAddr)}</span>
                                    <a href={getZillionExplorerLink(convertBase16ToBech32(selectedDelegAddr))} target="_blank" rel="noopener noreferrer" data-tip data-for="explorer-tip"><IconEditBox width="16" height="16" className="zillion-explorer-link"/></a>
                                    <ReactTooltip id="explorer-tip" place="bottom" type="dark" effect="solid">
                                        <span>View Wallet on Zillion Explorer</span>
                                    </ReactTooltip>
                                </div>
                            </div>

                            <div className="mx-1">
                                <GasSettings
                                    gasOption={gasOption}
                                    gasPrice={gasPrice}
                                    gasLimit={gasLimit}
                                    setGasOption={setGasOption}
                                    onBlurGasPrice={onBlurGasPrice}
                                    onBlurGasLimit={onBlurGasLimit}
                                    onGasPriceChange={onGasPriceChange}
                                    onGasLimitChange={onGasLimitChange}
                                />
                            </div>

                            <div className="mb-4">
                                <small><strong>Notes</strong></small>
                                <ul>
                                    <li><small>By clicking on <em>'Yes'</em>, all the stakes are transferred from the requestor's wallet to your wallet.</small></li>
                                    <li><small>This transfer process is <strong>irreversible.</strong></small></li>
                                </ul>
                            </div>

                            <div className="d-flex mt-4">
                                <div className="mx-auto">
                                    <button type="button" className="btn btn-user-action-cancel mx-2 shadow-none" onClick={() => setShowConfirmSwapBox(false)}>
                                        Cancel
                                    </button>
                                    <button type="button" className="btn btn-user-action mx-2 shadow-none" onClick={() => confirmDelegSwap(selectedDelegAddr)}>
                                        Yes
                                    </button>
                                </div>
                            </div>
                        </div>

                        :

                        <>
                        <div className="d-flex">
                            <button type="button" className="btn btn-notify-dropdown btn-theme ml-auto mt-1 mr-2" aria-label="Help" data-tip data-for="tutorial-tip" onClick={() => setShowHelpBox(true)}>
                                <IconQuestionCircle width="22" height="22"/>
                            </button>
                            <button type="button" className="close btn shadow-none mx-2 mt-2" data-dismiss="modal" aria-label="Close" onClick={handleClose}>
                                <span aria-hidden="true">&times;</span>
                            </button>
                            <ReactTooltip id="tutorial-tip" place="bottom" type="dark" effect="solid">
                                <strong>Help</strong>
                            </ReactTooltip>
                        </div>
                        <Tabs selectedIndex={tabIndex} onSelect={(index) => onSelectTabs(index)}>
                            <TabList>
                                <Tab>Change Stake Ownership { swapDelegModalData.swapRecipientAddress ? <span>(1)</span> : null }</Tab>
                                <Tab>
                                    Incoming Requests { swapDelegModalData.requestorList.length > 0 ? <span>({swapDelegModalData.requestorList.length})</span> : null}
                                </Tab>
                            </TabList>

                            <TabPanel>
                                <div className="modal-body">
                                    <p className="mb-4">Transfer <strong>ALL</strong> your existing stakes from one wallet to another by entering a new owner address.</p>
                                    {
                                        swapDelegModalData.swapRecipientAddress 
                                        ?
                                        <div className="row node-details-wrapper mb-4">
                                            <div className="col node-details-panel">
                                                <h3>Pending New Owner To Accept</h3>
                                                <span>{userAddress} (Connected Wallet)</span>
                                                <br/>
                                                <IconArrowDown width="16" height="16" className="swap-icon mt-2 mb-2"/>
                                                <br/>
                                                <span>{convertBase16ToBech32(swapDelegModalData.swapRecipientAddress)} (New Owner)</span>
                                            </div>
                                        </div>
                                        :
                                        null
                                    }

                                    
                                    {
                                        // show new address form if editing or no swap recipient
                                        (isEdit || !swapDelegModalData.swapRecipientAddress)
                                        ?
                                        <div className="edit-swap-addr">
                                            <div className="modal-label mb-2"><strong>Enter new owner address</strong><br/>(in bech32 format e.g. zil1xxxxxxxxxxxxx)</div>
                                            <div className="input-group mb-2">
                                                <input type="text" className="form-control shadow-none" value={newDelegAddr} onChange={handleNewDelegAddr} />
                                            </div> 
                                        </div>
                                        :
                                        null
                                    }

                                    <div className="edit-swap-notes">
                                        <small><strong>Notes</strong></small>
                                        <ul>
                                            <li><small>Beware of scams! Ensure that you know who the recipient is before sending the request.</small></li>
                                            <li><small>Process is <strong>irreversible</strong> once the recipient has accepted your request.</small></li>
                                        </ul>
                                        <p></p>
                                    </div>

                                    <div className="d-flex mt-4">
                                        {
                                            swapDelegModalData.swapRecipientAddress
                                            ?
                                            <div className="mx-auto">
                                                {
                                                    isEdit
                                                    ?
                                                    // allow users to send the modified address
                                                    <button type="button" className="btn btn-user-action mx-2 shadow-none" onClick={() => toggleConfirmSendRequestBox()}>Send Request</button>
                                                    :
                                                    <button type="button" className="btn btn-user-action mx-2 shadow-none" onClick={() => setIsEdit(true)}>Edit Request</button>
                                                }
                                                <button type="button" className="btn btn-user-action-cancel mx-2 shadow-none" onClick={() => toggleRevokeSwapBox()}>Revoke Request</button>
                                            </div>
                                            :
                                            // show new owner form
                                            <button type="button" className="btn btn-user-action mx-auto shadow-none" onClick={() => toggleConfirmSendRequestBox()}>Send Request</button>
                                        }
                                    </div>
                                </div>
                            </TabPanel>

                            <TabPanel>
                                {
                                    swapDelegModalData.requestorList.length === 0
                                    ?
                                    <div className="swap-deleg-requestor-list m-4">
                                        <p>You have no incoming requests.</p>
                                    </div>
                                    :
                                    <>
                                    <ul className="p-3 list-unstyled swap-deleg-requestor-list">
                                        {
                                            swapDelegModalData.requestorList.map((requestorAddr: string, index: number) => (
                                                <li key={index}>
                                                    <div className="flex mb-2">
                                                        <span>{convertBase16ToBech32(requestorAddr)}</span>
                                                        <a href={getZillionExplorerLink(convertBase16ToBech32(requestorAddr))} target="_blank" rel="noopener noreferrer" data-tip data-for="explorer-tip2"><IconEditBox width="16" height="16" className="zillion-explorer-link"/></a>
                                                        <div className="float-right btn-contract-group">
                                                            <button type="button" className="btn btn-contract-small shadow-none mx-2" onClick={() => toggleConfirmSwapBox(requestorAddr)}>Accept</button>
                                                            <button type="button" className="btn btn-contract-small-cancel shadow-none mx-2" onClick={() => toggleRejectSwapBox(requestorAddr)}>Reject</button>
                                                        </div>
                                                    </div>
                                                </li>
                                            ))
                                        }
                                    </ul>
                                    <ReactTooltip id="explorer-tip2" place="bottom" type="dark" effect="solid">
                                        <span>View Wallet on Zillion Explorer</span>
                                    </ReactTooltip>
                                    </>
                                }

                            </TabPanel>
                        </Tabs>
                        </>
                    }
                </div>
            </div>
        </div>
    );
}

export default SwapDelegModal;