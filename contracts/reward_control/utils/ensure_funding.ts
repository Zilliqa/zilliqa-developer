/* A script which ensures that all the accounts in secret.yaml are properly funded so they can
 * either deploy or test scripts. We fund both ethereum and zilliqa accounts with the given private keys.
 *
 * The funding source is taken from `fundingsource: zilliqa:` and we ensure that at least `amount` ZIL is
 * credited to each account. We fund from a zilliqa account because those are the accounts that get funded
 * by default in test servers :-p
 */

// I apologise for the horror below. It was originally written v quickly for ionise :-( - rrw 2023-07-20
import { Account, Transaction, TxParams, Wallet } from "@zilliqa-js/account";
import yargs from "yargs";
import { hideBin } from "yargs/helpers";
import { getSignerForAddress } from "../utils/hardhatUtils";
import { ethers } from "hardhat";
import { initZilliqa, Setup as ZilliqaSetup } from "hardhat-scilla-plugin";
import { Zilliqa } from "@zilliqa-js/zilliqa";
import {
  getAddressFromPrivateKey,
  toChecksumAddress,
} from "@zilliqa-js/crypto";
import { BN, units } from "@zilliqa-js/util";
import * as config from "./config.ts";
import * as utils from "./utils.ts";
import { Blockchain } from "@zilliqa-js/blockchain";
import { Wallet } from "@zilliqa-js/wallet";
import { Account as ZilliqaAccount } from "@zilliqa-js/account";

const ETHER_TO_WEI = 1000000000000000000n;

async function getFundingRequired(
  setup: ZilliqaSetup,
  fundAddress: string,
  amountQa: BN
): BN {
  let balance = await utils.getZilBalance(setup, fundAddress);
  if (balance.gte(amountQa)) {
    return new BN("0");
  }
  // otherwise we need to fund.
  let amountToFund = amountQa.sub(balance);
  return amountToFund;
}

async function ensureZilFunding(
  setup: ZilliqaSetup,
  funderKey: string,
  fundAddress: string,
  fundingAmountQa: BN
) {
  let amount = await getFundingRequired(setup, fundAddress, fundingAmountQa);
  if (amount.lte(new BN("0"))) {
    return;
  }
  // Get balance.
  console.log(`${fundAddress}: + ${amount}`);
  const txDefault: TxParams = {
    version: setup.version,
    gasPrice: setup.gasPrice,
    gasLimit: setup.gasLimit,
    amount: amount,
    toAddr: toChecksumAddress(fundAddress),
  };
  const txnData = setup.zilliqa.transactions.new(txDefault);
  console.log(`Funder key ${funderKey}`);
  let sender = await utils.getZilliqaBlockchainForPrivateKey(setup, funderKey);
  const txn = await sender.createTransaction(txnData, 50, 5000);
  if (txn.isRejected()) {
    throw Error(`Funding transaction rejected: ${txn}`);
  }
}

async function main() {
  let secrets = config.baseSecrets();
  let fundingPrivkey = secrets.fundingsource.zilliqa;
  // console.log(`${JSON.stringify(secrets)} ${JSON.stringify(secrets.fundingsource)}`)
  let fundingAmountQa = units.toQa(
    secrets.fundingsource.amount,
    units.Units.Zil
  );
  let fundingAmountEther = BigInt(secrets.fundingsource.amount);
  let fundingAmountWei = fundingAmountEther * ETHER_TO_WEI;
  let fundingAddress = utils.zilliqaAddressFromPrivateKey(fundingPrivkey);
  let keysToFund = config.getPrivKeys();
  console.log(
    `Funding amount is ${fundingAmountQa} Qa, ${fundingAmountWei} wei`
  );
  console.log(`Funding address is zil:${fundingAddress}`);
  // Now get the current network url and chain id
  let setup = utils.ensureZilliqa();

  console.log("Checking accounts .. ");
  for (var key of keysToFund) {
    for (var address of [
      utils.zilliqaAddressFromPrivateKey(key),
      utils.evmAddressFromPrivateKey(key),
    ]) {
      await ensureZilFunding(setup, fundingPrivkey, address, fundingAmountQa);
      let amount = await getFundingRequired(setup, address, fundingAmountQa);
      if (amount.gt(new BN("0"))) {
        throw Error(`I couldn't fund ${address}`);
      }
    }
  }
  console.log("Done");
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
