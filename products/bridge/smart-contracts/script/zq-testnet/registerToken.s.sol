// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {Script} from "forge-std/Script.sol";
import {LockAndReleaseTokenManagerUpgradeable} from "contracts/periphery/LockAndReleaseTokenManagerUpgradeable.sol";
import {ITokenManagerStructs} from "contracts/periphery/TokenManagerUpgradeable.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "forge-std/console.sol";

contract Deployment is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY_TESTNET");
        address owner = vm.addr(deployerPrivateKey);
        console.log("Owner is %s", owner);

        address token = 0x8618d39a8276D931603c6Bc7306af6A53aD2F1F3;
        address tokenManagerAddress = 0x1509988c41f02014aA59d455c6a0D67b5b50f129;

        ITokenManagerStructs.RemoteToken memory remote = ITokenManagerStructs
            .RemoteToken({
                token: 0x5190e8b4Bbe8C3a732BAdB600b57fD42ACbB9F4B,
                tokenManager: 0xA6D73210AF20a59832F264fbD991D2abf28401d0,
                chainId: 97
            });

        LockAndReleaseTokenManagerUpgradeable tokenManager = LockAndReleaseTokenManagerUpgradeable(
                tokenManagerAddress
            );

        vm.startBroadcast(deployerPrivateKey);

        tokenManager.registerToken(token, remote);

        ITokenManagerStructs.RemoteToken memory res = tokenManager
            .getRemoteTokens(token, remote.chainId);
        console.log(
            "RemoteToken %s, remoteTokenManager %s, remoteChainId %s",
            res.token,
            res.tokenManager,
            res.chainId
        );
        console.log(
            "Name %s Symbol %s Decimals %s",
            ERC20(token).name(),
            ERC20(token).symbol(),
            ERC20(token).decimals()
        );

        vm.stopBroadcast();
    }
}
