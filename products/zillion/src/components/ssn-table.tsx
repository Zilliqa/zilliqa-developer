import React, { useEffect, useMemo } from 'react';
import { useTable, useSortBy } from 'react-table';

import { SsnStatus, Role, ContractState, OperationStatus } from '../util/enum';
import { convertToProperCommRate, convertQaToCommaStr, computeStakeAmtPercent, getTruncatedAddress, computeTotalStakeAmt } from '../util/utils';
import { SsnStats, StakeModalData } from '../util/interface';
import ReactTooltip from 'react-tooltip';
import { useAppDispatch, useAppSelector } from '../store/hooks';
import SpinnerNormal from './spinner-normal';
import { UPDATE_STAKE_MODAL_DATA } from '../store/userSlice';


function Table({ columns, data, tableId, hiddenColumns, showStakeBtn }: any) {
    // showStakeBtn is true means displaying for delegators
    // for delegators view, don't sort by stake amount

    let tempInitialState = {};

    if (showStakeBtn) {
        // deleg view
        // don't sort by stake amount to prevent users from staking the top most everytime
        tempInitialState = {
            pageIndex: 0,
            hiddenColumns: hiddenColumns,
            sortBy: [
                {
                    id: 'name',
                    desc: false
                }
            ]
        }
    } else {
        // default sort by stake amt
        tempInitialState = {
            pageIndex: 0,
            hiddenColumns: hiddenColumns,
            sortBy: [
                {
                    id: 'stakeAmt',
                    desc: true
                }
            ]
        }
    }

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
            initialState: tempInitialState
        }, useSortBy);

    return (
        <>
            <table id={tableId} className="table table-responsive-lg " {...getTableProps()}>
                <thead>
                    {headerGroups.map(headerGroup => (
                        <tr {...headerGroup.getHeaderGroupProps()}>
                            {headerGroup.headers.map((column, index) => (
                                <th scope="col" {...column.getHeaderProps()}>
                                    {
                                        column.render('tipText') === '' ?
                                            column.render('Header') :
                                            <span className="ssn-table-header-with-tip" data-for='ssn-table-header-tip' data-tip={column.render('tipText')}>
                                                {column.render('Header')}
                                            </span>
                                    }
                                </th>
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
            <ReactTooltip id="ssn-table-header-tip" place="top" type="dark" effect="solid" />
        </>
    );
}

function SsnTable(props: any) {
    const dispatch = useAppDispatch();
    const role = useAppSelector(state => state.user.role);
    const loading: OperationStatus = useAppSelector(state => state.staking.is_ssn_stats_loading);
    const ssnList: SsnStats[] = useAppSelector(state => state.staking.ssn_list);
    const showStakeBtn = props.showStakeBtn ? props.showStakeBtn : false; // for deleg
    const totalStakeAmt = computeTotalStakeAmt(ssnList);

    const handleStake = (name: string, address: string, commRate: string) => {
        // set dashboard state variable
        dispatch(UPDATE_STAKE_MODAL_DATA({
            stake_modal: {
                ssnName: name,
                ssnAddress: address,
                commRate: commRate,
            } as StakeModalData
        }));
    }

    var array = [...ssnList];

    array= array.sort((a, b) => parseInt(b.stakeAmt) - parseInt(a.stakeAmt));

    const columns = useMemo(
        () => [
            {
                Header: 'name',
                accessor: 'name',
                tipText: ''
            },
            {
                Header: 'address',
                accessor: 'address',
                className: 'ssn-address',
                Cell: ({ row }: any) =>
                    <>
                        {getTruncatedAddress(row.original.address)}
                    </>,
                tipText: ''
            },
            {
                Header: 'api endpoint',
                accessor: 'apiUrl',
                Cell: ({ row }: any) => <span className="ssn-table-api-url">{row.original.apiUrl}</span>,
                tipText: 'Service API running by operator. Can be used as alternatives for Zilliqa API endpoint'
            },
            {
                Header: 'stake amount (ZIL)',
                accessor: 'stakeAmt',
                Cell: ({ row }: any) =>
                    <>
                        <span>{convertQaToCommaStr(row.original.stakeAmt)} ({computeStakeAmtPercent(row.original.stakeAmt, totalStakeAmt).toFixed(2)}&#37;)</span>
                    </>,
                tipText: 'Total amount being staked with this operator'
            },
            {
                Header: 'buffered deposit (ZIL)',
                accessor: 'bufferedDeposits',
                Cell: ({ row }: any) =>
                    <>
                        <span>{convertQaToCommaStr(row.original.bufferedDeposits)}</span>
                    </>,
                tipText: 'Total staked amount deposited in this cycle being considered for rewards in the next cycle'
            },
            {
                Header: 'Comm. Rate (%)',
                accessor: 'commRate',
                Cell: ({ row }: any) =>
                    <span>{convertToProperCommRate(row.original.commRate).toFixed(2)}</span>,
                tipText: 'Percentage of incoming rewards that the operator takes as commission'
            },
            {
                Header: 'Comm. Reward (ZIL)',
                accessor: 'commReward',
                Cell: ({ row }: any) =>
                    <span className="ssn-table-comm-reward">{convertQaToCommaStr(row.original.commReward)}</span>,
                tipText: 'Number of ZILs earned as commission by the operator'
            },
            {
                Header: 'Delegators',
                accessor: 'delegNum',
                tipText: 'Total number of delegators staking with this operator'
            },
            {
                Header: 'Status',
                accessor: 'status',
                Cell: ({ row }: any) =>
                    <>
                        <div className={row.original.status === SsnStatus.ACTIVE ? 'px-2 py-1 rounded ssn-table-status-active' : 'px-2 py-1 rounded ssn-table-status-inactive'}>
                            {
                                row.original.status === SsnStatus.INACTIVE ?
                                    <>Below<br />Min. Stake</> :
                                    row.original.status
                            }
                        </div>
                    </>,
                tipText: 'Determines whether the operator has met the minnimum stake amount and therefore ready to participate in staking and receive rewards'
            },
            {
                Header: 'Stake',
                accessor: 'stake',
                Cell: ({ row }: any) =>
                    <>
                        <button
                            type="button"
                            className="btn btn-contract-small shadow-none"
                            data-toggle="modal"
                            data-target="#delegate-stake-modal"
                            data-keyboard="false"
                            data-backdrop="static"
                            onClick={() => handleStake(row.original.name, row.original.address, row.original.commRate)}
                            disabled={ContractState.IS_PAUSED.toString() === 'true' ? true : false}>
                            Stake
                        </button>
                    </>,
                tipText: ''
            }
            // eslint-disable-next-line
        ], [array, role]
    )


    const getHiddenColumns = () => {
        // hide redudant info for certain group of users, e.g. commission reward
        // list the hidden column accessor names
        let hiddenColumns = ["address"];
        if (role !== undefined && role === Role.DELEGATOR && ContractState.IS_PAUSED.toString() !== 'true') {
            hiddenColumns.push("commReward", "apiUrl");
        } else if (role !== undefined && role === Role.DELEGATOR && ContractState.IS_PAUSED.toString() === 'true') {
            // hide stake button if contract state is paused
            hiddenColumns.push("stake");
        }

        if (showStakeBtn === false || role === Role.OPERATOR) {
            hiddenColumns.push("stake");
        }
        return hiddenColumns;
    }

    return (
        <>
            {
                loading === OperationStatus.PENDING &&
                <SpinnerNormal class="spinner-border dashboard-spinner mb-4" />
            }
            {
                loading === OperationStatus.COMPLETE &&
                <Table
                    columns={columns}
                    data={array}
                    className={props.tableId}
                    hiddenColumns={getHiddenColumns()}
                    showStakeBtn={showStakeBtn}
                />
            }
        </>
    );
}

export default SsnTable;
