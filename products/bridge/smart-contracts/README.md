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
- `ChainGateway`: `0xbA44BC29371E19117DA666B729A1c6e1b35DDb40`
- `ValidatorManager`: `0x71f3AD7cA177818399C9d79d74A6b284E4BEAAc9`
- `LockAndReleaseTokenManager`: `0x6D61eFb60C17979816E4cE12CD5D29054E755948`

Deterministic deployer not available, simple CREATE was used

### BSC Mainnet

- `ChainId`: 56
- `ChainGateway`: `0x3967f1a272Ed007e6B6471b942d655C802b42009`
- `ValidatorManager`: `0x936feD44EC4F46CE08158B536Df2f864c30C4b5F`
- `MintAndBurnTokenManager`: `0xF391A1Ee7b3ccad9a9451D2B7460Ac646F899f23`

`ChainGateway` and `ValidatorManager` were deployed using deterministic deployer [0x4e59b44847b379578588920ca78fbf26c0b4956c](https://github.com/Arachnid/deterministic-deployment-proxy). The salt used was `zilliqa`

### Zilliqa Testnet

- `ChainId`: 33101
- `ChainGateway`: `0x7370e69565BB2313C4dA12F9062C282513919230`
- `ValidatorManager`: `0x782F8afa1bA3137a214D49840688215a8A379fA8`
- `LockAndReleaseTokenManager`: `0x1509988c41f02014aA59d455c6a0D67b5b50f129`
- `TestTokenZRC2Proxy`: `0x8618d39a8276D931603c6Bc7306af6A53aD2F1F3`

Deterministic deployer not available, simple CREATE was used

### BSC Testnet

- `ChainId`: 97
- `ChainGateway`: `0xa9A14C90e53EdCD89dFd201A3bF94D867f8098fE`
- `ValidatorManager`: `0xCc1CB36d981ae2907cea385F615e879434D20B1C`
- `MintAndBurnTokenManager`: `0xA6D73210AF20a59832F264fbD991D2abf28401d0`
- `BridgedTestToken`: `0x5190e8b4Bbe8C3a732BAdB600b57fD42ACbB9F4B`
