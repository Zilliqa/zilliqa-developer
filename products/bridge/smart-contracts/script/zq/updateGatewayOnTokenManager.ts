import { ethers } from "hardhat";

async function main() {
  const newGatewayAddress = "0x18BCE81F9De993cdB2ebd680a44A8068B62D7f26";
  const tokenManagerAddress = "0xd10077bCE4A9D19068965dE519CED8a2fC1B096C";

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
