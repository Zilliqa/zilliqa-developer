import { expect } from "chai";
import { ethers } from "hardhat";
import hre from "hardhat";
import { ScillaContract } from "hardhat-scilla-plugin";
import { Account } from "@zilliqa-js/zilliqa";
import { initZilliqa } from "hardhat-scilla-plugin";

async function assertedTransfer(
  proxy: ZRC2ERC20Proxy,
  from: Wallet,
  to: Wallet,
  amt: number,
) {
  const receipt = await (await proxy.connect(from).transfer(to, amt)).wait();

  expect(
    receipt?.status === 1,
    `Transfer of ${amt} from ${from.address} to ${to.address} failed. Transaction status ${receipt?.status || "unknown"}`,
  );
}

/// not.to.be.reverted seems to not work here (it complains about null receipts), so ..
describe("basicTest", function () {
  let zrc2Contract: ScillaContract;
  let erc20Proxy: ZRC2ERC20Proxy;
  let zrc2Owner: Account;

  //let zrc2OwnerAddress: string;

  let zrc2OwnerEVM: Wallet;
  let proxyDeployer: Signer;
  let tokenHolder: Signer;

  const ZRC2_NAME: string = "ProxyTestToken";
  const ZRC2_SYMBOL: string = "PTT";
  const ZRC2_DECIMALS = 3;
  const ZRC2_SUPPLY = 1000;

  // 10 is too low for testnet, apparently :-(
  const CONFIRM_ATTEMPTS = 50;

  before(async function () {
    zrc2OwnerEVM = new ethers.Wallet(process.env.PRIVATE_KEY, ethers.provider);
    proxyDeployer = new ethers.Wallet(process.env.TEST_KEY_1, ethers.provider);
    tokenHolder = new ethers.Wallet(process.env.TEST_KEY_2, ethers.provider);
    console.log(`ChainId ${hre.network.config.chainId & 0x7fff}`);
    initZilliqa(
      hre.network.config.url,
      hre.network.config.chainId & 0x7fff,
      hre.network.config.accounts,
      CONFIRM_ATTEMPTS,
    );
    zrc2Owner = hre.zilliqa.getDefaultAccount();
    //zrc2OwnerAddress = zrc2Owner.address.toLowerCase();

    if (!process.env.CACHED) {
      console.log(`Deploying contracts ..`);
      zrc2Contract = await hre.deployScillaContract(
        "FungibleToken",
        zrc2OwnerEVM.address,
        ZRC2_NAME,
        ZRC2_SYMBOL,
        ZRC2_DECIMALS,
        ZRC2_SUPPLY,
      );
      console.log(
        `Sample ZRC-2 token ${zrc2Contract.address} owned by ${zrc2Owner.address}`,
      );
      const erc20Factory = (
        await ethers.getContractFactory("ZRC2ERC20Proxy")
      ).connect(proxyDeployer);
      erc20Proxy = await erc20Factory.deploy(
        zrc2Contract.address.toLowerCase(),
      );
      await erc20Proxy.waitForDeployment();
    } else {
      console.log(`Using predeployed contracts ..`);
      zrc2Contract = await hre.interactWithScillaContract(
        process.env.CACHED_ZRC2,
      );
      erc20Proxy = (await ethers.getContractFactory("ZRC2ERC20Proxy"))
        .connect(proxyDeployer)
        .attach(process.env.CACHED_ERC20);
      console.log(`proxy ${JSON.stringify(erc20Proxy)}`);
    }
    console.log(`ERC20 proxy deployed at ${erc20Proxy.target}`);
    console.log(` .... proxying to ZRC2 at ${await erc20Proxy.zrc2_proxy()}`);
    console.log(
      `Proxy deployer ${proxyDeployer.address} ; token holder ${tokenHolder.address}`,
    );
  });

  it("T0000 Should deploy successfully", async function () {
    expect(zrc2Contract.address).to.be.properAddress;
    expect(erc20Proxy.target).to.be.properAddress;
  });

  it("T0001 Should report parameters correctly", async function () {
    expect(await erc20Proxy.decimals()).to.equal(ZRC2_DECIMALS);
    expect(await erc20Proxy.symbol()).to.equal(ZRC2_SYMBOL);
    expect(await erc20Proxy.name()).to.equal(ZRC2_NAME);
    expect(await erc20Proxy.totalSupply()).to.equal(ZRC2_SUPPLY);
    expect((await erc20Proxy.zrc2_proxy()).toLowerCase()).to.equal(
      zrc2Contract.address.toLowerCase(),
    );
  });

  it("T0002 Should deal with transfers correctly", async function () {
    const AMT = 540;
    expect(await erc20Proxy.balanceOf(zrc2OwnerEVM.address)).to.equal(
      ZRC2_SUPPLY,
    );
    expect(await erc20Proxy.balanceOf(tokenHolder.address)).to.equal(0);
    await assertedTransfer(erc20Proxy, zrc2OwnerEVM, tokenHolder, AMT);
    expect(await erc20Proxy.balanceOf(tokenHolder.address)).to.equal(AMT);
    expect(await erc20Proxy.balanceOf(zrc2OwnerEVM.address)).to.equal(
      ZRC2_SUPPLY - AMT,
    );
    await assertedTransfer(erc20Proxy, tokenHolder, zrc2OwnerEVM, AMT);
  });

  // Use this test to "manually" transfer tokens back from the tokenHolder after a test fails (if it does).
  xit("MOVE", async function () {
    const AMT = 4;
    await assertedTransfer(erc20Proxy, tokenHolder, zrc2OwnerEVM, AMT);
  });

  it("T0003 should fail transfers if the user doesn't have balance", async function () {
    const AMT = 1;
    await expect(erc20Proxy.connect(tokenHolder).transfer(tokenHolder, AMT)).to
      .be.reverted;
  });

  it("T0004 should fail transferFrom if the user doesn't have allowance", async function () {
    expect(
      await erc20Proxy.allowance(zrc2OwnerEVM.address, tokenHolder.address),
    ).to.equal(0);
    await expect(
      erc20Proxy
        .connect(tokenHolder)
        .transferFrom(zrc2OwnerEVM, proxyDeployer, 1),
    ).to.be.reverted;
  });

  it("T0005 should succeed transferFrom if there is enough allowance", async function () {
    const AMT = 4;
    expect(await erc20Proxy.balanceOf(zrc2OwnerEVM.address)).to.equal(
      ZRC2_SUPPLY,
    );
    expect(await erc20Proxy.balanceOf(tokenHolder.address)).to.equal(0);
    expect(await erc20Proxy.balanceOf(proxyDeployer.address)).to.equal(0);
    {
      const receipt = await (
        await erc20Proxy.connect(zrc2OwnerEVM).approve(proxyDeployer, AMT)
      ).wait();
      expect(
        receipt?.status === 1,
        `Approval of ${AMT} from ${zrc2OwnerEVM.address} to ${proxyDeployer.address} failed`,
      );
    }
    expect(
      await erc20Proxy.allowance(zrc2OwnerEVM.address, proxyDeployer.address),
    ).to.equal(AMT);
    {
      const receipt = await (
        await erc20Proxy
          .connect(proxyDeployer)
          .transferFrom(zrc2OwnerEVM, tokenHolder, AMT)
      ).wait();
      expect(
        receipt?.status === 1,
        `transferFrom called by ${proxyDeployer.address} for ${zrc2OwnerEVM.address}  -> ${tokenHolder.address} for ${AMT} failed`,
      );
    }
    expect(await erc20Proxy.balanceOf(zrc2OwnerEVM.address)).to.equal(
      ZRC2_SUPPLY - AMT,
    );
    expect(await erc20Proxy.balanceOf(tokenHolder.address)).to.equal(AMT);
    expect(await erc20Proxy.balanceOf(proxyDeployer.address)).to.equal(0);
  });

  it("T0006 transfer 0005's tokens back", async function () {
    // Now transfer it all back.
    const AMT = 4;
    await assertedTransfer(erc20Proxy, tokenHolder, zrc2OwnerEVM, AMT);
    expect(await erc20Proxy.balanceOf(zrc2OwnerEVM.address)).to.equal(
      ZRC2_SUPPLY,
    );
    expect(await erc20Proxy.balanceOf(tokenHolder.address)).to.equal(0);
    expect(await erc20Proxy.balanceOf(proxyDeployer.address)).to.equal(0);
  });

  it("T0007 correctly limits allowances to 128 bits", async function () {
    expect(
      erc20Proxy
        .connect(zrc2OwnerEVM)
        .approve(
          tokenHolder.address,
          BigInt("170141183460469231731687303715884105729"),
        ),
    ).to.be.reverted;
  });
});
