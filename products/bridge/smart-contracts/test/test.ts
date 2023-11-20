import { anyValue } from "@nomicfoundation/hardhat-chai-matchers/withArgs";
import { expect } from "chai";
import { ethers } from "hardhat";
import {
  switchNetwork,
  obtainCalls,
  confirmCall,
  dispatchCall,
  confirmResult,
  deliverResult,
  queryCall,
  setupBridge,
} from "./utils";
import { Twin__factory } from "../typechain-types";

describe("Bridge", function () {
  async function setup() {
    const {
      collector,
      relayer1,
      relayer2,
      validators1,
      validators2,
      chainId1,
      chainId2,
    } = await setupBridge();

    switchNetwork(1);

    const salt = ethers.randomBytes(32);
    const twinInitHashCode = ethers.keccak256(Twin__factory.bytecode);
    const twinAddress = ethers.getCreate2Address(
      await relayer1.getAddress(),
      salt,
      twinInitHashCode
    );

    const initCall1 = Twin__factory.createInterface().encodeFunctionData(
      "initialize",
      [await relayer1.getAddress(), chainId2]
    );

    expect(await relayer1.deployTwin(salt, Twin__factory.bytecode, initCall1))
      .to.emit(relayer1, "Deployed")
      .withArgs(twinAddress);

    const twin1 = await ethers.getContractAt("Twin", twinAddress);

    const target1 = await ethers
      .deployContract("Target")
      .then(async (c) => c.waitForDeployment());

    switchNetwork(2);

    const initCall2 = Twin__factory.createInterface().encodeFunctionData(
      "initialize",
      [await relayer2.getAddress(), chainId1]
    );

    expect(await relayer2.deployTwin(salt, Twin__factory.bytecode, initCall2))
      .to.emit(relayer2, "Deployed")
      .withArgs(twinAddress);

    const twin2 = await ethers.getContractAt("Twin", twinAddress);

    const target2 = await ethers
      .deployContract("Target")
      .then(async (c) => c.waitForDeployment());

    return {
      collector,
      twin1,
      twin2,
      target1,
      target2,
      relayer1,
      relayer2,
      validators1,
      validators2,
      chainId1,
      chainId2,
    };
  }

  it("should increment a number in a remote call triggered on the Zilliqa network", async function () {
    const {
      twin1,
      target2,
      relayer1,
      relayer2,
      validators1,
      validators2,
      collector,
      chainId1,
      chainId2,
    } = await setup(); // instead of loadFixture(setup);
    const num = 123;
    const sourceChainId = chainId1;
    const targetChainId = chainId2;

    switchNetwork(1);

    const tx = await twin1
      .connect(validators1[0])
      .start(await target2.getAddress(), num, false);
    await tx.wait();
    await expect(tx)
      .to.emit(relayer1, "Relayed")
      .withArgs(
        targetChainId,
        await twin1.getAddress(),
        await target2.getAddress(),
        target2.interface.encodeFunctionData("test", [num]),
        false,
        twin1.interface.getFunction("finish").selector,
        anyValue
      );

    var { caller, callee, call, readonly, callback, nonce } = (
      await obtainCalls(validators1, relayer1)
    )[0];
    expect(readonly).to.be.false;

    var signatures = await confirmCall(
      validators1,
      collector,
      sourceChainId,
      caller,
      callee,
      call,
      readonly,
      callback,
      nonce
    );

    switchNetwork(2);

    success = true; // we expect the call to succeed
    var { success, result } = await dispatchCall(
      validators2,
      sourceChainId,
      relayer2,
      caller,
      callee,
      call,
      success,
      callback,
      nonce,
      signatures
    );
    expect(success).to.be.true;

    expect(result).to.equal(
      ethers.AbiCoder.defaultAbiCoder().encode(["uint256"], [num + 1])
    );

    switchNetwork(1);

    var signatures = await confirmResult(
      validators1,
      collector,
      targetChainId,
      caller,
      callback,
      success,
      result,
      nonce
    );

    await deliverResult(
      validators1,
      relayer1,
      targetChainId,
      caller,
      callback,
      success,
      result,
      nonce,
      signatures
    );

    const filter = twin1.filters.Succeeded;
    const logs = await twin1.queryFilter(filter);
    console.log("Incremented", num, "to", ethers.toNumber(logs[0].args[0]));
  });

  it("should fail to increase the number in a remote call because it is too large", async function () {
    const {
      collector,
      twin1,
      target2,
      relayer1,
      relayer2,
      validators1,
      validators2,
      chainId1,
      chainId2,
    } = await setup(); // instead of loadFixture(setup);
    const num = 1789;
    const sourceChainId = chainId1;
    const targetChainId = chainId2;

    switchNetwork(1);

    const tx = await twin1
      .connect(validators1[0])
      .start(await target2.getAddress(), num, false);
    await tx.wait();
    await expect(tx)
      .to.emit(relayer1, "Relayed")
      .withArgs(
        targetChainId,
        await twin1.getAddress(),
        await target2.getAddress(),
        target2.interface.encodeFunctionData("test", [num]),
        false,
        twin1.interface.getFunction("finish").selector,
        anyValue
      );

    var { caller, callee, call, readonly, callback, nonce } = (
      await obtainCalls(validators1, relayer1)
    )[0];
    expect(readonly).to.be.false;

    var signatures = await confirmCall(
      validators1,
      collector,
      sourceChainId,
      caller,
      callee,
      call,
      readonly,
      callback,
      nonce
    );

    switchNetwork(2);

    success = false; // we expect the call to fail
    var { success, result } = await dispatchCall(
      validators2,
      sourceChainId,
      relayer2,
      caller,
      callee,
      call,
      success,
      callback,
      nonce,
      signatures
    );
    expect(success).to.be.false;

    switchNetwork(1);

    var signatures = await confirmResult(
      validators1,
      collector,
      targetChainId,
      caller,
      callback,
      success,
      result,
      nonce
    );

    await deliverResult(
      validators1,
      relayer1,
      targetChainId,
      caller,
      callback,
      success,
      result,
      nonce,
      signatures
    );

    const filter = twin1.filters.Failed;
    const logs = await twin1.queryFilter(filter);
    expect(logs[0].args[0]).to.equal("Too large");

    console.log("Remote call failed with", logs[0].args[0]);
  });

  it("should increment a number in a remote view call triggered on the Zilliqa network", async function () {
    const {
      collector,
      twin1,
      target2,
      relayer1,
      relayer2,
      validators1,
      validators2,
      chainId2,
    } = await setup(); // instead of loadFixture(setup);
    const num = 124;
    const targetChainId = chainId2;

    switchNetwork(1);

    const tx = await twin1
      .connect(validators1[0])
      .start(await target2.getAddress(), num, true);
    await tx.wait();
    await expect(tx)
      .to.emit(relayer1, "Relayed")
      .withArgs(
        targetChainId,
        await twin1.getAddress(),
        await target2.getAddress(),
        target2.interface.encodeFunctionData("test", [num]),
        true,
        twin1.interface.getFunction("finish").selector,
        anyValue
      );

    var { caller, callee, call, readonly, callback, nonce } = (
      await obtainCalls(validators1, relayer1)
    )[0];
    expect(readonly).to.be.true;

    switchNetwork(2);

    var { success, response } = await queryCall(
      validators2,
      relayer2,
      caller,
      callee,
      call
    );
    expect(success).to.be.true;

    expect(response).to.equal(
      ethers.AbiCoder.defaultAbiCoder().encode(["uint256"], [num + 1])
    );

    switchNetwork(1);

    var signatures = await confirmResult(
      validators1,
      collector,
      targetChainId,
      caller,
      callback,
      success,
      response,
      nonce
    );

    await deliverResult(
      validators1,
      relayer1,
      targetChainId,
      caller,
      callback,
      success,
      response,
      nonce,
      signatures
    );

    const filter = twin1.filters.Succeeded;
    const logs = await twin1.queryFilter(filter);
    expect(logs).is.not.empty;

    const resNum = ethers.toNumber(logs[0].args[0]);
    expect(resNum).to.equal(num + 1);

    console.log("Incremented", num, "to", resNum);
  });

  it("should increment a number in a remote call triggered on the other network", async function () {
    const {
      collector,
      twin2,
      target1,
      relayer1,
      relayer2,
      validators1,
      validators2,
      chainId1,
      chainId2,
    } = await setup(); // instead of loadFixture(setup);
    const inputNum = 125;
    const expectedNum = inputNum + 1;
    const sourceChainId = chainId2;
    const targetChainId = chainId1;

    switchNetwork(2);

    const tx = await twin2
      .connect(validators2[0])
      .start(await target1.getAddress(), inputNum, false);
    await tx.wait();
    await expect(tx)
      .to.emit(relayer2, "Relayed")
      .withArgs(
        targetChainId,
        await twin2.getAddress(),
        await target1.getAddress(),
        target1.interface.encodeFunctionData("test", [inputNum]),
        false,
        twin2.interface.getFunction("finish").selector,
        anyValue
      );

    var { caller, callee, call, readonly, callback, nonce } = (
      await obtainCalls(validators2, relayer2)
    )[0];
    expect(readonly).to.be.false;

    switchNetwork(1);

    var signatures = await confirmCall(
      validators1,
      collector,
      sourceChainId,
      caller,
      callee,
      call,
      readonly,
      callback,
      nonce
    );

    success = true; // we expect the call to succeed
    var { success, result } = await dispatchCall(
      validators1,
      sourceChainId,
      relayer1,
      caller,
      callee,
      call,
      success,
      callback,
      nonce,
      signatures
    );
    expect(success).to.be.true;

    expect(result).to.equal(
      ethers.AbiCoder.defaultAbiCoder().encode(["uint256"], [expectedNum])
    );

    var signatures = await confirmResult(
      validators1,
      collector,
      targetChainId,
      caller,
      callback,
      success,
      result,
      nonce
    );

    switchNetwork(2);

    await deliverResult(
      validators2,
      relayer2,
      targetChainId,
      caller,
      callback,
      success,
      result,
      nonce,
      signatures
    );

    const filter = twin2.filters.Succeeded;
    const logs = await twin2.queryFilter(filter);
    const resNum = logs[0].args[0];
    expect(resNum).to.equal(expectedNum);

    console.log("Incremented", inputNum, "to", ethers.toNumber(resNum));
  });

  // TODO: add test for remote calls without return value

  // TODO: add test for remote calls with multiple return values

  // TODO: add test for simultaneous remote calls requested by the same contract

  // TODO: add test for replayed dispatch of the same remote call

  // TODO: add test for replayed delivery of the same results
});
