// ABOUTME: Unit tests for account.ts — the derived signer must equal getAddressFromPublicKey.
// ABOUTME: Framework-free; run via ts-node (see the "test" script in package.json).
import assert from 'assert';
import { getAddressFromPublicKey, fromBech32Address } from '@zilliqa-js/crypto';
import {
  normalizeBase16,
  addressesMatch,
  toAccount,
  toHex0x,
  accountFromPublicKey
} from './account';

// Deterministic fixture (private key d96e…0aba):
const PUB =
  '03bfad0f0b53cff5213b5947f3ddd66acee8906aba3610c111915aecc84092e052';
const GROUND_TRUTH = getAddressFromPublicKey(PUB); // 0x381f4008…caF796

function normalize() {
  assert.strictEqual(normalizeBase16('0xAbC123'), 'abc123');
  assert.strictEqual(normalizeBase16('AbC123'), 'abc123');
  assert.strictEqual(normalizeBase16(''), '');
  assert.strictEqual(normalizeBase16(undefined), '');
  console.log('OK normalize');
}

function matches() {
  assert.strictEqual(
    addressesMatch('0xAbc', 'abc'),
    true,
    '0x + case insensitive'
  );
  assert.strictEqual(addressesMatch('0xAbc', '0xdef'), false);
  assert.strictEqual(addressesMatch('', '0xabc'), false, 'empty never matches');
  assert.strictEqual(addressesMatch(undefined, undefined), false);
  console.log('OK matches');
}

function signerEqualsBackendCheck() {
  // The point of the fix: the address derived here must equal the backend's
  // getAddressFromPublicKey(publicKey), so verify-signature.ts passes by construction.
  const acc = accountFromPublicKey(PUB);
  assert.strictEqual(
    '0x' + acc.base16,
    GROUND_TRUTH.toLowerCase(),
    'signer base16 == getAddressFromPublicKey (the backend address check)'
  );
  assert.strictEqual(
    fromBech32Address(acc.bech32).toLowerCase(),
    GROUND_TRUTH.toLowerCase(),
    'bech32 round-trips to the same address'
  );
  console.log('OK signerEqualsBackendCheck', acc.bech32);
}

function accountForm() {
  const acc = toAccount('0xE1984B201376e5C82836d10166B8de4a5c0d2EfF');
  assert.strictEqual(acc.base16, 'e1984b201376e5c82836d10166b8de4a5c0d2eff');
  assert.ok(acc.bech32.startsWith('zil1'), 'bech32 produced');
  assert.strictEqual(
    toHex0x(acc),
    '0xe1984b201376e5c82836d10166b8de4a5c0d2eff',
    'toHex0x re-adds the 0x prefix to the normalised base16'
  );
  console.log('OK accountForm', acc.bech32);
}

(function run() {
  normalize();
  matches();
  signerEqualsBackendCheck();
  accountForm();
  console.log('ALL PASS');
})();
