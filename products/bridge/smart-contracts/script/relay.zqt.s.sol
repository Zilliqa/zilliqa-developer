// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {Script} from "forge-std/Script.sol";
import {ChainGateway} from "contracts/core/ChainGateway.sol";
import {Target} from "foundry/test/Target.sol";

contract Relay is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY_TESTNET");
        address chainGateway = 0x9e7FF4479511B8C497860633f3E136Fa21C99f5A;
        address target = 0x47eF3cc8B9D54db3A6d8D0175eb117D694575509;
        uint targetChainId = 97;

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
