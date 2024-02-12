// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {Script} from "forge-std/Script.sol";
import {ChainGateway} from "contracts/core/ChainGateway.sol";
import {Target} from "foundry/test/Target.sol";

contract Relay is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY_TESTNET");
        address chainGateway = 0xCD6D04BB823cBBEd853B43DFe421Be47fc49AbC5;
        address target = 0x6da9913794Fabe2482CB40cC9d7368f8332A998f;
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
