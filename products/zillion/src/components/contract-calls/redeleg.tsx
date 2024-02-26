import React, { useState, useMemo, useEffect, useCallback } from 'react';
import { useTable, useSortBy } from 'react-table';
import ReactTooltip from 'react-tooltip';
import { toast } from 'react-toastify';
import { trackPromise } from 'react-promise-tracker';

import ModalPending from '../contract-calls-modal/modal-pending';
import ModalSent from '../contract-calls-modal/modal-sent';
import Alert from '../alert';
import { bech32ToChecksum, convertZilToQa, convertQaToCommaStr, convertToProperCommRate, getTruncatedAddress, showWalletsPrompt, convertQaToZilFull, isDigits, computeGasFees, isRespOk, validateBalance } from '../../util/utils';
import { ProxyCalls, OperationStatus, TransactionType, AccountType } from '../../util/enum';
import { computeDelegRewards } from '../../util/reward-calculator';

import { useAppSelector } from '../../store/hooks';
import { StakeModalData } from '../../util/interface';
import { ZilSdk } from '../../zilliqa-api';
import { ZilSigner } from '../../zilliqa-signer';
import GasSettings from './gas-settings';
import BigNumber from 'bignumber.js';
import { logger } from '../../util/logger';


const { BN, units } = require('@zilliqa-js/util');


// hide the data that contains the sender address
// no point to transfer to same person
function Table({ columns, data, tableId, senderAddress, handleNodeSelect, hiddenColumns }: any) {
    const {
        getTableProps,
        getTableBodyProps,
        headerGroups,
        rows,
        prepareRow,
    } = useTable(
        {
            columns, 
            data,
            initialState : {
                hiddenColumns : hiddenColumns,
                sortBy: [
                    {
                        id: "name",
                        desc: false
                    },
                ]
            }
        }, useSortBy);

    return (
        <table id={tableId} className="table table-responsive-md" {...getTableProps()}>
            <thead>
                {headerGroups.map(headerGroup => (
                    <tr {...headerGroup.getHeaderGroupProps()}>
                        {headerGroup.headers.map(column => (
                            <th scope="col" {...column.getHeaderProps()}>{column.render('Header')}</th>
                        ))}
                    </tr>
                ))}
            </thead>
            <tbody {...getTableBodyProps()}>
                {rows.map((row, i) => {
                    prepareRow(row)
                    return (
                        <tr {...row.getRowProps()} onClick={() => handleNodeSelect(row.original)}>
                            {
                                (row.original as any).address !== senderAddress &&
                                
                                row.cells.map(cell => {
                                    return <td {...cell.getCellProps()}>{cell.render('Cell')}</td>
                                })
                            }
                        </tr>
                    )
                })}
            </tbody>
        </table>
    );
}


function ReDelegateStakeModal(props: any) {
    const { updateData, updateRecentTransactions } = props;
    const proxy = useAppSelector(state => state.blockchain.proxy);
    const impl = useAppSelector(state => state.blockchain.impl);
    const networkURL = useAppSelector(state => state.blockchain.blockchain);
    const userBase16Address = useAppSelector(state => state.user.address_base16);
    const ledgerIndex = useAppSelector(state => state.user.ledger_index);
    const accountType = useAppSelector(state => state.user.account_type);
    const minDelegStake = useAppSelector(state => state.staking.min_deleg_stake);
    const nodeSelectorOptions = useAppSelector(state => state.staking.ssn_dropdown_list);
    const stakeModalData: StakeModalData = useAppSelector(state => state.user.stake_modal_data);

    const minDelegStakeDisplay = units.fromQa(new BN(minDelegStake), units.Units.Zil);
    const fromSsn = stakeModalData.ssnAddress; // bech32
    const [toSsn, setToSsn] = useState('');
    const [toSsnName, setToSsnName] = useState('');
    const [delegAmt, setDelegAmt] = useState('0'); // in ZIL

    const [txnId, setTxnId] = useState('');
    const [isPending, setIsPending] = useState('');
    const [showNodeSelector, setShowNodeSelector] = useState(false);

    const defaultGasPrice = ZilSigner.getDefaultGasPrice();
    const defaultGasLimit = ZilSigner.getDefaultGasLimit();
    const [gasPrice, setGasPrice] = useState<string>(defaultGasPrice);
    const [gasLimit, setGasLimit] = useState<string>(defaultGasLimit);
    const [gasOption, setGasOption] = useState(false);

    
    // checks if there are any unwithdrawn rewards or buffered deposit
    const hasRewardToWithdraw = async () => {
        const ssnChecksumAddress = bech32ToChecksum(fromSsn).toLowerCase();
        
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
            Alert('info', "Unwithdrawn Rewards Found", "Please withdraw the rewards before transferring.");
            return true;
        }

        // secondary buffered deposits check
        // different map
        // check if user has buffered deposits
        if (last_buf_deposit_cycle_deleg_json.last_buf_deposit_cycle_deleg[userBase16Address].hasOwnProperty(ssnChecksumAddress)) {
                const lastDepositCycleDeleg = parseInt(last_buf_deposit_cycle_deleg_json.last_buf_deposit_cycle_deleg[userBase16Address][ssnChecksumAddress]);
                const lastRewardCycle = parseInt(last_reward_cycle_json.lastrewardcycle);
                if (lastRewardCycle <= lastDepositCycleDeleg) {
                    Alert('info', "Buffered Deposits Found", "Please wait for the next cycle before transferring.");
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
        //             Alert('info', "Buffered Deposits Found", "Please wait for 2 more cycles for your rewards to be issued before transferring.");
        //             return true;
        //         }
        // }

        return false;
    }

    const redeleg = async () => {
        let delegAmtQa;

        if (!fromSsn || !toSsn) {
            Alert('error', "Invalid Node", "Please select a node.");
            return null;
        }

        if (!delegAmt) {
            Alert('error', "Invalid Transfer Amount", "Transfer amount cannot be empty.");
            return null;
        } else {
            try {
                delegAmtQa = convertZilToQa(delegAmt);
            } catch (err) {
                // user input is malformed
                // cannot convert input zil amount to qa
                Alert('error', "Invalid Transfer Amount", "Please check your transfer amount again.");
                return null;
            }
        }

        if (await validateBalance(userBase16Address) === false) {
            const gasFees = computeGasFees(gasPrice, gasLimit);
            Alert('error', 
            "Insufficient Balance", 
            "Insufficient balance in wallet to pay for the gas fee.");
            Alert('error', "Gas Fee Estimation", "Current gas fee is around " + units.fromQa(gasFees, units.Units.Zil) + " ZIL.");
            return null;
        }

        setIsPending(OperationStatus.PENDING);

        // check if deleg has unwithdrawn rewards or buffered deposits for the from ssn address
        const hasRewards = await hasRewardToWithdraw();
        if (hasRewards) {
            setIsPending('');
            return null;
        }

        // create tx params

        // toAddr: proxy address

        const proxyChecksum = bech32ToChecksum(proxy);
        const fromSsnChecksumAddress = bech32ToChecksum(fromSsn).toLowerCase();
        const toSsnChecksumAddress = bech32ToChecksum(toSsn).toLowerCase();
        const currentAmtQa = stakeModalData.delegAmt;
        const leftOverQa = new BN(currentAmtQa).sub(new BN(delegAmtQa));

        // check if redeleg more than current deleg amount
        if (new BN(delegAmtQa).gt(new BN(currentAmtQa))) {
            Alert('info', "Invalid Transfer Amount", "You only have " + convertQaToCommaStr(currentAmtQa) + " ZIL to transfer." );
            setIsPending('');
            return null;
        } else if (!leftOverQa.isZero() && leftOverQa.lt(new BN(minDelegStake))) {
            // check leftover amount
            // if less than min stake amount
            Alert('info', "Invalid Transfer Amount", "Please leave at least " +  minDelegStakeDisplay + " ZIL (min. stake amount) or transfer ALL.");
            setIsPending('');
            return null;
        }

        // gas price, gas limit declared in account.ts
        let txParams = {
            toAddr: proxyChecksum,
            amount: new BN(0),
            code: "",
            data: JSON.stringify({
                _tag: ProxyCalls.REDELEGATE_STAKE,
                params: [
                    {
                        vname: 'ssnaddr',
                        type: 'ByStr20',
                        value: `${fromSsnChecksumAddress}`,
                    },
                    {
                        vname: 'to_ssn',
                        type: 'ByStr20',
                        value: `${toSsnChecksumAddress}`,
                    },
                    {
                        vname: 'amount',
                        type: 'Uint128',
                        value: `${delegAmtQa}`,
                    }
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

    // set default transfer amt to current stake amt
    const setDefaultDelegAmt = useCallback(() => {
        if (stakeModalData.delegAmt) {
            const tempDelegAmt = convertQaToZilFull(stakeModalData.delegAmt);
            setDelegAmt(tempDelegAmt);
        } else {
            setDelegAmt('0');
        }
    }, [stakeModalData.delegAmt]);

    const handleClose = () => {
        // txn success
        // invoke dashbaord methods
        if (txnId) {
            updateRecentTransactions(TransactionType.TRANSFER_STAKE, txnId);
            updateData();
        }
        
        // reset state
        // timeout to wait for modal to fade out before clearing
        // so that the animation is smoother
        toast.dismiss();
        setTimeout(() => {
            setToSsn('');
            setToSsnName('');
            setTxnId('');
            setDefaultDelegAmt();
            setShowNodeSelector(false);
            setGasOption(false);
            setGasPrice(defaultGasPrice);
            setGasLimit(defaultGasLimit);
        }, 150);
    }

    // row contains a json from react-table, similar to the react-table header declaration
    const handleNodeSelect = (row: any) => {
        setToSsn(row.address);
        setToSsnName(row.name);
        // reset the view
        toggleNodeSelector();
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

    const toggleNodeSelector = () => {
        logger("toggle node selector: %o", showNodeSelector);
        setShowNodeSelector(!showNodeSelector);
    }

    const columns = useMemo(
        () => [
            {
                Header: 'name',
                accessor: 'name'
            },
            {
                Header: 'address',
                accessor: 'address',
                Cell: ({ row }: any) => 
                    <>
                    <span data-tip={row.original.address}>
                        {getTruncatedAddress(row.original.address)}
                    </span>
                    <ReactTooltip place="bottom" type="dark" effect="float" />
                    </>
            },
            {
                Header: 'Delegators',
                accessor: 'delegNum',
            },
            {
                Header: 'Stake Amount (ZIL)',
                accessor: 'stakeAmt',
                Cell: ({ row }: any) => 
                    <>
                    <span>{convertQaToCommaStr(row.original.stakeAmt)}</span>
                    </>
            },
            {
                Header: 'Comm. Rate (%)',
                accessor: 'commRate',
                Cell: ({ row }: any) =>
                    <span>{convertToProperCommRate(row.original.commRate).toFixed(2)}</span>
            }
            // eslint-disable-next-line
        ], []
    )

    const getHiddenColumns = () => {
        let hiddenColumns = ["address"];
        return hiddenColumns;
    }

    useEffect(() => {
        setDefaultDelegAmt();
    }, [setDefaultDelegAmt]);

    return (
        <div id="redeleg-stake-modal" className="modal fade" tabIndex={-1} role="dialog" aria-labelledby="redelegModalLabel" aria-hidden="true">
            <div className="contract-calls-modal modal-dialog modal-dialog-centered modal-lg" role="document">
                <div className="modal-content">
                    {
                        isPending ?

                        <ModalPending />

                        :

                        txnId ?

                        <ModalSent txnId={txnId} networkURL={networkURL} handleClose={handleClose}/>

                        :

                        <>
                        <div className="modal-header">
                            <h5 className="modal-title" id="withdrawRewardModalLabel">Transfer Stake</h5>
                            <button type="button" className="close btn shadow-none" data-dismiss="modal" aria-label="Close" onClick={handleClose}>
                                <span aria-hidden="true">&times;</span>
                            </button>
                        </div>
                        <div className="modal-body">
                            {
                                !showNodeSelector &&
                                <>
                                    {/* sender */}
                                    <small><strong>From</strong></small>
                                    <div className="row node-details-wrapper mb-4">
                                        <div className="col node-details-panel mr-4">
                                            <h3>{stakeModalData.ssnName}</h3>
                                            <span>{stakeModalData.ssnAddress}</span>
                                        </div>
                                        <div className="col node-details-panel">
                                            <h3>Current Deposit</h3>
                                            <span>{convertQaToCommaStr(stakeModalData.delegAmt)} ZIL</span>
                                        </div>
                                    </div>

                                    {/* recipient*/}
                                    <small><strong>To</strong></small>
                                    
                                    {!toSsn &&
                                        <button type="button" className="mb-4 btn btn-contract btn-block shadow-none" onClick={() => toggleNodeSelector()}>Select a node</button>
                                    }

                                    { toSsn &&
                                        <>
                                        <div className="row node-details-wrapper mb-4">
                                            <div className="col node-details-panel">
                                                <h3>{toSsnName}</h3>
                                                <span>{toSsn}</span>
                                                <button type="button" className="btn btn-change-node shadow-none" onClick={() => toggleNodeSelector()}>Change</button>
                                            </div>
                                        </div>

                                        <div className="modal-label mb-2"><strong>Enter transfer amount</strong></div>
                                        <div className="input-group mb-4">
                                            <input type="text" className="form-control shadow-none" value={delegAmt} onChange={handleDelegAmt} />
                                            <div className="input-group-append">
                                                <span className="input-group-text pl-4 pr-3">ZIL</span>
                                            </div>
                                        </div>
                                        </>
                                    }

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
                                        <button type="button" className="btn btn-user-action mt-2 mx-auto shadow-none" onClick={redeleg}>Transfer Stake</button>
                                    </div>
                                </>
                            }

                            {
                                showNodeSelector &&

                                <>
                                    <h2 className="node-details-subheading mb-2">Select a node to transfer to</h2>
                                    <div id="transfer-stake-details" className="text-center">
                                        <Table 
                                            columns={columns} 
                                            data={nodeSelectorOptions} 
                                            senderAddress={stakeModalData.ssnAddress} 
                                            handleNodeSelect={handleNodeSelect}
                                            hiddenColumns={getHiddenColumns()} />
                                    </div>
                                    <div className="d-flex">
                                        <button type="button" className="btn btn-user-action-cancel mt-4 mx-auto shadow-none" onClick={() => toggleNodeSelector()}>Back</button>
                                    </div>
                                </>
                            }

                        </div>
                        </>
                    }
                </div>
            </div>
        </div>
    );
}

export default ReDelegateStakeModal;