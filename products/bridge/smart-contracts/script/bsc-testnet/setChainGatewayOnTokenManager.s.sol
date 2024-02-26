// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {Script} from "forge-std/Script.sol";
import {MintAndBurnTokenManagerUpgradeableV3} from "contracts/periphery/TokenManagerV3/MintAndBurnTokenManagerUpgradeableV3.sol";
import "forge-std/console.sol";

contract Update is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY_TESTNET");
        address owner = vm.addr(deployerPrivateKey);
        console.log("Deployer is %s", owner);

        address newChainGateway = 0xa9A14C90e53EdCD89dFd201A3bF94D867f8098fE; // UPDATE;
        address tokenManagerAddress = 0xA6D73210AF20a59832F264fbD991D2abf28401d0;

        vm.startBroadcast(deployerPrivateKey);
        MintAndBurnTokenManagerUpgradeableV3 tokenManager = MintAndBurnTokenManagerUpgradeableV3(
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
