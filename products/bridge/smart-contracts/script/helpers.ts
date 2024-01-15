import { ethers } from "hardhat";

export async function getRelayEvents(chainGatewayAddress: string) {
  const chainGateway = await ethers.getContractAt(
    "ChainGateway",
    chainGatewayAddress
  );

  const nonce = await chainGateway.nonce();
  console.log(nonce);

  const filter = chainGateway.filters.Relayed();
  const events = await chainGateway.queryFilter(filter, "earliest", "latest");
  console.log(events);

  return events;
}
