# Testing information

Once the bridge infrastructure has been setup as according to [here](../bridge-validators/README.md), use the following test commands:

1. Deploy contracts on both chains:

```sh
# Deploy on chainid 1
forge script script/deploy.s.sol:Deployment --fork-url http://localhost:8545 --broadcast
```

```sh
# Deploy on chainid 2
forge script script/deploy.s.sol:Deployment --fork-url http://localhost:8546 --broadcast
```

2.Run `relay` to relay messages across:

```sh
# Send message from chain 1 to 2
forge script script/relay.s.sol:Relay --fork-url http://localhost:8545 --broadcast
```

```sh
# Send message from chain 2 to 1
forge script script/relay.s.sol:Relay --fork-url http://localhost:8546 --broadcast
```

3.Verify if test relay had worked:

```sh
# Verify message received in chain 1 from 2
 forge script script/verify.s.sol:Verify --fork-url http://localhost:8545 --broadcast
```

```sh
# Verify message received in chain 2 from 1
 forge script script/verify.s.sol:Verify --fork-url http://localhost:8546 --broadcast
```

Depending on how many times relay has been run, it should show that many times on the respective target chains

## Contract Addresses

### Zilliqa Mainnet

- `ChainId`: 32769
- `ChainGateway`: `0xE76669e1cCc150194eB92581baE79Ef6fa0E248E`
- `ValidatorManager`: `0xF391A1Ee7b3ccad9a9451D2B7460Ac646F899f23`
- `LockAndReleaseTokenManager`: `0x6D61eFb60C17979816E4cE12CD5D29054E755948`

Deterministic deployer not available, simple CREATE was used

### BSC Mainnet

- `ChainId`: 56
- `ChainGateway`: `0x2114e979b7CFDd8b358502e00f50Fd5f7787Fe63`
- `ValidatorManager`: `0x5EDE85Ee7B2b4aefA88505Aa3893c1628FCeB0CE`
- `MintAndBurnTokenManager`: `0xF391A1Ee7b3ccad9a9451D2B7460Ac646F899f23`

`ChainGateway` and `ValidatorManager` were deployed using deterministic deployer [0x4e59b44847b379578588920ca78fbf26c0b4956c](https://github.com/Arachnid/deterministic-deployment-proxy). The salt used was `zilliqa`
