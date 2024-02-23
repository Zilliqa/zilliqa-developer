// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {Script} from "forge-std/Script.sol";
import {LockAndReleaseTokenManagerUpgradeableV3} from "contracts/periphery/TokenManagerV3/LockAndReleaseTokenManagerUpgradeableV3.sol";
import "forge-std/console.sol";

contract Update is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY_TESTNET");
        address owner = vm.addr(deployerPrivateKey);
        console.log("Deployer is %s", owner);

        address newChainGateway = 0x7370e69565BB2313C4dA12F9062C282513919230; // UPDATE;
        address tokenManagerAddress = 0x1509988c41f02014aA59d455c6a0D67b5b50f129;

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
