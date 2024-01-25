// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {Script} from "forge-std/Script.sol";
import {ChainGateway} from "contracts/core/ChainGateway.sol";
import {Target} from "foundry/test/Target.sol";

contract Relay is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY_TEST");
        address chainGateway = 0x3Be6E686397f04901Be15e3e02EDC0c7565e4b13;
        address target = 0xFC99557Ca42B3139f7a0eDAcCF84985235631815;
        uint targetChainId = 33101;

        vm.startBroadcast(deployerPrivateKey);

        ChainGateway(chainGateway).relay(
            targetChainId,
            target,
            abi.encodeWithSelector(Target.increment.selector),
            1_000_000
        );

        vm.stopBroadcast();
    }
}
