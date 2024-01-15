import { parseUnits } from "ethers";
import { ethers } from "hardhat";
import { getRelayEvents } from "../helpers";

async function main() {
  let tx, receipt;
  const tokenManagerAddress = "0xd10077bCE4A9D19068965dE519CED8a2fC1B096C";
  const chainGatewayAddress = "0x18BCE81F9De993cdB2ebd680a44A8068B62D7f26";

  const tokenAddress = "0x63B6ebD476C84bFDd5DcaCB3f974794FC6C2e721";
  const remoteChainID = 97;
  const remoteRecipient = "0xb494D016F2CF329224e2dB445aA748Cf96C18C29";
  const amount = parseUnits("100", 12);

  const tokenManager = await ethers.getContractAt(
    "LockAndReleaseTokenManagerUpgradeable",
    tokenManagerAddress
  );

  const token = await ethers.getContractAt("ERC20", tokenAddress);
  tx = await token.approve(tokenManagerAddress, amount);
  receipt = await tx.wait();
  console.log("Approve Tx", receipt?.hash);

  tx = await tokenManager.transfer(
    tokenAddress,
    remoteChainID,
    remoteRecipient,
    amount
  );
  receipt = await tx.wait();
  console.log("Transfer Tx", receipt?.hash);

  const events = await getRelayEvents(chainGatewayAddress);
  console.log(
    "Events",
    events.map((e) => e.args)
  );
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
