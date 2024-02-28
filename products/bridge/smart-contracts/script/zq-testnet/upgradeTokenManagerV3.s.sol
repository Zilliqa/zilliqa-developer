// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {Script} from "forge-std/Script.sol";
import {TokenManagerUpgradeable} from "contracts/periphery/TokenManagerUpgradeable.sol";
import {LockAndReleaseTokenManagerUpgradeableV3} from "contracts/periphery/TokenManagerV3/LockAndReleaseTokenManagerUpgradeableV3.sol";
import "forge-std/console.sol";

contract Upgrade is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY_TESTNET");
        address owner = vm.addr(deployerPrivateKey);
        console.log("Signer is %s", owner);

        // Constants
        address tokenManagerAddress = 0x1509988c41f02014aA59d455c6a0D67b5b50f129;

        TokenManagerUpgradeable tokenManager = TokenManagerUpgradeable(
            tokenManagerAddress
        );

        vm.startBroadcast(deployerPrivateKey);

        address implementationV2 = address(
            new LockAndReleaseTokenManagerUpgradeableV3()
        );
        bytes memory encodedReinitializerCall = "";
        tokenManager.upgradeToAndCall(
            implementationV2,
            encodedReinitializerCall
        );

        LockAndReleaseTokenManagerUpgradeableV3 tokenManagerV3 = LockAndReleaseTokenManagerUpgradeableV3(
                tokenManagerAddress
            );
        console.log("Pending Owner is %s", tokenManagerV3.pendingOwner());

        vm.stopBroadcast();
    }
}
