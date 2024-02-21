// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {Script} from "forge-std/Script.sol";
import {MintAndBurnTokenManagerUpgradeableV3} from "contracts/periphery/TokenManagerV3/MintAndBurnTokenManagerUpgradeableV3.sol";
import "forge-std/console.sol";

contract Unpause is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY_OWNER");
        address owner = vm.addr(deployerPrivateKey);
        console.log("Deployer is %s", owner);

        address tokenManagerAddress = 0xF391A1Ee7b3ccad9a9451D2B7460Ac646F899f23;

        vm.startBroadcast(deployerPrivateKey);
        MintAndBurnTokenManagerUpgradeableV3 tokenManager = MintAndBurnTokenManagerUpgradeableV3(
                tokenManagerAddress
            );
        tokenManager.unpause();
        console.log(
            "TokenManager %s, paused: %s",
            tokenManagerAddress,
            tokenManager.paused()
        );
        vm.stopBroadcast();
    }
}
