// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {Script} from "forge-std/Script.sol";
import {Relayer} from "contracts/core/Relayer.sol";
import {ValidatorManager} from "contracts/core/ValidatorManager.sol";
import {MintAndBurnTokenManagerUpgradeable} from "contracts/periphery/MintAndBurnTokenManagerUpgradeable.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {ChainGateway} from "contracts/core/ChainGateway.sol";
import "forge-std/console.sol";

contract Deployment is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY_TESTNET");
        address owner = vm.addr(deployerPrivateKey);
        console.log("Owner is %s", owner);

        address[] memory validators = new address[](1);
        validators[0] = owner;

        vm.startBroadcast(deployerPrivateKey);

        // Deploy Validator Manager
        ValidatorManager validatorManager = new ValidatorManager{
            salt: "zilliqa"
        }(validators[0]);
        validatorManager.initialize(validators);
        console.log(
            "ValidatorManager Deployed: %s, owner is validator: %s, and size %s",
            address(validatorManager),
            validatorManager.isValidator(validators[0]),
            validatorManager.validatorsSize()
        );

        // Deploy Chain Gateway
        ChainGateway chainGateway = new ChainGateway{salt: "zilliqa"}(
            address(validatorManager),
            validators[0]
        );
        console.log(
            "ChainGateway Deployed: %s, with validatorManager %s",
            address(chainGateway),
            address(chainGateway.validatorManager())
        );

        // Deploy MintAndBurnTokenManager
        address implementation = address(
            new MintAndBurnTokenManagerUpgradeable()
        );
        bytes memory initializeData = abi.encodeCall(
            MintAndBurnTokenManagerUpgradeable.initialize,
            address(chainGateway)
        );

        address proxy = address(
            new ERC1967Proxy(implementation, initializeData)
        );

        MintAndBurnTokenManagerUpgradeable tokenManager = MintAndBurnTokenManagerUpgradeable(
                proxy
            );
        console.log(
            "MintAndBurnTokenManager Proxy deployed to %s, with owner %s and gateway %s",
            address(tokenManager),
            tokenManager.owner(),
            tokenManager.getGateway()
        );

        // Register TokenManager to ChainGateway
        chainGateway.register(proxy);
        console.log(
            "TokenManager %s, registered to %s ChainGateway: %s",
            address(tokenManager),
            address(chainGateway),
            chainGateway.registered(proxy)
        );

        vm.stopBroadcast();
    }
}
