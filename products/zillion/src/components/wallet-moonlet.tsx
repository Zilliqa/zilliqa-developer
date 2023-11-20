/**
 * @deprecated
 */
import React from 'react';

function MoonletWallet(props: any) {

    const unlockWallet = async () => {
        console.log("unlock by moonlet");

        // if no error
        // call parent function to redirect to dashboard
        props.onSuccessCallback();
    }

    return (
        <div>
            <h2>Load Wallet using Moonlet</h2>
            <button type="button" className="btn btn-success mx-2" onClick={unlockWallet}>Unlock Wallet</button>
            <button type="button" className="btn btn-primary mx-2" onClick={props.onReturnCallback}>Back</button>
        </div>
    );
}

export default MoonletWallet;