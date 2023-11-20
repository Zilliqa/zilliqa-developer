import { toBech32Address, fromBech32Address } from '@zilliqa-js/crypto';
import { Explorer, NetworkURL, Network, TransactionType, AccountType, Constants, OperationStatus } from './enum';
import Alert from '../components/alert';
import { ZilSigner } from '../zilliqa-signer';
import { getBlockchainExplorer } from './config-json-helper';
import { SsnStats } from './interface';
import { ZilSdk } from '../zilliqa-api';
const { BN, validation, units } = require('@zilliqa-js/util');
const BigNumber = require('bignumber.js');


// config.js from public folder
const blockchain_explorer_config = getBlockchainExplorer();

export const bech32ToChecksum = (address: string) => {
    if (validation.isAddress(address)) {
        // convert to checksum format
        return fromBech32Address(toBech32Address(address))
    }

    if (validation.isBech32(address)) {
        return fromBech32Address(address);
    }
    return address;
};

export const convertBase16ToBech32 = (address: string) => {
    if (validation.isAddress(address)) {
        return toBech32Address(address);
    }
    return address;
}

export const convertZilToQa = (amount: string) => {
    return units.toQa(amount, units.Units.Zil);
};

// convert from sdk "float" representation to percentage
// sdk "float" is represented in 1e7 format
export const convertToProperCommRate = (rate: string) => {
    if (!rate) {
        return 0;
    }
    let commRate = new BigNumber(rate).dividedBy(10**7);
    return commRate;
};

// compute the total stake amount using the list of ssnlist stake
// used for ssn-table because if we depend on the totalstakeamount fetch from the saga
// the update would delay, causing a issue where totalstakeamount is 0
export const computeTotalStakeAmt = (ssnlist: SsnStats[]) => {
    let totalStakeAmt = new BigNumber(0);
    for (const ssnstats of ssnlist) {
        totalStakeAmt = totalStakeAmt.plus(new BigNumber(ssnstats.stakeAmt));
    }
    return totalStakeAmt;
}

// compute the stake amount as a percentage of total stake amount
// returns a BigNumber
export const computeStakeAmtPercent = (inputStake: string, totalStake: string) => {
    if (!inputStake || !totalStake || totalStake === '0') {
        return 0;
    }
    const inputStakeBN = new BigNumber(inputStake);
    const totalStakeBN = new BigNumber(totalStake);
    const stakePercentage = inputStakeBN.dividedBy(totalStakeBN).times(100);
    return stakePercentage;
}

// checks if input contains only postive numbers
// returns true if so, otherwise returns false
export const isDigits = (input: string) => {
    return /^\d+$/.test(input);
}

export const computeGasFees = (gasPrice: string, gasLimit: string) => {
    // console.log("compute gas fees util: ", gasPrice);
    // console.log("compute gas limit util: ", gasLimit);
    return new BN(gasPrice.toString()).mul(new BN(gasLimit.toString()));
}

// convert commission rate from percentage to contract comm rate
// userInputRate is a float
// returns a big number
export const percentToContractCommRate = (userInputRate: string) => {
    if (!userInputRate) {
        return 0;
    }
    let scillaFloat = new BigNumber(userInputRate).times(10**7);
    return scillaFloat;
};

// convert balances and other numbers into string
// with commas as thousand separators and decimals places
export const convertQaToCommaStr = (inputVal: string) => {
    let zil = units.fromQa(new BN(inputVal), units.Units.Zil);
    let zilProperDecimalStr = new BigNumber(zil).toFixed(3);
    const splitAmt = zilProperDecimalStr.split('.');
    
    // add comma separator to front part
    let frontAmt = splitAmt[0].replace(/(.)(?=(\d{3})+$)/g,'$1,')
    let backAmt = splitAmt[1];
    return frontAmt + "." + backAmt;
}

// show the full zil amount
export const convertQaToZilFull = (amount: string) => {
    let result = "0.00";
    try {
        result = units.fromQa(new BN(amount), units.Units.Zil);
    } catch (err) {
        result = "0.00";
    }
    return result;
}

// convert gzil amount in 15 decimal places to a comma represented string
export const convertGzilToCommaStr = (inputVal: string) => {
    const gzil = new BigNumber(inputVal).shiftedBy(-15).toFixed(3);
    const splitAmt = gzil.split('.');

    // add comma separator to front part
    let frontAmt = splitAmt[0].replace(/(.)(?=(\d{3})+$)/g,'$1,')
    let backAmt = splitAmt[1];
    return frontAmt + "." + backAmt;
}

export const convertNetworkUrlToLabel = (url: string) => {
    let label = '';
    switch (url) {
        case NetworkURL.MAINNET:
            label = Network.MAINNET;        
            break;
        case NetworkURL.TESTNET:
            label = Network.TESTNET;
            break;
        default:
            label = Network.TESTNET;
            break;
    }
    return label;
}

export const getTxnLink = (txnId: string, networkURL: string) => {
    let link = "";

    if (blockchain_explorer_config === Explorer.VIEWBLOCK) {
        if (networkURL === NetworkURL.MAINNET) {
            link = "https://viewblock.io/zilliqa/tx/0x" + txnId
        } else {
            link = "https://viewblock.io/zilliqa/tx/0x" + txnId  + "?network=testnet"
        }
    } else {
        // devex
        link = "https://devex.zilliqa.com/tx/0x" + txnId + "?network=" + networkURL;
    }

    return link;
}

export const getAddressLink = (address: string, networkURL: string) => {
    let link = "";

    if (blockchain_explorer_config === Explorer.VIEWBLOCK) {
        if (networkURL === NetworkURL.MAINNET) {
            link = "https://viewblock.io/zilliqa/address/" + address
        } else {
            link = "https://viewblock.io/zilliqa/address/" + address  + "?network=testnet"
        }
    } else {
        // devex
        link = "https://devex.zilliqa.com/address/" + address + "?network=" + networkURL;
    }
    
    return link;
}

export const getZillionExplorerLink = (address: string) => {
    let domain = window.location.origin;
    return domain + "/address/" + address;
}

// returns the zil address with '...'
export const getTruncatedAddress = (address: string) => {
    if (!address) {
        return "";
    }
    const addressLen = address.length;
    const front = address.substring(0, 6);
    const end = address.substring(addressLen-4);
    return front.concat("...", end);
}

export const getTransactionText = (txnType: TransactionType) => {
    switch (txnType) {
        case TransactionType.CLAIM_REWARDS:
            return "Claim Rewards";
        case TransactionType.COMPLETE_STAKE_WITHDRAW:
            return "Complete Stake Withdrawal";
        case TransactionType.DELEGATE_STAKE:
            return "Delegate Stake";
        case TransactionType.INITIATE_STAKE_WITHDRAW:
            return "Initiate Stake Withdrawal";
        case TransactionType.TRANSFER_STAKE:
            return "Transfer Stake";
        case TransactionType.UPDATE_COMM_RATE:
            return "Update Commission Rate";
        case TransactionType.UPDATE_RECV_ADDR:
            return "Update Receiving Address";
        case TransactionType.WITHDRAW_COMM:
            return "Withdraw Commission";
        case TransactionType.REQUEST_DELEG_SWAP:
            return "Request Delegator Swap";
        case TransactionType.REVOKE_DELEG_SWAP:
            return "Revoke Delegator Swap";
        case TransactionType.CONFIRM_DELEG_SWAP:
            return "Accept Swap Request";
        case TransactionType.REJECT_DELEG_SWAP:
            return "Reject Swap Request";
        default:
            return "Error";
    }
}

// show information to prompt users
// for used during contract calls
export const showWalletsPrompt = (accountType: string) => {
    if (accountType === AccountType.LEDGER) {
        Alert('info', "Info", "Accessing the ledger device for keys.");
        Alert('info', "Info", "Please follow the instructions on the device.");
        return;
    }

    if (accountType === AccountType.ZILPAY) {
        Alert('info', "Info", "Please follow the instructions on ZilPay.");
        return;
    }
}

// check if wallet has sufficient balance to pay for gas fees
// for used during contract calls
// @param address wallet address in base16
// returns true if balance is greater than or equal to gas fees; otherwise returns false
export const validateBalance = async (address: string) => {
    // fetch a new balance in case, user's balance is not updated on frontend
    const balance = await ZilSdk.getBalance(address);
    const gasFees = ZilSigner.getGasFees();
    // console.log("user bal: ", balance);
    // console.log("gasFees: ", gasFees.toString());
    if (new BN(balance.toString()).gte(new BN(gasFees.toString()))) {
        return true;
    }
    return false;
}

export const calculateBlockRewardCountdown = (blockNum: number, currentNetworkURL: string) => {
    let sampleRewardBlockNum = 0;
    let rewardBlockCount = 0;

    if (currentNetworkURL === NetworkURL.MAINNET) {
        sampleRewardBlockNum = Constants.SAMPLE_REWARD_BLOCK_MAINNET;
        rewardBlockCount = Constants.REWARD_BLOCK_COUNT_MAINNET;
    } else {
        sampleRewardBlockNum = Constants.SAMPLE_REWARD_BLOCK_TESTNET;
        rewardBlockCount = Constants.REWARD_BLOCK_COUNT_TESTNET;
    }

    const blockDiff = blockNum - sampleRewardBlockNum;
    const blockTraverse = blockDiff % rewardBlockCount;
    const blockCountdown = rewardBlockCount - blockTraverse;
    
    return blockCountdown;
}

/**
 * used by saga to check if response from fetching a contract state has any errors
 * @param obj the contract result
 * @returns true if response has no errors, false otherwise
 */
 export const isRespOk = (obj: any): boolean => {
    if (
        obj !== undefined &&
        obj !== null &&
        obj !== OperationStatus.ERROR
    ) {
        return true;
    }
    return false;
}