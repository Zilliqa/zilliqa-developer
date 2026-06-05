import { ethers, hashMessage, SigningKey } from 'ethers';
import { getAddressFromPublicKey } from '@zilliqa-js/crypto';

/**
 * Verifies an EIP-191 personal_sign signature.
 *
 * @param message   - The raw string that was passed to personal_sign (msg.msg from the request).
 * @param signature - The 0x-prefixed hex signature returned by personal_sign.
 * @param address   - The expected signer address (with or without 0x prefix).
 */
export function verifyEVMSignature(
  message: string,
  signature: string,
  address: string
): boolean {
  const recovered = ethers.verifyMessage(message, signature);
  // Normalise both to lowercase hex without 0x for comparison
  const normalised = address.startsWith('0x') ? address.slice(2) : address;
  return recovered.slice(2).toLowerCase() === normalised.toLowerCase();
}

/**
 * Derives the signer's canonical Zilliqa address (SHA256-based) from an EIP-191 personal_sign
 * signature, by recovering the public key. This is the address ZilPay shows for the same key —
 * i.e. where the user's gZIL/ZRC2 balances and space membership live. We use it to normalise an
 * EVM (MetaMask) submitter to their Zilliqa identity so the gZIL gate, the pinned voter-scoring
 * snapshot, and the members/score checks all resolve correctly.
 *
 * @param message   - The raw string passed to personal_sign (msg.msg from the request).
 * @param signature - The 0x-prefixed hex signature returned by personal_sign.
 * @returns The checksummed Zilliqa base16 address ("0x…").
 */
export function zilliqaAddressFromEVMSignature(
  message: string,
  signature: string
): string {
  const digest = hashMessage(message); // EIP-191 digest, matching personal_sign
  const uncompressed = SigningKey.recoverPublicKey(digest, signature); // "0x04…"
  const compressed = SigningKey.computePublicKey(uncompressed, true); // "0x02/03…"
  return getAddressFromPublicKey(compressed.replace(/^0x/, ''));
}
