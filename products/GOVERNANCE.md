# Zilliqa Governance Portal

This document explains how `governance-api` and `governance-snapshot` work together to deliver the Zilliqa governance system — a gasless, off-chain, signature-based voting platform.

## Products

| Product | Stack | Purpose |
|---|---|---|
| `governance-api` | Node.js + TypeScript + Express + Sequelize + PostgreSQL | REST API backend |
| `governance-snapshot` | Vue 2 + Vuex + TypeScript | Single-page frontend |

## High-Level Architecture

```
User's browser (governance-snapshot)
      │
      │  POST /api/message  (submit proposal or vote)
      │  GET  /api/spaces   (space configs)
      │  GET  /api/:space/proposals
      │  GET  /api/:space/proposal/:id
      ▼
governance-api  ──── Zilliqa blockchain (balance queries, signature verification)
      │
      ├── IPFS / Pinata  (immutable storage for proposals and votes)
      └── PostgreSQL     (indexed proposal and vote records)
```

The frontend never writes directly to a database or blockchain. Every proposal and vote is:
1. Signed locally in the user's ZilPay wallet (Schnorr signature — no gas).
2. Sent to the API, which validates, pins to IPFS, and stores a database record.
3. Fetched back for display by reading the database index and IPFS content.

## Wallet Authentication

The frontend uses `@snapshot-labs/lock` wrapping the **ZilPay** browser extension:

1. User clicks "Connect Wallet".
2. `LockPlugin.login()` calls `window.zilPay.wallet.connect()`.
3. ZilPay prompts for permission; on approval it returns a provider.
4. The Vuex `web3` module subscribes to account/network change events and stores `{ base16, bech32 }` addresses.
5. The connector name is persisted in `localStorage` so the session survives page reloads.

No session token or server-side auth exists. Identity is proven entirely by cryptographic signature on each action.

## Governance Spaces

A **space** is a named governance context tied to a specific ZRC-2 token. Space metadata (name, token address, voting strategies, quorum default) comes from the `@snapshot-labs/snapshot-spaces` npm package and is served verbatim by `GET /api/spaces`.

The frontend can also be deployed on a custom domain that maps directly to a single space (via `domains.json` in snapshot-spaces), in which case the UI defaults to that space without showing a space selector.

## Proposal Lifecycle

### 1. Creation

The user fills in a form in the `Create.vue` view:

| Field | Constraint |
|---|---|
| Title | ≤ 256 characters |
| Body | ≤ 40,000 characters |
| Choices | ≥ 2 |
| Snapshot block | > 0, must exist |
| Voting window | `start < end`, both Unix timestamps |
| Quorum | 0–100% |

On submit, the frontend constructs a JSON message envelope:

```json
{
  "address": "<user bech32>",
  "msg": "{\"version\":\"0.1.2\",\"timestamp\":<unix>,\"token\":\"<token_addr>\",\"type\":\"proposal\",\"payload\":{...}}",
  "sig": { "message": "...", "publicKey": "...", "signature": "..." }
}
```

`signMessage()` calls `zilPay.wallet.sign(msg)` to produce a Schnorr signature without any on-chain transaction.

The API (`POST /api/message`) then:

1. **Validates the envelope** — timestamp within 30 s, protocol version, payload field count.
2. **Verifies the signature** — SHA-256 hashes the message, derives address from public key, calls `schnorr.verify()`.
3. **Checks the minimum balance** — the proposer must hold ≥ 30 gZIL at the current block.
4. **Snapshots token balances** — queries the Zilliqa blockchain for every holder's balance, including:
   - Direct ZRC-2 token holdings
   - Proportional stake in Zilswap liquidity pools
   - Proportional stake in XCAD pools
5. **Pins to IPFS** via Pinata — the pinned object includes `{ balances, totalSupply, address, msg, sig, version }`. The resulting IPFS hash becomes the proposal's canonical identifier.
6. **Writes a database record** — `Message { type: 'proposal', author_ipfs_hash, space, token, payload, ... }`.
7. **Returns** `{ ipfsHash }` to the frontend, which shows a success notification.

### 2. Viewing Proposals

`GET /api/:space/proposals` returns all proposals for a space ordered by timestamp. For each proposal, the frontend also calls `getScores()` to compute the proposer's voting power at the snapshot block.

The `Proposals.vue` view renders a card per proposal showing: title, author, voting window, current vote distribution, and quorum progress.

### 3. Voting

#### Browsing a Proposal

`Proposal.vue` loads a proposal by its IPFS hash:

1. Fetch proposal content from IPFS (`https://gateway.pinata.cloud/ipfs/<hash>`) — this returns the immutable object pinned at creation time, including the balance snapshot.
2. `GET /api/:space/proposal/:id` returns all votes (keyed by voter address).
3. For each voter, `getScores()` resolves their balance from the IPFS-stored snapshot (no live blockchain query needed).
4. Results are aggregated: vote count, total balance, and total score per choice.

#### Casting a Vote

1. User selects a choice; a confirmation modal (`Modal/Confirm.vue`) shows the choice, snapshot block, and the user's voting power.
2. User confirms; the frontend calls `send({ type: 'vote', payload: { proposal: <ipfs_hash>, choice: <1-indexed int>, metadata: {} } })`.
3. ZilPay signs the vote message locally.
4. `POST /api/message` validates:
   - Payload has exactly 3 fields.
   - The referenced proposal exists in the database.
   - Current time is within the proposal's `[start, end)` window.
5. The vote is pinned to IPFS and a `Message { type: 'vote', proposal_id: <proposal_ipfs_hash> }` record is inserted.
6. The frontend reloads the proposal to reflect the new totals.

## Voting Power & Balance Snapshots

Voting power is calculated by the `zrc2-balance-of` strategy implemented in `helpers/get-scores.ts`.

The balance snapshot taken at proposal creation time is the authoritative source. This prevents vote buying: a user who acquires tokens after the snapshot block cannot use them on that proposal.

**Resolution order:**
1. Check `proposal.balances[address]` (from the IPFS-pinned object).
2. If missing, fall back to a live blockchain query for `getSmartContractSubState(token, 'balances', [address])` at the snapshot block.

**DEX-adjusted balance formula:**

```
total_balance = direct_balance
              + (user_zilswap_lp / total_zilswap_lp) × pool_token_reserve
              + (user_xcad_lp   / total_xcad_lp)    × xcad_token_reserve
```

This ensures users who provide liquidity on Zilswap or XCAD retain governance power proportional to their pooled tokens.

## Signature Verification

All messages use Zilliqa's Schnorr signature scheme:

| Step | Detail |
|---|---|
| Hash | SHA-256 of the raw message string |
| Sign | `zilPay.wallet.sign(msg)` → `{ message, publicKey, signature }` |
| Verify | `schnorr.verify(hash, sig_bytes, pubkey)` + address derivation check |

The backend (`utils/verify-signature.ts`) performs both checks: the cryptographic signature and that the public key maps to the claimed address.

## Data Storage

### PostgreSQL — `Message` table

Indexed, queryable records for listing and lookup.

| Column | Description |
|---|---|
| `address` | Proposer/voter wallet address |
| `type` | `"proposal"` or `"vote"` |
| `space` | Space key |
| `token` | Token address |
| `author_ipfs_hash` | IPFS hash (primary identifier) |
| `payload` | JSON payload (name, choices, etc.) |
| `proposal_id` | For votes: IPFS hash of parent proposal |
| `sig` | Signature object |
| `timestamp` | Unix seconds |

### IPFS / Pinata — immutable content

Proposals and votes are pinned as JSON objects. The IPFS hash is content-addressed, making it tamper-evident: any change to the content would produce a different hash.

- **Proposal pin:** includes the balance snapshot — serves as the voting-power source of truth.
- **Vote pin:** includes the signed vote — provides a censorship-resistant audit trail.

## API Reference

| Method | Path | Description |
|---|---|---|
| `GET` | `/api/` | Health check; returns network, version, relayer address |
| `POST` | `/api/message` | Submit a proposal or vote |
| `GET` | `/api/spaces/:key?` | Fetch all spaces, or a single space by key |
| `GET` | `/api/:space/proposals` | List all proposals for a space |
| `GET` | `/api/:space/proposal/:id` | List all votes on a proposal |

## Environment Configuration

### governance-api

| Variable | Required | Description |
|---|---|---|
| `PINATA_API_KEY` | Yes | Pinata API key for IPFS pinning |
| `PINATA_SECRET_API_KEY` | Yes | Pinata secret |
| `POSTGRES_DB` | Yes (prod) | Database name |
| `POSTGRES_USER` | Yes (prod) | Database user |
| `POSTGRES_PASSWORD` | Yes (prod) | Database password |
| `POSTGRES_HOST` | Yes (prod) | Database host |
| `NODE_ENV` | Yes | `development` → SQLite; `production` → PostgreSQL + mainnet |
| `PORT` | No | HTTP port (default 3000) |
| `RELAYER_PK` | No | Private key for optional transaction relaying |

### governance-snapshot (`public/config.js`)

| Variable | Description |
|---|---|
| `VUE_APP_HUB_URL` | Base URL of `governance-api` (e.g. `https://governance-api.zilliqa.com`) |
| `VUE_APP_IPFS_NODE` | IPFS gateway hostname (default `gateway.pinata.cloud`) |

## Local Development

```sh
# governance-api
cd products/governance-api
cp example.env .env      # fill in credentials
npm install
npm start                # ts-node, port 3000, SQLite in dev mode

# governance-snapshot
cd products/governance-snapshot
yarn install
yarn serve               # NODE_OPTIONS=--openssl-legacy-provider; port 8080
```

The frontend expects `governance-api` at the URL set in `public/config.js`. For local development, point `VUE_APP_HUB_URL` to `http://localhost:3000`.

## Deployment

Both products are deployed to Kubernetes via Kustomize overlays and the internal `z` CLI tool:

```
products/governance-api/cd/overlays/staging
products/governance-api/cd/overlays/production
products/governance-snapshot/cd/overlays/staging
products/governance-snapshot/cd/overlays/production
```

The API is stateless aside from the PostgreSQL database and external IPFS pins, making horizontal scaling straightforward.

## Key Design Properties

- **Gasless** — no on-chain transactions required to vote or propose.
- **Snapshot-based** — voting power is locked at proposal creation; token transfers after the snapshot do not affect voting.
- **DEX-aware** — liquidity providers on Zilswap and XCAD retain proportional governance rights.
- **Tamper-evident** — proposals and votes are content-addressed on IPFS; the database is an index, not the source of truth.
- **Minimum stake to propose** — 30 gZIL required to submit a proposal, preventing spam.
