import React, { useMemo } from 'react';
import { useTable, useSortBy } from 'react-table';
import ReactTooltip from 'react-tooltip';

import { OperationStatus } from '../util/enum';
import { convertQaToCommaStr, convertQaToZilFull } from '../util/utils';

import { DelegStakingPortfolioStats } from '../util/interface';
import DelegatorDropdown from './delegator-dropdown';
import { useAppSelector } from '../store/hooks';
import SpinnerNormal from './spinner-normal';


function Table({ columns, data }: any) {
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
                pageIndex: 0,
                sortBy: [
                    {
                        id: 'delegAmt',
                        desc: true
                    }
                ]
            }
        }, useSortBy);
    
    return (
        <table className="table table-responsive-sm" {...getTableProps()}>
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

function StakingPortfolio(props: any) {
    // const networkURL = props.network;
    // const data: DelegStakingPortfolioStats[] = props.data;
    const data: DelegStakingPortfolioStats[] = useAppSelector(state => state.user.deleg_staking_portfolio_list);
    const loading: OperationStatus = useAppSelector(state => state.user.is_deleg_stats_loading);

    const columns = useMemo(
        () => [
            {
                Header: 'name',
                accessor: 'ssnName',
            },
            {
                Header: 'deposit (ZIL)',
                accessor: 'delegAmt',
                Cell: ({ row }: any) => <span>{convertQaToCommaStr(row.original.delegAmt)}</span>
            },
            {
                Header: 'rewards (ZIL)',
                accessor: 'rewards',
                Cell: ({ row }: any) => 
                    <>
                    <span data-for="rewards-tip" data-tip={convertQaToZilFull(row.original.rewards)}>{convertQaToCommaStr(row.original.rewards)}</span>
                    <ReactTooltip id="rewards-tip" place="bottom" type="dark" effect="solid" />
                    </>
            },
            {
                Header: 'actions',
                accessor: 'actions',
                Cell: ({ row }: any) =>
                    <>
                    <DelegatorDropdown
                        ssnName={row.original.ssnName}
                        ssnAddress={row.original.ssnAddress}
                        delegAmt={row.original.delegAmt}
                        rewards={row.original.rewards} />
                    </>
            }
        ], []
    );

    return (
        <>
        {
            loading === OperationStatus.PENDING &&
            <SpinnerNormal class="spinner-border dashboard-spinner mb-4" />
        }
        {
            loading === OperationStatus.COMPLETE &&
            <>
             { (data.length === 0) && <div className="d-block px-4 pb-3 text-left"><em>You have not deposited in any nodes yet.</em></div> }
             { (data.length > 0) && <Table columns={columns} data={data} /> }
            </>
        }
        </>
    );
}

export default StakingPortfolio;