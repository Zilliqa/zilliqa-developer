# Devex Apollo

## Introduction

Devex Apollo uses [Apollo Data Graph Platform](https://www.apollographql.com/) to enable querying of blockkchain data using GraphQL qquery.

## Features

Devex Apollo does the following

- Crawl the [Zilliqa](https://www.zilliqa.com/) testnet/mainnet
- Populate the on-chain data into [MongoDB](https://www.mongodb.com/)
- Enable queries using GraphQL

## Use cases

With Devex Apollo,

- Extend [Devex](https://devex.zilliqa.com/?network=https%3A%2F%2Fapi.zilliqa.com) with historical transaction of an account or contracts
- Provide data for construction of smart contract call graph
- Data analytics for DApp

## Deploying applications with z

`z` is the one-stop shop for the Zilliqa provisioning and deployment operations. To deploy applications with z ensure the `z`
binary is installed in your operative system PATH environment variable. For more details about `z` please refer to the [documentation](https://github.com/Zilliqa/devops/blob/main/docs/z2.md).

## Deploying applications to localdev

To deploy the localdev/development environment go to the project folder in the zilliqa-developer repository:

```sh
cd ./products/devex-apollo
```

The `./products/devex-apollo/z.yaml` contains all the relevant configurations for the development environment.
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
   git checkout -b users/<username>/add_devex_apollo_to_staging_cluster
   ```

2. In the file `infra/live/gcp/non-production/prj-d-staging/z_ase1.yaml` add the following:

   - in `apps` stanza add:

     ```yaml
     clusters:
       staging:
         apps:
           devex:
             repo: https://github.com/Zilliqa/zilliqa-developer
             path: products/devex-apollo/cd/overlays/staging
             track: staging
             type: kustomize
     ```

   - in `subdomains` stanza add:

     ```yaml
     infrastructure:
     dns:
       vars:
       subdomains:
         devex-apollo: {}
     ```

3. Push the changes

   ```sh
   git add .
   git commit -m "Add Devex Apollo to staging cluster"
   git push origin users/<username>/add_devex_apollo_to_staging_cluster
   ```

4. Open a Pull Request to the main branch

5. Apply the changes

   ```sh
   z plan
   z apply
   ```

### Deploy the application

```sh
z app sync --cache-dir=.cache devex-apollo
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
           devex-apollo:
             repo: https://github.com/Zilliqa/zilliqa-developer
             path: products/devex-apollo/cd/overlays/production
             track: production
             type: kustomize
     ```

   - in `subdomains` stanza add:

     ```yaml
     infrastructure:
     dns:
       vars:
       subdomains:
         devex-apollo: {}
     ```

3. Push the changes

   ```sh
   git add .
   git commit -m "Add Devex Apollo to production cluster"
   git push origin users/<username>/add_devex_apollo_to_production_cluster
   ```

4. Open a Pull Request to the main branch

5. Apply the changes

   ```sh
   z plan
   z apply
   ```

### Deploy the application

```sh
z app sync --cache-dir=.cache devex-apollo
```

Verify your application is running correct from the production URL and with `kubectl` commands (if required).
