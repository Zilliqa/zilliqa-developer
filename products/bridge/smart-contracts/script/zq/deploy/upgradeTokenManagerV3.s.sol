// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {Script} from "forge-std/Script.sol";
import {TokenManagerUpgradeable} from "contracts/periphery/TokenManagerUpgradeable.sol";
import {LockAndReleaseTokenManagerUpgradeableV3} from "contracts/periphery/TokenManagerV3/LockAndReleaseTokenManagerUpgradeableV3.sol";
import "forge-std/console.sol";

contract Upgrade is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY_OWNER");
        address owner = vm.addr(deployerPrivateKey);
        console.log("Signer is %s", owner);

        // Constants
        address tokenManagerAddress = 0x6D61eFb60C17979816E4cE12CD5D29054E755948;

        TokenManagerUpgradeable tokenManager = TokenManagerUpgradeable(
            tokenManagerAddress
        );

        vm.startBroadcast(deployerPrivateKey);

        address newImplementation = address(
            new LockAndReleaseTokenManagerUpgradeableV3()
        );
        bytes memory encodedReinitializerCall = "";
        tokenManager.upgradeToAndCall(
            newImplementation,
            encodedReinitializerCall
        );

        LockAndReleaseTokenManagerUpgradeableV3 tokenManagerV3 = LockAndReleaseTokenManagerUpgradeableV3(
                tokenManagerAddress
            );
        console.log("Pending Owner is %s", tokenManagerV3.pendingOwner());

        vm.stopBroadcast();
    }
}
