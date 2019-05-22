# Network Upgrade

This doc describe the way to conduct the token transfer to secure the mining rewards from our own mainnet nodes, including all these nodes:

- 5 lookup nodes
- 5 multipliers
- 180 normal nodes
- 700 new node

Of all these nodes, 5 lookup nodes and 5 multipliers are continuously receiving mining rewards so a periodical transfer is needed. The rest are no longer in the network so it's only one-time effort to secure the rewards and it has been done after the end of bootstrap.

## Upgrade on Separated Network

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

## Rolling Upgrade

Create a working directory. Some code and data will be stored here and this directory will be referred as **project root**.

```bash
mkdir mainnet-token-transfer
```

