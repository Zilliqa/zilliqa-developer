// ABOUTME: Asserts zilliqaAddressFromEVMSignature recovers the signer's canonical Zilliqa address.
// ABOUTME: Run with `node --require ts-node/register lib/utils/verify-evm-signature.test.ts`.
import assert from "assert";
import { Wallet, SigningKey } from "ethers";
import { getAddressFromPublicKey } from "@zilliqa-js/crypto";
import {
  zilliqaAddressFromEVMSignature,
  verifyEVMSignature,
} from "./verify-evm-signature";

// A MetaMask user signs with their EVM (Keccak) address, but their gZIL/ZRC2 balances and
// space membership live under their Zilliqa (SHA256) address. This proves we can recover the
// Zilliqa address from the EVM personal_sign signature alone.

async function derivesCanonicalZilliqaAddress() {
  const pk = "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";
  const wallet = new Wallet(pk);
  const message = JSON.stringify({ version: "0.1.2", type: "proposal", token: "duck" });
  const signature = await wallet.signMessage(message); // EIP-191 personal_sign

  // Ground truth derived straight from the private key (independent of the recovery path).
  const expectedZil = getAddressFromPublicKey(
    new SigningKey(pk).compressedPublicKey.replace(/^0x/, "")
  );

  const got = zilliqaAddressFromEVMSignature(message, signature);

  assert.strictEqual(
    got.toLowerCase(),
    expectedZil.toLowerCase(),
    "derives the signer's canonical Zilliqa address from the EVM signature"
  );
  assert.notStrictEqual(
    got.toLowerCase(),
    wallet.address.toLowerCase(),
    "the Zilliqa address must differ from the EVM 0x address (the whole point)"
  );
  // The same signature still verifies against the EVM address (unchanged behaviour).
  assert.strictEqual(
    verifyEVMSignature(message, signature, wallet.address),
    true,
    "EVM signature still verifies against the EVM address"
  );
  console.log("OK derivesCanonicalZilliqaAddress:", got, "(EVM:", wallet.address + ")");
}

(async () => {
  await derivesCanonicalZilliqaAddress();
  console.log("ALL PASS");
})().catch((e) => {
  console.error("FAIL:", e.message);
  process.exit(1);
});
