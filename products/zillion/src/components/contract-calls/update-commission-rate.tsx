import React, { useState } from 'react';
import { toast } from 'react-toastify';
import { trackPromise } from 'react-promise-tracker';

import { AccountType, OperationStatus, ProxyCalls, TransactionType } from "../../util/enum";
import { bech32ToChecksum, computeGasFees, convertToProperCommRate, isDigits, percentToContractCommRate, showWalletsPrompt, validateBalance } from '../../util/utils';
import Alert from '../alert';

import ModalPending from '../contract-calls-modal/modal-pending';
import ModalSent from '../contract-calls-modal/modal-sent';
import { useAppSelector } from '../../store/hooks';
import { ZilSigner } from '../../zilliqa-signer';
import { units } from '@zilliqa-js/zilliqa';
import BigNumber from 'bignumber.js';
import GasSettings from './gas-settings';
import { logger } from '../../util/logger';

const { BN } = require('@zilliqa-js/util');

function UpdateCommRateModal(props: any) {
    const proxy = useAppSelector(state => state.blockchain.proxy);
    const networkURL = useAppSelector(state => state.blockchain.blockchain);
    const wallet = useAppSelector(state => state.user.address_base16);
    const ledgerIndex = useAppSelector(state => state.user.ledger_index);
    const accountType = useAppSelector(state => state.user.account_type);
    const commRate = useAppSelector(state => state.user.operator_stats.commRate);

    const defaultGasPrice = ZilSigner.getDefaultGasPrice();
    const defaultGasLimit = ZilSigner.getDefaultGasLimit();
    const [gasPrice, setGasPrice] = useState<string>(defaultGasPrice);
    const [gasLimit, setGasLimit] = useState<string>(defaultGasLimit);
    const [gasOption, setGasOption] = useState(false);

    const { updateData, updateRecentTransactions } = props;
    const [newRate, setNewRate] = useState('');
    const [txnId, setTxnId] = useState('')
    const [isPending, setIsPending] = useState('');

    const updateCommRate = async () => {
        if (!newRate || !newRate.match(/\d/)) {
            Alert('error', "Invalid Commission Rate", "Please enter an appropriate value, e.g. 10.5");
            return null;
        }

        if (newRate.length > 9) {
            Alert('error', "Invalid Commission Rate", "Commission rate should have a maximum of 7 decimals only.");
            return null;
        }

        if (await validateBalance(wallet) === false) {
            const gasFees = computeGasFees(gasPrice, gasLimit);
            Alert('error', 
            "Insufficient Balance", 
            "Insufficient balance in wallet to pay for the gas fee.");
            Alert('error', "Gas Fee Estimation", "Current gas fee is around " + units.fromQa(gasFees, units.Units.Zil) + " ZIL.");
            return null;
        }

        // create tx params

        // toAddr: proxy address
        const proxyChecksum = bech32ToChecksum(proxy);
        const contractCommRate = percentToContractCommRate(newRate);

        logger("new rate: %o", contractCommRate);

        // gas price, gas limit declared in account.ts
        let txParams = {
            toAddr: proxyChecksum,
            amount: new BN(0),
            code: "",
            data: JSON.stringify({
                _tag: ProxyCalls.UPDATE_COMM,
                params: [
                    {
                        vname: 'new_rate',
                        type: 'Uint128',
                        value: `${contractCommRate}`,
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

    const handleClose = () => {
        // txn success
        // invoke dashboard methods
        if (txnId) {
            updateRecentTransactions(TransactionType.UPDATE_COMM_RATE, txnId);
            updateData();
        }

        // reset state
        // timeout to wait for modal to fade out before clearing
        // so that the animation is smoother
        toast.dismiss();
        setTimeout(() => {
            setNewRate('');
            setTxnId('');
            setIsPending('');
            setGasOption(false);
            setGasPrice(defaultGasPrice);
            setGasLimit(defaultGasLimit);
        }, 150);
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

    return (
        <div id="update-comm-rate-modal" className="modal fade" tabIndex={-1} role="dialog" aria-labelledby="updateCommRateModalLabel" aria-hidden="true">
            <div className="contract-calls-modal modal-dialog modal-dialog-centered" role="document">
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
                            <h5 className="modal-title" id="updateCommRateModalLabel">Update Commission Rate</h5>
                            <button type="button" className="close btn shadow-none" data-dismiss="modal" aria-label="Close" onClick={handleClose}>
                                <span aria-hidden="true">&times;</span>
                            </button>
                        </div>
                        <div className="modal-body">
                            <div className="row node-details-wrapper mb-4">
                                <div className="col node-details-panel">
                                    <h3>Current Commission Rate</h3>
                                    <span>{commRate ? convertToProperCommRate(commRate).toFixed(2) : '0.00'}&#37;</span>
                                </div>
                            </div>
                            <div className="input-group mb-4">
                                <input type="text" className="form-control shadow-none" value={newRate} onChange={(e:any) => setNewRate(e.target.value)} placeholder="Enter new rate" maxLength={9} />
                                <div className="input-group-append">
                                    <span className="input-group-text pl-4 pr-3">%</span>
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
                            <div className="d-flex mt-2">
                                <button type="button" className="btn btn-user-action mx-auto shadow-none" onClick={updateCommRate}>Update</button>
                            </div>
                        </div>
                        </>
                    }
                </div>
            </div>
        </div>
    );
}

export default UpdateCommRateModal;