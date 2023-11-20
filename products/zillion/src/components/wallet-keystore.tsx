import { toBech32Address } from '@zilliqa-js/zilliqa';
import React, { useState } from 'react';
import { AccountType, OperationStatus } from '../util/enum';
import { logger } from '../util/logger';
import { ZilSigner } from '../zilliqa-signer';
import Alert from './alert';

function WalletKeystore(props: any) {
    const [filename, setFilename] = useState("");
    const [passphrase, setPassphrase] = useState("");
    const [keystore, setKeystore] = useState();

    const handleFile = (e: any) => {
        let keystoreFile = e.target.files[0];
        setFilename(keystoreFile.name);
        setKeystore(keystoreFile);
    }

    const handlePassword = (e: any) => {
        setPassphrase(e.target.value);
    }

    const unlockWallet = () => {
        if (keystore === undefined || keystore === null || keystore === "") {
            Alert('error', 'Keystore not found', 'Please upload a keystore file.');
            return;
        }

        const reader = new FileReader();
        reader.readAsText(keystore!);

        reader.onload = async () => {
            // show loading state
            props.onWalletLoadingCallback();

            const keystoreJSON = reader.result as string;
            const address = await ZilSigner.addWalletByKeystore(keystoreJSON, passphrase); // base16 with 0x

            if (address !== OperationStatus.ERROR) {
                logger("wallet add success: ", address);

                const bech32Address = toBech32Address(address);

                // call parent function to redirect to dashboard
                props.onSuccessCallback(address.toLowerCase(), bech32Address, AccountType.KEYSTORE);
            } else {
                Alert('error', 'Keystore Decrypt Error', 'Please ensure your passphrase is correct.');
            }
        }
        
        reader.onerror = (e) => {
            console.error(e);
        }
    }

    return (
        <div className="wallet-access">
            <h2>Access wallet via Keystore</h2>
            <div>
                <div id="keystore">
                    <p className="file-name">{filename}</p>
                    <label htmlFor="browsekeystore">{filename ? "Select wallet file" : "Select wallet file"}</label>
                    <input type="file" id="browsekeystore" onChange={handleFile} />
                </div>
                <input id="keystore-passphrase" type="password" name="password" className="p-2" placeholder="Enter your passphrase" value={passphrase} onChange={handlePassword}/>
            </div>
            <br/>
            <button type="button" className="btn btn-user-action mx-2" onClick={unlockWallet}>Unlock Wallet</button>
            <button type="button" className="btn btn-user-action-cancel mx-2" onClick={props.onReturnCallback}>Back</button>
        </div>
    );
}

export default WalletKeystore;