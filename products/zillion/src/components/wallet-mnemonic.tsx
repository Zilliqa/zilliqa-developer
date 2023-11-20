/**
 * @deprecated
 */
import React, { useState } from 'react';
import Alert from './alert';


function MnemonicWallet(props: any) {
    const role = props.role;
    const [mnemonic, setMnemonic] = useState('');
    const [mnemonicIndex, setMnemonicIndex] = useState(0);
    const [password, setPassword] = useState('');

    const handleError = () => {
        Alert('error', 'Wallet Decrypt Error', 'Please ensure there are no carriage returns.');
    }

    const handleMnemonic = (e: any) => {
        setMnemonic(e.target.value);
    }

    const handleMnemonicIndex = (e: any) => {
        setMnemonicIndex(parseInt(e.target.value));
    }

    const handlePassword = (e:any) => {
        setPassword(e.target.value);
    }

    const unlockWallet = async () => {
        console.log("unlock by mnemonic");
        // console.log("mnemonic: %o", mnemonic);
        // console.log("mnemonic index: %o", mnemonicIndex);
        // const address = await Account.addWalletByMnemonic(mnemonic, mnemonicIndex, password);

        // if (address !== "error") {
        //     console.log("wallet add success: %o", address);
        //     // show loading state
        //     props.onWalletLoadingCallback();

        //     // update context
        //     initParams(address, AccountType.MNEMONIC);
        //     await updateRole(address, role);
        //     updateAuth();

        //     // no error
        //     // call parent function to redirect to dashboard
        //     props.onSuccessCallback();
        // } else {
        //     handleError();
        // }
    };

    return (
        <div className="wallet-access">
            <h2 className="mb-4">Access Wallet via Mnemonic Phrase</h2>
            <div className="form-group">
                <label htmlFor="mnemonic-select"><strong>Mnemonic Type</strong></label>
                <br/>
                <select id="mnemonic-select" value={mnemonicIndex} onChange={handleMnemonicIndex} className="form-control-xs">
                    <option value='0'>Default</option>
                    <option value='1'>Password Based</option>
                </select>
            </div>
            <div className="form-group">
                <label>
                    <strong>Mnemonic Phrase</strong>
                    <textarea id="mnemonic-phrase" value={mnemonic} className="form-control" onChange={handleMnemonic} />
                </label>
            </div>

            {
                (mnemonicIndex === 1) &&

                <div className="form-group mb-4">
                    <input id="mnemonic-password-input" className="p-2" type="password" name="password" value={password} placeholder="Enter your password" onChange={handlePassword} />
                </div>
            }
            
            <button type="button" className="btn btn-user-action mx-2" onClick={unlockWallet}>Unlock Wallet</button>
            <button type="button" className="btn btn-user-action-cancel mx-2" onClick={props.onReturnCallback}>Back</button>
        </div>
    );
}

export default MnemonicWallet;