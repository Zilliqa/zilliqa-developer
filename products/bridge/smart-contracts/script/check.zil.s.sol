// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {Script} from "forge-std/Script.sol";
import "forge-std/console.sol";
import {ValidatorManager} from "contracts/core/ValidatorManager.sol";
import {ChainGateway} from "contracts/core/ChainGateway.sol";

// Relayer Address: 0x46242abc24c6ba2d6B91a2a2e18008eeCac5eD71

contract Check is Script {
    function run() external {
        uint256 runnerPrivateKey = vm.envUint("PRIVATE_KEY_TEST");
        uint256 validator2 = vm.envUint("PRIVATE_KEY2");

        address validatorManagerAddress = 0xF74C8a0AF3B03d7135C7fFb816774f24d0053A3B;
        // address chainGatewayAddress = 0x3Be6E686397f04901Be15e3e02EDC0c7565e4b13;

        ValidatorManager validatorManager = ValidatorManager(
            validatorManagerAddress
        );
        // ChainGateway chainGateway = ChainGateway(chainGatewayAddress);

        vm.startBroadcast(runnerPrivateKey);

        validatorManager.addValidator(vm.addr(validator2));

        vm.stopBroadcast();
    }
}
