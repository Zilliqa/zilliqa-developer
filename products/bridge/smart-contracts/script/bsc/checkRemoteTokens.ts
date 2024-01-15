import { ethers } from "hardhat";

async function main() {
  const tokenManagerAddress = "0xd10077bCE4A9D19068965dE519CED8a2fC1B096C";
  const localToken = "0x6d78c86D66DfE5Be5F55FBAA8B1d3FD28edfF396";
  const remoteChainID = 33101;

  const tokenManager = await ethers.getContractAt(
    "MintAndBurnTokenManagerUpgradeable",
    tokenManagerAddress
  );

  const res = await tokenManager.getRemoteTokens(localToken, remoteChainID);
  console.log(res);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
