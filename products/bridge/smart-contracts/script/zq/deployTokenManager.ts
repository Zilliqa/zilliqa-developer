import { ethers, upgrades } from "hardhat";
import { config } from "../config";

async function main() {
  const gatewayContract = config.zq.chainGateway;

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
