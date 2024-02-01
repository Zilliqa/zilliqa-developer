// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {Script} from "forge-std/Script.sol";
import {LockAndReleaseTokenManagerUpgradeable} from "contracts/periphery/LockAndReleaseTokenManagerUpgradeable.sol";
import {ITokenManagerStructs} from "contracts/periphery/TokenManagerUpgradeable.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "forge-std/console.sol";

contract Transfer is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY_TESTNET");
        address owner = vm.addr(deployerPrivateKey);
        console.log("Owner is %s", owner);

        address tokenManagerAddress = 0x1509988c41f02014aA59d455c6a0D67b5b50f129;
        address tokenAddress = 0x8618d39a8276D931603c6Bc7306af6A53aD2F1F3;

        uint remoteChainId = 97;
        address remoteRecipient = owner;
        uint amount = 1000;

        ERC20 token = ERC20(tokenAddress);
        LockAndReleaseTokenManagerUpgradeable tokenManager = LockAndReleaseTokenManagerUpgradeable(
                tokenManagerAddress
            );

        console.log(
            "Owner Balance: %d, TokenManagerBalance %d, %s",
            token.balanceOf(owner),
            token.balanceOf(tokenManagerAddress),
            token.name()
        );

        vm.startBroadcast(deployerPrivateKey);

        token.approve(tokenManagerAddress, amount);
        tokenManager.transfer(
            tokenAddress,
            remoteChainId,
            remoteRecipient,
            amount
        );

        vm.stopBroadcast();

        console.log(
            "New Owner Balance: %d, TokenManagerBalance %d",
            token.balanceOf(owner),
            token.balanceOf(tokenManagerAddress)
        );
    }
}
