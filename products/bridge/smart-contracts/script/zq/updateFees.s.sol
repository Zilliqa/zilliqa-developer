// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {Script} from "forge-std/Script.sol";
import {TokenManagerUpgradeableV2} from "contracts/periphery/TokenManagerV2/TokenManagerUpgradeableV2.sol";
import "forge-std/console.sol";

contract Update is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY_TEST");
        address tokenManagerAddress = 0x6D61eFb60C17979816E4cE12CD5D29054E755948;
        // uint newFees = 1 ether;
        uint newFees = 60 ether;

        TokenManagerUpgradeableV2 tokenManager = TokenManagerUpgradeableV2(
            tokenManagerAddress
        );
        vm.startBroadcast(deployerPrivateKey);

        tokenManager.setFees(newFees);

        console.log("New fees are", tokenManager.getFees());

        vm.stopBroadcast();
    }
}
