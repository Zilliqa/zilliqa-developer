import { getRelayEvents } from "../helpers";

async function main() {
  const chainGatewayAddress = "0x18BCE81F9De993cdB2ebd680a44A8068B62D7f26";

  const events = await getRelayEvents(chainGatewayAddress);

  console.log(events.map((e) => e.args));
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
