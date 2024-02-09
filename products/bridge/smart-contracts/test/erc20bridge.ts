// TODO: fix tests
/*
import { anyValue } from "@nomicfoundation/hardhat-chai-matchers/withArgs";
import { expect } from "chai";

import {
  switchNetwork,
  setupBridge,
  dispatchMessage as sendCrossChainMessage,
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
      chainId1,
      chainId2,
    } = await setupBridge();

    const salt = ethers.randomBytes(32);
    const bridgeInitHashCode = ethers.keccak256(ERC20Bridge__factory.bytecode);
    const bridgeAddress = ethers.getCreate2Address(
      await relayer1.getAddress(),
      salt,
      bridgeInitHashCode
    );

    switchNetwork(1);

    const initCall1 = ERC20Bridge__factory.createInterface().encodeFunctionData(
      "initialize",
      [await relayer1.getAddress(), chainId2]
    );

    await expect(
      relayer1.deployTwin(salt, ERC20Bridge__factory.bytecode, initCall1)
    )
      .to.emit(relayer1, "TwinDeployment")
      .withArgs(bridgeAddress);

    const bridge1 = await ethers.getContractAt("ERC20Bridge", bridgeAddress);
    await bridge1.depositFee({
      value: ethers.parseEther("1"),
    });

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

    const initCall2 = ERC20Bridge__factory.createInterface().encodeFunctionData(
      "initialize",
      [await relayer2.getAddress(), chainId1]
    );

    await expect(
      relayer2.deployTwin(salt, ERC20Bridge__factory.bytecode, initCall2)
    )
      .to.emit(relayer2, "TwinDeployment")
      .withArgs(bridgeAddress);
    const bridge2 = await ethers.getContractAt("ERC20Bridge", bridgeAddress);
    await bridge2.depositFee({
      value: ethers.parseEther("1"),
    });

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
      chainId1,
      chainId2,
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
      chainId1,
      chainId2,
    } = await setup();

    const value = 12;
    const sourceChainId = chainId1;
    const targetChainId = chainId2;
    switchNetwork(1);

    expect(await token1.balanceOf(await bridge1.getAddress())).to.equal(0);
    const balance1 = ethers.toNumber(await token1.balanceOf(tester1.address));

    let tx = await token1
      .connect(tester1)
      .approve(await bridge1.getAddress(), value);
    await tx.wait();
    await expect(tx)
      .to.emit(token1, "Approval")
      .withArgs(tester1.address, await bridge1.getAddress(), value);

    tx = await bridge1
      .connect(tester1)
      .bridge(await token2.getAddress(), tester1.address, value);
    await tx.wait();
    await expect(tx)
      .to.emit(relayer1, "Relayed")
      .withArgs(
        targetChainId,
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

    const balance2 = ethers.toNumber(await token2.balanceOf(tester2.address));

    await sendCrossChainMessage(
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
      false
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
      chainId1,
      chainId2,
    } = await setup();

    const value = 23;
    const sourceChainId = chainId2;
    const targetChainId = chainId1;

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
    await expect(tx)
      .to.emit(token2, "Approval")
      .withArgs(tester2.address, await bridge2.getAddress(), value);

    tx = await bridge2
      .connect(tester2)
      .exit(await token1.getAddress(), tester2.address, value);
    await tx.wait();
    await expect(tx)
      .to.emit(relayer2, "Relayed")
      .withArgs(
        targetChainId,
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

    const balance1 = ethers.toNumber(await token1.balanceOf(tester1.address));

    await sendCrossChainMessage(
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
      false
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
*/
