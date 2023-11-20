import React, { useState } from 'react';
import { toast } from 'react-toastify';
import { trackPromise } from 'react-promise-tracker';

import { bech32ToChecksum, computeGasFees, isDigits, showWalletsPrompt, validateBalance } from '../../util/utils';
import { AccountType, OperationStatus, ProxyCalls, TransactionType } from "../../util/enum";
import Alert from '../alert';

import ModalPending from '../contract-calls-modal/modal-pending';
import ModalSent from '../contract-calls-modal/modal-sent';
import { useAppSelector } from '../../store/hooks';
import { ZilSigner } from '../../zilliqa-signer';
import { units } from '@zilliqa-js/zilliqa';
import BigNumber from 'bignumber.js';
import GasSettings from './gas-settings';

const { BN } = require('@zilliqa-js/util');


function UpdateReceiverAddress(props: any) {
    const proxy = useAppSelector(state => state.blockchain.proxy);
    const networkURL = useAppSelector(state => state.blockchain.blockchain);
    const wallet = useAppSelector(state => state.user.address_base16);
    const ledgerIndex = useAppSelector(state => state.user.ledger_index);
    const accountType = useAppSelector(state => state.user.account_type);
    const receiver = useAppSelector(state => state.user.operator_stats.receiver);

    const defaultGasPrice = ZilSigner.getDefaultGasPrice();
    const defaultGasLimit = ZilSigner.getDefaultGasLimit();
    const [gasPrice, setGasPrice] = useState<string>(defaultGasPrice);
    const [gasLimit, setGasLimit] = useState<string>(defaultGasLimit);
    const [gasOption, setGasOption] = useState(false);

    const { updateData, updateRecentTransactions } = props;
    const [newAddress, setNewAddress] = useState('');
    const [txnId, setTxnId] = useState('');
    const [isPending, setIsPending] = useState('');

    const updateAddress = async () => {
        if (!newAddress) {
            Alert('error', "Invalid Address", "Receiving address should be bech32 or checksum format.");
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
        const newReceivingAddress = bech32ToChecksum(newAddress).toLowerCase();

        // gas price, gas limit declared in account.ts
        let txParams = {
            toAddr: proxyChecksum,
            amount: new BN(0),
            code: "",
            data: JSON.stringify({
                _tag: ProxyCalls.UPDATE_RECV_ADDR,
                params: [
                    {
                        vname: 'new_addr',
                        type: 'ByStr20',
                        value: `${newReceivingAddress}`,
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
            updateRecentTransactions(TransactionType.UPDATE_RECV_ADDR, txnId);
            updateData();
        }

        // reset state
        // timeout to wait for modal to fade out before clearing
        // so that the animation is smoother
        toast.dismiss();
        setTimeout(() => {
            setNewAddress('');
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
        <div id="update-recv-addr-modal" className="modal fade" tabIndex={-1} role="dialog" aria-labelledby="updateRecvAddrModalLabel" aria-hidden="true">
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
                            <h5 className="modal-title" id="updateRecvAddrModalLabel">Update Receiving Address</h5>
                            <button type="button" className="close btn shadow-none" data-dismiss="modal" aria-label="Close" onClick={handleClose}>
                                <span aria-hidden="true">&times;</span>
                            </button>
                        </div>
                        <div className="modal-body">
                            <div className="row node-details-wrapper mb-4">
                                <div className="col node-details-panel">
                                    <h3>Current Receiving Address</h3>
                                    <span>{receiver ? receiver : 'none'}</span>
                                </div>
                            </div>
                            <input type="text" className="mb-4" value={newAddress} onChange={(e:any) => setNewAddress(e.target.value)} placeholder="Enter new address in bech32 format" />
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
                            <button type="button" className="btn btn-user-action mx-auto shadow-none" onClick={updateAddress}>Update</button>
                            </div>
                        </div>
                        </>
                    }
                </div>
            </div>
        </div>
    );
}

export default UpdateReceiverAddress;