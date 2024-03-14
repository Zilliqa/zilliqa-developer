const { Zilliqa } = require("@zilliqa-js/zilliqa");
const { getAddressFromPrivateKey } = require("@zilliqa-js/crypto");
const { bytes } = require("@zilliqa-js/util");
const { getDefaultNetwork } = require("./helper");

const TESTNET_VERSION = bytes.pack(
  getDefaultNetwork().chainId,
  getDefaultNetwork().version
);
const zilliqa = new Zilliqa(getDefaultNetwork().url);

function useKey(privateKey) {
  const address = getAddressFromPrivateKey(privateKey);
  const accounts = Object.keys(zilliqa.wallet.accounts);
  if (
    accounts.findIndex((a) => a.toLowerCase() === address.toLowerCase()) < 0
  ) {
    zilliqa.wallet.addByPrivateKey(privateKey);
  }
  zilliqa.wallet.setDefault(address);
}

exports.TESTNET_VERSION = TESTNET_VERSION;
exports.zilliqa = zilliqa;
exports.useKey = useKey;
