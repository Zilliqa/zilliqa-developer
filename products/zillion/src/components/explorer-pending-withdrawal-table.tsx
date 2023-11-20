import React, {useMemo } from 'react';
import { useTable, useSortBy } from 'react-table';

import Spinner from './spinner';
import { PromiseArea } from '../util/enum';
import { convertQaToCommaStr } from '../util/utils';
import IconQuestionCircle from './icons/question-circle';
import ReactTooltip from 'react-tooltip';


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

function ExplorerPendingWithdrawalTable(props: any) {
    const data = props.data;
    const totalWithdrawAmt = props.totalWithdrawAmt;

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
        <div id="delegator-complete-withdraw-details" className="container px-1 py-3 mb-4">
            <div id="complete-withdraw-explorer-accordion">
                <div className="card text-center">
                    <h6 className="text-left px-4 pt-4">Pending Stake Withdrawals&nbsp;
                        <span data-tip data-for="withdraw-question">
                            <IconQuestionCircle width="16" height="16" className="section-icon" />
                        </span>
                    </h6>
                    <div className="text-center">
                        <Spinner class="spinner-border dashboard-spinner mb-4" area={PromiseArea.PROMISE_GET_EXPLORER_PENDING_WITHDRAWAL} />
                    </div>
                    <div className="card-header d-flex justify-content-between" id="complete-withdraw-accordion-header">
                        <div>
                            <span>You can now withdraw <strong>{convertQaToCommaStr(totalWithdrawAmt)}</strong> ZIL</span>
                        </div>
                    </div>

                    <div id="complete-withdraw-details" className={ data.length > 0 ? 'collapse show' : 'collapse' } aria-labelledby="complete-withdraw-accordion-header" data-parent="#complete-withdraw-explorer-accordion">
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
    );
}

export default ExplorerPendingWithdrawalTable;