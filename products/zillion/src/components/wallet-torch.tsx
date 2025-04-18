import React from "react";
import { AccountType, Environment } from "../util/enum";
import Alert from "./alert";

import { toBech32Address } from "@zilliqa-js/crypto";
import { getEnvironment } from "../util/config-json-helper";

function WalletTorch(props: any) {
  // config.js from public folder
  const env = getEnvironment();

  const unlockWallet = async () => {
    // Getting Torch inject.
    const torch = (window as any).torch;

    // Checking on Torch inject and wallet state.
    if (!torch) {
      Alert("warn", "Torch Not Installed", "Please install Torch wallet.");
      return null;
    } else if (!torch.wallet.isEnable) {
      Alert("warn", "Locked Wallet", "Please unlock wallet on Torch.");
      return null;
    }

    try {
      // Shell try ask user about access.
      const connected = await torch.wallet.connect();

      // Checking access.
      if (!connected) {
        Alert(
          "error",
          "Locked Wallet",
          "Please allow Torch to access this app."
        );
        return null;
      }

      const { base16 } = torch.wallet.defaultAccount;
      const bech32Address = toBech32Address(base16);

      // request parent to show spinner while updating
      props.onWalletLoadingCallback();

      // request parent to redirect to dashboard
      props.onSuccessCallback(base16, bech32Address, AccountType.TORCH);
    } catch (err) {
      console.error("error unlocking via torch...: %o", err);
      Alert(
        "error",
        "Unable to access Torch",
        "Please check if there is a new Torch version or clear your browser cache."
      );
    }
  };

  return (
    <>
      <div className="wallet-access">
        <h2>Access wallet using Torch</h2>

        {env === Environment.PROD ? (
          <p className="my-4">
            <strong>Note:</strong> We remind all users to set your Torch network
            to <strong>Mainnet</strong>
          </p>
        ) : (
          <p className="my-4">
            <strong>Note:</strong> We remind all users to set your Torch network
            to <strong>Testnet</strong>
          </p>
        )}

        <button
          type="button"
          className="btn btn-user-action mx-2"
          onClick={unlockWallet}
        >
          Unlock Wallet
        </button>
        <button
          type="button"
          className="btn btn-user-action-cancel mx-2"
          onClick={props.onReturnCallback}
        >
          Back
        </button>
      </div>
    </>
  );
}

export default WalletTorch;
