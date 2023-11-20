import React, { useState, useEffect, useCallback } from 'react';
import { trackPromise } from 'react-promise-tracker';
import { toast } from 'react-toastify';

import Alert from '../alert';
import { bech32ToChecksum, convertZilToQa, convertQaToCommaStr, showWalletsPrompt, convertQaToZilFull, validateBalance, isDigits, computeGasFees, isRespOk } from '../../util/utils';
import { AccountType, OperationStatus, ProxyCalls, TransactionType } from '../../util/enum';
import { computeDelegRewards } from '../../util/reward-calculator';

import ModalPending from '../contract-calls-modal/modal-pending';
import ModalSent from '../contract-calls-modal/modal-sent';
import { useAppSelector } from '../../store/hooks';
import { StakeModalData } from '../../util/interface';
import { ZilSdk } from '../../zilliqa-api';
import { ZilSigner } from '../../zilliqa-signer';
import BigNumber from 'bignumber.js';
import GasSettings from './gas-settings';
import { logger } from '../../util/logger';


const { BN, units } = require('@zilliqa-js/util');


function WithdrawStakeModal(props: any) {
    const proxy = useAppSelector(state => state.blockchain.proxy);
    const impl = useAppSelector(state => state.blockchain.impl);
    const networkURL = useAppSelector(state => state.blockchain.blockchain);
    const wallet = useAppSelector(state => state.user.address_base16);
    const userBase16Address = useAppSelector(state => state.user.address_base16);
    const ledgerIndex = useAppSelector(state => state.user.ledger_index);
    const accountType = useAppSelector(state => state.user.account_type);
    const minDelegStake = useAppSelector(state => state.staking.min_deleg_stake);
    const minDelegStakeDisplay = units.fromQa(new BN(minDelegStake), units.Units.Zil);
    const stakeModalData: StakeModalData = useAppSelector(state => state.user.stake_modal_data);
    const { updateData, updateRecentTransactions } = props;

    const defaultGasPrice = ZilSigner.getDefaultGasPrice();
    const defaultGasLimit = ZilSigner.getDefaultGasLimit();
    const [gasPrice, setGasPrice] = useState<string>(defaultGasPrice);
    const [gasLimit, setGasLimit] = useState<string>(defaultGasLimit);
    const [gasOption, setGasOption] = useState(false);

    const ssnAddress = stakeModalData.ssnAddress; // bech32
    const [withdrawAmt, setWithdrawAmt] = useState('0'); // in ZIL
    const [txnId, setTxnId] = useState('');
    const [isPending, setIsPending] = useState('');


    // checks if there are any unwithdrawn rewards
    const hasRewardToWithdraw = async () => {
        const ssnChecksumAddress = bech32ToChecksum(ssnAddress).toLowerCase();
        
        const last_reward_cycle_json = await ZilSdk.getSmartContractSubState(impl, "lastrewardcycle");
        const last_buf_deposit_cycle_deleg_json = await ZilSdk.getSmartContractSubState(impl, "last_buf_deposit_cycle_deleg", [userBase16Address]);

        if (!isRespOk(last_reward_cycle_json)) {
            return false;
        }

        if (!isRespOk(last_buf_deposit_cycle_deleg_json)) {
            return false;
        }

        // compute rewards
        const delegRewards = new BN(await computeDelegRewards(impl, ssnChecksumAddress, userBase16Address));

        if (delegRewards.gt(new BN(0))) {
            logger("you have delegated rewards: %o", delegRewards);
            Alert('info', "Unwithdrawn Rewards Found", "Please withdraw the rewards before withdrawing the staked amount.");
            return true;
        }

        // check if user has buffered deposits
        if (last_buf_deposit_cycle_deleg_json.last_buf_deposit_cycle_deleg[userBase16Address].hasOwnProperty(ssnChecksumAddress)) {
                const lastDepositCycleDeleg = parseInt(last_buf_deposit_cycle_deleg_json.last_buf_deposit_cycle_deleg[userBase16Address][ssnChecksumAddress]);
                const lastRewardCycle = parseInt(last_reward_cycle_json.lastrewardcycle);
                if (lastRewardCycle <= lastDepositCycleDeleg) {
                    Alert('info', "Buffered Deposits Found", "Please wait for the next cycle before withdrawing the staked amount.");
                    return true;
                }
        }

        // corner case check
        // if user has buffered deposits
        // happens if user first time deposit
        // reward is zero but contract side warn has unwithdrawn rewards
        // user cannot withdraw zero rewards from UI
        // if (contract.buff_deposit_deleg.hasOwnProperty(userBase16Address) &&
        //     contract.buff_deposit_deleg[userBase16Address].hasOwnProperty(ssnChecksumAddress)) {
        //         const buffDepositMap: any = contract.buff_deposit_deleg[userBase16Address][ssnChecksumAddress];
        //         const lastCycleDelegNum = Object.keys(buffDepositMap).sort().pop() || '0';
        //         const lastRewardCycle = parseInt(contract.lastrewardcycle);

        //         if (lastRewardCycle < parseInt(lastCycleDelegNum + 2)) {
        //             // deposit still in buffer 
        //             // have to wait for 2 cycles to receive rewards to clear buffer
        //             Alert('info', "Buffered Deposits Found", "Please wait for 2 more cycles for your rewards to be issued before withdrawing.");
        //             return true;
        //         }
        // }

        return false;
    }

    const withdrawStake = async () => {
        let withdrawAmtQa;

        if (!ssnAddress) {
            Alert('error', "Invalid Node", "node address should be bech32 or checksum format.");
            return null;
        }

        if (!withdrawAmt) {
            Alert('error', "Invalid Withdraw Amount", "Withdraw amount cannot be empty.");
            return null;
        } else {
            try {
                withdrawAmtQa = convertZilToQa(withdrawAmt);
            } catch (err) {
                // user input is malformed
                // cannot convert input zil amount to qa
                Alert('error', "Invalid Withdraw Amount", "Please check your withdraw amount again.");
                return null;
            }
        }

        if (await validateBalance(wallet) === false) {
            const gasFees = computeGasFees(gasPrice, gasLimit);
            Alert('error', 
            "Insufficient Balance", 
            "Insufficient balance in wallet to pay for the gas fee.");
            Alert('error', "Gas Fee Estimation", "Current gas fee is around " + units.fromQa(gasFees, units.Units.Zil) + " ZIL.");
            return null;
        }

        setIsPending(OperationStatus.PENDING);

        // check if deleg has unwithdrawn rewards or buffered deposits for this ssn address
        const hasRewards = await hasRewardToWithdraw();
        if (hasRewards) {
            setIsPending('');
            return null;
        }

        // create tx params

        // toAddr: proxy address
        const proxyChecksum = bech32ToChecksum(proxy);
        const ssnChecksumAddress = bech32ToChecksum(ssnAddress).toLowerCase();
        const delegAmtQa = stakeModalData.delegAmt;
        const leftOverQa = new BN(delegAmtQa).sub(new BN(withdrawAmtQa));

        // check if withdraw more than delegated
        if (new BN(withdrawAmtQa).gt(new BN(delegAmtQa))) {
            Alert('info', "Invalid Withdraw Amount", "You only have " + convertQaToCommaStr(delegAmtQa) + " ZIL to withdraw." );
            setIsPending('');
            return null;
        } else if (!leftOverQa.isZero() && leftOverQa.lt(new BN(minDelegStake))) {
            // check leftover amount
            // if less than min stake amount
            Alert('info', "Invalid Withdraw Amount", "Please leave at least " +  minDelegStakeDisplay + " ZIL (min. stake amount) or withdraw ALL.");
            setIsPending('');
            return null;
        }

        // gas price, gas limit declared in account.ts
        let txParams = {
            toAddr: proxyChecksum,
            amount: new BN(0),
            code: "",
            data: JSON.stringify({
                _tag: ProxyCalls.WITHDRAW_STAKE_AMT,
                params: [
                    {
                        vname: 'ssnaddr',
                        type: 'ByStr20',
                        value: `${ssnChecksumAddress}`,
                    },
                    {
                        vname: 'amt',
                        type: 'Uint128',
                        value: `${withdrawAmtQa}`,
                    },
                ]
            }),
            gasPrice: gasPrice,
            gasLimit: gasLimit,
        };
        
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

    // set default withdraw amount to current deleg amt
    const setDefaultWithdrawAmt = useCallback(() => {
        if (stakeModalData.delegAmt) {
            const tempDelegAmt = convertQaToZilFull(stakeModalData.delegAmt);
            setWithdrawAmt(tempDelegAmt);
        } else {
            setWithdrawAmt('0');
        }
    }, [stakeModalData.delegAmt]);

    const handleClose = () => {
        // txn success
        // invoke dashboard methods
        if (txnId) {
            updateRecentTransactions(TransactionType.INITIATE_STAKE_WITHDRAW, txnId);
            updateData();
        }
        
        // reset state
        // timeout to wait for modal to fade out before clearing
        // so that the animation is smoother
        toast.dismiss();
        setTimeout(() => {
            setDefaultWithdrawAmt();
            setTxnId('');
            setGasOption(false);
            setGasPrice(defaultGasPrice);
            setGasLimit(defaultGasLimit);
        }, 150);
    }

    const handleWithdrawAmt = (e: any) => {
        setWithdrawAmt(e.target.value);
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

    useEffect(() => {
        setDefaultWithdrawAmt();
    }, [setDefaultWithdrawAmt]);

    return (
        <div id="withdraw-stake-modal" className="modal fade" tabIndex={-1} role="dialog" aria-labelledby="withdrawStakeModalLabel" aria-hidden="true">
            <div className="contract-calls-modal modal-dialog modal-dialog-centered modal-lg" role="document">
                <div className="modal-content">
                    {
                        isPending ?

                        <ModalPending />

                        :

                        txnId ?

                        <ModalSent txnId={txnId} networkURL={networkURL} handleClose={handleClose} />

                        :

                        <>
                        <div className="modal-header">
                            <h5 className="modal-title" id="withdrawStakeModalLabel">Initiate Stake Withdrawal</h5>
                            <button type="button" className="close btn shadow-none" data-dismiss="modal" aria-label="Close" onClick={handleClose}>
                                <span aria-hidden="true">&times;</span>
                            </button>
                        </div>
                        <div className="modal-body">
                            <div className="row node-details-wrapper mb-4">
                                <div className="col node-details-panel mr-4">
                                    <h3>{stakeModalData.ssnName}</h3>
                                    <span>{stakeModalData.ssnAddress}</span>
                                </div>
                                <div className="col node-details-panel">
                                    <h3>Deposit</h3>
                                    <span>{convertQaToCommaStr(stakeModalData.delegAmt)} ZIL</span>
                                </div>
                            </div>

                            <div className="modal-label mb-2"><strong>Enter withdrawal amount</strong></div>
                            <div className="input-group mb-4">
                                <input type="text" className="form-control shadow-none" value={withdrawAmt} onChange={handleWithdrawAmt} />
                                <div className="input-group-append">
                                    <span className="input-group-text pl-4 pr-3">ZIL</span>
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
                            <div className="d-flex">
                                <button type="button" className="btn btn-user-action mx-auto mt-2 shadow-none" onClick={withdrawStake}>Initiate</button>
                            </div>
                        </div>
                        </>
                    }
                </div>
            </div>
        </div>
    );
}

export default WithdrawStakeModal;