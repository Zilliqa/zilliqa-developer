// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {Script} from "forge-std/Script.sol";
import {TokenManagerUpgradeable} from "contracts/periphery/TokenManagerUpgradeable.sol";
import {MintAndBurnTokenManagerUpgradeableV2} from "contracts/periphery/TokenManagerV2/MintAndBurnTokenManagerUpgradeableV2.sol";
import "forge-std/console.sol";

contract Upgrade is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY_TESTNET");
        address owner = vm.addr(deployerPrivateKey);
        console.log("Signer is %s", owner);

        // Constants
        address tokenManagerAddress = 0xA6D73210AF20a59832F264fbD991D2abf28401d0;
        uint fees = 0.00025 ether; // 0.00025 BNB

        TokenManagerUpgradeable tokenManager = TokenManagerUpgradeable(
            tokenManagerAddress
        );

        vm.startBroadcast(deployerPrivateKey);

        address implementationV2 = address(
            new MintAndBurnTokenManagerUpgradeableV2()
        );
        bytes memory encodedReinitializerCall = abi.encodeCall(
            MintAndBurnTokenManagerUpgradeableV2.reinitialize,
            (fees)
        );
        tokenManager.upgradeToAndCall(
            implementationV2,
            encodedReinitializerCall
        );

        vm.stopBroadcast();

        MintAndBurnTokenManagerUpgradeableV2 tokenManagerV2 = MintAndBurnTokenManagerUpgradeableV2(
                tokenManagerAddress
            );
        console.log("New fees are %s", tokenManagerV2.getFees());
    }
}
