export enum AccountType {
    PRIVATEKEY = "PRIVATEKEY",
    KEYSTORE = "KEYSTORE",
    MNEMONIC = "MNEMONIC",
    ZILPAY = "ZILPAY",
    MOONLET = "MOONLET",
    LEDGER = "LEDGER",
    NONE = ""
}


// all other constants
export enum Constants {
    MANUAL_REFRESH_DELAY=5000,
    REFRESH_RATE = 30000,
    DEFAULT_GAS_PRICE = 2000000000,
    DEFAULT_GAS_LIMIT = 30000,
    MAX_GZIL_SUPPLY = 722700,
    REWARD_BLOCK_COUNT_MAINNET = 2200,
    REWARD_DS_BLOCK_COUNT_MAINNET = 22,         // NOT IN USE
    REWARD_BLOCK_COUNT_TESTNET = 200,
    REWARD_DS_BLOCK_COUNT_TESTNET = 2,          // NOT IN USE
    REWARD_ZIL_PER_DAY = 1734000,               // ZIL
    SAMPLE_REWARD_BLOCK_MAINNET = 1166800,
    SAMPLE_REWARD_BLOCK_TESTNET = 2041000,
    FINAL_GZIL_REWARD_BLOCK_MAINNET = 1483713,
    LEDGER_VENDOR_ID = 11415
}

export enum Environment {
    DEV = "dev",
    PROD = "prod",
    STAGE = "stage"
}

export enum Explorer {
    DEVEX = "devex",
    VIEWBLOCK = "viewblock"
}

export enum LedgerIndex {
    DEFAULT = -1,
}

export enum Network {
    TESTNET = "testnet",
    MAINNET = "mainnet",
    ISOLATED_SERVER = "isolated_server",
    PRIVATE = "private"
}

export enum NetworkURL {
    TESTNET = "https://dev-api.zilliqa.com",
    MAINNET = "https://api.zilliqa.com",
    ISOLATED_SERVER = "https://zilliqa-isolated-server.zilliqa.com"
}

export enum OperationStatus {
    ERROR = "ERROR",
    COMPLETE = "COMPLETE",
    PENDING = "PENDING",
    IDLE = "IDLE",
}

export enum PromiseArea {
    PROMISE_GET_BALANCE = "PROMISE_GET_BALANCE",
    PROMISE_GET_CONTRACT = "PROMISE_GET_CONTRACT",
    PROMISE_GET_DELEG_STATS = "PROMISE_GET_DELEG_STATS",
    PROMISE_GET_DELEG_SWAP_REQUESTS = "PROMISE_GET_DELEG_SWAP_REQUESTS",
    PROMISE_GET_EXPLORER_STATS = "PROMISE_GET_EXPLORER_STATS",
    PROMISE_GET_EXPLORER_PENDING_WITHDRAWAL = "PROMISE_GET_EXPLORER_PENDING_WITHDRAWAL",
    PROMISE_GET_OPERATOR_STATS = "PROMISE_GET_OPERATOR_STATS",
    PROMISE_GET_PENDING_WITHDRAWAL = "PROMISE_GET_PENDING_WITHDRAWAL",
    PROMISE_GET_STAKE_PORTFOLIO = 'PROMISE_GET_STAKE_PORTFOLIO',
    PROMISE_GET_SSN_STATS = "PROMISE_GET_SSN_STATS",
    PROMISE_WITHDRAW_COMM = "PROMISE_WITHDRAW_COMM",
    PROMISE_LANDING_STATS = "PROMISE_LANDING_STATS",
}

export enum Role {
    DELEGATOR = "DELEGATOR",
    OPERATOR = "OPERATOR",
    NONE = "",
}

export enum SsnStatus {
    ACTIVE = "Active",
    INACTIVE = "Below Min. Stake"
}

export enum ProxyCalls {
    COMPLETE_WITHDRAWAL = "CompleteWithdrawal",
    DELEGATE_STAKE = "DelegateStake",
    REDELEGATE_STAKE = "ReDelegateStake",
    CONFIRM_DELEG_SWAP = "ConfirmDelegatorSwap",
    REJECT_DELEG_SWAP = "RejectDelegatorSwap",
    REQUEST_DELEG_SWAP = "RequestDelegatorSwap",
    REVOKE_DELEG_SWAP = "RevokeDelegatorSwap",
    UPDATE_COMM = "UpdateComm",
    UPDATE_RECV_ADDR = "UpdateReceivingAddr",
    WITHDRAW_COMM = "WithdrawComm",
    WITHDRAW_STAKE_AMT = "WithdrawStakeAmt",
    WITHDRAW_STAKE_REWARDS = "WithdrawStakeRewards",
}

export enum TransactionType {
    CLAIM_REWARDS = 1,
    COMPLETE_STAKE_WITHDRAW = 2,
    DELEGATE_STAKE = 3,
    INITIATE_STAKE_WITHDRAW = 4,
    TRANSFER_STAKE = 5,
    UPDATE_COMM_RATE = 6,
    UPDATE_RECV_ADDR = 7,
    WITHDRAW_COMM = 8,
    REQUEST_DELEG_SWAP = 9,
    REVOKE_DELEG_SWAP = 10,
    CONFIRM_DELEG_SWAP = 11,
    REJECT_DELEG_SWAP = 12,
}

export enum ButtonText {
    NOT_AVAILABLE = "Not Available"
}

// 'true': disabled all contract call buttons, add banner to dashboard and main page
export enum ContractState {
    IS_PAUSED = "false"
}
