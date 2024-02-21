// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {Script} from "forge-std/Script.sol";
import {LockAndReleaseTokenManagerUpgradeableV3} from "contracts/periphery/TokenManagerV3/LockAndReleaseTokenManagerUpgradeableV3.sol";
import "forge-std/console.sol";

contract Update is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY_OWNER");
        address owner = vm.addr(deployerPrivateKey);
        console.log("Deployer is %s", owner);

        address newChainGateway = 0xbA44BC29371E19117DA666B729A1c6e1b35DDb40; // UPDATE;
        address tokenManagerAddress = 0x6D61eFb60C17979816E4cE12CD5D29054E755948;

        vm.startBroadcast(deployerPrivateKey);
        LockAndReleaseTokenManagerUpgradeableV3 tokenManager = LockAndReleaseTokenManagerUpgradeableV3(
                tokenManagerAddress
            );
        tokenManager.setGateway(newChainGateway);
        console.log(
            "TokenManager %s, newChainGateway: %s",
            tokenManagerAddress,
            tokenManager.getGateway()
        );
        vm.stopBroadcast();
    }
}
