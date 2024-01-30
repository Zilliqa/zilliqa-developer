import { ethers, upgrades } from "hardhat";
import { config } from "../config";

async function main() {
  const tokenManagerAddress = config.zq.tokenManager;

  const TokenManagerFactory = await ethers.getContractFactory(
    "LockAndReleaseTokenManagerUpgradeableV2"
  );
  const tokenManager = await upgrades.upgradeProxy(
    tokenManagerAddress,
    TokenManagerFactory
  );
  await tokenManager.waitForDeployment();
  console.log("TokenManager upgrade on:", await tokenManager.getAddress());
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
