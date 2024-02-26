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
const DEBUG = false;

describe(utils.TestGeneral(0, "TestParams"), function () {
  // let hello: Contract;
  let scillaRewardsParamsContract: ScillaContract;

  before(async function () {
    utils.ensureZilliqa();
    utils.setZilliqaSignerToAccountByHardhatWallet(WALLET_INDEX_0);
    let owner =
      utils.getZilliqaAddressForAccountByHardhatWallet(WALLET_INDEX_0);
    if (DEBUG) {
      console.log(`Address ${owner}`);
    }
    scillaRewardsParamsContract = await hre.deployScillaContract(
      "RewardsParams",
      owner
    );
    utils.checkScillaTransactionSuccess(scillaRewardsParamsContract);

    if (DEBUG) {
      console.log(
        `Rewards Params Contract: ${scillaRewardsParamsContract.address}`
      );
    }
    // console.log(`${JSON.stringify(scillaRewardsParamsContract)}`);
  });

  it("Contract should be deployed successfully", async function () {
    expect(scillaRewardsParamsContract.address).to.be.properAddress;
  });

  // check the default values after contract deployment

  it("Admin should be the first wallet", async function () {
    let firstAdmin =
      utils.getZilliqaAddressForAccountByHardhatWallet(WALLET_INDEX_0);
    expect(await scillaRewardsParamsContract.contractadmin()).to.be.eq(
      firstAdmin.toLowerCase()
    );
  });

  it("Percent precision should be 100", async function () {
    expect(await scillaRewardsParamsContract.percent_precision()).to.be.eq(100);
  });

  it("Default base reward should be 2000", async function () {
    expect(await scillaRewardsParamsContract.base_reward_in_percent()).to.be.eq(
      2000
    );
  });

  it("Default lookup reward should be 4000", async function () {
    expect(
      await scillaRewardsParamsContract.lookup_reward_in_percent()
    ).to.be.eq(4000);
  });

  it("Default node reward should be 4000", async function () {
    expect(await scillaRewardsParamsContract.node_reward_in_percent()).to.be.eq(
      4000
    );
  });

  it("Default coinbase reward per ds should be 204000000000000000", async function () {
    expect(await scillaRewardsParamsContract.coinbase_reward_per_ds()).to.be.eq(
      204000000000000000
    );
  });

  it("Default reward mul in millis should be 1668", async function () {
    expect(
      await scillaRewardsParamsContract.reward_each_mul_in_millis()
    ).to.be.eq(1668);
  });

  it("Default base_reward mul in millis should be 4726", async function () {
    expect(
      await scillaRewardsParamsContract.base_reward_mul_in_millis()
    ).to.be.eq(4726);
  });

  // changing the parameters by admin

  it("Admin can change base reward to 25", async function () {
    let result1 = await scillaRewardsParamsContract.ChangeBaseReward(25);
    expect(await scillaRewardsParamsContract.base_reward_in_percent()).to.be.eq(
      25
    );
  });

  it("Admin can change lookup reward to 35", async function () {
    let result1 = await scillaRewardsParamsContract.ChangeLookupReward(35);
    expect(
      await scillaRewardsParamsContract.lookup_reward_in_percent()
    ).to.be.eq(35);
  });

  it("Admin can change node reward to 25", async function () {
    let result1 = await scillaRewardsParamsContract.ChangeNodeReward(25);
    expect(await scillaRewardsParamsContract.node_reward_in_percent()).to.be.eq(
      25
    );
  });

  it("Admin can change coinbase reward per ds to 180000000000000000", async function () {
    let result1 = await scillaRewardsParamsContract.ChangeCoinbaseReward(
      180000000000000000
    );
    expect(await scillaRewardsParamsContract.coinbase_reward_per_ds()).to.be.eq(
      180000000000000000
    );
  });

  it("Admin can change reward_each_mul_in_millis to 400", async function () {
    let result1 = await scillaRewardsParamsContract.ChangeRewardEachMulInMillis(
      400
    );
    expect(
      await scillaRewardsParamsContract.reward_each_mul_in_millis()
    ).to.be.eq(400);
  });

  it("Admin can change reward_base_mul_in_millis to 392", async function () {
    let result1 = await scillaRewardsParamsContract.ChangeBaseRewardMulInMillis(
      392
    );
    expect(
      await scillaRewardsParamsContract.base_reward_mul_in_millis()
    ).to.be.eq(392);
  });

  // negative test cases
  it("A non admin wallet cannot set base reward", async function () {
    let acc_2 =
      utils.getZilliqaAccountForAccountByHardhatWallet(WALLET_INDEX_2);

    const CALL_CONTRACT_FAILED = 7;
    try {
      let result = await scillaRewardsParamsContract
        .connect(acc_2)
        .ChangeBaseReward(35);
      expect(result.receipt.success).to.be.false;
      expect(result.receipt.errors["0"].length).to.equal(1);
      expect(result.receipt.errors[0][0]).to.equal(CALL_CONTRACT_FAILED);
    } catch (error) {
      console.log("Error: ", error);
    }

    // verify that the base reward has not changed
    expect(await scillaRewardsParamsContract.base_reward_in_percent()).to.be.eq(
      25
    );
  });

  it("A non admin wallet cannot set lookup reward", async function () {
    let acc_2 =
      utils.getZilliqaAccountForAccountByHardhatWallet(WALLET_INDEX_2);

    const CALL_CONTRACT_FAILED = 7;
    try {
      let result = await scillaRewardsParamsContract
        .connect(acc_2)
        .ChangeLookupReward(50);
      expect(result.receipt.success).to.be.false;
      expect(result.receipt.errors["0"].length).to.equal(1);
      expect(result.receipt.errors[0][0]).to.equal(CALL_CONTRACT_FAILED);
    } catch (error) {
      console.log("Error: ", error);
    }

    // verify that the lookup reward has not changed
    expect(
      await scillaRewardsParamsContract.lookup_reward_in_percent()
    ).to.be.eq(35);
  });

  it("A non admin wallet cannot set coinbase reward per ds", async function () {
    let acc_2 =
      utils.getZilliqaAccountForAccountByHardhatWallet(WALLET_INDEX_2);

    const CALL_CONTRACT_FAILED = 7;
    try {
      let result = await scillaRewardsParamsContract
        .connect(acc_2)
        .ChangeCoinbaseReward(190000000000000000);
      expect(result.receipt.success).to.be.false;
      expect(result.receipt.errors["0"].length).to.equal(1);
      expect(result.receipt.errors[0][0]).to.equal(CALL_CONTRACT_FAILED);
    } catch (error) {
      console.log("Error: ", error);
    }

    // verify that the coinbase reward has not changed
    expect(await scillaRewardsParamsContract.coinbase_reward_per_ds()).to.be.eq(
      180000000000000000
    );
  });

  it("A non admin wallet cannot set a new admin", async function () {
    let acc_2 =
      utils.getZilliqaAccountForAccountByHardhatWallet(WALLET_INDEX_2);
    let newAdmin =
      utils.getZilliqaAddressForAccountByHardhatWallet(WALLET_INDEX_2);

    const CALL_CONTRACT_FAILED = 7;
    try {
      let result = await scillaRewardsParamsContract
        .connect(acc_2)
        .UpdateAdmin(newAdmin);
      expect(result.receipt.success).to.be.false;
      expect(result.receipt.errors["0"].length).to.equal(1);
      expect(result.receipt.errors[0][0]).to.equal(CALL_CONTRACT_FAILED);
    } catch (error) {
      console.log("Error: ", error);
    }

    // verify that the admin has not changed
    let firstAdmin =
      utils.getZilliqaAddressForAccountByHardhatWallet(WALLET_INDEX_0);
    expect(await scillaRewardsParamsContract.contractadmin()).to.be.eq(
      firstAdmin.toLowerCase()
    );
  });

  // test setting new admin
  it("The current admin can set a new admin", async function () {
    // utils.setZilliqaSignerToAccountByHardhatWallet(WALLET_INDEX_0);
    let acc_0 =
      utils.getZilliqaAccountForAccountByHardhatWallet(WALLET_INDEX_0);
    let newAdmin =
      utils.getZilliqaAddressForAccountByHardhatWallet(WALLET_INDEX_1);
    if (DEBUG) {
      console.log(`Admin changing to: ${newAdmin}`);
    }
    let result1 = await scillaRewardsParamsContract
      .connect(acc_0)
      .UpdateAdmin(newAdmin);

    // let us check the state of the stagingcontractadmin
    let stagingcontractadmin =
      await scillaRewardsParamsContract.stagingcontractadmin();
    expect(stagingcontractadmin.arguments[0]).to.equal(newAdmin.toLowerCase());

    // this change does not change the admin, it has to be claimed by the new admin wallet
    // so the admin should remain unchanged
    let firstAdmin =
      utils.getZilliqaAddressForAccountByHardhatWallet(WALLET_INDEX_0);
    expect(await scillaRewardsParamsContract.contractadmin()).to.be.eq(
      firstAdmin.toLowerCase()
    );
  });

  // a negative case for claiming to be a new admin
  it("A non-target wallet cannot claim new admin", async function () {
    let acc_2 =
      utils.getZilliqaAccountForAccountByHardhatWallet(WALLET_INDEX_2);

    const CALL_CONTRACT_FAILED = 7;
    try {
      let result = await scillaRewardsParamsContract
        .connect(acc_2)
        .ClaimAdmin();
      expect(result.receipt.success).to.be.false;
      expect(result.receipt.errors["0"].length).to.equal(1);
      expect(result.receipt.errors[0][0]).to.equal(CALL_CONTRACT_FAILED);
    } catch (error) {
      console.log("Error: ", error);
    }

    // there should be no change of admin
    let firstAdmin =
      utils.getZilliqaAddressForAccountByHardhatWallet(WALLET_INDEX_0);
    expect(await scillaRewardsParamsContract.contractadmin()).to.be.eq(
      firstAdmin.toLowerCase()
    );

    // there should be no change of staging admin
    let stagingcontractadmin =
      await scillaRewardsParamsContract.stagingcontractadmin();
    let targetAdmin =
      utils.getZilliqaAddressForAccountByHardhatWallet(WALLET_INDEX_1);
    expect(stagingcontractadmin.arguments[0]).to.equal(
      targetAdmin.toLowerCase()
    );
  });

  // the designated wallet should be able to claim the new admin
  it("Target new admin wallet can claim new admin", async function () {
    let acc_1 =
      utils.getZilliqaAccountForAccountByHardhatWallet(WALLET_INDEX_1);
    let result2 = await scillaRewardsParamsContract.connect(acc_1).ClaimAdmin();
    let targetAdmin =
      utils.getZilliqaAddressForAccountByHardhatWallet(WALLET_INDEX_1);

    expect(await scillaRewardsParamsContract.contractadmin()).to.be.eq(
      targetAdmin.toLowerCase()
    );
  });
});
