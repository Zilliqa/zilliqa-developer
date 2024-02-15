# snapshot-hub

REST API for snapshot frontend.

## Installation

```
npm install
```

## Start server

```
npm start
```

### Environment variables

- `PINATA_API_KEY` - API key of [pinata](https://pinata.cloud/).
- `PINATA_SECRET_API_KEY` - SECRET key of [pinata](https://pinata.cloud/).
- `NODE_ENV` - development, test or production
- `POSTGRES_DB` # DataBase name.
- `POSTGRES_PASSWORD` # DataBase password.
- `POSTGRES_USER` # DataBase username.
- `POSTGRES_HOST` # DataBase host for example (127.0.0.1) for production build use 'postgres'.

## Setup database

- sequelize config - `lib/config/sequelize.ts`

## Build docker container

```bash
$ docker-compose build # run building.
$ docker-compose up -d # runing.
```

## Snapshot Spaces Commit

https://github.com/Zilliqa/snapshot-spaces/commit/238e87aad231351a51727b06208ab407f0de1dcc

20220216

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
   git checkout -b users/<username>/add_governance-api_to_staging_cluster
   ```

2. In the file `infra/live/gcp/non-production/prj-d-staging/z_ase1.yaml` add the following:

   - in `apps` stanza add:

     ```yaml
     clusters:
       staging:
         apps:
           governance-api:
             repo: https://github.com/Zilliqa/zilliqa-developer
             path: products/governance-api/cd/overlays/staging
             track: staging
             type: kustomize
     ```

   - in `subdomains` stanza add:

     ```yaml
     infrastructure:
     dns:
       vars:
       subdomains:
         governance-api: {}
     ```

3. Push the changes

   ```sh
   git add .
   git commit -m "Add governance-api to staging cluster"
   git push origin users/<username>/add_governance-api_to_staging_cluster
   ```

4. Open a Pull Request to the main branch

5. Apply the changes

   ```sh
   z plan
   z apply
   ```

### Deploy the application

```sh
z app sync --cache-dir=.cache governance-api
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
   git checkout -b users/<username>/add_governance-api_to_production_cluster
   ```

2. In the file `infra/live/gcp/production/prj-p-prod-apps/z_ase1.yaml` add the following:

   - in `apps` stanza add:

     ```yaml
     clusters:
       production:
         apps:
           governance-api:
             repo: https://github.com/Zilliqa/zilliqa-developer
             path: products/governance-api/cd/overlays/production
             track: production
             type: kustomize
     ```

   - in `subdomains` stanza add:

     ```yaml
     infrastructure:
     dns:
       vars:
       subdomains:
         governance-api: {}
     ```

3. Push the changes

   ```sh
   git add .
   git commit -m "Add governance-api to production cluster"
   git push origin users/<username>/add_governance-api_to_production_cluster
   ```

4. Open a Pull Request to the main branch

5. Apply the changes

   ```sh
   z plan
   z apply
   ```

### Deploy the application

```sh
z app sync --cache-dir=.cache governance-api
```

Verify your application is running correct from the production URL and with `kubectl` commands (if required).
