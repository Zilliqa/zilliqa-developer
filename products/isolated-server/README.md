# Dockerized Zilliqa Isolated Server

## Prerequisites

### Local Build

Docker installed, please follow the steps
[here](https://docs.docker.com/get-docker/).

## Administration

### Building Isolated Server Container

#### Local Build

The isolated server can be built locally via the following command:

```sh
docker build -t isolated-server:1.0 .
```

there will be an image named `isolated-server:1.0` built.

---

### Running Isolated Server Container

#### Local Run

There are a few methods to configure and run the isolated server, all of them
requiring augmentations to the docker run command.

##### Running the isolated server as is

```sh
docker run -d -p 5555:5555 \
  --name isolated-server \
  isolated-server:1.0
```

And the api will be available at `http://localhost:5555`.

The above command will launch a clean state isolated server with ephermeral
storage and seeding all the default accounts via `boot.json`.

---

##### Running the isolated server with persistence

Enabling non-ephermeral persistence can be done by mounting a volume onto the
container via the following argument
`-v $(pwd)/persistence:/zilliqa/persistence`.

Do note however, there is now two different docker run commands. The first one
is required on all **first** launch of Isolated Server with persistence storage.

```sh
docker run -d -p 5555:5555 \
  -v $(pwd)/persistence:/zilliqa/persistence \
  --name isolated-server \
  isolated-server:1.0
```

The following command must be run on all subsequent Isolated Server container
launches. Note the addition of a new argument `--env MODE="load"`. This
environment variable forces the Isolated Server to launch while attempting to
load persistence from the container directory: `/zilliqa/persistence`.

```sh
docker run -d -p 5555:5555 \
  -v $(pwd)/persistence:/zilliqa/persistence \
  --env MODE="load" \
  --name isolated-server \
  isolated-server:1.0
```

---

##### Running the isolated server with manual block increase

Enabling manual mode can be done by the following command.

If manual mode is enabled, the following api call is available
`IncreaseBlocknum`

Do note however, if you run in manual mode, persistence storage is not
supported.

```sh
docker run -d -p 5555:5555 \
  --env MANUAL_MODE="true" \
  --name isolated-server \
  isolated-server:1.0
```

---

##### Additional Run Arguments

The Isolated Server run script also supports modifications to the following
parameters

| environment variable | default value | description                                                                          |
| -------------------- | ------------- | ------------------------------------------------------------------------------------ |
| $T                   | 5000          | The time before progressing each block in Isolated Server.                           |
| $UUID                | uuid          | Theuuid that's used as a verification for pausing and unpausing the Isolated server. |

Docker run command overriding the 2 variables:

```sh
docker run -d -p 5555:5555 \
  --env T="2000" \
  --env UUID="randomstring" \
  --name isolated-server \
  isolated-server:1.0
```

---

##### Stopping the Isolated Server

```sh
docker stop isolated-server
```

##### Removing the Stopped Isolated Server

```sh
docker rm isolated-server
```

---

##### Available APIs

- `CreateTransaction` : Input a transaction json payload
- `GetLatestTxBlock` : Get the information on the latest tx block, not available
  in manual mode.
- `IncreaseBlocknum` : Increase the blocknum by a given input, only available in
  manual mode.
- `GetSmartContractSubState` : Get the state of a smart contract
- `GetSmartContractCode` : Get code at a given address
- `GetMinimumGasPrice` : Get the minimum gas price
- `SetMinimumGasPrice`: Set the minimum gas price
- `GetBalance`: Get balance and nonce of a account
- `GetSmartContracts`: get smart contract for an address
- `GetNetworkID`: get the network ID of the isolated server
- `GetSmartContractInit` : get init json for a SC.
- `GetTransaction`: Get Transaction info by hash

- `CheckPause`: Checks if the Isolated Server is paused. Requires UUID
- `TogglePause`: Toggles the Isolated Server between pause and unpause state.
  Requires UUID.
