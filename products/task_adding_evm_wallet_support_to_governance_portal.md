# TASK: adding evm wallet support to governance portal

## Overall goal

I want to add EVM wallet support (as alternative to existing ZilPay support) to the Governance portal (`products/governance-snapshot`, a Vue 2 SPA).

The chosen approach is to use **`window.ethereum`** directly — the standard browser injected provider present in MetaMask, Ledger Live browser extension, Brave Wallet, Coinbase Wallet, and any other browser extension wallet. No additional dependencies are required.

`@web3modal/standalone` + `@wagmi/core` were considered but rejected: wagmi v2 and viem are ESM-only packages that conflict with Vue CLI 4's Webpack 4 bundler, requiring invasive config changes that are not worth taking on in a legacy project.

The trade-off: WalletConnect QR code flow is not supported. This is acceptable because the target users connect hardware Ledgers via MetaMask or Ledger Live, both of which inject `window.ethereum`.


## Implementation decisions

- The information about governance can be found in `products/GOVERNANCE.md`
- All balances — including those of EVM wallet users — are stored on the non-EVM (Scilla) side of Zilliqa. Balance retrieval requires no changes because ZRC-2 `balances` maps are already keyed by base16 addresses, which is the natural format for EVM accounts (just lowercase, without the `0x` prefix). The `getLiquidity` call in the backend and the `getScores` strategy in the frontend both work with base16 addresses and are therefore compatible with EVM wallet addresses out of the box.
- Zilliqa blockchain reads in the frontend (for `getScores`, `getBlockNumber`, and `getTotalSupply`) should use `@zilliqa-js/zilliqa` with a direct JSON-RPC connection to `https://api.zilliqa.com/`, the same approach used by `governance-api` in `lib/zilliqa/custom-fetch.ts`. This removes the dependency on ZilPay as a read provider entirely — the wallet is only needed for signing, not for chain queries.
- The API will accept both Schnorr (ZilPay) and ECDSA (EVM) signatures. A `sigType` flag will be added to the message body so the backend can route to the correct verification path without needing to infer the signature type from the address format.
- EVM wallets will sign using `personal_sign` (EIP-191). The frontend passes the existing `msg.msg` JSON string to `personal_sign`; the backend prepends the `\x19Ethereum Signed Message:\n<len>` prefix, hashes with keccak256, and calls `ecrecover` to recover the signer address. No changes to the message structure are required.
- EVM wallet addresses are stored in the same format as ZilPay addresses: lowercase hex without the `0x` prefix. The frontend must strip `0x` and lowercase the address before including it in the message body. The backend's existing `validation.isBech32` guard at `message.ts:208` already handles this correctly — an EVM hex address is not bech32, so it falls through to `.toLowerCase()`, which is a no-op once `0x` has been stripped by the frontend.
- The Vuex `web3` state shape (`{ base16, bech32 }`) stays unchanged for EVM wallet users. When an EVM wallet connects, the bech32 value is derived from the EVM address using `toBech32Address` from `@zilliqa-js/crypto`. This keeps all existing UI and logic that reads `web3.account.base16` or `web3.account.bech32` working without modification.
- The `sig` field in `POST /api/message` differs by wallet type. ZilPay submissions keep the existing `{ message, publicKey, signature }` shape. EVM wallet submissions use `{ message, signature }` — there is no separate public key field, as the backend recovers the signer's address from the signature via `ecrecover`. The `sigType` flag (see above) tells the backend which shape to expect and which verification path to take.
- Both ZilPay and EVM wallets coexist as options, one active at a time. An EVM connector entry is added alongside `zlp` in `connectors.json`; whichever the user picks populates the single `web3.account` state. No simultaneous dual-connection is supported.
- The 30 gZIL minimum balance policy for submitting a proposal applies equally to EVM wallet users. The backend check requires no changes — `getLiquidity` already works with base16 addresses, which is the natural format for EVM accounts.
- Session persistence for EVM wallets uses `_lock.connector` in `localStorage` (same as ZilPay). On page load, `init` checks `_lock.connector`; if it is `evm`, it calls `window.ethereum.request({ method: 'eth_accounts' })` (no user prompt) to verify the session is still active before restoring it. `window.ethereum` has no explicit `disconnect()` — logout clears the `localStorage` key and resets app state only.

## Implementation plan

The plan is split into three phases. Each phase is independently testable before moving to the next.

---

### [Completed] Phase 1 — Decouple blockchain reads from ZilPay (frontend)

All current blockchain reads (`getBlockNumber`, `getTotalSupply`, ZRC-2 balance queries) are routed through the ZilPay browser extension object. This is the first thing to fix because it is a prerequisite for the rest: once reads are independent, the wallet is used only for signing and the EVM connector has parity immediately.

**Step 1 — Create a shared Zilliqa RPC instance**

Create `products/governance-snapshot/src/helpers/zilliqa.ts`:

```ts
import { Zilliqa } from '@zilliqa-js/zilliqa';
export const zilliqa = new Zilliqa('https://api.zilliqa.com/');
```

This singleton is imported wherever a direct JSON-RPC call is needed.

**Step 2 — Rewrite `getBlockNumber` and `getTotalSupply` in `src/helpers/web3.ts`**

Current signatures take a `provider` (ZilPay object). Replace both implementations to use the shared `zilliqa` instance and drop the `provider` parameter:

```ts
// was: export async function getBlockNumber(provider) { ... provider.blockchain... }
export async function getBlockNumber(): Promise<number> {
  const chainInfo: any = await zilliqa.blockchain.getBlockChainInfo();
  return parseInt(chainInfo.result.NumTxBlocks);
}

// was: export async function getTotalSupply(provider, address) { ... provider.blockchain... }
export async function getTotalSupply(address: string): Promise<string> {
  const res: any = await zilliqa.blockchain.getSmartContractSubState(address, 'total_supply');
  if (res?.result?.total_supply) return res.result.total_supply;
  throw new Error('cannot fetch total_supply');
}
```

`signMessage` in the same file stays untouched for now (Phase 2 handles it).

**Step 3 — Rewrite the `zrc2-balance-of` strategy in `src/helpers/get-scores.ts`**

Replace the two ZilPay calls with direct RPC and standard address normalization:

- `provider.crypto.normaliseAddress(address)` → use `validation.isBech32(address) ? fromBech32Address(address).toLowerCase() : address.toLowerCase()` (both imported from `@zilliqa-js/zilliqa`)
- `provider.blockchain.getSmartContractSubState(...)` → `zilliqa.blockchain.getSmartContractSubState(...)`

The `provider` parameter of the `strategy` function and of `getScores` is no longer used after this change; keep it in the signature (as optional `_provider?: any`) to avoid touching callers for now.

**Step 4 — Update callers of the rewritten functions in `src/store/modules/app.ts`**

Four call sites:

| Line (approx) | Current call | New call |
|---|---|---|
| `getProposals` ~119 | `zilPay = await waitZilPay()` | remove; pass `null` or drop arg |
| `getProposals` ~128 | `getScores(strategies, zilPay, addresses)` | `getScores(strategies, null, addresses)` |
| `getProposal` ~153 | `zilPay = await waitZilPay()` | remove |
| `getProposal` ~169 | `getScores(strategies, zilPay, addresses)` | `getScores(strategies, null, addresses)` |
| `getPower` ~220 | `const zilPay = await waitZilPay()` | remove |
| `getPower` ~221 | `const blockNumber = await getBlockNumber(zilPay)` | `const blockNumber = await getBlockNumber()` |
| `getPower` ~223 | `getScores(strategies, zilPay, [address], ...)` | `getScores(strategies, null, [address], ...)` |

Also update the two call sites in `src/views/Create.vue` (~lines 210-211) that call `getBlockNumber(zilPay)` and `getTotalSupply(zilPay, address)` — remove the `zilPay` argument from both.

After this phase the app works exactly as before, but ZilPay is no longer involved in any read path.

---

### [Completed] Phase 2 — EVM connector and store wiring (frontend)

No new dependencies. All EVM wallet interaction goes through `window.ethereum`.

**Step 5 — Add EVM entry to `src/helpers/connectors.json`**

```json
{
  "zlp": { "id": "zlp", "name": "ZilPay" },
  "evm": { "id": "evm", "name": "EVM Wallet" }
}
```

**Step 6 — Add `EVMConnector` class to `src/helpers/plugins/LockPlugin.ts`**

Add after the existing `ZilPay` class:

```ts
export class EVMConnector extends LockConnector {
  async connect() {
    if (!window['ethereum']) throw new Error('No EVM wallet found. Install MetaMask or another browser wallet.');
    const accounts: string[] = await window['ethereum'].request({ method: 'eth_requestAccounts' });
    if (!accounts.length) throw new Error('No accounts returned by wallet.');
    return { isEVM: true, address: accounts[0] };
  }

  async isLoggedIn(): Promise<boolean> {
    if (!window['ethereum']) return false;
    const accounts: string[] = await window['ethereum'].request({ method: 'eth_accounts' });
    return accounts.length > 0;
  }

  async logout() {
    // window.ethereum has no disconnect method; app state is cleared by useLock.logout()
  }
}
```

The return value `{ isEVM: true, address }` is the "provider" object that `useLock` assigns to `this.provider`. `isEVM` acts as a sentinel to distinguish EVM sessions in the store; `address` carries the connected account so `loadProvider` does not need to make an additional RPC call.

**Step 7 — Register `EVMConnector` in `src/auth.ts`**

```ts
import { LockPlugin, ZilPay as zlp, EVMConnector as evm } from '@/helpers/plugins/LockPlugin';
const connectors = { zlp, evm };
```

No other change needed; the loop that calls `lock.addConnector` already handles all entries.

**Step 8 — Update `loadProvider` in `src/store/modules/web3.ts`**

Add an EVM branch before the existing ZilPay logic:

```ts
import { toBech32Address } from '@zilliqa-js/crypto';

loadProvider: async ({ commit }) => {
  commit('LOAD_PROVIDER_REQUEST');
  try {
    if (auth.provider?.isEVM) {
      // EVM path — address is carried on the provider sentinel from EVMConnector.connect()
      const base16 = auth.provider.address.slice(2).toLowerCase();
      const bech32 = toBech32Address('0x' + base16);
      commit('LOAD_PROVIDER_SUCCESS', { account: { base16, bech32 }, name: bech32 });

      // Subscribe to future account changes (e.g. user switches wallet in MetaMask)
      window['ethereum'].on('accountsChanged', (accounts: string[]) => {
        if (accounts.length) {
          const b16 = accounts[0].slice(2).toLowerCase();
          const b32 = toBech32Address('0x' + b16);
          commit('HANDLE_ACCOUNTS_CHANGED', { base16: b16, bech32: b32 });
        } else {
          commit('LOGOUT');
        }
      });
      return;
    }

    // Existing ZilPay path (unchanged below this point)
    if (auth.provider) {
      auth.provider.wallet.observableAccount().subscribe(async (account: any) => {
        commit('HANDLE_ACCOUNTS_CHANGED', account);
      });
      auth.provider.wallet.observableNetwork().subscribe((net: string) => {
        commit('HANDLE_CHAIN_CHANGED', net);
      });
    }
    const net = auth.provider.wallet.net;
    commit('HANDLE_CHAIN_CHANGED', net);
    const account = auth.provider.wallet.defaultAccount;
    const name = auth.provider.wallet.defaultAccount.bech32;
    commit('LOAD_PROVIDER_SUCCESS', { account, name });
  } catch (e) {
    commit('LOAD_PROVIDER_FAILURE', e);
    return Promise.reject();
  }
}
```

**Step 9 — Update session rehydration in `src/store/modules/app.ts` (`init` action)**

The existing `getConnector()` call already handles the EVM case: it reads `_lock.connector` from `localStorage`, then calls `EVMConnector.isLoggedIn()` which calls `eth_accounts` (no user prompt). If accounts are still available the session is restored; otherwise `getConnector()` returns `false` and the user must log in again.

No changes required here beyond ensuring `EVMConnector` is registered (Step 7).

**Step 10 — Update `signMessage` in `src/helpers/web3.ts`**

Add the EVM signing branch:

```ts
export async function signMessage(web3: any, msg: string) {
  if (web3?.isEVM) {
    // personal_sign (EIP-191) via window.ethereum
    const accounts: string[] = await window['ethereum'].request({ method: 'eth_accounts' });
    const signature: string = await window['ethereum'].request({
      method: 'personal_sign',
      params: [msg, accounts[0]]
    });
    return { message: msg, signature };
  }
  // ZilPay Schnorr signing (existing)
  return await web3.wallet.sign(msg);
}
```

**Step 11 — Update `send()` in `src/store/modules/app.ts` to include `sigType`**

After building `msg.sig`, add `sigType` to the body sent to the API:

```ts
send: async ({ commit, dispatch, rootState }, { token, type, payload }) => {
  const auth = getInstance();
  commit('SEND_REQUEST');
  try {
    const msg: any = {
      address: rootState.web3.account.base16,   // already lowercase hex, no 0x (invariant from loadProvider)
      msg: JSON.stringify({ version, timestamp: (Date.now() / 1e3).toFixed(), token, type, payload })
    };
    const sigType = auth.provider?.isEVM ? 'evm' : 'schnorr';
    msg.sig = await signMessage(auth.web3, msg.msg);
    msg.sigType = sigType;
    const result = await client.request('message', msg);
    // ... rest unchanged
  }
}
```

**Step 12 — Fix the connector icon in `src/components/Modal/Account.vue`**

The current icon URL `https://zilpay.xyz/icons/${connector.id}.png` does not have an EVM entry. Replace the `<img>` tag with a conditional:

```html
<img
  v-if="connector.id === 'zlp'"
  src="https://zilpay.xyz/icons/zlp.png"
  height="28" width="28" class="mr-1 v-align-middle"
/>
<Icon
  v-else
  name="wallet"
  size="28"
  class="mr-1 v-align-middle"
/>
```

Use whichever icon name exists in the project's icon set, or inline a small SVG.

---

### [Completed] Phase 3 — Backend EVM signature verification

**Step 13 — Add `ethers` to `products/governance-api/package.json`**

```sh
cd products/governance-api && yarn add ethers
```

`ethers.utils.verifyMessage` handles the EIP-191 prefix and keccak256 hash internally, exactly matching what `personal_sign` produces.

**Step 14 — Create `lib/utils/verify-evm-signature.ts`**

```ts
import { ethers } from 'ethers';

/**
 * Verifies an EIP-191 personal_sign signature.
 *
 * @param message   - The raw string that was passed to personal_sign (msg.msg from the request).
 * @param signature - The 0x-prefixed hex signature returned by personal_sign.
 * @param address   - The expected signer address, lowercase hex WITHOUT the 0x prefix.
 */
export function verifyEVMSignature(
  message: string,
  signature: string,
  address: string
): boolean {
  const recovered = ethers.utils.verifyMessage(message, signature);
  // recovered is a checksummed 0x... address; normalise to match stored format
  return recovered.slice(2).toLowerCase() === address.toLowerCase();
}
```

**Step 15 — Update `lib/routes/message.ts` to route by `sigType`**

Import the new function at the top:

```ts
import { verifyEVMSignature } from '../utils/verify-evm-signature';
```

Replace the signature verification block (currently lines 182-198) with:

```ts
try {
  let checked: boolean;
  if (body.sigType === 'evm') {
    checked = verifyEVMSignature(
      body.sig.message,
      body.sig.signature,
      body.address           // lowercase hex without 0x, validated by frontend before submission
    );
  } else {
    // Default to Schnorr for ZilPay and legacy submissions
    checked = verifySignature(
      body.sig.message,
      body.sig.publicKey,
      body.sig.signature,
      body.address
    );
  }
  if (!checked) throw new Error('signature mismatch');
} catch (err) {
  return res.status(400).json({
    code: ErrorCodes.INCORRECT_SIGNATURE,
    error_description: 'incorrect signature',
  });
}
```

No other changes are needed in `message.ts`. The address normalization at lines 207-210 already handles lowercase hex without `0x` (the `else` branch returns `String(body.address).toLowerCase()`). The `getLiquidity` call, the IPFS pin, and the database write all work without modification.

---

### Testing checklist

After each phase:

- **Phase 1**: Proposals and voting still work with ZilPay; no ZilPay errors appear in the console for read calls (`getBlockNumber`, `getTotalSupply`, balance queries).
- **Phase 2**: EVM wallet button appears in the Connect modal. Connecting MetaMask (injected) populates `web3.account` with a valid `base16`/`bech32` pair. Refreshing the page re-hydrates the EVM session via `eth_accounts`. ZilPay still works.
- **Phase 3**: Submitting a proposal and casting a vote from an EVM wallet succeeds end-to-end; Schnorr submissions from ZilPay continue to work.
