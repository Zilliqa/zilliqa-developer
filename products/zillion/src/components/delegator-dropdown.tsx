import React from 'react';
import { StakeModalData } from '../util/interface';
import { ButtonText, ContractState } from '../util/enum';
import { useAppDispatch } from '../store/hooks';
import { UPDATE_STAKE_MODAL_DATA } from '../store/userSlice';


function DelegatorDropdown(props: any) {
    const dispatch = useAppDispatch();
    // from staking-portfolio
    // modal data is a state variable on dashboard
    // delegAmt, rewards in Qa
    const { 
        ssnName,
        ssnAddress,
        delegAmt,
        rewards,
    } = props;

    const handleClaimRewards = () => {
        dispatch(UPDATE_STAKE_MODAL_DATA({
            stake_modal: {
                ssnName: ssnName,
                ssnAddress: ssnAddress,
                rewards: rewards,
            } as StakeModalData
        }));
    };

    const handleTransferStake = () => {
        dispatch(UPDATE_STAKE_MODAL_DATA({
            stake_modal: {
                ssnName: ssnName,
                ssnAddress: ssnAddress,
                delegAmt: delegAmt,
            } as StakeModalData
        }));
    };

    const handleWithdrawStake = () => {
        dispatch(UPDATE_STAKE_MODAL_DATA({
            stake_modal: {
                ssnName: ssnName,
                ssnAddress: ssnAddress,
                delegAmt: delegAmt,
            } as StakeModalData
        }));
    };

    return (
        <div id="delegator-dropdown" className="dropdown dropright">
            <button 
                className={ContractState.IS_PAUSED.toString() === 'true' ?
                            'btn btn-contract-small-disabled dropdown-toggle shadow-none' :
                            'btn btn-contract-small dropdown-toggle shadow-none'}
                data-display="static" 
                type="button" 
                id="dropdown-menu-btn" 
                data-toggle="dropdown" 
                aria-haspopup="true" 
                aria-expanded="false"
                disabled={ContractState.IS_PAUSED.toString() === 'true' ? true : false}>
                    {ContractState.IS_PAUSED.toString() === 'true' ? ButtonText.NOT_AVAILABLE : 'Manage'}
            </button>
            <div className="dropdown-menu delegator-menu animate__animated animate__fadeIn" aria-labelledby="dropdown-menu-btn">
                <button 
                    type="button" 
                    className="btn btn-deleg-action shadow-none" 
                    data-toggle="modal" 
                    data-target="#withdraw-reward-modal" 
                    data-keyboard="false" 
                    data-backdrop="static"
                    onClick={handleClaimRewards}
                    disabled={ContractState.IS_PAUSED.toString() === 'true' ? true : false}>
                        {ContractState.IS_PAUSED.toString() === 'true' ? ButtonText.NOT_AVAILABLE : 'Claim Rewards'}
                </button>
                <button 
                    type="button" 
                    className="btn btn-deleg-action shadow-none" 
                    data-toggle="modal" 
                    data-target="#redeleg-stake-modal" 
                    data-keyboard="false" 
                    data-backdrop="static"
                    onClick={handleTransferStake}
                    disabled={ContractState.IS_PAUSED.toString() === 'true' ? true : false}>
                        {ContractState.IS_PAUSED.toString() === 'true' ? ButtonText.NOT_AVAILABLE : 'Transfer Stake'}
                </button>
                <button
                    type="button"
                    className="btn btn-deleg-action shadow-none" 
                    data-toggle="modal" 
                    data-target="#withdraw-stake-modal" 
                    data-keyboard="false" 
                    data-backdrop="static"
                    onClick={handleWithdrawStake}
                    disabled={ContractState.IS_PAUSED.toString() === 'true' ? true : false}>
                        {ContractState.IS_PAUSED.toString() === 'true' ? ButtonText.NOT_AVAILABLE : 'Initiate Stake Withdrawal'}
                </button>
            </div>
        </div>
    );
}

export default DelegatorDropdown;