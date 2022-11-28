# Mainnet Token Transfer

This doc describe the way to conduct the token transfer to secure the mining rewards from our own mainnet nodes, including all these nodes:

- 5 lookup nodes
- 5 multipliers
- 180 normal nodes
- 700 new node

Of all these nodes, 5 lookup nodes and 5 multipliers are continuously receiving mining rewards so a periodical transfer is needed. The rest are no longer in the network so it's only one-time effort to secure the rewards and it has been done after the end of bootstrap.

## Requirement

- Tools:

  - Docker
  - [zilliqa-cli](https://github.com/Zilliqa/zilliqa-cli)

- Keypair files:

  - lookup node keypair file (e.g., `lookup_keys.txt`)
  - multiplier keypair file (e.g., `multiplier_keys.txt`)
  - normal node keypair file
  - new node keypair file

- 2 Cold wallet address
- Zilliqa network API endpoint (e.g., `https://api.zilliqa.com`)

## Security

If you are transfering token for mainnet, you need to log in to the mainnet bastion as the credentials cannot leave the machine. Therefore, all the operation has to be done on the mainnet bastion.

Also, make sure the cold wallet addresses are correct and ready for use.

## Working directory

Create a working directory. Some code and data will be stored here and this directory will be referred as **project root**.

```bash
mkdir mainnet-token-transfer
```

## Preparing data

In the project root, clone this [gist](https://gist.github.com/Gnnng/90768383244012fba2adf9e72619fcc7) to `prepare` folder.

```bash
git clone https://gist.github.com/90768383244012fba2adf9e72619fcc7.git prepare
```

In the `prepare` folder, save the following content into `prepare.sh`. This is a wrapper for `prpare.py`. To customize, see `./prepare.py -h` to have a better understanding of the options.

```bash
#!/bin/bash

# Change this to fit your need
testnet=/home/ubuntu/testnet/testnet-expo-rolling/configmap

cat $testnet/keys.txt | tail -n +421 | head -n180 > n180.txt

./prepare.py \
        --lookup $testnet/lookup_keys.txt \
        --lookup $testnet/multiplier_keys.txt  \
        --normal n180.txt \
        --normal new_keys.txt \
        -m mapping.json \
        -o from.xml \
        --to 422c85ab78F955776898C646F4A81A2d4c0b0f4d \
        --to d942c5606f3Fb2E34F1C0933C9406F0453bE7f9A
```

Edit the file `prepare.sh` to make sure

- The variable `testnet` points to the correct path of the configmap.
- `new_keys.txt` is created, which contains the new node keypairs.
- `--to` option is set and two cold wallet addresses are passed to this option.

Simply run `./run.sh` to generate the following file needed by `zilliqa-cli priority-trasfer`. Be patient and it may take a while to finish.

- `from.xml`
- `mapping.json`

## Transfering token

In the project root, clone the private repository `zilliqa-cli`. Installation is not needed as we will run this in docker environment.

> Access permission will be asked for cloing `zilliqa-cli` as it's private repository.

```bash
cd mainnet
git clone https://github.com/Zilliqa/zilliqa-cli/
```

Run the following commands from the project root.

```bash
docker run --rm -it \
  -e MSG_VERSION=1 \
  -e CHAIN_ID=1 \
  -e API_URL=https://api.zilliqa.com \
  -v $(pwd)/zilliqa-cli:/zilliqa-cli \
  -v $(pwd)/prepare:/prepare \
  node:11 \
  /prepare/transfer.sh
```

## Troubleshooting

If you want to get into the running container and debugging. Run the following to have a terminal session. The `--rm` option will ensure the container will be removed after you exit the session.

```console
$ docker run --rm -it -v $(pwd)/zilliqa-cli:/zilliqa-cli -v $(pwd)/prepare:/prepare node:11 bash
root@3aa7cf6b4c57:/# exit
```
