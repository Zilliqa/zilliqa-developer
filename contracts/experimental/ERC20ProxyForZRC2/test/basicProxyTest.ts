import { expect } from "chai";
import { ethers } from "hardhat";
import hre from "hardhat";
import { ScillaContract } from "hardhat-scilla-plugin";
import { Account } from "@zilliqa-js/zilliqa";
import { initZilliqa } from "hardhat-scilla-plugin";

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
      zrc2Contract = await hre.interactWithScillaContract(
        "0x178ABcED2552522F131E7C89E80E622862E6c00E",
      );
      erc20Proxy = (await ethers.getContractFactory("ZRC2ERC20Proxy"))
        .connect(proxyDeployer)
        .attach("0x0C9fb168f7155Ea54aAaCafdbD9A652bd895b4a4");
    }
    console.log(`ERC20 proxy deployed at ${erc20Proxy.target}`);
    console.log(` .... proxying to ZRC2 at ${await erc20Proxy.zrc2_proxy()}`);
    console.log(
      `Proxy deployer ${proxyDeployer.address} ; token holder ${tokenHolder.address}`,
    );
  });

  it("0000 Should deploy successfully", async function () {
    expect(zrc2Contract.address).to.be.properAddress;
    expect(erc20Proxy.target).to.be.properAddress;
  });

  it("0001 Should report parameters correctly", async function () {
    expect(await erc20Proxy.decimals()).to.equal(ZRC2_DECIMALS);
    expect(await erc20Proxy.symbol()).to.equal(ZRC2_SYMBOL);
    expect(await erc20Proxy.name()).to.equal(ZRC2_NAME);
    expect(await erc20Proxy.totalSupply()).to.equal(ZRC2_SUPPLY);
    expect((await erc20Proxy.zrc2_proxy()).toLowerCase()).to.equal(
      zrc2Contract.address.toLowerCase(),
    );
  });

  it("0002 Should deal with transfers correctly", async function () {
    const AMT = 540;
    expect(await erc20Proxy.balanceOf(zrc2OwnerEVM.address)).to.equal(
      ZRC2_SUPPLY,
    );
    expect(await erc20Proxy.balanceOf(tokenHolder.address)).to.equal(0);
    await (
      await erc20Proxy.connect(zrc2OwnerEVM).transfer(tokenHolder, AMT)
    ).wait();
    expect(await erc20Proxy.balanceOf(tokenHolder.address)).to.equal(AMT);
    expect(await erc20Proxy.balanceOf(zrc2OwnerEVM.address)).to.equal(
      ZRC2_SUPPLY - AMT,
    );
    await (
      await erc20Proxy.connect(tokenHolder).transfer(zrc2OwnerEVM, AMT)
    ).wait();
  });

  it("MOVE", async function () {
    const AMT = 4;
    await expect(erc20Proxy.connect(tokenHolder).transfer(zrc2OwnerEVM, AMT))
      .not.to.be.reverted;
  });

  it("0003 should fail transfers if the user doesn't have balance", async function () {
    const AMT = 1;
    await expect(erc20Proxy.connect(tokenHolder).transfer(tokenHolder, AMT)).to
      .be.reverted;
  });

  it("0004 should fail transferFrom if the user doesn't have allowance", async function () {
    expect(
      await erc20Proxy.allowance(zrc2OwnerEVM.address, tokenHolder.address),
    ).to.equal(0);
    await expect(
      erc20Proxy
        .connect(tokenHolder)
        .transferFrom(zrc2OwnerEVM, proxyDeployer, 1),
    ).to.be.reverted;
  });

  it("0005 should succeed transferFrom if there is enough allowance", async function () {
    const AMT = 4;
    expect(await erc20Proxy.balanceOf(zrc2OwnerEVM.address)).to.equal(
      ZRC2_SUPPLY,
    );
    expect(await erc20Proxy.balanceOf(tokenHolder.address)).to.equal(0);
    expect(await erc20Proxy.balanceOf(proxyDeployer.address)).to.equal(0);
    await (
      await erc20Proxy.connect(zrc2OwnerEVM).approve(proxyDeployer, AMT)
    ).wait();
    expect(
      await erc20Proxy.allowance(zrc2OwnerEVM.address, proxyDeployer.address),
    ).to.equal(AMT);
    await (
      await erc20Proxy
        .connect(proxyDeployer)
        .transferFrom(zrc2OwnerEVM, tokenHolder, AMT)
    ).wait();
    expect(await erc20Proxy.balanceOf(zrc2OwnerEVM.address)).to.equal(
      ZRC2_SUPPLY - AMT,
    );
    expect(await erc20Proxy.balanceOf(tokenHolder.address)).to.equal(AMT);
    expect(await erc20Proxy.balanceOf(proxyDeployer.address)).to.equal(0);
  });

  it("0006 transfer 0005's tokens back", async function () {
    // Now transfer it all back.
    await (
      await erc20Proxy.connect(tokenHolder).transfer(zrc2OwnerEVM, AMT)
    ).wait();
    expect(await erc20Proxy.balanceOf(zrc2OwnerEVM.address)).to.equal(
      ZRC2_SUPPLY,
    );
    expect(await erc20Proxy.balanceOf(tokenHolder.address)).to.equal(0);
    expect(await erc20Proxy.balanceOf(proxyDeployer.address)).to.equal(0);
  });

  it("0007 correctly limits allowances to 128 bits", async function () {
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
