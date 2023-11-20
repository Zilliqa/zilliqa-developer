import React from 'react';
import { AccountType, Environment } from '../util/enum';
import Alert from './alert';

import { toBech32Address } from '@zilliqa-js/crypto';
import { getEnvironment } from '../util/config-json-helper';


function WalletZilPay(props: any) {
    // config.js from public folder
    const env = getEnvironment();

    const unlockWallet = async () => {

        // Getting ZilPay inject.
        const zilPay = (window as any).zilPay;

        // Checking on ZilPay inject and wallet state.
        if (!zilPay) {
            Alert('warn', 'ZilPay Not Installed', 'Please install ZilPay wallet.');
            return null;

        } else if (!zilPay.wallet.isEnable) {
            Alert('warn', 'Locked Wallet', 'Please unlock wallet on ZilPay.');
            return null;
        }

        try {
            // Shell try ask user about access.
            const connected = await zilPay.wallet.connect();

            // Checking access.
            if (!connected) {
                Alert('error', 'Locked Wallet', 'Please allow ZilPay to access this app.');
                return null;
            }

            const { base16 } = zilPay.wallet.defaultAccount;
            const bech32Address = toBech32Address(base16);

            // request parent to show spinner while updating
            props.onWalletLoadingCallback();
            
            // request parent to redirect to dashboard
            props.onSuccessCallback(base16, bech32Address, AccountType.ZILPAY);
        } catch (err) {
            console.error("error unlocking via zilpay...: %o", err);
            Alert('error', 'Unable to access ZilPay', 'Please check if there is a new ZilPay version or clear your browser cache.');
        }
    }

    return (
        <>
        <div className="wallet-access">
            <h2>Access wallet using ZilPay</h2>
            
            { env === Environment.PROD ? 
                <p className="my-4"><strong>Note:</strong> We remind all users to set your ZilPay network to <strong>Mainnet</strong></p> :
                <p className="my-4"><strong>Note:</strong> We remind all users to set your ZilPay network to <strong>Testnet</strong></p>
            }
            
            <button type="button" className="btn btn-user-action mx-2" onClick={unlockWallet}>Unlock Wallet</button>
            <button type="button" className="btn btn-user-action-cancel mx-2" onClick={props.onReturnCallback}>Back</button>
        </div>
        </>
    );
}

export default WalletZilPay;