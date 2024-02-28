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
const DEBUG = false;

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

describe(utils.TestGeneral(0, "TestMultiSigRewardsParam"), function () {
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

    const owner_list: string[] = [owner_0, owner_1, owner_2];
    if (DEBUG) {
      console.log(owner_list);
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
    if (DEBUG) {
      console.log(`All owners have non-zero funds: good!`);
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

  it("Contract should be deployed successfully", async function () {
    expect(scillaMultiSigRewardsParamContract.address).to.be.properAddress;
    expect(scillaRewardsParamsContract.address).to.be.properAddress;
  });

  it("Admin of the rewards param contract should be the multi sig wallet", async function () {
    const multiSigAddress = scillaMultiSigRewardsParamContract.address;
    expect(await scillaRewardsParamsContract.contractadmin()).to.be.eq(
      multiSigAddress.toLowerCase()
    );
  });

  it("Multi sig wallet can change base reward to 25", async function () {
    const rewardsContractAddress = scillaRewardsParamsContract.address;
    let acc_0 =
      utils.getZilliqaAccountForAccountByHardhatWallet(WALLET_INDEX_0);

    let result = await scillaMultiSigRewardsParamContract
      .connect(acc_0)
      .SubmitCustomChangeBaseRewardTransaction(rewardsContractAddress, 25);
    const multisig_txn_id = result.receipt.event_logs[0].params[0].value;
    if (DEBUG) {
      console.log("Multisig txn id is: ", multisig_txn_id);
    }

    let acc_1 =
      utils.getZilliqaAccountForAccountByHardhatWallet(WALLET_INDEX_1);
    let result_1 = await scillaMultiSigRewardsParamContract
      .connect(acc_1)
      .SignTransaction(multisig_txn_id);
    let result_2 = await scillaMultiSigRewardsParamContract
      .connect(acc_1)
      .ExecuteTransaction(multisig_txn_id);

    expect(await scillaRewardsParamsContract.base_reward_in_percent()).to.be.eq(
      25
    );
  });

  it("Multi sig wallet can change lookup reward to 35", async function () {
    const rewardsContractAddress = scillaRewardsParamsContract.address;
    let acc_0 =
      utils.getZilliqaAccountForAccountByHardhatWallet(WALLET_INDEX_0);

    let result = await scillaMultiSigRewardsParamContract
      .connect(acc_0)
      .SubmitCustomChangeLookupRewardTransaction(rewardsContractAddress, 35);
    const multisig_txn_id = result.receipt.event_logs[0].params[0].value;
    if (DEBUG) {
      console.log("Multisig txn id is: ", multisig_txn_id);
    }

    let acc_1 =
      utils.getZilliqaAccountForAccountByHardhatWallet(WALLET_INDEX_1);
    let result_1 = await scillaMultiSigRewardsParamContract
      .connect(acc_1)
      .SignTransaction(multisig_txn_id);
    let result_2 = await scillaMultiSigRewardsParamContract
      .connect(acc_1)
      .ExecuteTransaction(multisig_txn_id);

    expect(
      await scillaRewardsParamsContract.lookup_reward_in_percent()
    ).to.be.eq(35);
  });

  it("Multi sig wallet can change node reward to 25", async function () {
    const rewardsContractAddress = scillaRewardsParamsContract.address;
    let acc_0 =
      utils.getZilliqaAccountForAccountByHardhatWallet(WALLET_INDEX_0);

    let result = await scillaMultiSigRewardsParamContract
      .connect(acc_0)
      .SubmitCustomChangeNodeRewardTransaction(rewardsContractAddress, 25);
    const multisig_txn_id = result.receipt.event_logs[0].params[0].value;
    if (DEBUG) {
      console.log("Multisig txn id is: ", multisig_txn_id);
    }

    let acc_1 =
      utils.getZilliqaAccountForAccountByHardhatWallet(WALLET_INDEX_1);
    let result_1 = await scillaMultiSigRewardsParamContract
      .connect(acc_1)
      .SignTransaction(multisig_txn_id);
    let result_2 = await scillaMultiSigRewardsParamContract
      .connect(acc_1)
      .ExecuteTransaction(multisig_txn_id);

    expect(await scillaRewardsParamsContract.node_reward_in_percent()).to.be.eq(
      25
    );
  });

  it("Multi sig wallet can change RewardEachMulInMillis to 1234", async function () {
    const rewardsContractAddress = scillaRewardsParamsContract.address;
    let acc_0 =
      utils.getZilliqaAccountForAccountByHardhatWallet(WALLET_INDEX_0);

    let result = await scillaMultiSigRewardsParamContract
      .connect(acc_0)
      .SubmitCustomChangeRewardEachMulInMillisTransaction(
        rewardsContractAddress,
        1234
      );
    const multisig_txn_id = result.receipt.event_logs[0].params[0].value;
    if (DEBUG) {
      console.log("Multisig txn id is: ", multisig_txn_id);
    }

    let acc_1 =
      utils.getZilliqaAccountForAccountByHardhatWallet(WALLET_INDEX_1);
    let result_1 = await scillaMultiSigRewardsParamContract
      .connect(acc_1)
      .SignTransaction(multisig_txn_id);
    let result_2 = await scillaMultiSigRewardsParamContract
      .connect(acc_1)
      .ExecuteTransaction(multisig_txn_id);

    expect(
      await scillaRewardsParamsContract.reward_each_mul_in_millis()
    ).to.be.eq(1234);
  });

  it("Multi sig wallet can change BaseRewardMulInMillis to 4567", async function () {
    const rewardsContractAddress = scillaRewardsParamsContract.address;
    let acc_0 =
      utils.getZilliqaAccountForAccountByHardhatWallet(WALLET_INDEX_0);

    let result = await scillaMultiSigRewardsParamContract
      .connect(acc_0)
      .SubmitCustomChangeBaseRewardMulInMillisTransaction(
        rewardsContractAddress,
        4567
      );
    const multisig_txn_id = result.receipt.event_logs[0].params[0].value;
    if (DEBUG) {
      console.log("Multisig txn id is: ", multisig_txn_id);
    }

    let acc_1 =
      utils.getZilliqaAccountForAccountByHardhatWallet(WALLET_INDEX_1);
    let result_1 = await scillaMultiSigRewardsParamContract
      .connect(acc_1)
      .SignTransaction(multisig_txn_id);
    let result_2 = await scillaMultiSigRewardsParamContract
      .connect(acc_1)
      .ExecuteTransaction(multisig_txn_id);

    expect(
      await scillaRewardsParamsContract.base_reward_mul_in_millis()
    ).to.be.eq(4567);
  });

  it("Multi sig wallet can change coinbase reward per ds to 180000000000000000", async function () {
    const rewardsContractAddress = scillaRewardsParamsContract.address;
    let acc_0 =
      utils.getZilliqaAccountForAccountByHardhatWallet(WALLET_INDEX_0);

    let result = await scillaMultiSigRewardsParamContract
      .connect(acc_0)
      .SubmitCustomChangeCoinbaseRewardTransaction(
        rewardsContractAddress,
        180000000000000000
      );
    const multisig_txn_id = result.receipt.event_logs[0].params[0].value;
    if (DEBUG) {
      console.log("Multisig txn id is: ", multisig_txn_id);
    }

    let acc_1 =
      utils.getZilliqaAccountForAccountByHardhatWallet(WALLET_INDEX_1);
    let result_1 = await scillaMultiSigRewardsParamContract
      .connect(acc_1)
      .SignTransaction(multisig_txn_id);
    let result_2 = await scillaMultiSigRewardsParamContract
      .connect(acc_1)
      .ExecuteTransaction(multisig_txn_id);

    expect(await scillaRewardsParamsContract.coinbase_reward_per_ds()).to.be.eq(
      180000000000000000
    );
  });

  // negative test cases
  it("A wallet that is not in the multi sig wallet list cannot submit transaction", async function () {
    const rewardsContractAddress = scillaRewardsParamsContract.address;
    let acc_3 =
      utils.getZilliqaAccountForAccountByHardhatWallet(WALLET_INDEX_3);

    const CALL_CONTRACT_FAILED = 7;
    try {
      let result = await scillaMultiSigRewardsParamContract
        .connect(acc_3)
        .SubmitCustomChangeBaseRewardTransaction(rewardsContractAddress, 25);
      expect(result.receipt.success).to.be.false;
      expect(result.receipt.errors["0"].length).to.equal(1);
      expect(result.receipt.errors[0][0]).to.equal(CALL_CONTRACT_FAILED);
    } catch (error) {
      console.log("Error: ", error);
    }
  });

  it("A wallet that is not in the multi sig wallet list cannot sign transaction", async function () {
    const rewardsContractAddress = scillaRewardsParamsContract.address;

    let acc_0 =
      utils.getZilliqaAccountForAccountByHardhatWallet(WALLET_INDEX_0);

    let result = await scillaMultiSigRewardsParamContract
      .connect(acc_0)
      .SubmitCustomChangeBaseRewardTransaction(rewardsContractAddress, 25);
    expect(result.receipt.success).to.be.true;
    const multisig_txn_id = result.receipt.event_logs[0].params[0].value;
    if (DEBUG) {
      console.log("Multisig txn id is: ", multisig_txn_id);
    }

    let acc_3 =
      utils.getZilliqaAccountForAccountByHardhatWallet(WALLET_INDEX_3);

    const CALL_CONTRACT_FAILED = 7;
    try {
      let result_1 = await scillaMultiSigRewardsParamContract
        .connect(acc_3)
        .SignTransaction(multisig_txn_id);

      expect(result_1.receipt.success).to.be.false;
      expect(result_1.receipt.errors["0"].length).to.equal(1);
      expect(result_1.receipt.errors[0][0]).to.equal(CALL_CONTRACT_FAILED);
    } catch (error) {
      console.log("Error: ", error);
    }
  });

  it("A wallet that is not in the multi sig wallet list cannot execute transaction", async function () {
    const rewardsContractAddress = scillaRewardsParamsContract.address;

    let acc_0 =
      utils.getZilliqaAccountForAccountByHardhatWallet(WALLET_INDEX_0);

    let result = await scillaMultiSigRewardsParamContract
      .connect(acc_0)
      .SubmitCustomChangeBaseRewardTransaction(rewardsContractAddress, 25);
    expect(result.receipt.success).to.be.true;
    const multisig_txn_id = result.receipt.event_logs[0].params[0].value;
    if (DEBUG) {
      console.log("Multisig txn id is: ", multisig_txn_id);
    }

    let acc_1 =
      utils.getZilliqaAccountForAccountByHardhatWallet(WALLET_INDEX_1);
    let result_1 = await scillaMultiSigRewardsParamContract
      .connect(acc_1)
      .SignTransaction(multisig_txn_id);
    expect(result_1.receipt.success).to.be.true;

    let acc_3 =
      utils.getZilliqaAccountForAccountByHardhatWallet(WALLET_INDEX_3);

    const CALL_CONTRACT_FAILED = 7;
    try {
      let result_2 = await scillaMultiSigRewardsParamContract
        .connect(acc_3)
        .ExecuteTransaction(multisig_txn_id);

      expect(result_2.receipt.success).to.be.false;
      expect(result_2.receipt.errors["0"].length).to.equal(1);
      expect(result_2.receipt.errors[0][0]).to.equal(CALL_CONTRACT_FAILED);
    } catch (error) {
      if (DEBUG) {
        console.log("Error: ", error);
      }
    }
  });
});
