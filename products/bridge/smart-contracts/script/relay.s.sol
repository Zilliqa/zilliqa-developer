// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {Script} from "forge-std/Script.sol";
import {ChainGateway} from "contracts/core/ChainGateway.sol";
import {Target} from "foundry/test/Target.sol";

// Relayer Address: 0x46242abc24c6ba2d6B91a2a2e18008eeCac5eD71

contract Relay is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address chainGateway = 0x4DF88A0dF446b2cb14Ed57d12F48255758DE842a;
        address target = 0x287b0F2491653E5Cb93981AcF7fb30576480015D;
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
