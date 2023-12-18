# Testing information

Once the bridge infrastructure has been setup as according to [here](../bridge-validators/README.md), use the following test commands:

1. Deploy contracts on both chains:

```sh
# Deploy on chainid 1
forge script script/deploy.s.sol:Deploy --fork-url http://localhost:8545 --broadcast
```

```sh
# Deploy on chainid 2
forge script script/deploy.s.sol:Deploy --fork-url http://localhost:8546 --broadcast
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
