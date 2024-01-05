// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {Script} from "forge-std/Script.sol";
import {ChainGateway} from "contracts/core/ChainGateway.sol";
import {Target} from "foundry/test/Target.sol";

// Relayer Address: 0x46242abc24c6ba2d6B91a2a2e18008eeCac5eD71

contract Relay is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address chainGateway = 0x4a50E1Be218279ca980EAb216d95E0A48B5aE872;
        address target = 0x9cB4b20da1fA0caA96221aD7a80139DdbBEC266e;
        vm.startBroadcast(deployerPrivateKey);

        ChainGateway(chainGateway).relay(
            block.chainid == 1 ? 2 : 1,
            target,
            abi.encodeWithSelector(Target.increment.selector),
            1_000_000
        );

        vm.stopBroadcast();
    }
}
