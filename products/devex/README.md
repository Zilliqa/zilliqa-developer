# Devex - Zilliqa Dev Explorer

This is a developer-focused lightweight explorer to connect to the Zilliqa's
networks and local testnets.

As an explorer, Devex is unable to interact with the blockchain other than
pulling and displaying data. If you wish to interact with the blockchain (i.e.
create contracts, create transactions and so on..), do check out our
feature-filled [Scilla IDE](https://ide.zilliqa.com/#/)

## Features

- Built on top of Zilliqa's Javascript Library
- Provides developers with an intuitive GUI to explore any Zilliqa network
- Detailed and organised display for the following data:
  - Accounts
  - Contracts
  - Transactions
  - DS Blocks
  - Transaction Blocks
- Allows developers to add local testnets to the list of default networks to
  switch between
- Supports exploring of Zilliqa Isolated Servers. More info on
  [Isolated Servers here](https://github.com/Zilliqa/Zilliqa/blob/master/ISOLATED_SERVER_setup.md)
- Labelling System that allows developers to save often-visited entities, and
  share it using the import/export feature
- Dark Mode

## Setting Up

## Available Scripts

- `yarn install` to install dependencies
- `yarn start` to run the app on `localhost:3000`
- `yarn build` to build the app for production

## Preloading Networks

The explorer allows developers to define default networks to be shipped with the
explorer

This is done by adding a JSON file named `networks.json` into public folder
`%PROJ_DIR%/public` before building the application or injecting it into the
build post-build into the build's root directory `%BUILD_DIR%/`

Format: Array of key-value pairs where the network url is the key and the
network name is the value

An example is given below

```json
{
  "networks": [
    { "https://api.zilliqa.com": "Mainnet" },
    { "https://dev-api.zilliqa.com": "Testnet" },
    { "https://zilliqa-isolated-server.zilliqa.com": "Isolated Server" }
  ]
}
```

## Deploying applications with z

`z` is the one-stop shop for the Zilliqa provisioning and deployment operations. To deploy applications with z ensure the `z`
binary is installed in your operative system PATH environment variable. For more details about `z` please refer to the [documentation](https://github.com/Zilliqa/devops/blob/main/docs/z2.md).

## Deploying applications to localdev

To deploy the localdev/development environment go to the project folder in the zilliqa-developer repository:

```sh
cd ./products/devex
```

The `./products/devex/z.yaml` contains all the relevant configurations for the development environment.
Now set the following environment variables to reference the project's `z.yaml` file:

- `Z_ENV` to the path in which your `z.yaml` resides.
- `ZQ_USER` to your username (the bit before `@` in your email address)

for example:

```sh
export Z_ENV=z.yaml
export ZQ_USER=<user_id>@zilliqa.com
```

Create the local kind cluster (if not created previously):

```sh
z local create
```

Execute the manifests (in this case for ensuring the installation of the ingress-nginx controller, required for localdev/development environments):

```sh
z k-apply
```

Build and push the image:

```sh
make image/build-and-push
```

And deploy the application to your local cluster with:

```sh
z app sync
```

Verify your application is running correct from the `http://localhost` URL and with `kubectl` commands (if required).

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
   git checkout -b users/<username>/add_devex_to_staging_cluster
   ```

2. In the file `infra/live/gcp/non-production/prj-d-staging/z_ase1.yaml` add the following:

   - in `apps` stanza add:

     ```yaml
     clusters:
       staging:
         apps:
           devex:
             repo: https://github.com/Zilliqa/zilliqa-developer
             path: products/devex/cd/overlays/staging
             track: staging
             type: kustomize
     ```

   - in `subdomains` stanza add:

     ```yaml
     infrastructure:
     dns:
       vars:
       subdomains:
         devex: {}
     ```

3. Push the changes

   ```sh
   git add .
   git commit -m "Add Devex to staging cluster"
   git push origin users/<username>/add_devex_to_staging_cluster
   ```

4. Open a Pull Request to the main branch

5. Apply the changes

   ```sh
   z plan
   z apply
   ```

### Deploy the application

```sh
z app sync --cache-dir=.cache devex
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
   git checkout -b users/<username>/add_devex_to_production_cluster
   ```

2. In the file `infra/live/gcp/production/prj-p-prod-apps/z_ase1.yaml` add the following:

   - in `apps` stanza add:

     ```yaml
     clusters:
       production:
         apps:
           devex:
             repo: https://github.com/Zilliqa/zilliqa-developer
             path: products/devex/cd/overlays/production
             track: production
             type: kustomize
     ```

   - in `subdomains` stanza add:

     ```yaml
     infrastructure:
     dns:
       vars:
       subdomains:
         devex: {}
     ```

3. Push the changes

   ```sh
   git add .
   git commit -m "Add Devex to production cluster"
   git push origin users/<username>/add_devex_to_production_cluster
   ```

4. Open a Pull Request to the main branch

5. Apply the changes

   ```sh
   z plan
   z apply
   ```

### Deploy the application

```sh
z app sync --cache-dir=.cache devex
```

Verify your application is running correct from the production URL and with `kubectl` commands (if required).
