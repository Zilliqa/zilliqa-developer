import { ethers } from "hardhat";
import { config } from "../config";

async function main() {
  const tokenManagerAddress = config.zq.tokenManager;
  const chainGatewayAddress = config.zq.chainGateway;

  const chainGateway = await ethers.getContractAt(
    "ChainGateway",
    chainGatewayAddress
  );

  const tx = await chainGateway.register(tokenManagerAddress);
  await tx.wait();

  console.log(tx.hash);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
