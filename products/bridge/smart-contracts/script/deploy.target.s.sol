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
        address chainGateway = 0xCD6D04BB823cBBEd853B43DFe421Be47fc49AbC5;

        vm.startBroadcast(deployerPrivateKey);

        new Target{salt: "salt"}(chainGateway);

        vm.stopBroadcast();
    }
}
