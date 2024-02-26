import { sha256 } from "js-sha256";
import { schnorr } from "@zilliqa-js/crypto";
import { getAddressFromPublicKey } from "@zilliqa-js/crypto";

export function verifySignature(
  message: string,
  publicKey: string,
  signature: string,
  address: string
) {
  const hashStr = sha256(message);
  const hashBytes = Buffer.from(hashStr, "hex");
  const bytecSignature = schnorr.toSignature(signature);
  const fromPubkey = getAddressFromPublicKey(publicKey);

  if (String(fromPubkey).toLowerCase() !== String(address).toLowerCase()) {
    throw new Error("incorrect address.");
  }

  return schnorr.verify(
    hashBytes,
    bytecSignature,
    Buffer.from(publicKey, "hex")
  );
}
