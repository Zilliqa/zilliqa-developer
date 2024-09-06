import { expect } from "chai";
import { ethers } from "hardhat";
import hre from "hardhat";
import { ScillaContract } from "hardhat-scilla-plugin";
import { Account } from "@zilliqa-js/zilliqa";
import { initZilliqa } from "hardhat-scilla-plugin";

/// not.to.be.reverted seems to not work here (it complains about null receipts), so ..
describe("burnTest", function () {
  let zrc2Contract: ScillaContract;
  let erc20Proxy: Contract;
  let zrc2Owner: Account;

  //let zrc2OwnerAddress: string;

  let zrc2OwnerEVM: Signer;
  let proxyDeployer: Signer;
  let tokenHolder: Signer;

  const ZRC2_NAME: string = "BPTestToken";
  const ZRC2_SYMBOL: string = "PBT";
  const ZRC2_DECIMALS = 4;
  const ZRC2_SUPPLY = 2000;

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
        "FungibleTokenBurnable",
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
        await ethers.getContractFactory("ZRC2ERC20ProxyBurnable")
      ).connect(proxyDeployer);
      erc20Proxy = await erc20Factory.deploy(
        zrc2Contract.address.toLowerCase(),
      );
      await erc20Proxy.waitForDeployment();
    } else {
      zrc2Contract = await hre.interactWithScillaContract(
        "0x5aDEf0BbBEbD54831E2300Aa58f58d18f5E8E66F",
      );
      erc20Proxy = (await ethers.getContractFactory("ZRC2ERC20ProxyBurnable"))
        .connect(proxyDeployer)
        .attach("0x7C670086208545f8D49edeef2bc351dD0667B932");
    }
    console.log(`ERC20 proxy deployed at ${erc20Proxy.target}`);
    console.log(` .... proxying to ZRC2 at ${await erc20Proxy.zrc2_proxy()}`);
    console.log(
      `Proxy deployer ${proxyDeployer.address} ; token holder ${tokenHolder.address}`,
    );
  });

  it("B0000 Should deploy successfully", async function () {
    expect(zrc2Contract.address).to.be.properAddress;
    expect(erc20Proxy.target).to.be.properAddress;
  });

  it("B0001 Should be able to burn tokens that you own", async function () {
    const balance = await erc20Proxy.balanceOf(zrc2OwnerEVM.address);
    const supply = await erc20Proxy.totalSupply();
    const AMT = 1;
    expect(balance).to.be.at.least(AMT);
    const receipt = await (
      await erc20Proxy.connect(zrc2OwnerEVM).burn(AMT)
    ).wait();
    expect(
      receipt?.status === 1,
      `Burn of ${AMT} from ${zrc2OwnerEVM.address} failed`,
    );
    expect(await erc20Proxy.balanceOf(zrc2OwnerEVM.address)).to.equal(
      balance - BigInt(AMT),
    );
    expect(await erc20Proxy.totalSupply()).to.equal(supply - BigInt(AMT));
  });

  it("B0002 Should be able to burn tokens from an allowance", async function () {
    const balance = await erc20Proxy.balanceOf(tokenHolder.address);
    const holderBalance = await erc20Proxy.balanceOf(zrc2OwnerEVM.address);
    const supply = await erc20Proxy.totalSupply();
    const AMT = 1;
    expect(balance).to.equal(0);
    {
      const receipt = await (
        await erc20Proxy.connect(zrc2OwnerEVM).approve(tokenHolder.address, AMT)
      ).wait();
      expect(
        receipt?.status === 1,
        `Approval of ${AMT} from ${zrc2OwnerEVM.address} for ${tokenHolder.address} failed`,
      );
    }
    {
      const receipt = await (
        await erc20Proxy
          .connect(tokenHolder)
          .burnFrom(zrc2OwnerEVM.address, AMT)
      ).wait();
      expect(
        receipt?.status === 1,
        `BurnFrom for ${AMT} from ${zrc2OwnerEVM.address} issued by ${tokenHolder.address} failed`,
      );
    }
    const holderBalanceAfter = await erc20Proxy.balanceOf(zrc2OwnerEVM.address);
    expect(holderBalance - holderBalanceAfter).to.equal(BigInt(AMT));
    const supplyAfter = await erc20Proxy.totalSupply();
    expect(supply - supplyAfter).to.equal(BigInt(AMT));
  });

  it("B0003 Should not be able to burn tokens you don't own", async function () {
    //const balance = await erc20Proxy.balanceOf(tokenHolder.address);
    const AMT = 1;
    await expect(erc20Proxy.connect(tokenHolder).burn(AMT)).to.be.reverted;
    const allowance = await erc20Proxy.allowance(
      zrc2OwnerEVM.address,
      tokenHolder.address,
    );
    expect(allowance).to.equal(BigInt(0));
    await expect(
      erc20Proxy.connect(tokenHolder).burnFrom(zrc2OwnerEVM.address, AMT),
    ).to.be.reverted;
  });
});
