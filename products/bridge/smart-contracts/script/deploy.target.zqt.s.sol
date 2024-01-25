// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {Script} from "forge-std/Script.sol";
import {Relayer} from "contracts/core/Relayer.sol";
import {ValidatorManager} from "contracts/core/ValidatorManager.sol";
import {ChainGateway} from "contracts/core/ChainGateway.sol";
import {Target} from "foundry/test/Target.sol";
import "forge-std/console.sol";

contract Deployment is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY_TEST");
        address chainGateway = 0x3Be6E686397f04901Be15e3e02EDC0c7565e4b13;

        vm.startBroadcast(deployerPrivateKey);

        new Target(chainGateway);

        vm.stopBroadcast();
    }
}
