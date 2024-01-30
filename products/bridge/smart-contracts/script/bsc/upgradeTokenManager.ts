import { ethers, upgrades } from "hardhat";
import { config } from "../config";

async function main() {
  const tokenManagerAddress = config.bsc.tokenManager;

  const TokenManagerFactory = await ethers.getContractFactory(
    "MintAndBurnTokenManagerUpgradeableV2"
  );
  const tokenManager = await upgrades.upgradeProxy(
    tokenManagerAddress,
    TokenManagerFactory
  );
  await tokenManager.waitForDeployment();
  console.log("TokenManager deployed to:", await tokenManager.getAddress());
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
