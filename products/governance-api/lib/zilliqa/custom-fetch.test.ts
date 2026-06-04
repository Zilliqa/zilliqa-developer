// ABOUTME: Framework-free assertions for getLiquidity's submitter-scoped balance fetch.
// ABOUTME: Run with `node --require ts-node/register lib/zilliqa/custom-fetch.test.ts`.
import assert from "assert";
import { blockchain } from "./custom-fetch";

// The Scilla ZRC2 `balances` map is keyed by lowercase 0x addresses (verified against
// api.zilliqa.com). getLiquidity must (a) fetch ONLY the submitter's entry — not the whole
// multi-MB map — and (b) read it back with a normalized lowercase-0x key.

async function fullMapFetchTest() {
  // The balances RPC MUST fetch the FULL holder map (index []): it is pinned to IPFS as the
  // whole-electorate voter-scoring snapshot. Scoping it to the submitter collapses vote tallies
  // to the submitter alone (regression C1). The submitter's own balance is still read back via a
  // normalized lowercase-0x key (fixes a latent bech32 checksum mismatch).
  const b: any = new blockchain();
  let sent: any;
  b._send = async (batch: any[]) => {
    sent = batch;
    return [
      { result: { pools: {} } },
      { result: { balances: {} } },
      { result: { xpools: {} } },
      { result: { xbalances: {} } },
      { result: { balances: { "0xabc": "123", "0xdef": "999" } } },
      { result: { total_supply: "1000" } },
    ];
  };

  // Mixed-case submitter input to prove read-key normalization.
  const out = await b.getLiquidity("0xA845c1034CD077bd8D32be0447239c7E4be6cb21", "0xABC");

  assert.deepStrictEqual(
    sent[4].params[2],
    [],
    "balances must fetch the FULL map (index []) for the voter-scoring snapshot (guards C1)"
  );
  assert.strictEqual(out.userBalance, "123", "submitter balance read via normalized lowercase key");
  assert.ok("0xdef" in out.balances, "full electorate snapshot retained for scoring");
  console.log("OK fullMapFetchTest");
}

async function nullResultTest() {
  const b: any = new blockchain();
  b._send = async () => [
    { result: null },
    { result: null },
    { result: null },
    { result: null },
    { result: null },
    { result: null },
  ];
  const out = await b.getLiquidity("0xA845c1034CD077bd8D32be0447239c7E4be6cb21", "0xfe56");
  assert.strictEqual(out.userBalance, "0", "missing balance => '0' (no throw)");
  assert.strictEqual(out.totalSupply, "0", "missing total_supply => '0' (no throw)");
  console.log("OK nullResultTest");
}

async function lpHolderCreditedWithoutDirectBalance() {
  // A submitter whose gZIL sits ONLY in ZilSwap liquidity (no direct balance entry) must still
  // be credited via the seeded key. Without the seed this returns "0" and wrongly blocks them.
  const b: any = new blockchain();
  const TOKEN = "0xa845c1034cd077bd8d32be0447239c7e4be6cb21";
  const OWNER = "0xabc"; // -> ownerKey "0xabc"
  b._send = async () => [
    { result: { pools: { [TOKEN]: { arguments: ["zilReserve", "1000000"] } } } },
    { result: { balances: { [TOKEN]: { "0xabc": "50" } } } }, // owner = 100% of a 50-unit pool
    { result: { xpools: {} } },
    { result: { xbalances: {} } },
    { result: { balances: {} } }, // NO direct token balance for the owner
    { result: { total_supply: "999" } },
  ];
  const out = await b.getLiquidity(TOKEN, OWNER);
  assert.strictEqual(
    out.userBalance,
    "1000000",
    "LP-only holder must be credited their pool share via the seeded key"
  );
  console.log("OK lpHolderCreditedWithoutDirectBalance");
}

(async () => {
  await fullMapFetchTest();
  await nullResultTest();
  await lpHolderCreditedWithoutDirectBalance();
  console.log("ALL PASS");
})().catch((e) => {
  console.error("FAIL:", e.message);
  process.exit(1);
});
