// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {Script} from "forge-std/Script.sol";
import {MintAndBurnTokenManagerUpgradeable} from "contracts/periphery/MintAndBurnTokenManagerUpgradeable.sol";
import {ITokenManagerStructs} from "contracts/periphery/TokenManagerUpgradeable.sol";
import {BridgedToken} from "contracts/periphery/BridgedToken.sol";
import "forge-std/console.sol";

contract Deployment is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY_TESTNET");
        address owner = vm.addr(deployerPrivateKey);
        console.log("Owner is %s", owner);

        address tokenManagerAddress = 0xA6D73210AF20a59832F264fbD991D2abf28401d0;

        string memory tokenName = "test token";
        string memory tokenSymbol = "TST";
        uint8 tokenDecimals = 3;

        address remoteToken = 0x8618d39a8276D931603c6Bc7306af6A53aD2F1F3;
        address remoteTokenManager = 0x1509988c41f02014aA59d455c6a0D67b5b50f129;
        uint remoteChainId = 33101;

        MintAndBurnTokenManagerUpgradeable tokenManager = MintAndBurnTokenManagerUpgradeable(
                tokenManagerAddress
            );

        vm.recordLogs();
        vm.startBroadcast(deployerPrivateKey);

        BridgedToken token = tokenManager.deployToken(
            tokenName,
            tokenSymbol,
            tokenDecimals,
            remoteToken,
            remoteTokenManager,
            remoteChainId
        );
        console.log("BridgedToken Deployed: %s", address(token));
        ITokenManagerStructs.RemoteToken memory remote = tokenManager
            .getRemoteTokens(address(token), remoteChainId);
        console.log(
            "RemoteToken %s, remoteTokenManager %s, remoteChainId %s",
            remote.token,
            remote.tokenManager,
            remote.chainId
        );
        console.log(
            "Name: %s Symbol: %s Decimals: %s",
            token.name(),
            token.symbol(),
            token.decimals()
        );

        vm.stopBroadcast();
    }
}
