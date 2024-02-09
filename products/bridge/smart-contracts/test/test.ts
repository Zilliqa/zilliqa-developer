// TODO: fix tests
/*
import { anyValue } from "@nomicfoundation/hardhat-chai-matchers/withArgs";
import { expect } from "chai";
import { ethers } from "hardhat";
import {
  switchNetwork,
  setupBridge,
  dispatchMessage,
  dispatchCall,
  deliverResult,
} from "./utils";
import { Twin__factory } from "../typechain-types";
import { parseEther } from "ethers";

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

    await expect(
      await relayer1.deployTwin(salt, Twin__factory.bytecode, initCall1)
    )
      .to.emit(relayer1, "TwinDeployment")
      .withArgs(twinAddress);

    const twin1 = await ethers.getContractAt("Twin", twinAddress);

    await twin1.depositFee({
      value: parseEther("1"),
    });

    const target1 = await ethers
      .deployContract("Target")
      .then(async (c) => c.waitForDeployment());

    switchNetwork(2);

    const initCall2 = Twin__factory.createInterface().encodeFunctionData(
      "initialize",
      [await relayer2.getAddress(), chainId1]
    );

    await expect(
      await relayer2.deployTwin(salt, Twin__factory.bytecode, initCall2)
    )
      .to.emit(relayer2, "TwinDeployment")
      .withArgs(twinAddress);

    const twin2 = await ethers.getContractAt("Twin", twinAddress);

    await twin2.depositFee({
      value: parseEther("1"),
    });

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
    const readonly = false;

    switchNetwork(1);

    const tx = await twin1
      .connect(validators1[0])
      .start(await target2.getAddress(), num, readonly);
    await tx.wait();
    await expect(tx)
      .to.emit(relayer1, "Relayed")
      .withArgs(
        targetChainId,
        await twin1.getAddress(),
        await target2.getAddress(),
        target2.interface.encodeFunctionData("test", [num]),
        readonly,
        twin1.interface.getFunction("finish").selector,
        anyValue
      );

    await dispatchMessage(
      1,
      2,
      sourceChainId,
      targetChainId,
      validators1,
      relayer1,
      validators2,
      relayer2,
      validators1,
      collector,
      true,
      readonly
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
    const readonly = false;

    switchNetwork(1);

    const tx = await twin1
      .connect(validators1[0])
      .start(await target2.getAddress(), num, readonly);
    await tx.wait();
    await expect(tx)
      .to.emit(relayer1, "Relayed")
      .withArgs(
        targetChainId,
        await twin1.getAddress(),
        await target2.getAddress(),
        target2.interface.encodeFunctionData("test", [num]),
        readonly,
        twin1.interface.getFunction("finish").selector,
        anyValue
      );

    await dispatchMessage(
      1,
      2,
      sourceChainId,
      targetChainId,
      validators1,
      relayer1,
      validators2,
      relayer2,
      validators1,
      collector,
      false,
      readonly
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
      chainId1,
      chainId2,
    } = await setup(); // instead of loadFixture(setup);
    const num = 124;
    const expectedNum = num + 1;
    const sourceChainId = chainId1;
    const targetChainId = chainId2;
    const readonly = true;

    switchNetwork(1);

    const tx = await twin1
      .connect(validators1[0])
      .start(await target2.getAddress(), num, readonly);
    await tx.wait();
    await expect(tx)
      .to.emit(relayer1, "Relayed")
      .withArgs(
        targetChainId,
        await twin1.getAddress(),
        await target2.getAddress(),
        target2.interface.encodeFunctionData("test", [num]),
        readonly,
        twin1.interface.getFunction("finish").selector,
        anyValue
      );

    await dispatchMessage(
      1,
      2,
      sourceChainId,
      targetChainId,
      validators1,
      relayer1,
      validators2,
      relayer2,
      validators1,
      collector,
      true,
      readonly,
      ethers.AbiCoder.defaultAbiCoder().encode(["uint256"], [expectedNum])
    );

    const filter = twin1.filters.Succeeded;
    const logs = await twin1.queryFilter(filter);
    expect(logs).is.not.empty;

    const resNum = ethers.toNumber(logs[0].args[0]);
    expect(resNum).to.equal(expectedNum);

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
    const readonly = false;

    switchNetwork(2);

    const tx = await twin2
      .connect(validators2[0])
      .start(await target1.getAddress(), inputNum, readonly);
    await tx.wait();
    await expect(tx)
      .to.emit(relayer2, "Relayed")
      .withArgs(
        targetChainId,
        await twin2.getAddress(),
        await target1.getAddress(),
        target1.interface.encodeFunctionData("test", [inputNum]),
        readonly,
        twin2.interface.getFunction("finish").selector,
        anyValue
      );

    await dispatchMessage(
      2,
      1,
      sourceChainId,
      targetChainId,
      validators2,
      relayer2,
      validators1,
      relayer1,
      validators1,
      collector,
      true,
      readonly,
      ethers.AbiCoder.defaultAbiCoder().encode(["uint256"], [expectedNum])
    );

    const filter = twin2.filters.Succeeded;
    const logs = await twin2.queryFilter(filter);
    const resNum = logs[0].args[0];
    expect(resNum).to.equal(expectedNum);

    console.log("Incremented", inputNum, "to", ethers.toNumber(resNum));
  });

  it("should make remote call without return value", async function () {
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
    const num = 124;
    const expectedNum = num + 1;
    const sourceChainId = chainId1;
    const targetChainId = chainId2;
    const readonly = false;

    switchNetwork(1);

    const tx = await twin1
      .connect(validators1[0])
      .startNoReturn(await target2.getAddress(), num, readonly);
    await tx.wait();
    await expect(tx)
      .to.emit(relayer1, "Relayed")
      .withArgs(
        targetChainId,
        await twin1.getAddress(),
        await target2.getAddress(),
        target2.interface.encodeFunctionData("testNoReturn", [num]),
        readonly,
        twin1.interface.getFunction("finishNoReturn").selector,
        anyValue
      );

    await dispatchMessage(
      1,
      2,
      sourceChainId,
      targetChainId,
      validators1,
      relayer1,
      validators2,
      relayer2,
      validators1,
      collector,
      true,
      readonly,
      ""
    );

    const filter = twin1.filters.SucceededNoReturn;
    const logs = await twin1.queryFilter(filter);
    expect(logs).is.not.empty;

    const targetLogs = await target2.queryFilter(target2.filters.TestNoReturn);
    expect(targetLogs[0].args[0]).to.equal(expectedNum);
  });

  it("should make remote call with multiple return values", async function () {
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
    const num = 124;
    const sourceChainId = chainId1;
    const targetChainId = chainId2;
    const readonly = false;

    switchNetwork(1);

    const tx = await twin1
      .connect(validators1[0])
      .startMultipleReturn(await target2.getAddress(), num, readonly);
    await tx.wait();
    await expect(tx)
      .to.emit(relayer1, "Relayed")
      .withArgs(
        targetChainId,
        await twin1.getAddress(),
        await target2.getAddress(),
        target2.interface.encodeFunctionData("testMultipleReturn", [num]),
        readonly,
        twin1.interface.getFunction("finishMultipleReturn").selector,
        anyValue
      );

    await dispatchMessage(
      1,
      2,
      sourceChainId,
      targetChainId,
      validators1,
      relayer1,
      validators2,
      relayer2,
      validators1,
      collector,
      true,
      readonly,
      ethers.AbiCoder.defaultAbiCoder().encode(
        ["uint256", "uint256", "uint256"],
        [num + 1, num + 2, num + 3]
      )
    );

    const filter = twin1.filters.SucceededMultipleReturn;
    const logs = await twin1.queryFilter(filter);
    expect(logs).is.not.empty;
    expect(logs[0].args).to.deep.equal([
      BigInt(num + 1),
      BigInt(num + 2),
      BigInt(num + 3),
    ]);
  });

  it.only("should handle multiple remote calls requested by the same contract", async function () {
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
    const num = 124;
    const num2 = 130;
    const sourceChainId = chainId1;
    const targetChainId = chainId2;
    const readonly = false;

    switchNetwork(1);

    const tx = await twin1
      .connect(validators1[0])
      .startSum(await target2.getAddress(), num, readonly);
    await tx.wait();
    await expect(tx)
      .to.emit(relayer1, "Relayed")
      .withArgs(
        targetChainId,
        await twin1.getAddress(),
        await target2.getAddress(),
        target2.interface.encodeFunctionData("testSum", [num]),
        readonly,
        twin1.interface.getFunction("finishSum").selector,
        anyValue
      );

    await relayer2.connect(validators2[0]).warmup();

    const { dispatchTxn } = await dispatchMessage(
      1,
      2,
      sourceChainId,
      targetChainId,
      validators1,
      relayer1,
      validators2,
      relayer2,
      validators1,
      collector,
      true,
      readonly,
      ethers.AbiCoder.defaultAbiCoder().encode(["uint256"], [num + 1])
    );

    const filter = twin1.filters.Succeeded;
    const logs = await twin1.queryFilter(filter);
    expect(logs).is.not.empty;
    expect(logs[0].args).to.deep.equal([BigInt(num + 1)]);

    expect(await target2.num()).to.equal(num + 1);
    await expect(dispatchTxn)
      .to.emit(target2, "TestSum")
      .withArgs(num + 1);

    switchNetwork(1);

    await relayer2.connect(validators2[0]).refundFee();

    const tx2 = await twin1
      .connect(validators1[0])
      .startSum(await target2.getAddress(), num2, readonly);
    await tx2.wait();
    await expect(tx2)
      .to.emit(relayer1, "Relayed")
      .withArgs(
        targetChainId,
        await twin1.getAddress(),
        await target2.getAddress(),
        target2.interface.encodeFunctionData("testSum", [num2]),
        readonly,
        twin1.interface.getFunction("finishSum").selector,
        anyValue
      );

    const { dispatchTxn: dispatchTxn2 } = await dispatchMessage(
      1,
      2,
      sourceChainId,
      targetChainId,
      validators1,
      relayer1,
      validators2,
      relayer2,
      validators1,
      collector,
      true,
      readonly,
      ethers.AbiCoder.defaultAbiCoder().encode(["uint256"], [num2 + num + 1])
    );

    const logs2 = await twin1.queryFilter(twin1.filters.Succeeded);
    expect(logs2).is.not.empty;
    expect(logs2[1].args).to.deep.equal([BigInt(num2 + num + 1)]);

    expect(await target2.num()).to.equal(num2 + num + 1);
    await expect(dispatchTxn2)
      .to.emit(target2, "TestSum")
      .withArgs(num2 + num + 1);
  });

  it("should fail to replay the same remote call, only first one going through", async function () {
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
    const readonly = false;

    switchNetwork(1);

    const tx = await twin1
      .connect(validators1[0])
      .start(await target2.getAddress(), num, readonly);
    await tx.wait();
    await expect(tx)
      .to.emit(relayer1, "Relayed")
      .withArgs(
        targetChainId,
        await twin1.getAddress(),
        await target2.getAddress(),
        target2.interface.encodeFunctionData("test", [num]),
        readonly,
        twin1.interface.getFunction("finish").selector,
        anyValue
      );

    const { nonce, callSignatures } = await dispatchMessage(
      1,
      2,
      sourceChainId,
      targetChainId,
      validators1,
      relayer1,
      validators2,
      relayer2,
      validators1,
      collector,
      true,
      readonly
    );

    const filter = twin1.filters.Succeeded;
    const logs = await twin1.queryFilter(filter);
    console.log("Incremented", num, "to", ethers.toNumber(logs[0].args[0]));

    // Second attempt to dispatch would fail
    const repeatedDispatch = dispatchCall(
      validators2,
      sourceChainId,
      targetChainId,
      relayer2,
      await twin1.getAddress(),
      await target2.getAddress(),
      target2.interface.encodeFunctionData("test", [num]),
      readonly,
      twin1.interface.getFunction("finish").selector,
      nonce,
      callSignatures
    );
    expect(repeatedDispatch).to.be.revertedWithCustomError(
      relayer2,
      "AlreadyDispatched"
    );
  });

  it("should fail to replay the same delivery call, only first one going through", async function () {
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
    const readonly = false;

    switchNetwork(1);

    const tx = await twin1
      .connect(validators1[0])
      .start(await target2.getAddress(), num, readonly);
    await tx.wait();
    await expect(tx)
      .to.emit(relayer1, "Relayed")
      .withArgs(
        targetChainId,
        await twin1.getAddress(),
        await target2.getAddress(),
        target2.interface.encodeFunctionData("test", [num]),
        readonly,
        twin1.interface.getFunction("finish").selector,
        anyValue
      );

    const { nonce, resultSignatures, result } = await dispatchMessage(
      1,
      2,
      sourceChainId,
      targetChainId,
      validators1,
      relayer1,
      validators2,
      relayer2,
      validators1,
      collector,
      true,
      readonly
    );

    const filter = twin1.filters.Succeeded;
    const logs = await twin1.queryFilter(filter);
    console.log("Incremented", num, "to", ethers.toNumber(logs[0].args[0]));

    // Second attempt to deliver result would fail
    const repeatedResult = deliverResult(
      validators1,
      relayer1,
      sourceChainId,
      targetChainId,
      await twin1.getAddress(),
      target2.interface.encodeFunctionData("test", [num]),
      true,
      result,
      nonce,
      resultSignatures
    );
    expect(repeatedResult).to.be.revertedWithCustomError(
      relayer1,
      "AlreadyResumed"
    );
  });
});

*/
