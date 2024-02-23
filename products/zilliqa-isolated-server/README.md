# Dockerized Zilliqa Isolated Server

## Prerequisites

### Local Build

Docker installed, please follow the steps [here](https://docs.docker.com/get-docker/).

## Administration

### Building Isolated Server Container

#### Local Build

The isolated server can be built locally via the following command: <br />
`docker build -t isolated-server:1.0 .` <br />
there will be an image named isolated-server:1.0 built.

---

### Running Isolated Server Container

#### Local Run

There are a few methods to configure and run the isolated server, all of them requiring augmentations to the docker run command.

##### Running the isolated server as is:

```
docker run -d -p 5555:5555 \
  --name isolated-server \
  isolated-server:1.0
```

And the api will be available at `http://localhost:5555` <br />
The above command will launch a clean state isolated server with ephermeral storage and seeding all the default accounts via `boot.json`.

---

##### Running the isolated server with persistence

Enabling non-ephermeral persistence can be done by mounting a volume onto the container via the following argument `-v $(pwd)/persistence:/zilliqa/persistence`.

Do note however, there is now two different docker run commands. The first one is required on all **first** launch of Isolated Server with persistence storage.

```
docker run -d -p 5555:5555 \
  -v $(pwd)/persistence:/zilliqa/persistence \
  --name isolated-server \
  isolated-server:1.0
```

The following command must be run on all subsequent Isolated Server container launches. Note the addition of a new argument `--env MODE="load"`. This environment variable forces the Isolated Server to launch while attempting to load persistence from the container directory: `/zilliqa/persistence`.

```
docker run -d -p 5555:5555 \
  -v $(pwd)/persistence:/zilliqa/persistence \
  --env MODE="load" \
  --name isolated-server \
  isolated-server:1.0
```

---

##### Running the isolated server with manual block increase

Enabling manual mode can be done by the following command.

If manual mode is enabled, the following api call is available `IncreaseBlocknum`

Do note however, if you run in manual mode, persistence storage is not supported.

```
docker run -d -p 5555:5555 \
  --env MANUAL_MODE="true" \
  --name isolated-server \
  isolated-server:1.0
```

---

##### Additional Run Arguments

The Isolated Server run script also supports modifications to the following parameters
| environment variable | default value | description |
|---|---|---|
|$T|5000|The time before progressing each block in Isolated Server.
|$UUID|uuid|The uuid that's used as a verification for pausing and unpausing the Isolated server.

Docker run command overriding the 2 variables:

```
docker run -d -p 5555:5555 \
  --env T="2000" \
  --env UUID="randomstring" \
  --name isolated-server \
  isolated-server:1.0
```

---

##### Stopping the Isolated Server

```
docker stop isolated-server
```

##### Removing the Stopped Isolated Server

```
docker rm isolated-server
```

---

##### Available APIs

- `CreateTransaction` : Input a transaction json payload
- `GetLatestTxBlock` : Get the information on the latest tx block, not available in manual mode.
- `IncreaseBlocknum` : Increase the blocknum by a given input, only available in manual mode.
- `GetSmartContractSubState` : Get the state of a smart contract
- `GetSmartContractCode` : Get code at a given address
- `GetMinimumGasPrice` : Get the minimum gas price
- `SetMinimumGasPrice`: Set the minimum gas price
- `GetBalance`: Get balance and nonce of a account
- `GetSmartContracts`: get smart contract for an address
- `GetNetworkID`: get the network ID of the isolated server
- `GetSmartContractInit` : get init json for a SC.
- `GetTransaction `: Get Transaction info by hash

- `CheckPause`: Checks if the Isolated Server is paused. Requires UUID
- `TogglePause`: Toggles the Isolated Server between pause and unpause state. Requires UUID.

## Deploying applications with z

`z` is the one-stop shop for the Zilliqa provisioning and deployment operations. To deploy applications with z ensure the `z`
binary is installed in your operative system PATH environment variable. For more details about `z` please refer to the [documentation](https://github.com/Zilliqa/devops/blob/main/docs/z2.md).

## Deploying applications to localdev

To deploy the localdev/development environment go to the project folder in the zilliqa-internal repository:

```sh
cd ./products/zilliqa-isolated-server
```

The `./products/zilliqa-isolated-server/z.yaml` contains all the relevant configurations for the development environment.
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

Build and push the images:

```sh
make image/build-and-push
```

And deploy the application to your local cluster with:

```sh
z app sync
```

Verify your application is running correct from the `http://localhost` URL and with `kubectl` commands (if required).

## Deploying applications to production

To deploy the production environment we need to clone the devops repository and execute `z` from there:

```sh
git clone https://github.com/Zilliqa/devops.git
cd devops
source setenv
```

### Set the following environment variables (production)

- `Z_ENV` to the path in which your `z.yaml` resides.
- `ZQ_USER` to your username (the bit before `@` in your email address)
- `GITHUB_PAT` (if you are deploying staging or production apps) to a classic PAT with all the repo permissions ticked.

for example:

```sh
export Z_ENV=`pwd`/infra/live/gcp/production/prj-p-prod-apps/z_ase1.yaml
export ZQ_USER=<user_id>@zilliqa.com
export GITHUB_PAT=<GITHUB_PAT>
```

### Login to Google Cloud (production)

```sh
z login
```

### Login to 1password (production)

```sh
eval $(op signin)
```

### Add the application to the production `z.yaml` file. Skip this step if it is an existing application

1. Create a branch:

   ```sh
   git checkout -b users/<username>/add_zilliqa_isolated_-_server_to_production_cluster
   ```

2. In the file `infra/live/gcp/production/prj-p-prod-apps/z_ase1.yaml` add the following:

   - in `apps` stanza add:

     ```yaml
     clusters:
       production:
         apps:
           zilliqa-isolated-server:
             repo: https://github.com/Zilliqa/zilliqa-developer
             path: products/zilliqa-isolated-server/cd/overlays/production
             registry: public
             track: production
             type: kustomize
     ```

   - in `subdomains` stanza add:

     ```yaml
     infrastructure:
     dns:
       vars:
       subdomains:
         zilliqa-isolated-server: {}
     ```

3. Push the changes

   ```sh
   git add .
   git commit -m "Add zilliqa-isolated-server to production cluster"
   git push origin users/<username>/add_zilliqa_isolated_server_to_production_cluster
   ```

4. Open a Pull Request to the main branch

5. Apply the changes

   ```sh
   z plan
   z apply
   ```

### Deploy the application (production)

```sh
z app sync --cache-dir=.cache zilliqa-isolated-server
```

Verify your application is running correct from the staging URL and with `kubectl` commands (if required).
