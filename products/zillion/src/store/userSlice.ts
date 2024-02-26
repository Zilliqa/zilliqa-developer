import { createSlice } from '@reduxjs/toolkit'
import { AccountType, LedgerIndex, OperationStatus, Role } from '../util/enum'
import { DelegStakingPortfolioStats, DelegStats, initialDelegStats, initialOperatorStats, initialStakeModalData, initialSwapDelegModalData, OperatorStats, PendingWithdrawStats, StakeModalData, SwapDelegModalData } from '../util/interface';

interface UserState {
    address_bech32: string,
    address_base16: string,
    account_type: AccountType,
    authenticated: boolean,
    balance: string,                                    // zils in Qa
    gzil_balance: string,
    complete_withdrawal_amt: string,                    // amount that is allowed to complete withdraw
    ledger_index: number,
    role: Role,                                         // actual role
    selected_role: Role,                                // role that the user selects when signing in
    deleg_stats: DelegStats,                            // track the delegator stats, i fuser is a delegator
    deleg_staking_portfolio_list: DelegStakingPortfolioStats[],     // list of ssn info that a delegator has staked with
    operator_stats: OperatorStats                       // track the operator stats, if user is an operator
    pending_withdraw_list: PendingWithdrawStats[]       // track pending withdrawals
    stake_modal_data: StakeModalData                    // hold info about a selected ssn; for dropdown selection use when choosing whether to stake, transfer, claim etc 
    swap_deleg_modal_data: SwapDelegModalData,          // hold delegator swap request
    is_deleg_stats_loading: OperationStatus,            // delegator stats and staking portfolio status indicator
    is_operator_stats_loading: OperationStatus          // status indicator for loading operator stats
}

const initialState: UserState = {
    address_bech32: '',
    address_base16: '',
    account_type: AccountType.NONE,
    authenticated: false,
    balance: '0',
    gzil_balance: '0',
    complete_withdrawal_amt: '0',
    ledger_index: LedgerIndex.DEFAULT,
    role: Role.NONE,
    operator_stats: initialOperatorStats,
    selected_role: Role.NONE,
    deleg_stats: initialDelegStats,
    deleg_staking_portfolio_list: [],
    pending_withdraw_list: [],
    stake_modal_data: initialStakeModalData,
    swap_deleg_modal_data: initialSwapDelegModalData,
    is_deleg_stats_loading: OperationStatus.IDLE,
    is_operator_stats_loading: OperationStatus.IDLE,
}


/**
 * stores user's wallet information
 */
const userSlice = createSlice({
    name: 'user',
    initialState: initialState,
    reducers: {
        INIT_USER(state, action) {
            const { address_base16, address_bech32, account_type, authenticated, selected_role } = action.payload
            state.address_base16 = address_base16.toLowerCase()
            state.address_bech32 = address_bech32.toLowerCase()
            state.account_type = account_type
            state.authenticated = authenticated
            state.selected_role = selected_role
        },
        UPDATE_ADDRESS(state, action) {
            const { address_base16, address_bech32 } = action.payload
            state.address_base16 = address_base16.toLowerCase()
            state.address_bech32 = address_bech32.toLowerCase()
        },
        UPDATE_BALANCE(state, action) {
            const { balance } = action.payload
            state.balance = balance
        },
        UPDATE_COMPLETE_WITHDRAWAL_AMT(state, action) {
            const { complete_withdrawal_amt } = action.payload
            state.complete_withdrawal_amt = complete_withdrawal_amt
        },
        UPDATE_DELEG_STATS(state, action) {
            const { deleg_stats } = action.payload
            state.deleg_stats = deleg_stats
        },
        UPDATE_DELEG_PORTFOLIO(state, action) {
            const { portfolio_list } = action.payload
            state.deleg_staking_portfolio_list = portfolio_list
        },
        UPDATE_GZIL_BALANCE(state, action) {
            const { gzil_balance } = action.payload
            state.gzil_balance = gzil_balance
        },
        UPDATE_LEDGER_INDEX(state, action) {
            const { ledger_index } = action.payload
            state.ledger_index = ledger_index
        },
        UPDATE_OPERATOR_STATS(state, action) {
            const { operator_stats } = action.payload
            state.operator_stats = operator_stats
        },
        UPDATE_PENDING_WITHDRAWAL_LIST(state, action) {
            const { pending_withdraw_list } = action.payload
            state.pending_withdraw_list = pending_withdraw_list
        },
        UPDATE_ROLE(state, action) {
            const { role } = action.payload
            state.role = role
        },
        UPDATE_STAKE_MODAL_DATA(state, action) {
            const { stake_modal } = action.payload
            state.stake_modal_data = {...stake_modal}
        },
        UPDATE_SWAP_DELEG_MODAL(state, action) {
            const { swap_deleg_modal } = action.payload
            state.swap_deleg_modal_data = swap_deleg_modal
        },
        UPDATE_FETCH_DELEG_STATS_STATUS(state, action) {
            state.is_deleg_stats_loading = action.payload
        },
        UPDATE_FETCH_OPERATOR_STATS_STATUS(state, action) {
            state.is_operator_stats_loading = action.payload
        },
        RESET_USER_STATE(state) {
            state.address_bech32 = initialState.address_bech32
            state.address_base16 = initialState.address_base16
            state.account_type = initialState.account_type
            state.authenticated = initialState.authenticated
            state.balance = initialState.balance
            state.gzil_balance = initialState.gzil_balance
            state.complete_withdrawal_amt = initialState.complete_withdrawal_amt
            state.ledger_index = initialState.ledger_index
            state.role = initialState.role
            state.deleg_stats = initialState.deleg_stats
            state.deleg_staking_portfolio_list = initialState.deleg_staking_portfolio_list
            state.operator_stats = initialState.operator_stats
            state.pending_withdraw_list = initialState.pending_withdraw_list
            state.stake_modal_data = initialState.stake_modal_data
            state.swap_deleg_modal_data = initialState.swap_deleg_modal_data
            state.is_deleg_stats_loading = initialState.is_deleg_stats_loading
            state.is_operator_stats_loading = initialState.is_operator_stats_loading
        },
        QUERY_AND_UPDATE_BALANCE() {},
        QUERY_AND_UPDATE_GZIL_BALANCE() {},
        QUERY_AND_UPDATE_ROLE() {},
        QUERY_AND_UPDATE_DELEGATOR_STATS() {},
        QUERY_AND_UPDATE_OPERATOR_STATS() {}, 
        QUERY_AND_UPDATE_USER_STATS() {},
        POLL_USER_DATA_START() {},
        POLL_USER_DATA_STOP() {},
    },
})

export const {
    INIT_USER,
    QUERY_AND_UPDATE_BALANCE,
    QUERY_AND_UPDATE_GZIL_BALANCE,
    QUERY_AND_UPDATE_ROLE,
    QUERY_AND_UPDATE_DELEGATOR_STATS,
    QUERY_AND_UPDATE_OPERATOR_STATS,
    QUERY_AND_UPDATE_USER_STATS,
    POLL_USER_DATA_START,
    POLL_USER_DATA_STOP,
    UPDATE_ADDRESS,
    UPDATE_BALANCE,
    UPDATE_COMPLETE_WITHDRAWAL_AMT,
    UPDATE_DELEG_STATS,
    UPDATE_DELEG_PORTFOLIO,
    UPDATE_GZIL_BALANCE,
    UPDATE_LEDGER_INDEX,
    UPDATE_OPERATOR_STATS,
    UPDATE_PENDING_WITHDRAWAL_LIST,
    UPDATE_ROLE,
    UPDATE_STAKE_MODAL_DATA,
    UPDATE_SWAP_DELEG_MODAL,
    UPDATE_FETCH_DELEG_STATS_STATUS,
    UPDATE_FETCH_OPERATOR_STATS_STATUS,
    RESET_USER_STATE,
} = userSlice.actions

export default userSlice.reducer;