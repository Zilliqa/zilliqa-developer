import { ethers } from "hardhat";
import { config } from "../config";

async function main() {
  const chainGatewayAddress = config.zq.chainGateway;

  const chainGateway = await ethers.getContractAt(
    "ChainGateway",
    chainGatewayAddress
  );

  console.log("Current Nonce", await chainGateway.nonce());

  console.log("Missing txns");
  for (let i = 0; i < 76; i++) {
    const res = await chainGateway.dispatched(56, i);
    if (!res) {
      console.log(i, res);
    }
  }
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
