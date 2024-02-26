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

        address tokenManagerAddress = 0xd10077bCE4A9D19068965dE519CED8a2fC1B096C;
        address tokenAddress = 0x63B6ebD476C84bFDd5DcaCB3f974794FC6C2e721;

        uint remoteChainId = 97;
        address remoteRecipient = owner;
        uint amount = 10;

        ERC20 token = ERC20(tokenAddress);
        LockAndReleaseTokenManagerUpgradeable tokenManager = LockAndReleaseTokenManagerUpgradeable(
                tokenManagerAddress
            );

        vm.startBroadcast(deployerPrivateKey);
        console.log(
            "Owner Balance: %d, TokenManagerBalance %d, %s",
            token.balanceOf(owner),
            token.balanceOf(tokenManagerAddress),
            token.name()
        );

        token.approve(tokenManagerAddress, amount);
        tokenManager.transfer(
            tokenAddress,
            remoteChainId,
            remoteRecipient,
            amount
        );

        console.log(
            "New Owner Balance: %d, TokenManagerBalance %d",
            token.balanceOf(owner),
            token.balanceOf(tokenManagerAddress)
        );
        vm.stopBroadcast();
    }
}
