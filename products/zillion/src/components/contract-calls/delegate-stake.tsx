import React, { useState, useEffect, useCallback } from 'react';
import { trackPromise } from 'react-promise-tracker';
import { toast } from 'react-toastify';

import Alert from '../alert';
import { bech32ToChecksum, convertZilToQa, convertToProperCommRate, showWalletsPrompt, convertQaToCommaStr, isDigits } from '../../util/utils';
import { AccountType, OperationStatus, ProxyCalls, TransactionType } from '../../util/enum';

import ModalPending from '../contract-calls-modal/modal-pending';
import ModalSent from '../contract-calls-modal/modal-sent';
import { useAppSelector } from '../../store/hooks';
import { StakeModalData } from '../../util/interface';
import { ZilSigner } from '../../zilliqa-signer';
import GasSettings from './gas-settings';

const BigNumber = require('bignumber.js');
const { BN, units } = require('@zilliqa-js/util');


function DelegateStakeModal(props: any) {
    const proxy = useAppSelector(state => state.blockchain.proxy);
    const networkURL = useAppSelector(state => state.blockchain.blockchain);
    const ledgerIndex = useAppSelector(state => state.user.ledger_index);
    const accountType = useAppSelector(state => state.user.account_type);
    const minDelegStake = useAppSelector(state => state.staking.min_deleg_stake);
    const balance = useAppSelector(state => state.user.balance); // Qa
    const minDelegStakeDisplay = units.fromQa(new BN(minDelegStake), units.Units.Zil); // for display
    
    const defaultGasPrice = ZilSigner.getDefaultGasPrice();
    const defaultGasLimit = ZilSigner.getDefaultGasLimit();
    const [gasPrice, setGasPrice] = useState<string>(defaultGasPrice);
    const [gasLimit, setGasLimit] = useState<string>(defaultGasLimit);
    const [gasOption, setGasOption] = useState(false);

    const { updateData, updateRecentTransactions } = props;
    const stakeModalData: StakeModalData = useAppSelector(state => state.user.stake_modal_data);

    const ssnAddress = stakeModalData.ssnAddress; // bech32

    const [delegAmt, setDelegAmt] = useState('0'); // in ZIL
    const [txnId, setTxnId] = useState('');
    const [isPending, setIsPending] = useState('');


    const delegateStake = async () => {
        if (!ssnAddress) {
            Alert('error', "Invalid Node", "Node address should be bech32 or checksum format.");
            return null;
        }

        let delegAmtQa = '';

        if (!delegAmt) {
            Alert('error', "Invalid Stake Amount", "Stake amount shoud not be empty.");
            return null;
        } else {
            try {
                delegAmtQa = convertZilToQa(delegAmt);
                const isLessThanMinDeleg = new BigNumber(delegAmtQa).isLessThan(minDelegStake);
                

                if (isLessThanMinDeleg) {
                    Alert('error', "Invalid Stake Amount", "Minimum stake amount is " + minDelegStakeDisplay + " ZIL.");
                    return null;
                }

                const gasFeesQa = new BigNumber(gasPrice).multipliedBy(gasLimit);
                const combinedFees = new BigNumber(delegAmtQa).plus(gasFeesQa);
                const combinedFeesZil = units.fromQa(new BN(combinedFees.toString()), units.Units.Zil);
                const remaningBalance = new BigNumber(balance).minus(delegAmtQa);
                const isBalanceSufficient = remaningBalance.isGreaterThan(gasFeesQa);

                if (!isBalanceSufficient) {
                    Alert('error', 
                        "Insufficient Balance", 
                        "Your wallet balance is insufficient to pay for the staked amount and gas fees combined. Total amount required is " + combinedFeesZil + " ZIL.");
                    Alert('error', "Gas Fee Estimation", "Current gas fee is around " + units.fromQa(new BN(gasFeesQa), units.Units.Zil) + " ZIL.");
                    return null;
                }

            } catch (err) {
                // user input is malformed
                // cannot convert input zil amount to qa
                Alert('error', "Invalid Stake Amount", "Minimum stake amount is " + minDelegStakeDisplay + " ZIL.");
                return null;
            }
        }

        // create tx params

        const proxyChecksum = bech32ToChecksum(proxy);
        const ssnChecksumAddress = bech32ToChecksum(ssnAddress).toLowerCase();

        // gas price, gas limit declared in account.ts
        let txParams = {
            toAddr: proxyChecksum,
            amount: new BN(`${delegAmtQa}`),
            code: "",
            data: JSON.stringify({
                _tag: ProxyCalls.DELEGATE_STAKE,
                params: [
                    {
                        vname: 'ssnaddr',
                        type: 'ByStr20',
                        value: `${ssnChecksumAddress}`,
                    }
                ]
            }),
            gasPrice: gasPrice,
            gasLimit: gasLimit,
        };

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

    const setDefaultStakeAmt = useCallback(() => {
        if (minDelegStake) {
            setDelegAmt(units.fromQa(new BN(minDelegStake), units.Units.Zil).toString());
        } else {
            setDelegAmt('0');
        }
    }, [minDelegStake]);

    const handleClose = () => {
        // txn success
        // invoke dashboard functions to update recent transactions and poll data
        if (txnId) {
            updateRecentTransactions(TransactionType.DELEGATE_STAKE, txnId);
            updateData();
        }
        
        // reset state
        // timeout to wait for modal to fade out before clearing
        // so that the animation is smoother
        toast.dismiss();
        setTimeout(() => {
            setDefaultStakeAmt();
            setTxnId('');
            setGasOption(false);
            setGasPrice(defaultGasPrice);
            setGasLimit(defaultGasLimit);
        }, 150);
    }

    const handleDelegAmt = (e: any) => {
        setDelegAmt(e.target.value);
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
        setDefaultStakeAmt();
    }, [setDefaultStakeAmt]);

    return (
        <div id="delegate-stake-modal" className="modal fade" tabIndex={-1} role="dialog" aria-labelledby="delegateStakeModalLabel" aria-hidden="true">
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
                            <h5 className="modal-title" id="delegateStakeModalLabel">Stake</h5>
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
                                    <h3>Commission Rate</h3>
                                    <span>{convertToProperCommRate(stakeModalData.commRate).toFixed(2)}%</span>
                                </div>
                            </div>

                            <div className="modal-label mb-2"><strong>Enter stake amount</strong></div>
                            <div className="input-group mb-2">
                                <input type="text" className="form-control shadow-none" value={delegAmt} onChange={handleDelegAmt} />
                                <div className="input-group-append">
                                    <span className="input-group-text pl-4 pr-3">ZIL</span>
                                </div>
                            </div>
                            <p><small><strong>Available</strong>: <strong>{convertQaToCommaStr(balance.toString())}</strong> ZIL</small></p>

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
                                    <li><small>Minimum staking amount is <strong>{minDelegStakeDisplay}</strong> ZIL.</small></li>
                                    <li><small>Please ensure you have at least <strong>100 ZIL</strong> after staking to pay for gas fees for future transactions such as withdrawal.</small></li>
                                </ul>
                            </div>
                            <div className="d-flex">
                                <button type="button" className="btn btn-user-action mx-auto shadow-none" onClick={delegateStake}>Stake</button>
                            </div>
                        </div>
                         </>
                     }
                 </div>
            </div>
        </div>
    );
}

export default DelegateStakeModal;