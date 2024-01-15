import { ethers } from "hardhat";

async function main() {
  const tokenManagerAddress = "0xd10077bCE4A9D19068965dE519CED8a2fC1B096C";

  const localToken = "0x63B6ebD476C84bFDd5DcaCB3f974794FC6C2e721";
  const remoteToken = "0x6d78c86D66DfE5Be5F55FBAA8B1d3FD28edfF396";
  const remoteTokenManager = "0xd10077bCE4A9D19068965dE519CED8a2fC1B096C";
  const remoteChainId = 97;

  const tokenManager = await ethers.getContractAt(
    "LockAndReleaseTokenManagerUpgradeable",
    tokenManagerAddress
  );

  const tx = await tokenManager.registerToken(
    localToken,
    remoteToken,
    remoteTokenManager,
    remoteChainId
  );
  const receipt = await tx.wait();
  console.log(receipt);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
