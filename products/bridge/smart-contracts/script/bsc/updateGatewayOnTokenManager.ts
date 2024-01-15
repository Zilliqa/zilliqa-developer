import { ethers } from "hardhat";

async function main() {
  const newGatewayAddress = "0x5cE584e24f6703f3197Ca83d442807cB82474f8D";
  const tokenManagerAddress = "0xd10077bCE4A9D19068965dE519CED8a2fC1B096C";

  const tokenManager = await ethers.getContractAt(
    "MintAndBurnTokenManagerUpgradeable",
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
