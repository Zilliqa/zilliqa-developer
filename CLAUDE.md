# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Overview

`zilliqa-developer` is a Bazel-based monorepo containing SDKs, documentation, and products for the Zilliqa blockchain ecosystem. The build system is Bazel (via Bazelisk), with pnpm workspaces managing JS packages.

## Build System

The primary build tool is **Bazelisk** (Bazel wrapper). Most targets are built, tested, and run via Bazel:

```sh
bazelisk build [target]       # Build a target
bazelisk run [target]         # Run an executable target
bazelisk test [target]        # Test a target
bazelisk query "//..."        # List all targets
bazelisk query "//zilliqa/..."  # List targets in a subfolder
bazelisk query "deps(//zilliqa/js/util:pkg)"  # Find target dependencies
```

Useful flags: `--verbose_failures`, `--test_output=all`, `--sandbox_debug`

Set `DISABLE_WORKSPACE_STATUS=1` (or `--workspace_status_command=echo`) to skip git queries and avoid key/password prompts on every build.

## JS SDK (zilliqa/js)

The JS SDK packages live in `zilliqa/js/` and are managed as a pnpm workspace. Packages: `account`, `blockchain`, `contract`, `core`, `crypto`, `proto`, `subscriptions`, `typings`, `util`, `zilliqa` — all published as `@zilliqa-js/*`.

```sh
# Install dependencies
cd zilliqa/js && pnpm i

# Build all packages
pnpm -r build

# Run tests (via Bazel)
bazelisk test //zilliqa/js/...

# Run tests via Jest (from repo root)
npx jest
```

Jest config is at `jest.config.js` — tests match `zilliqa/js/**/test/?(*.)+(spec|test).(ts|js)`.

Each package builds three distribution formats (cjs, esm, umd) via `tsc` and `rollup`.

## Linting & Formatting

Uses [trunk.io](https://trunk.io) to manage multiple linters (eslint, prettier, black, flake8, gofmt, rustfmt, shellcheck, etc.):

```sh
trunk check    # Check code style
trunk fmt      # Auto-format where possible
```

Trunk is enforced by CI. Several directories are excluded from linting (see `.trunk/trunk.yaml`), including `products/governance-api`, `products/governance-snapshot`, `products/zillion`, `products/multisig`, and others.

## Products

Each product under `products/` is largely self-contained. Common setup patterns:

| Product | Stack | Dev command |
|---|---|---|
| `devex` | React (CRA) | `yarn start` (port 3000) |
| `neo-savant` | Vue 2 + Vite | `yarn dev` |
| `governance-snapshot` | Vue 2 | `yarn serve` (NODE_OPTIONS=--openssl-legacy-provider) |
| `governance-api` | Node.js/Express + Sequelize + PostgreSQL | `npm start` (ts-node) |
| `zillion` | React | `yarn` then configure `public/config.js` |
| `pdt` | Rust | `cargo run` |
| `faucet-service` | Go | `make deps && make build` |
| `eth-spout` | Rust | Configured via environment variables |
| `laksaj` | Java (Maven via Bazel) | Bazel build |
| `isolated-server` | Docker | `docker build` / `docker run` |
| `bridge` | Solidity smart contracts | pnpm workspace |
| `devex-apollo` | Node.js + Apollo + MongoDB | docker-compose |

## Architecture

### JS SDK Architecture

The `@zilliqa-js/zilliqa` package is the main entry point, re-exporting from these sub-packages:
- `@zilliqa-js/core` — HTTP/WebSocket provider, JSON-RPC
- `@zilliqa-js/account` — wallet, account management
- `@zilliqa-js/blockchain` — Blockchain API queries
- `@zilliqa-js/contract` — Scilla contract deployment and interaction
- `@zilliqa-js/crypto` — key generation, signing (schnorr), address formats
- `@zilliqa-js/util` — BN.js utilities, unit conversion, validation
- `@zilliqa-js/subscriptions` — WebSocket event subscriptions
- `@zilliqa-js/proto` — protobuf definitions

### Governance System

Two related products:
- `governance-api` — Express REST API backend (TypeScript, Sequelize ORM, PostgreSQL). Entry: `index.ts` → `lib/server.ts`. Uses IPFS (Pinata) for proposal storage.
- `governance-snapshot` — Vue 2 SPA frontend. Connects to `governance-api` and integrates with `@snapshot-labs` libraries.

### Devex (Block Explorer)

React SPA (`products/devex`) using Apollo Client for GraphQL queries against `devex-apollo` (Node.js + MongoDB backend that crawls Zilliqa chain data).

### Deployment

Products are deployed via the `z` CLI tool (Zilliqa internal DevOps tool). See each product's README for `z.yaml` configuration and staging/production deployment instructions.

## Contracts

- `contracts/audited/` — production-deployed contracts
- `contracts/experimental/` — contracts under development, including `ERC20ProxyForZRC2` (pnpm workspace package)
- `contracts/gaming_contracts/`, `contracts/reward_control/` — domain-specific contracts

## Key Configuration Files

- `WORKSPACE` / `BUILD` — Bazel workspace and build rules
- `pnpm-workspace.yaml` — pnpm workspace package list
- `tsconfig.base.json` — shared TypeScript base config
- `.trunk/trunk.yaml` — linter configuration and ignore rules
- `jest.config.js` — root Jest config for JS SDK tests
