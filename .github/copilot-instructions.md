# GitHub Copilot Instructions

> Project-specific coding standards and architecture guidance for the `zilliqa-developer` monorepo.

---

## Architecture

### Repository Structure

- This is a **Bazel-based monorepo** using Bazelisk as the build wrapper.
- JS packages are managed as a **pnpm workspace** under `zilliqa/js/`.
- Products are self-contained applications under `products/`.
- Contracts live under `contracts/` — `audited/` for production, `experimental/` for in-development.

### JS SDK Packages (`zilliqa/js/`)

All packages are published under the `@zilliqa-js/*` namespace:

- `core` — HTTP/WebSocket provider, JSON-RPC
- `account` — wallet and account management
- `blockchain` — Blockchain API queries
- `contract` — Scilla contract deployment and interaction
- `crypto` — key generation, schnorr signing, address formats
- `util` — BN.js utilities, unit conversion, validation
- `subscriptions` — WebSocket event subscriptions
- `proto` — protobuf definitions
- `zilliqa` — main entry point, re-exports all sub-packages

### Products

Each product under `products/` is self-contained. Key products:

| Product | Stack |
|---|---|
| `devex` | React (CRA) |
| `neo-savant` | Vue 2 + Vite |
| `governance-snapshot` | Vue 2 |
| `governance-api` | Node.js/Express + Sequelize + PostgreSQL |
| `zillion` | React |
| `pdt` | Rust |
| `faucet-service` | Go |
| `eth-spout` | Rust |
| `laksaj` | Java (Maven via Bazel) |
| `bridge` | Solidity smart contracts (pnpm workspace) |
| `devex-apollo` | Node.js + Apollo + MongoDB |

### Governance System

- `governance-api` — Express REST API backend (TypeScript, Sequelize ORM, PostgreSQL). Uses IPFS (Pinata) for proposal storage.
- `governance-snapshot` — Vue 2 SPA frontend connecting to `governance-api` and integrating `@snapshot-labs` libraries.

### Block Explorer (`devex`)

React SPA using Apollo Client for GraphQL queries against the `devex-apollo` backend (Node.js + MongoDB).

---

## Code Style

### General

- Follow language-idiomatic conventions for each file's language (TypeScript, Go, Rust, Java, Solidity).
- Use **imperative style** for commit messages and documentation.
- Respect the linting rules enforced by [trunk.io](https://trunk.io) — it runs eslint, prettier, black, flake8, gofmt, rustfmt, shellcheck, and others.

### TypeScript / JavaScript

- Use TypeScript for all JS SDK and product code where applicable.
- Each JS SDK package targets three distribution formats: **cjs**, **esm**, and **umd** (via `tsc` and `rollup`).
- Shared TypeScript configuration is in `tsconfig.base.json` — extend it rather than duplicating settings.
- Tests match the Jest pattern: `zilliqa/js/**/test/?(*.)+(spec|test).(ts|js)`.

### Bazel

- Use `BUILD` files consistently for all Bazel targets.
- Prefer `bazelisk` over `bazel` to ensure consistent Bazel version usage.
- Use `bazelisk query` to inspect targets and dependencies before making structural changes.

---

## Testing

- JS SDK tests run via **Jest** (config at `jest.config.js` in the repo root).
- Bazel is the canonical test runner for all targets: `bazelisk test //zilliqa/js/...`.
- Always run tests after modifying SDK packages or contracts.
- For Go and Rust, use their native test frameworks (`go test`, `cargo test`).

---

## Patterns

### Dependency Management

- JS/TS packages: use **pnpm** (not npm or yarn) for the monorepo workspace.
- Install workspace deps from `zilliqa/js/`: `pnpm i`.
- Do not mix package managers within the same product.

### Linting

- Linting is managed by **trunk**. Avoid bypassing trunk checks.
- Some directories are intentionally excluded from trunk (e.g., `products/governance-api`, `products/governance-snapshot`, `products/zillion`, `products/multisig`).
- Do not add those directories back to trunk linting unless intentional.

### Deployment

- Products are deployed via the internal `z` CLI tool.
- Each deployable product has a `z.yaml` configuration — do not remove or rename it.

### Contracts

- Production-deployed contracts live in `contracts/audited/` — treat these as immutable references.
- Experimental contracts in `contracts/experimental/` may be modified, but changes must be reviewed carefully.
