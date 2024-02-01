import { ethers } from "hardhat";

async function main() {
  const zrc2Address: `0x${string}` =
    "0x8b4939cc7988fdc11e30a2b7e9d26362d1cb1aa3";

  const contract = await ethers.deployContract(
    "ZRC2ProxyForZRC2",
    [zrc2Address],
    {
      gasLimit: 1_000_000,
    }
  );

  await contract.waitForDeployment();

  console.log("ZRC2ProxyForZRC2 deployed to:", await contract.getAddress());
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
