import React, {useMemo } from 'react';
import { useTable, useSortBy } from 'react-table';
import ReactTooltip from 'react-tooltip';

import IconQuestionCircle from './icons/question-circle';
import Spinner from './spinner';
import { PromiseArea, ButtonText, ContractState } from '../util/enum';
import { convertQaToCommaStr } from '../util/utils';
import { PendingWithdrawStats } from '../util/interface';
import { useAppSelector } from '../store/hooks';


function Table({ columns, data, tableId }: any) {
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
                sortBy: [
                    {
                        id: "blkNumCountdown",
                        desc: false
                    }
                ]
            }
        }, useSortBy);

    return (
        <table id={tableId} className="table table-responsive-lg" {...getTableProps()}>
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
                        <tr {...row.getRowProps()}>
                            {row.cells.map(cell => {
                                return <td {...cell.getCellProps()}>{cell.render('Cell')}</td>
                            })}
                        </tr>
                    )
                })}
            </tbody>
        </table>
    );
}

function CompleteWithdrawalTable(props: any) {
    const data: PendingWithdrawStats[] = useAppSelector(state => state.user.pending_withdraw_list);
    const completeWithdrawAmt = useAppSelector(state => state.user.complete_withdrawal_amt);

    const columns = useMemo(
        () => [
            {
                Header: 'pending blocks till claim',
                accessor: 'blkNumCountdown'
            },
            {
                Header: 'progress',
                accessor: 'progress',
                Cell: ({ row }: any) => <span>{row.original.progress}%</span>
            },
            {
                Header: 'amount (ZIL)',
                accessor: 'amount',
                Cell: ({ row }: any) => <span>{convertQaToCommaStr(row.original.amount)}</span>
            }
        ], []
    );

    return (
        <>
        { data.length !== 0 &&

        <div id="delegator-complete-withdraw-details" className="col-12 mt-2 px-1 py-3">
            <div id="complete-withdraw-accordion">
                <div className="card text-center">
                    <h6 className="inner-section-heading px-4 pt-4">Pending Stake Withdrawals&nbsp;
                        <span data-tip data-for="withdraw-question">
                            <IconQuestionCircle width="16" height="16" className="section-icon" />
                        </span>
                    </h6>
                    <div className="text-center">
                        <Spinner class="spinner-border dashboard-spinner mb-4" area={PromiseArea.PROMISE_GET_DELEG_STATS} />
                    </div>
                    <div className="card-header d-flex justify-content-between" id="complete-withdraw-accordion-header">
                        <div>
                            <span><em>You can now withdraw <strong>{convertQaToCommaStr(completeWithdrawAmt)}</strong> ZIL</em></span>
                        </div>
                        <div className="btn-group">
                            { 
                                data.length !== 0 && 
                                <button 
                                    className="btn btn-inner-contract mr-4 shadow-none" 
                                    data-toggle="modal" 
                                    data-target="#complete-withdrawal-modal" 
                                    data-keyboard="false" 
                                    data-backdrop="static" 
                                    disabled={ContractState.IS_PAUSED.toString() === 'true' ? true : false}>
                                        {ContractState.IS_PAUSED.toString() === 'true' ? ButtonText.NOT_AVAILABLE : 'Complete Stake Withdrawals'}
                                </button> 
                            }
                            { 
                                data.length !== 0 && 
                                <button 
                                    className="btn btn-inner-contract-2 mr-4 shadow-none" 
                                    data-toggle="collapse" 
                                    data-target="#complete-withdraw-details" 
                                    aria-expanded="true" 
                                    aria-controls="complete-withdraw-details">
                                        View Details
                                </button> 
                            }
                        </div>
                    </div>

                    <div id="complete-withdraw-details" className="collapse" aria-labelledby="complete-withdraw-accordion-header" data-parent="#complete-withdraw-accordion">
                        <div className="card-body">
                            { data.length === 0 && <em>You have no ZILs ready for withdrawal yet.</em> }
                            <Table columns={columns} data={data} />
                        </div>
                    </div>

                </div>
            </div>
            <ReactTooltip id="withdraw-question" place="bottom" type="dark" effect="solid">
                <span>When you initiate a stake withdrawal, the amount is not withdrawn immediately.</span>
                <br/>
                <span>The amount is processed at a certain block number and only available to withdraw<br/>once the required block number is reached.</span>
            </ReactTooltip>
        </div>
        }

        </>
    )

}

export default CompleteWithdrawalTable;