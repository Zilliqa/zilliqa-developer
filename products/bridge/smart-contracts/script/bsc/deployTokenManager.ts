import { ethers, upgrades } from "hardhat";
import { config } from "../config";

async function main() {
  const gatewayContract = config.bsc.chainGateway;

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
