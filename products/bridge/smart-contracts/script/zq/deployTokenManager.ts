import { ethers, upgrades } from "hardhat";

async function main() {
  const gatewayContract = "0xd10077bCE4A9D19068965dE519CED8a2fC1B096C";

  const LockAndReleaseTokenManagerFactory = await ethers.getContractFactory(
    "LockAndReleaseTokenManagerUpgradeable"
  );
  const tokenManager = await upgrades.deployProxy(
    LockAndReleaseTokenManagerFactory,
    [gatewayContract]
  );
  await tokenManager.waitForDeployment();
  console.log(
    "LockAndReleaseTokenManager deployed to:",
    await tokenManager.getAddress()
  );
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
