// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {Script} from "forge-std/Script.sol";
import {TokenManagerUpgradeable} from "contracts/periphery/TokenManagerUpgradeable.sol";
import {LockAndReleaseTokenManagerUpgradeableV2} from "contracts/periphery/TokenManagerV2/LockAndReleaseTokenManagerUpgradeableV2.sol";
import "forge-std/console.sol";

contract Upgrade is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY_TEST");
        address owner = vm.addr(deployerPrivateKey);
        console.log("Signer is %s", owner);

        // Constants
        address tokenManagerAddress = 0x6D61eFb60C17979816E4cE12CD5D29054E755948;
        uint fees = 60 ether; // 60 ZIL

        TokenManagerUpgradeable tokenManager = TokenManagerUpgradeable(
            tokenManagerAddress
        );

        vm.startBroadcast(deployerPrivateKey);

        address implementationV2 = address(
            new LockAndReleaseTokenManagerUpgradeableV2()
        );
        bytes memory encodedReinitializerCall = abi.encodeCall(
            LockAndReleaseTokenManagerUpgradeableV2.reinitialize,
            (fees)
        );
        tokenManager.upgradeToAndCall(
            implementationV2,
            encodedReinitializerCall
        );

        LockAndReleaseTokenManagerUpgradeableV2 tokenManagerV2 = LockAndReleaseTokenManagerUpgradeableV2(
                tokenManagerAddress
            );
        console.log("New fees are %s", tokenManagerV2.getFees());

        vm.stopBroadcast();
    }
}
