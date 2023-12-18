// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {Script} from "forge-std/Script.sol";
import {Target} from "foundry/test/Target.sol";
import "forge-std/console.sol";

contract Verify is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address target = 0x287b0F2491653E5Cb93981AcF7fb30576480015D;
        vm.startBroadcast(deployerPrivateKey);

        uint x = Target(target).count();

        console.log(x);

        vm.stopBroadcast();
    }
}
