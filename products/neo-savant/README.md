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

## Deploying applications with z

`z` is the one-stop shop for the Zilliqa provisioning and deployment operations. To deploy applications with z ensure the `z`
binary is installed in your operative system PATH environment variable. For more details about `z` please refer to the [documentation](https://github.com/Zilliqa/devops/blob/main/docs/z2.md).

## Deploying applications to localdev

To deploy the localdev/development environment go to the project folder in the zilliqa-developer repository:

```sh
cd ./products/neo-savant
```

The `./products/neo-savant/z.yaml` contains all the relevant configurations for the development environment.
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
   git checkout -b users/<username>/add_neo_savant_to_staging_cluster
   ```

2. In the file `infra/live/gcp/non-production/prj-d-staging/z_ase1.yaml` add the following:

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

3. Push the changes

   ```sh
   git add .
   git commit -m "Add Neo Savant to staging cluster"
   git push origin users/<username>/add_neo_savant_to_staging_cluster
   ```

4. Open a Pull Request to the main branch

5. Apply the changes

   ```sh
   z plan
   z apply
   ```

### Deploy the application

```sh
z app sync --cache-dir=.cache neo-savant
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
   git checkout -b users/<username>/add_neo_savant_to_production_cluster
   ```

2. In the file `infra/live/gcp/production/prj-p-prod-apps/z_ase1.yaml` add the following:

   - in `apps` stanza add:

     ```yaml
     clusters:
       production:
         apps:
           neo-savant:
             repo: https://github.com/Zilliqa/zilliqa-developer
             path: products/neo-savant/cd/overlays/production
             track: production
             type: kustomize
     ```

   - in `subdomains` stanza add:

     ```yaml
     infrastructure:
     dns:
       vars:
       subdomains:
         ide: {}
     ```

3. Push the changes

   ```sh
   git add .
   git commit -m "Add Neo Savant to production cluster"
   git push origin users/<username>/add_neo_savant_to_production_cluster
   ```

4. Open a Pull Request to the main branch

5. Apply the changes

   ```sh
   z plan
   z apply
   ```

### Deploy the application

```sh
z app sync --cache-dir=.cache neo-savant
```

Verify your application is running correct from the production URL and with `kubectl` commands (if required).
