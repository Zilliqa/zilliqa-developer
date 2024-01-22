import { ethers } from "hardhat";

async function main() {
  const zrc2_address: `0x${string}` = "0x";

  const contract = await ethers.deployContract("ZRC2ProxyForZRC2", [
    zrc2_address,
  ]);

  await contract.waitForDeployment();

  console.log("ZRC2ProxyForZRC2 deployed to:", await contract.getAddress());
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
