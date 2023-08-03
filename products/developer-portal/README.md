# Developer portal

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
cd ./products/developer-portal
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
   git checkout -b users/<username>/add_developer_portal_to_staging_cluster
   ```

1. In the file `infra/live/gcp/non-production/prj-d-staging/z_ase1.yaml` add the following:

   - in `apps` stanza add:

     ```yaml
     clusters:
       staging:
         apps:
           developer-portal:
             repo: https://github.com/Zilliqa/zilliqa-developer
             path: products/developer-portal/cd/overlays/staging
             track: staging
             type: kustomize
     ```

   - in `subdomains` stanza add:

     ```yaml
     infrastructure:
     dns:
       vars:
       subdomains:
         zilliqa-devportal: {}
     ```

1. Push the changes

   ```sh
   git add .
   git commit -m "Add Developer Portal to staging cluster"
   git push origin users/<username>/add_developer_portal_to_staging_cluster
   ```

1. Open a Pull Request to the main branch

1. Apply the changes

   ```sh
   z plan
   z apply
   ```

### Deploy the application

```sh
z app sync --cache-dir=.cache developer-portal
```
