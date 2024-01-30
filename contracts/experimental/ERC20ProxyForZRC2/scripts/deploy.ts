import { ethers } from "hardhat";

async function main() {
  const zrc2Address: `0x${string}` =
    "0x929c314bf271259fc03b58f51627f1ae2baf1039";

  const contract = await ethers.deployContract("ZRC2ProxyForZRC2", [
    zrc2Address,
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
