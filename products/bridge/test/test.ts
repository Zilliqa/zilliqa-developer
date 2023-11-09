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
} from "./utils";
import { ERC20Bridge__factory, Target__factory } from "../typechain-types";

describe("Bridge", function () {
  async function setup() {
    switchNetwork(1);

    const signers1 = await ethers.getSigners();
    const Relayer1 = await ethers.getContractFactory("CollectorRelayer");
    const relayer1 = await Relayer1.deploy();
    await relayer1.waitForDeployment();
    const Twin1 = await ethers.getContractFactory("Twin");
    const twin1 = await Twin1.deploy();
    await twin1.waitForDeployment();
    let tx = await twin1.setRelayer(await relayer1.getAddress());
    await tx.wait();
    expect(tx).not.to.be.reverted;
    const Target1 = await ethers.getContractFactory("Target");
    const target1 = await Target1.connect(signers1[1]).deploy();
    await target1.waitForDeployment();

    switchNetwork(2);

    const signers2 = await ethers.getSigners();
    const Relayer2 = await ethers.getContractFactory("Relayer");
    const relayer2 = await Relayer2.deploy();
    await relayer2.waitForDeployment();
    const Twin2 = await ethers.getContractFactory("Twin");
    const twin2 = await Twin2.deploy();
    await twin2.waitForDeployment();
    tx = await twin2.setRelayer(await relayer2.getAddress());
    await tx.wait();
    expect(tx).not.to.be.reverted;
    const Target2 = await ethers.getContractFactory("Target");
    const target2 = await Target2.connect(signers2[1]).deploy();
    await target2.waitForDeployment();

    const size = (await relayer2.getValidators()).length + 1;
    return {
      twin1,
      twin2,
      target1,
      target2,
      relayer1,
      relayer2,
      validators1: signers1.slice(1, size),
      validators2: signers2.slice(1, size),
    };
  }

  it("should increment a number in a remote call triggered on the Zilliqa network", async function () {
    const { twin1, target2, relayer1, relayer2, validators1, validators2 } =
      await setup(); // instead of loadFixture(setup);
    const num = 123;

    switchNetwork(1);

    const tx = await twin1
      .connect(validators1[0])
      .start(await target2.getAddress(), num, false);
    await tx.wait();
    await expect(tx)
      .to.emit(relayer1, "Relayed")
      .withArgs(
        await twin1.getAddress(),
        await target2.getAddress(),
        target2.interface.encodeFunctionData("test", [num]),
        false,
        ERC20Bridge__factory.createInterface().getFunction("finish").selector,
        anyValue
      );

    var { caller, callee, call, readonly, callback, nonce } = (
      await obtainCalls(validators1, relayer1)
    )[0];
    expect(readonly).to.be.false;

    var { signerIndices, signatures } = await confirmCall(
      validators1,
      relayer1,
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
      relayer2,
      caller,
      callee,
      call,
      success,
      callback,
      nonce,
      signerIndices,
      signatures
    );
    expect(success).to.be.true;

    expect(result).to.equal(
      ethers.AbiCoder.defaultAbiCoder().encode(["uint256"], [num + 1])
    );

    switchNetwork(1);

    var { signerIndices, signatures } = await confirmResult(
      validators1,
      relayer1,
      caller,
      callback,
      success,
      result,
      nonce
    );

    await deliverResult(
      validators1,
      relayer1,
      caller,
      callback,
      success,
      result,
      nonce,
      signerIndices,
      signatures
    );

    const blockNum = await ethers.provider.getBlockNumber();

    const filter = twin1.filters.Succeeded();
    const logs = await twin1.queryFilter(filter, blockNum - 100, blockNum);
    console.log("Incremented", num, "to", ethers.toNumber(logs[0].args[0]));
  });

  it("should fail to increase the number in a remote call because it is too large", async function () {
    const { twin1, target2, relayer1, relayer2, validators1, validators2 } =
      await setup(); // instead of loadFixture(setup);
    const num = 1789;

    switchNetwork(1);

    const tx = await twin1
      .connect(validators1[0])
      .start(await target2.getAddress(), num, false);
    await tx.wait();
    await expect(tx)
      .to.emit(relayer1, "Relayed")
      .withArgs(
        await twin1.getAddress(),
        await target2.getAddress(),
        Target__factory.createInterface().encodeFunctionData("test", [num]),
        false,
        ERC20Bridge__factory.createInterface().getFunction("finish").selector,
        anyValue
      );

    var { caller, callee, call, readonly, callback, nonce } = (
      await obtainCalls(validators1, relayer1)
    )[0];
    expect(readonly).to.be.false;

    var { signerIndices, signatures } = await confirmCall(
      validators1,
      relayer1,
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
      relayer2,
      caller,
      callee,
      call,
      success,
      callback,
      nonce,
      signerIndices,
      signatures
    );
    expect(success).to.be.false;

    switchNetwork(1);

    var { signerIndices, signatures } = await confirmResult(
      validators1,
      relayer1,
      caller,
      callback,
      success,
      result,
      nonce
    );

    await deliverResult(
      validators1,
      relayer1,
      caller,
      callback,
      success,
      result,
      nonce,
      signerIndices,
      signatures
    );

    const blockNum = await ethers.provider.getBlockNumber();
    const filter = twin1.filters.Failed;
    const logs = await twin1.queryFilter(filter, blockNum - 100, blockNum);
    expect(logs[0].args[0]).to.equal("Too large");

    console.log("Remote call failed with", logs[0].args[0]);
  });

  it("should increment a number in a remote view call triggered on the Zilliqa network", async function () {
    const { twin1, target2, relayer1, relayer2, validators1, validators2 } =
      await setup(); // instead of loadFixture(setup);
    const num = 124;

    switchNetwork(1);

    const tx = await twin1
      .connect(validators1[0])
      .start(await target2.getAddress(), num, true);
    await tx.wait();
    await expect(tx)
      .to.emit(relayer1, "Relayed")
      .withArgs(
        await twin1.getAddress(),
        await target2.getAddress(),
        Target__factory.createInterface().encodeFunctionData("test", [num]),
        true,
        ERC20Bridge__factory.createInterface().getFunction("finish").selector,
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

    var { signerIndices, signatures } = await confirmResult(
      validators1,
      relayer1,
      caller,
      callback,
      success,
      response,
      nonce
    );

    await deliverResult(
      validators1,
      relayer1,
      caller,
      callback,
      success,
      response,
      nonce,
      signerIndices,
      signatures
    );

    const blockNum = await ethers.provider.getBlockNumber();
    const logs = await validators1[signerIndices[0]].provider.getLogs({
      fromBlock: blockNum - 100,
      toBlock: blockNum,
      address: caller,
      topics: [ethers.id("Succeeded(uint256)")],
    });
    var res = ethers.AbiCoder.defaultAbiCoder().decode(
      ["uint256"],
      logs[0].data
    );
    console.log("Incremented", num, "to", ethers.toNumber(res[0]));
  });

  it("should increment a number in a remote call triggered on the other network", async function () {
    const { twin2, target1, relayer1, relayer2, validators1, validators2 } =
      await setup(); // instead of loadFixture(setup);
    const num = 125;

    switchNetwork(2);

    const tx = await twin2
      .connect(validators2[0])
      .start(await target1.getAddress(), num, false);
    await tx.wait();
    await expect(tx)
      .to.emit(relayer2, "Relayed")
      .withArgs(
        await twin2.getAddress(),
        await target1.getAddress(),
        Target__factory.createInterface().encodeFunctionData("test", [num]),
        false,
        ERC20Bridge__factory.createInterface().getFunction("finish").selector,
        anyValue
      );

    var { caller, callee, call, readonly, callback, nonce } = (
      await obtainCalls(validators2, relayer2)
    )[0];
    expect(readonly).to.be.false;

    switchNetwork(1);

    var { signerIndices, signatures } = await confirmCall(
      validators1,
      relayer1,
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
      relayer1,
      caller,
      callee,
      call,
      success,
      callback,
      nonce,
      signerIndices,
      signatures
    );
    expect(success).to.be.true;

    expect(result).to.equal(
      ethers.AbiCoder.defaultAbiCoder().encode(["uint256"], [num + 1])
    );

    var { signerIndices, signatures } = await confirmResult(
      validators1,
      relayer1,
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
      caller,
      callback,
      success,
      result,
      nonce,
      signerIndices,
      signatures
    );

    const blockNum = await ethers.provider.getBlockNumber();
    const filter = twin2.filters.Succeeded;
    const logs = await twin2.queryFilter(filter, blockNum - 100, blockNum);
    const res = logs[0].args[0];
    expect(res).to.equal(num + 1);

    console.log("Incremented", num, "to", ethers.toNumber(res));
  });

  // TODO: add test for remote calls without return value

  // TODO: add test for remote calls with multiple return values

  // TODO: add test for simultaneous remote calls requested by the same contract

  // TODO: add test for replayed dispatch of the same remote call

  // TODO: add test for replayed delivery of the same results
});
