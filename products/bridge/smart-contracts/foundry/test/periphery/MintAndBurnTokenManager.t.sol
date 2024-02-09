// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity 0.8.20;

import {Tester} from "foundry/test/Tester.sol";
import {MintAndBurnTokenManagerUpgradeable} from "contracts/periphery/MintAndBurnTokenManagerUpgradeable.sol";
import {ITokenManagerStructs, ITokenManagerEvents} from "contracts/periphery/TokenManagerUpgradeable.sol";
import {BridgedToken} from "contracts/periphery/BridgedToken.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades/Upgrades.sol";

contract MintAndBurnTokenManagerTests is Tester, ITokenManagerEvents {
    address deployer = vm.addr(1);
    address chainGateway = vm.addr(102);

    address remoteTokenManager = vm.addr(100);
    address remoteToken = vm.addr(101);
    uint remoteChainId = 101;

    MintAndBurnTokenManagerUpgradeable tokenManager;
    BridgedToken bridgedToken;

    function setUp() external {
        vm.startPrank(deployer);
        address implementation = address(
            new MintAndBurnTokenManagerUpgradeable()
        );
        address proxy = address(
            new ERC1967Proxy(
                implementation,
                abi.encodeCall(
                    MintAndBurnTokenManagerUpgradeable.initialize,
                    chainGateway
                )
            )
        );
        tokenManager = MintAndBurnTokenManagerUpgradeable(proxy);

        // Deploy bridged ERC20
        bridgedToken = tokenManager.deployToken(
            "USDZ",
            "Zilliqa USD",
            remoteToken,
            remoteTokenManager,
            remoteChainId
        );
        vm.stopPrank();
    }

    function test_TokenOwnershipTransferred() external {
        address newOwner = vm.addr(200);

        vm.expectEmit(address(tokenManager));
        emit ITokenManagerEvents.TokenRemoved(
            address(bridgedToken),
            remoteChainId
        );
        vm.prank(deployer);
        tokenManager.transferTokenOwnership(
            address(bridgedToken),
            remoteChainId,
            newOwner
        );

        ITokenManagerStructs.RemoteToken memory newRemoteToken = tokenManager
            .getRemoteTokens(address(bridgedToken), remoteChainId);
        ITokenManagerStructs.RemoteToken memory expected;
        // Verify if owner has been updated
        assertEq(bridgedToken.owner(), newOwner);
        // Verify if remoteToken has been deleted
        assertEq(abi.encode(newRemoteToken), abi.encode(expected));
    }
}
