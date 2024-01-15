import { ethers } from "hardhat";
import { config } from "../config";

async function main() {
  const newGatewayAddress = config.zq.chainGateway;
  const tokenManagerAddress = config.zq.tokenManager;

  const tokenManager = await ethers.getContractAt(
    "LockAndReleaseTokenManagerUpgradeable",
    tokenManagerAddress
  );

  const tx = await tokenManager.setGateway(newGatewayAddress);
  const receipt = await tx.wait();
  console.log(receipt?.hash);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
