# Zilliqa Staking Wallet App

## Getting Started



### Development
1. `yarn`


2. Tweak the settings in `public/config.js`
```
    networks_config: {
        testnet: {
            proxy: "<proxy_checksum_address>",
            impl: "<implementation_contract_checksum_address>",
            blockchain: "https://dev-api.zilliqa.com",
            node_status: "https://testnet-viewer.zilliqa.com",
            api_list : [
                "https://dev-api.zilliqa.com",
            ]
        },
        mainnet: {
            proxy: "<proxy_checksum_address>",
            impl: "<implementation_contract_checksum_address>",
            blockchain: "https://api.zilliqa.com",
            node_status: "https://staking-viewer.zilliqa.com",
            api_list : [
                "https://api.zilliqa.com",
            ]
        },
        isolated_server: {
            proxy: "<proxy_checksum_address>",
            impl: "<implementation_contract_checksum_address>",
            blockchain: "https://zilliqa-isolated-server.zilliqa.com",
            node_status: "",
            api_list : [
                "https://zilliqa-isolated-server.zilliqa.com",
            ]
        }
    },
    blockchain_explorer_config: "viewblock",
    refresh_rate_config: 3000,
    api_max_retry_attempt: 10,
    environment_config: "dev"
```

3. (Optional) If you are interested in using the isolated server, set `environment_config` to `dev` otherwise, set to `stage` for testnet and `prod` for mainnet respectively.

This enables the "Isolated Server" option in the network selection menu and disables authentications checks in the dashboard.

4. Execute `yarn dev` and browse to `https://localhost:3000/`. HTTPS is required due to hardware ledger support.

5. On the home page, select the network. Next, select _Sign in as Delegators_ or _Sign in as Operators_

6. An operator can see "Staking Performance", "Staked Seed Nodes" whereas delegators can only see "Staked Seed Nodes"

7. Explore and execute any actions

8. If the contract details doesn't get updated, click on the "Dashboard" wordings on the navigation bar to manually refresh


### Production

1. Follow Steps (1) and (2) as stated in **Development** section.

2. Set the `environment_config` flag in `config.js` to `prod`.

3. Assuming the hosted server is running `https`, execute `yarn start`.

4. Done.


### Enable / Disable Stake Operations
Adjust these parameters to disable all stake operations and display a warning banner:

1. At `src/util/enum.ts`, set the `enum ContractState` to **'true'** with the single quotes.
-- This disables all the stake contract calls buttons and replaces the wordings to **"Not Available"**

2. To change the banner display for the main page, edit `src/components/warning-banner.tsx`.

3. To change the banner display for dashboard, edit `src/components/warning-dashboard-banner.tsx`.


## Caveats
The wallet app does not store your private keys or passphrases for privacy and security concerns. The wallet session object is disconnected upon refreshing the browser. We advise you to avoid refreshing the browser; all statistics are retrieved at regular intervals as configured by `config.js`.


## Deploying applications with z

`z` is the one-stop shop for the Zilliqa provisioning and deployment operations. To deploy applications with z ensure the `z`
binary is installed in your operative system PATH environment variable. For more details about `z` please refer to the [documentation](https://github.com/Zilliqa/devops/blob/main/docs/z2.md).

## Deploying applications to staging

To deploy the staging environment we need to clone the devops repository and execute `z` from there:

```sh
git clone https://github.com/Zilliqa/devops.git
cd devops
source setenv
```

### Set the following environment variables

- `Z_ENV` to the path in which your `z.yaml` resides.
- `ZQ_USER` to your username (the bit before `@` in your email address)
- `GITHUB_PAT` (if you are deploying staging or production apps) to a classic PAT with all the repo permissions ticked.

for example:

```sh
export Z_ENV=`pwd`/infra/live/gcp/non-production/prj-d-staging/z_ase1.yaml
export ZQ_USER=<user_id>@zilliqa.com
export GITHUB_PAT=<GITHUB_PAT>
```

### Login to Google Cloud

```sh
z login
```

### Add the application to the staging `z.yaml` file. Skip this step if it is an existing application

1. Create a branch:

   ```sh
   git checkout -b users/<username>/add_zillion_to_staging_cluster
   ```

2. In the file `infra/live/gcp/non-production/prj-d-staging/z_ase1.yaml` add the following:

   - in `apps` stanza add:

     ```yaml
     clusters:
       staging:
         apps:
           zillion:
             repo: https://github.com/Zilliqa/zilliqa-developer
             path: products/zillion/cd/overlays/staging
             track: staging
             type: kustomize
     ```

   - in `subdomains` stanza add:

     ```yaml
     infrastructure:
     dns:
       vars:
       subdomains:
         zillion: {}
     ```

3. Push the changes

   ```sh
   git add .
   git commit -m "Add zillion to staging cluster"
   git push origin users/<username>/add_zillion_to_staging_cluster
   ```

4. Open a Pull Request to the main branch

5. Apply the changes

   ```sh
   z plan
   z apply
   ```

### Deploy the application

```sh
z app sync --cache-dir=.cache zillion
```

Verify your application is running correct from the staging URL and with `kubectl` commands (if required).

## Deploying applications to production

To deploy the production environment we need to clone the devops repository and execute `z` from there:

```sh
git clone https://github.com/Zilliqa/devops.git
cd devops
source setenv
```

### Set the following environment variables

- `Z_ENV` to the path in which your `z.yaml` resides.
- `ZQ_USER` to your username (the bit before `@` in your email address)
- `GITHUB_PAT` (if you are deploying staging or production apps) to a classic PAT with all the repo permissions ticked.

for example:

```sh
export Z_ENV=`pwd`/infra/live/gcp/production/prj-p-prod-apps/z_ase1.yaml
export ZQ_USER=<user_id>@zilliqa.com
export GITHUB_PAT=<GITHUB_PAT>
```

### Login to Google Cloud

```sh
z login
```

### Add the application to the production `z.yaml` file. Skip this step if it is an existing application

1. Create a branch:

   ```sh
   git checkout -b users/<username>/add_zillion_to_production_cluster
   ```

2. In the file `infra/live/gcp/production/prj-p-prod-apps/z_ase1.yaml` add the following:

   - in `apps` stanza add:

     ```yaml
     clusters:
       production:
         apps:
           zillion:
             repo: https://github.com/Zilliqa/zilliqa-developer
             path: products/zillion/cd/overlays/production
             track: production
             type: kustomize
     ```

   - in `subdomains` stanza add:

     ```yaml
     infrastructure:
     dns:
       vars:
       subdomains:
         zillion: {}
     ```

3. Push the changes

   ```sh
   git add .
   git commit -m "Add zillion to production cluster"
   git push origin users/<username>/add_zillion_to_production_cluster
   ```

4. Open a Pull Request to the main branch

5. Apply the changes

   ```sh
   z plan
   z apply
   ```

### Deploy the application

```sh
z app sync --cache-dir=.cache zillion
```

Verify your application is running correct from the production URL and with `kubectl` commands (if required).
