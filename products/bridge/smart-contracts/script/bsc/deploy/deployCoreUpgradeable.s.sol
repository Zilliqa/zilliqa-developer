// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {Script} from "forge-std/Script.sol";
import {ValidatorManagerUpgradeable} from "contracts/core-upgradeable/ValidatorManagerUpgradeable.sol";
import {ChainGatewayUpgradeable} from "contracts/core-upgradeable/ChainGatewayUpgradeable.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import "forge-std/console.sol";

contract Deployment is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY_OWNER");
        // Address: 0x5807b970DB344d9d2386BbF5c3ec4cDA5CCdF1C8
        address owner = vm.addr(deployerPrivateKey);
        console.log("Owner is %s", owner);

        address[] memory validators = new address[](1);
        address tokenManager = 0xF391A1Ee7b3ccad9a9451D2B7460Ac646F899f23;
        validators[0] = owner;

        vm.startBroadcast(deployerPrivateKey);

        // Deploy Validator Manager
        address vmImplementation = address(
            new ValidatorManagerUpgradeable{salt: "zilliqa"}()
        );
        bytes memory vmInitCall = abi.encodeWithSelector(
            ValidatorManagerUpgradeable.initialize.selector,
            owner,
            validators
        );
        address vmProxy = address(
            new ERC1967Proxy{salt: "zilliqa"}(vmImplementation, vmInitCall)
        );
        ValidatorManagerUpgradeable validatorManager = ValidatorManagerUpgradeable(
                vmProxy
            );
        console.log(
            "ValidatorManager Deployed: %s, owner is validator: %s, and size %s",
            address(validatorManager),
            validatorManager.isValidator(validators[0]),
            validatorManager.validatorsSize()
        );

        // Deploy Chain Gateway
        address cgImplementation = address(
            new ChainGatewayUpgradeable{salt: "zilliqa"}()
        );
        bytes memory cgInitCall = abi.encodeWithSelector(
            ChainGatewayUpgradeable.initialize.selector,
            address(validatorManager),
            owner
        );
        address cgProxy = address(
            new ERC1967Proxy{salt: "zilliqa"}(cgImplementation, cgInitCall)
        );
        ChainGatewayUpgradeable chainGateway = ChainGatewayUpgradeable(cgProxy);
        console.log(
            "ChainGateway Deployed: %s, with validatorManager %s",
            address(chainGateway),
            address(chainGateway.validatorManager())
        );

        // Register TokenManager to ChainGateway
        chainGateway.register(tokenManager);
        console.log(
            "TokenManager %s, registered to %s ChainGateway: %s",
            address(tokenManager),
            address(chainGateway),
            chainGateway.registered(tokenManager)
        );

        vm.stopBroadcast();
    }
}
