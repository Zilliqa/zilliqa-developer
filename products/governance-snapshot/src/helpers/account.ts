// ABOUTME: Pure helpers to derive and compare the wallet account that actually signs a message.
// ABOUTME: One place for address normalisation so msg.address always equals the real signer.
import { getAddressFromPublicKey, toBech32Address } from '@zilliqa-js/crypto';

export interface Account {
  base16: string; // lowercase hex, no 0x prefix
  bech32: string;
}

// Normalise any Zilliqa/EVM address to lowercase hex without the 0x prefix.
export function normalizeBase16(address?: string | null): string {
  if (!address) return '';
  return String(address)
    .replace(/^0x/i, '')
    .toLowerCase();
}

// True only when both addresses are non-empty and refer to the same account.
export function addressesMatch(a?: string | null, b?: string | null): boolean {
  const na = normalizeBase16(a);
  return na !== '' && na === normalizeBase16(b);
}

// Build a normalised { base16, bech32 } pair from any address form.
export function toAccount(address: string): Account {
  const base16 = normalizeBase16(address);
  return { base16, bech32: toBech32Address('0x' + base16) };
}

// The 0x-prefixed lowercase hex form, the single owner of the "+0x" convention
// used on the wire (msg.address) and for explorer links.
export function toHex0x(account: Account): string {
  return '0x' + account.base16;
}

// Derive the signer from a Schnorr signature's public key using the SAME
// function the backend uses (getAddressFromPublicKey in verify-signature.ts),
// so the backend's address check passes by construction.
export function accountFromPublicKey(publicKey: string): Account {
  return toAccount(getAddressFromPublicKey(publicKey));
}
