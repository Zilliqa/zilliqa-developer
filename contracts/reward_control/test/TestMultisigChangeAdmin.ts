import { anyValue } from "@nomicfoundation/hardhat-chai-matchers/withArgs";
import { expect } from "chai";
import hre, { ethers } from "hardhat";
import { Contract, Signer } from "ethers";
import hre from "hardhat";
import { ScillaContract } from "hardhat-scilla-plugin";
import * as utils from "../utils/utils.ts";

let WALLET_INDEX_0 = 0;
let WALLET_INDEX_1 = 1;
let WALLET_INDEX_2 = 2;
let WALLET_INDEX_3 = 3;
const DEBUG = true;

interface ByStr20Type {
  constructor: string;
  argtypes: string[];
  arguments: string[];
}

function createByStr20Object(address: string): ByStr20Type {
  return {
    constructor: "ByStr20",
    argtypes: ["ByStr20"],
    arguments: [address],
  };
}
describe(utils.TestGeneral(0, "TestMultiSigChangeAdmin"), function () {
  // let hello: Contract;
  let scillaRewardsParamsContract: ScillaContract;
  let scillaMultiSigRewardsParamContract: ScillaContract;
  let scillaMultiSigRewardsParamContractNew: ScillaContract;

  before(async function () {
    utils.ensureZilliqa();

    utils.setZilliqaSignerToAccountByHardhatWallet(WALLET_INDEX_0);
    let owner_0 =
      utils.getZilliqaAddressForAccountByHardhatWallet(WALLET_INDEX_0);
    if (DEBUG) {
      console.log(`Address 0 ${owner_0}`);
    }
    let owner_1 =
      utils.getZilliqaAddressForAccountByHardhatWallet(WALLET_INDEX_1);
    if (DEBUG) {
      console.log(`Address 1 ${owner_1}`);
    }
    let owner_2 =
      utils.getZilliqaAddressForAccountByHardhatWallet(WALLET_INDEX_2);
    if (DEBUG) {
      console.log(`Address 2 ${owner_2}`);
    }

    if ((await hre.zilliqa.getBalanceForAddress(owner_0))[0].isZero()) {
      throw Error(`Account {owner_0} is not funded`);
    }
    if ((await hre.zilliqa.getBalanceForAddress(owner_1))[0].isZero()) {
      throw Error(`Account {owner_1} is not funded`);
    }
    if ((await hre.zilliqa.getBalanceForAddress(owner_2))[0].isZero()) {
      throw Error(`Account {owner_2} is not funded`);
    }
    const owner_list: string[] = [owner_0, owner_1, owner_2];
    if (DEBUG) {
      console.log(owner_list);
    }

    let non_owner =
      utils.getZilliqaAddressForAccountByHardhatWallet(WALLET_INDEX_3);
    if (DEBUG) {
      console.log(`Non-owner address ${non_owner}`);
    }
    if ((await hre.zilliqa.getBalanceForAddress(non_owner))[0].isZero()) {
      throw Error(`Account {non_owner} is not funded`);
    }
    if (DEBUG) {
      console.log(`All owners and non-owner have non-zero funds: good!`);
    }

    let num_of_required_signatures = 2;
    scillaMultiSigRewardsParamContract = await hre.deployScillaContract(
      "MultiSigWalletRewardsParam",
      owner_list,
      num_of_required_signatures
    );
    // console.log(scillaMultiSigRewardsParamContract)
    utils.checkScillaTransactionSuccess(scillaMultiSigRewardsParamContract);

    const multiSigAddress = scillaMultiSigRewardsParamContract.address;
    if (DEBUG) {
      console.log(`MultiSig address is ${multiSigAddress}`);
    }

    scillaRewardsParamsContract = await hre.deployScillaContract(
      "RewardsParams",
      multiSigAddress
    );
    utils.checkScillaTransactionSuccess(scillaRewardsParamsContract);

    if (DEBUG) {
      console.log(
        `Rewards Params Contract Address: ${scillaRewardsParamsContract.address}`
      );
    }
  });

  // changing admin
  it("Multi sig wallet can change admin to a new multisig wallet", async function () {
    let owner_0 =
      utils.getZilliqaAddressForAccountByHardhatWallet(WALLET_INDEX_0);
    let owner_2 =
      utils.getZilliqaAddressForAccountByHardhatWallet(WALLET_INDEX_2);
    let owner_3 =
      utils.getZilliqaAddressForAccountByHardhatWallet(WALLET_INDEX_3);

    if (DEBUG) {
      console.log(`Address 0 ${owner_0}`);
      console.log(`Address 2 ${owner_2}`);
      console.log(`Address 3 ${owner_3}`);
    }

    const owner_list: string[] = [owner_0, owner_2, owner_3];

    let num_of_required_signatures = 2;
    let scillaMultiSigRewardsParamContractNew = await hre.deployScillaContract(
      "MultiSigWalletRewardsParam",
      owner_list,
      num_of_required_signatures
    );

    utils.checkScillaTransactionSuccess(scillaMultiSigRewardsParamContractNew);

    const multiSigAddressNew = scillaMultiSigRewardsParamContractNew.address;
    if (DEBUG) {
      console.log(`New MultiSig address is ${multiSigAddressNew}`);
    }

    // changing the admin wallet of the underlyiong rewards param contract to the new multisig
    let acc_0 =
      utils.getZilliqaAccountForAccountByHardhatWallet(WALLET_INDEX_0);
    const rewardsContractAddress = scillaRewardsParamsContract.address;
    let result = await scillaMultiSigRewardsParamContract
      .connect(acc_0)
      .SubmitCustomUpdateAdminTransaction(
        rewardsContractAddress,
        multiSigAddressNew
      );
    const multisig_txn_id = result.receipt.event_logs[0].params[0].value;
    if (DEBUG) {
      console.log("[1] Multisig txn id is: ", multisig_txn_id);
    }

    if (DEBUG) {
      console.log("Get balance for idx 1");
    }
    let acc_1 =
      utils.getZilliqaAccountForAccountByHardhatWallet(WALLET_INDEX_1);
    let result_1 = await scillaMultiSigRewardsParamContract
      .connect(acc_1)
      .SignTransaction(multisig_txn_id);
    let result_2 = await scillaMultiSigRewardsParamContract
      .connect(acc_1)
      .ExecuteTransaction(multisig_txn_id);

    if (DEBUG) {
      console.log("Get balance for idx 2");
    }
    // claim the new admin
    let acc_2 =
      utils.getZilliqaAccountForAccountByHardhatWallet(WALLET_INDEX_2);
    if (DEBUG) {
      console.log("Get balance for idx 3");
    }
    let acc_3 =
      utils.getZilliqaAccountForAccountByHardhatWallet(WALLET_INDEX_3);
    if (DEBUG) {
      console.log("XX1");
    }
    let result_3 = await scillaMultiSigRewardsParamContractNew
      .connect(acc_2)
      .SubmitCustomClaimAdminTransaction(rewardsContractAddress);
    const multisig_txn_id_1 = result_3.receipt.event_logs[0].params[0].value;
    if (DEBUG) {
      console.log("[2] Multisig txn id is: ", multisig_txn_id_1);
    }

    if (DEBUG) {
      console.log("Signing .. ");
    }
    let result_4 = await scillaMultiSigRewardsParamContractNew
      .connect(acc_3)
      .SignTransaction(multisig_txn_id_1);
    let result_5 = await scillaMultiSigRewardsParamContractNew
      .connect(acc_3)
      .ExecuteTransaction(multisig_txn_id_1);

    expect(scillaMultiSigRewardsParamContractNew.address).to.be.properAddress;
    expect(await scillaRewardsParamsContract.contractadmin()).to.be.eq(
      multiSigAddressNew.toLowerCase()
    );
  });
});
