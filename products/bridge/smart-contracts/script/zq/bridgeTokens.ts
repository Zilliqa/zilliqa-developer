import { ethers } from "hardhat";
import { getRelayEvents } from "../helpers";
import { config } from "../config";

async function main() {
  let tx, receipt;
  const tokenManagerAddress = config.zq.tokenManager;
  const chainGatewayAddress = config.zq.chainGateway;

  const tokenAddress = config.zq.token;
  const remoteChainID = config.zq.remoteChainId;
  const remoteRecipient = config.zq.remoteRecipient;
  const amount = config.zq.amount;

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
