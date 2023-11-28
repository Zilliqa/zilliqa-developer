interface DelegStats {
    globalAPY: string,
    zilRewards: string,
    gzilRewards: string,
    gzilBalance: string,
    totalDeposits: string,
}

interface DelegStakingPortfolioStats {
    ssnName: string,
    ssnAddress: string,
    delegAmt: string,
    rewards: string,
}

interface OperatorStats {
    name: string
    stakeAmt: string,
    bufferedDeposits: string,
    commRate: string,
    commReward: string,
    delegNum: string,
    receiver: string,
}

interface PendingWithdrawStats {
    amount: string,
    blkNumCountdown: string,
    blkNumCheck: string,
    progress: string,
}

interface SsnStats {
    address: string,
    name: string,
    apiUrl: string,
    stakeAmt: string,
    bufferedDeposits: string,
    commRate: string,
    commReward: string,
    delegNum: string,
    status: string,
}

interface NodeOptions {
    address: string,
    name: string,
    stakeAmt: string,
    delegNum: string,
    commRate: string,
}

interface LandingStats {
    circulatingSupplyStake: string,
    nodesNum: string,
    delegNum: string,
    gzil: string,
    remainingGzil: string,
    totalDeposits: string,
    estRealtimeAPY: string,
}

// unified interface to hold all staking information
// that a delegator has selected from dropdown
interface StakeModalData {
    ssnName: string,
    ssnAddress: string,
    commRate: string,
    rewards: string,
    delegAmt: string,
}

interface SwapDelegModalData {
    swapRecipientAddress: string,
    requestorList: string[],
}

export const initialDelegStats: DelegStats = {
    globalAPY: '0',
    zilRewards: '0',
    gzilRewards: '0',
    gzilBalance: '0',
    totalDeposits: '0',
}

export const initialLandingStats: LandingStats = {
    circulatingSupplyStake: '0',
    nodesNum: '0',
    delegNum: '0',
    gzil: '0',
    remainingGzil: '0',
    totalDeposits: '0',
    estRealtimeAPY: '0',
}

export const initialOperatorStats: OperatorStats = {
    name: '',
    stakeAmt: '0',
    bufferedDeposits: '0',
    commRate: '0',
    commReward: '0',
    delegNum: '0',
    receiver: '0',
}

export const initialSwapDelegModalData: SwapDelegModalData = {
    swapRecipientAddress: '',
    requestorList: []
}

export const initialStakeModalData: StakeModalData = {
    ssnName: '',
    ssnAddress: '',
    commRate: '0',
    rewards: '0',
    delegAmt: '0',
}

export type {
    DelegStats,
    DelegStakingPortfolioStats,
    LandingStats,
    NodeOptions,
    OperatorStats,
    PendingWithdrawStats,
    SsnStats,
    StakeModalData,
    SwapDelegModalData,
}