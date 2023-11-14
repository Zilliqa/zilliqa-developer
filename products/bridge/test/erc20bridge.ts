import { anyValue } from "@nomicfoundation/hardhat-chai-matchers/withArgs";
import { expect } from "chai";

import {
  switchNetwork,
  obtainCalls,
  confirmCall,
  dispatchCall,
  confirmResult,
  deliverResult,
  setupBridge,
} from "./utils";
import { ethers } from "hardhat";
import { ERC20Bridge__factory } from "../typechain-types";

describe("ERC20Bridge", function () {
  async function setup() {
    const {
      collector,
      validators1,
      validators2,
      relayer1,
      relayer2,
      tester1,
      tester2,
      twinDeployer1,
      twinDeployer2,
    } = await setupBridge();

    const salt = ethers.randomBytes(32);
    const bridgeInitHashCode = ethers.keccak256(ERC20Bridge__factory.bytecode);
    const bridgeAddress = ethers.getCreate2Address(
      await relayer1.getAddress(),
      salt,
      bridgeInitHashCode
    );

    switchNetwork(1);

    expect(await relayer1.deployTwin(salt, ERC20Bridge__factory.bytecode))
      .to.emit(relayer1, "Deployed")
      .withArgs(bridgeAddress);

    const bridge1 = await ethers.getContractAt("ERC20Bridge", bridgeAddress);

    const token1 = await ethers
      .deployContract("MyToken", [bridgeAddress], twinDeployer1)
      .then(async (c) => c.waitForDeployment());
    await token1
      .connect(twinDeployer1)
      .transfer(tester1.address, 100)
      .then(async (tx) => {
        await tx.wait();
        expect(tx).not.to.be.reverted;
      });

    switchNetwork(2);

    expect(await relayer2.deployTwin(salt, ERC20Bridge__factory.bytecode))
      .to.emit(relayer2, "Deployed")
      .withArgs(bridgeAddress);
    const bridge2 = await ethers.getContractAt("ERC20Bridge", bridgeAddress);

    // Mimic CREATE2 deployments with signers that have matching nonce (twinDeployer1 & twinDeployer2)
    const token2 = await ethers
      .deployContract("MyToken", [await bridge2.getAddress()], twinDeployer2)
      .then(async (c) => c.waitForDeployment());

    await token2
      .connect(twinDeployer2)
      .transfer(tester2.address, 100)
      .then(async (tx) => {
        await tx.wait();
        expect(tx).not.to.be.reverted;
      });

    return {
      collector,
      bridge1,
      bridge2,
      token1,
      token2,
      relayer1,
      relayer2,
      tester1,
      tester2,
      validators1,
      validators2,
    };
  }

  it("should bridge some value in a remote call triggered on the Zilliqa network", async function () {
    const {
      bridge1,
      token1,
      token2,
      relayer1,
      relayer2,
      tester1,
      tester2,
      validators1,
      validators2,
      collector,
    } = await setup();

    const value = 12;

    switchNetwork(1);

    expect(await token1.balanceOf(await bridge1.getAddress())).to.equal(0);
    const balance1 = ethers.toNumber(await token1.balanceOf(tester1.address));

    let tx = await token1
      .connect(tester1)
      .approve(await bridge1.getAddress(), value);
    await tx.wait();
    expect(tx)
      .to.emit(token1, "Approval")
      .withArgs(tester1.address, await bridge1.getAddress(), value);

    tx = await bridge1
      .connect(tester1)
      .bridge(await token2.getAddress(), tester1.address, value);
    await tx.wait();
    expect(tx)
      .to.emit(relayer1, "Relayed")
      .withArgs(
        await bridge1.getAddress(),
        await token2.getAddress(),
        token2.interface.encodeFunctionData("mint", [tester1.address, value]),
        false,
        bridge1.interface.getFunction("finish").selector,
        anyValue
      )
      .to.emit(bridge1, "Started")
      .withArgs(await token2.getAddress(), tester1.address, value);

    expect(await token1.balanceOf(await bridge1.getAddress())).to.equal(value);
    expect(await token1.balanceOf(tester1.address)).to.equal(balance1 - value);
    console.log(
      tester1.address,
      "balance changed from",
      balance1,
      "to",
      balance1 - value
    );

    var { caller, callee, call, readonly, callback, nonce } = (
      await obtainCalls(validators1, relayer1)
    )[0];
    expect(readonly).to.be.false;

    var signatures = await confirmCall(
      validators1,
      collector,
      caller,
      callee,
      call,
      readonly,
      callback,
      nonce
    );

    switchNetwork(2);

    const balance2 = ethers.toNumber(await token2.balanceOf(tester2.address));

    success = anyValue;
    var { success, result } = await dispatchCall(
      validators2,
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

    switchNetwork(1);

    var signatures = await confirmResult(
      validators1,
      collector,
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
      signatures
    );

    const filters = bridge1.filters.Succeeded;
    const logs = await bridge1.queryFilter(filters);
    expect(logs).is.not.empty;

    expect(await token2.balanceOf(tester2.address)).to.equal(balance2 + value);
    console.log(
      tester2.address,
      "balance changed from",
      balance2,
      "to",
      balance2 + value
    );
  });

  it("should bridge back some value in a remote call triggered on the other network", async function () {
    const {
      collector,
      bridge2,
      token1,
      token2,
      relayer1,
      relayer2,
      tester1,
      tester2,
      validators1,
      validators2,
    } = await setup();

    const value = 23;

    switchNetwork(1);

    // simulate bridging of the amount (see previous test) by simply sending it to bridge1
    // Ensures the bridge has enough funds to back
    expect(await token1.balanceOf(await bridge2.getAddress())).to.equal(0);

    let tx = await token1
      .connect(tester1)
      .transfer(bridge2.getAddress(), value);
    await tx.wait();
    expect(tx).not.to.be.reverted;
    expect(await token1.balanceOf(await bridge2.getAddress())).to.equal(value);

    switchNetwork(2);

    expect(await token2.balanceOf(await bridge2.getAddress())).to.equal(0);
    const balance2 = ethers.toNumber(await token2.balanceOf(tester2.address));

    tx = await token2
      .connect(tester2)
      .approve(await bridge2.getAddress(), value);
    await tx.wait();
    expect(tx)
      .to.emit(token2, "Approval")
      .withArgs(tester2.address, await bridge2.getAddress(), value);

    tx = await bridge2
      .connect(tester2)
      .exit(await token1.getAddress(), tester2.address, value);
    await tx.wait();
    expect(tx)
      .to.emit(relayer2, "Relayed")
      .withArgs(
        await bridge2.getAddress(),
        await token1.getAddress(),
        token1.interface.encodeFunctionData("transfer", [
          tester2.address,
          value,
        ]),
        false,
        bridge2.interface.getFunction("finish").selector,
        anyValue
      )
      .to.emit(bridge2, "Started")
      .withArgs(await token2.getAddress(), tester1.address, value);

    expect(await token2.balanceOf(await bridge2.getAddress())).to.equal(0);
    expect(await token2.balanceOf(tester2.address)).to.equal(balance2 - value);
    console.log(
      tester2.address,
      "balance changed from",
      balance2,
      "to",
      balance2 - value
    );

    var { caller, callee, call, readonly, callback, nonce } = (
      await obtainCalls(validators2, relayer2)
    )[0];
    expect(readonly).to.be.false;

    switchNetwork(1);

    var signatures = await confirmCall(
      validators1,
      collector,
      caller,
      callee,
      call,
      readonly,
      callback,
      nonce
    );

    const balance1 = ethers.toNumber(await token1.balanceOf(tester1.address));

    success = anyValue;
    var { success, result } = await dispatchCall(
      validators1,
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
      ethers.AbiCoder.defaultAbiCoder().encode(["bool"], [true])
    );

    var signatures = await confirmResult(
      validators1,
      collector,
      caller,
      callback,
      success,
      result,
      nonce
    );

    await deliverResult(
      validators2,
      relayer2,
      caller,
      callback,
      success,
      result,
      nonce,
      signatures
    );

    const filter = bridge2.filters.Succeeded;
    const logs = await bridge2.queryFilter(filter);
    expect(logs).is.not.empty;

    expect(await token1.balanceOf(tester1.address)).to.equal(balance1 + value);
    expect(await token2.balanceOf(bridge2.getAddress())).to.equal(0);
    expect(await token2.balanceOf(tester2.getAddress())).to.equal(
      balance2 - value
    );
    console.log(
      tester1.address,
      "balance changed from",
      balance1,
      "to",
      balance1 + value
    );
  });
});
