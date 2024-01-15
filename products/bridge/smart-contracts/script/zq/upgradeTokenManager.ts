import { ethers, upgrades } from "hardhat";

async function main() {
  const tokenManagerAddress = "0xd10077bCE4A9D19068965dE519CED8a2fC1B096C";

  const TokenManagerFactory = await ethers.getContractFactory(
    "LockAndReleaseTokenManagerUpgradeable"
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
