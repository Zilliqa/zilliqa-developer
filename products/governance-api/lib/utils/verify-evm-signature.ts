import { ethers } from 'ethers';

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
