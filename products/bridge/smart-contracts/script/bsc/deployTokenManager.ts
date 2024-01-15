import { ethers, upgrades } from "hardhat";

async function main() {
  const gatewayContract = "0x587cA00ac16EF2b96B7ff1D9E810B20b942b11c3";

  const TokenManagerFactory = await ethers.getContractFactory(
    "MintAndBurnTokenManagerUpgradeable"
  );
  const tokenManager = await upgrades.deployProxy(TokenManagerFactory, [
    gatewayContract,
  ]);
  await tokenManager.waitForDeployment();
  console.log("TokenManager deployed to:", await tokenManager.getAddress());
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
