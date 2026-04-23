# Snapshot


Snapshot is an off-chain gasless multi-governance client with easy to verify and hard to contest results for Zilliqa ecosystem.

This project is a fork of https://github.com/balancer-labs/snapshot. Do support the Snapshot team @ https://gitcoin.co/grants/1093/snapshot

## Voting flow

### Overview
The voting system is an **off-chain, signature-based** governance model (similar to Snapshot.org). Instead of sending a transaction on the blockchain (which costs gas), users sign a message with their wallet. This signature proves the authenticity of the vote. The "weight" or power of the vote is calculated based on the user's token balance at a specific block height (snapshot) on the Zilliqa blockchain.

### 1. Wallet Identification
To participate, a user must connect their Zilliqa wallet (e.g., ZilPay).
- **Source**: [src/store/modules/web3.ts](src/store/modules/web3.ts)
- **Mechanism**: The application initializes the wallet provider and subscribes to account changes.
- **Address**: The application retrieves the connected **Bech32** or **Base16** address. This address serves as the voter's identity throughout the session.

### 2. Voting Power Calculation (The "Weight")
A vote is not just "one person, one vote"; it is weighted by the number of tokens held.
- **Source**: [src/helpers/get-scores.ts](src/helpers/get-scores.ts) (specifically the `getScores` function)
- **Strategy**: The application primarily uses the `zrc2-balance-of` strategy.
- **Snapshot Logic**:
   - Each proposal has a `snapshot` block number.
   - When a user views a proposal, the app calculates their voting power by querying the Zilliqa blockchain.
   - It requests the **balance of the governance token** for the specific wallet address **at that specific block height**.
   - This ensures users cannot buy tokens *after* a proposal starts to influence the vote.

### 3. Casting the Vote
When a particular wallet address initiates a vote, the following process occurs:

#### A. Interaction
- The user selects a choice (e.g., "For", "Against") in the [Proposal.vue](src/views/Proposal.vue) view.
- Upon clicking "Vote", a confirmation modal ([Modal/Confirm.vue](src/components/Modal/Confirm.vue)) appears.

#### B. Message Construction
When confirmed, the application ([src/store/modules/app.ts](src/store/modules/app.ts)) constructs a message envelope (`msg` variable) . This includes the **user's wallet address** (identifying the voter) and the JSON-stringified vote payload. Including the address allows the backend to know which account is performing the action, which will be verified against the signature.

```json
{
  "address": "<user-base16-address>",
  "msg": JSON.stringify({
    "version": "...",
    "timestamp": "<current-timestamp>",
    "token": "<space-token-symbol>",
    "type": "vote",
    "payload": {
      "proposal": "<proposal-id>",
      "choice": <choice-index>,
      "metadata": {}
    }
  })
}
```

#### C. Cryptographic Signature
- **Action**: `signMessage` in [src/helpers/web3.ts](src/helpers/web3.ts).
- The application requests the connected wallet (e.g., ZilPay) to **sign** this message.
- This action occurs off-chain. It does not broadcast a transaction to the network.
- The resulting signature is a cryptographic proof that the owner of the `particular wallet address` authorized this specific choice.
- The signature is saved in `msg.sign` field.

#### D. Submission
- The application bundles the **Message**, the **Address**, and the **Signature**.
- It sends this package via an HTTP POST request to the backend hub (API) using [src/helpers/client.ts](src/helpers/client.ts).

### Summary
1.  **Identify**: The address is identified by the connected wallet.
2.  **Quantify**: The system queries the blockchain to see how many tokens that address held at the snapshot block.
3.  **Authorize**: The address owner calculates the vote intention locally and cryptographically signs it using their private key (managed by the wallet software).
4.  **Submit**: The signed intent is relayed to a centralized hub for aggregation.

## Wallet Connectivity

The portal supports two wallet types, both of which require **mainnet**. Testnet is not supported and no fix is planned, as this portal is scheduled for deprecation.

| Wallet | Connector ID | Notes |
|--------|-------------|-------|
| ZilPay | `zlp` | Native Zilliqa wallet; displays bech32 address; links to the configured network explorer |
| EVM Wallet (e.g. MetaMask) | `evm` | MetaMask-compatible; displays `0x`-prefixed address; links to [zilliqa.blockscout.com](https://zilliqa.blockscout.com) (mainnet only — hardcoded) |

The EVM wallet block-explorer link is hardcoded to `https://zilliqa.blockscout.com`, so it will always point to mainnet regardless of the connected network. This is a known limitation that will not be addressed before deprecation.

## Development


### Project setup
```
yarn install
```

### Compiles and hot-reloads for development
```
yarn run serve
```

### Compiles and minifies for production
```
yarn run build
```

### Lints and fixes files
```
yarn run lint
```

## Pre-built app directory notes

We are including the app directory as part of the code base because the legacy build process needs to be reviewed and corrected with newer dependencies versions. Once this is completed the Dockerfile could include the yarn install and build commands.

```
COPY ./package.json ./
RUN yarn install
COPY . ./
RUN yarn build
```

## Snapshot Spaces Commit
https://github.com/Zilliqa/snapshot-spaces/commit/238e87aad231351a51727b06208ab407f0de1dcc

## Deployment

### Deploying applications with z

`z` is the one-stop shop for the Zilliqa provisioning and deployment operations. To deploy applications with z ensure the `z`
binary is installed in your operative system PATH environment variable. For more details about `z` please refer to the [documentation](https://github.com/Zilliqa/devops/blob/main/docs/z2.md).

### Deploying applications to staging

To deploy the staging environment we need to clone the devops repository and execute `z` from there:

```sh
git clone https://github.com/Zilliqa/devops.git
cd devops
source setenv
```

#### Set the following environment variables

- `Z_ENV` to the path in which your `z.yaml` resides.
- `ZQ_USER` to your username (the bit before `@` in your email address)
- `GITHUB_PAT` (if you are deploying staging or production apps) to a classic PAT with all the repo permissions ticked.

for example:

```sh
export Z_ENV=`pwd`/infra/live/gcp/non-production/prj-d-staging/z_ase1.yaml
export ZQ_USER=<user_id>@zilliqa.com
export GITHUB_PAT=<GITHUB_PAT>
```

#### Login to Google Cloud

```sh
z login
```

#### Add the application to the staging `z.yaml` file. Skip this step if it is an existing application

1. Create a branch:

   ```sh
   git checkout -b users/<username>/add_governance-snapshot_to_staging_cluster
   ```

2. In the file `infra/live/gcp/non-production/prj-d-staging/z_ase1.yaml` add the following:

   - in `apps` stanza add:

     ```yaml
     clusters:
       staging:
         apps:
           governance-snapshot:
             repo: https://github.com/Zilliqa/zilliqa-developer
             path: products/governance-snapshot/cd/overlays/staging
             track: staging
             type: kustomize
     ```

   - in `subdomains` stanza add:

     ```yaml
     infrastructure:
     dns:
       vars:
       subdomains:
         governance-snapshot: {}
     ```

3. Push the changes

   ```sh
   git add .
   git commit -m "Add governance-snapshot to staging cluster"
   git push origin users/<username>/add_governance-snapshot_to_staging_cluster
   ```

4. Open a Pull Request to the main branch

5. Apply the changes

   ```sh
   z plan
   z apply
   ```

#### Deploy the application

```sh
z app sync --cache-dir=.cache governance-snapshot
```

Verify your application is running correct from the staging URL and with `kubectl` commands (if required).

### Deploying applications to production

To deploy the production environment we need to clone the devops repository and execute `z` from there:

```sh
git clone https://github.com/Zilliqa/devops.git
cd devops
source setenv
```

#### Set the following environment variables

- `Z_ENV` to the path in which your `z.yaml` resides.
- `ZQ_USER` to your username (the bit before `@` in your email address)
- `GITHUB_PAT` (if you are deploying staging or production apps) to a classic PAT with all the repo permissions ticked.

for example:

```sh
export Z_ENV=`pwd`/infra/live/gcp/production/prj-p-prod-apps/z_ase1.yaml
export ZQ_USER=<user_id>@zilliqa.com
export GITHUB_PAT=<GITHUB_PAT>
```

#### Login to Google Cloud

```sh
z login
```

#### Add the application to the production `z.yaml` file. Skip this step if it is an existing application

1. Create a branch:

   ```sh
   git checkout -b users/<username>/add_governance-snapshot_to_production_cluster
   ```

2. In the file `infra/live/gcp/production/prj-p-prod-apps/z_ase1.yaml` add the following:

   - in `apps` stanza add:

     ```yaml
     clusters:
       production:
         apps:
           governance-snapshot:
             repo: https://github.com/Zilliqa/zilliqa-developer
             path: products/governance-snapshot/cd/overlays/production
             track: production
             type: kustomize
     ```

   - in `subdomains` stanza add:

     ```yaml
     infrastructure:
     dns:
       vars:
       subdomains:
         governance-snapshot: {}
     ```

3. Push the changes

   ```sh
   git add .
   git commit -m "Add governance-snapshot to production cluster"
   git push origin users/<username>/add_governance-snapshot_to_production_cluster
   ```

4. Open a Pull Request to the main branch

5. Apply the changes

   ```sh
   z plan
   z apply
   ```

#### Deploy the application

```sh
z app sync --cache-dir=.cache governance-snapshot
```

Verify your application is running correct from the production URL and with `kubectl` commands (if required).


## License

[MIT](LICENSE).

20220216