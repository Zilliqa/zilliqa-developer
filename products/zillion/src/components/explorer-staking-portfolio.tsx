import React, { useMemo } from 'react';
import { useTable, useSortBy } from 'react-table';
import { DelegStakingPortfolioStats } from '../util/interface';
import { getAddressLink, convertQaToCommaStr, convertQaToZilFull } from '../util/utils';
import ReactTooltip from 'react-tooltip';
import Spinner from './spinner';
import { PromiseArea } from '../util/enum';
import { useAppSelector } from '../store/hooks';


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
        <table className="table table-responsive-lg" {...getTableProps()}>
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

function ExplorerStakingPortfolio(props: any) {
    const data: DelegStakingPortfolioStats[] = props.data;
    const networkURL = useAppSelector(state => state.blockchain.blockchain);

    const columns = useMemo(
        () => [
            {
                Header: 'name',
                accessor: 'ssnName',
            },
            {
                Header: 'address',
                accessor: 'ssnAddress',
                Cell: ({ row }: any) => <a href={getAddressLink(row.original.ssnAddress, networkURL)} target="_blank" rel="noopener noreferrer">{row.original.ssnAddress}</a>
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
        ], [networkURL]
    );

    return (
        <>
        <Spinner class="spinner-border dashboard-spinner mb-4" area={PromiseArea.PROMISE_GET_EXPLORER_STATS} />
        { data.length === 0 && <div className="d-block text-left"><em>No information found.</em></div> }
        { data.length > 0 && <Table columns={columns} data={data} /> }
        </>
    );
}

export default ExplorerStakingPortfolio;