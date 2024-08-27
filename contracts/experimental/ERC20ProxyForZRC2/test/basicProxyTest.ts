import {expect} from "chai";
import {ethers} from "hardhat";
import hre from "hardhat";
import {ScillaContract} from "hardhat-scilla-plugin";
import {Account} from "@zilliqa-js/zilliqa";


describe("basicTest", function () {
  let zrc2: ScillaContract;
  let erc20Proxy: Contract;
  let zrc2Owner: Account;

  let zrc2OwnerEVM: Signer;
  let proxyDeployer : Signer;
  let tokenHolder: Signer;

  const ZRC2_NAME : string = "ProxyTestToken";
  const ZRC2_SYMBOL : string = "PTT";
  const ZRC2_DECIMALS = 3;
  const ZRC2_SUPPLY = 1000;

  before (async function() {
    proxyDeployer = new ethers.Wallet(process.env.TEST_KEY_1, ethers.provider);
    tokenHolder = new ethers.Wallet(process.env.TEST_KEY_2, ethers.provider);
    zrc2Owner = hre.zilliqa.getDefaultAccount();

    scillaContract = await hre.deployScillaContract("FungibleToken.scilla",
                                                    zrc2Owner.address,
                                                    ZRC2_NAME,
                                                    ZRC2_SYMBOL,
                                                    ZRC2_DECIMALS,
                                                    ZRC2_SUPPLY);
    console.log(`Sample ZRC-2 token ${scillaContract.address} owned by ${zrc2Owner.address}`);
    console.log(`Proxy deployer ${proxyDeployer.account} ; token holder ${tokenHolder.account}`);
  });

  it("Should deploy successfully", async function () {
    expect(zrc2.address).to.be.properAddress;
  });

});

