# Neo-savant IDE

Neo-savant is a fully-fledged IDE used for writing, testing and deploying Scilla smart contracts painlessly. It can be tried out at [https://ide.zilliqa.com/](https://ide.zilliqa.com/).

Neo-Savant helps Scilla developers to create and deploy Smart Contracts using an automated development environment, in-browser, with quick and intuitive controls.

## Features

- Intuitive UI for easy deployment/contract invocation.
- Multiple networks supported: Testnet, Mainnet and a Simulated Environment where you can test out contracts without spending $ZIL
- Account management using Keystore, Ledger or ZilPay.
- Simple, persistent file manager for managing your contracts that allows for renaming/deletion.
- Possibility to import already deployed contracts and call their transitions.
- Support for event in contracts, with automatic notifications in the UI.
- Support for arbitrary gas price/gas limit in deployment/calls.

## Project setup

```bash
npm install
```

### Compiles and hot-reloads for development

```bash
npm run serve
```

### Compiles and minifies for production

```bash
npm run build
```

### Run your tests

```bash
npm run test
```

### Lints and fixes files

```bash
npm run lint
```

### Customize configuration

See [Configuration Reference](https://cli.vuejs.org/config/).

## License

Neo-savant IDE is licenced under [GPLv3](LICENSE).

## Deploying applications with z (internal tool one-stop shop for the Zilliqa provisioning and deployment operations)

For more details about `z` please refer to the [documentation](https://github.com/Zilliqa/devops/blob/main/docs/z2.md).

## Deploying applications to localdev

Applications are specified in the `apps` stanzas of the `z.yaml` file.
A typical configuration looks something like this:

```yaml
backend: kind

clusters:
  cluster_name:
    apps:
      app1:
        path: products/app1/deployment
        track: development
        type: kustomize
      apps2:
        path: products/app2/development
        track: development
        type: kustomize
```

Clone the devops repo:

```sh
git clone https://github.com/Zilliqa/devops.git
cd devops
source setenv
```

Set the following environment variables:

- `Z_ENV` to the path in which your `z.yaml` resides.
- `ZQ_USER` to your username (the bit before `@` in your email address)

for example:

```sh
export Z_ENV=/path/to/z.yaml
export ZQ_USER=<user_id>@zilliqa.com
```

Build and push the image:

```sh
## from this repo base directory
cd ./products/neo-savant
make image/build
make image/push
```

And deploy the application with the:

```sh
z app sync
```

## Deploying applications to staging

Applications are specified in the `apps` and `registries` stanzas of
the `z.yaml` file. A typical configuration looks something like this:

```yaml
registries:
  staging: asia-docker.pkg.dev/prj-d-devops-services-4dgwlsse/zilliqa-pub
clusters:
  cluster_name:
    apps:
      app1:
        path: products/app1/deployment
        track: staging
        repo: https://github.com/zilliqa-internal
        type: kustomize
      apps2:
        path: products/app2/development
        track: staging
        repo: https://github.com/zilliqa-internal
        type: kustomize
```

### Clone the devops repo

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
   git checkout -b users/<username>/add_<application_name>_to_staging_cluster
   ```

1. In the file `infra/live/gcp/non-production/prj-d-staging/z_ase1.yaml` add the following:

   - in `apps` stanza add:

     ```yaml
     clusters:
       staging:
         apps:
           neo-savant:
             repo: https://github.com/Zilliqa/zilliqa-developer
             path: products/neo-savant/cd/overlays/staging
             track: staging
             type: kustomize
     ```

   - in `subdomains` stanza add:

     ```yaml
     infrastructure:
     dns:
       vars:
       subdomains:
       neo-savant-ide: {}
     ```

1. Push the changes

   ```sh
   git add .
   git commit -m "Add Neo Savant to staging cluster"
   git push origin users/<username>/add_neo_savant_to_staging_cluster
   ```

1. Open a Pull Request to the main branch

1. Apply the changes

   ```sh
   z plan
   z apply
   ```

### Deploy the application

```sh
z app sync --cache-dir=.cache neo-savant
```
