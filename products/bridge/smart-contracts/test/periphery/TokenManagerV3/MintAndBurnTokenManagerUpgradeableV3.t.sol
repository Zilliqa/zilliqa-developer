// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity 0.8.20;

import {Tester} from "test/Tester.sol";
import {BridgedToken} from "contracts/periphery/BridgedToken.sol";
import {MintAndBurnTokenManagerUpgradeableV2} from "contracts/periphery/TokenManagerV2/MintAndBurnTokenManagerUpgradeableV2.sol";
import {MintAndBurnTokenManagerUpgradeableV3} from "contracts/periphery/TokenManagerV3/MintAndBurnTokenManagerUpgradeableV3.sol";
import {ITokenManager, ITokenManagerFees, ITokenManagerStructs, ITokenManagerEvents} from "contracts/periphery/TokenManagerV2/TokenManagerUpgradeableV2.sol";
import {ITokenManagerFeesEvents} from "contracts/periphery/TokenManagerV2/TokenManagerFees.sol";
import {IRelayer} from "contracts/core/Relayer.sol";
import {PausableUpgradeable} from "@openzeppelin/contracts-upgradeable/utils/PausableUpgradeable.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import {MintAndBurnTokenManagerDeployer} from "test/periphery/TokenManagerDeployers/MintAndBurnTokenManagerDeployer.sol";

interface IPausable {
    event Paused(address account);
    event Unpaused(address account);
}

contract MintAndBurnTokenManagerUpgradeableV3Tests is
    Tester,
    ITokenManagerEvents,
    ITokenManagerStructs,
    ITokenManagerFeesEvents,
    IPausable,
    MintAndBurnTokenManagerDeployer
{
    address deployer = vm.createWallet("deployer").addr;
    address chainGateway = vm.createWallet("chainGateway").addr;
    address user = vm.createWallet("user").addr;

    RemoteToken remoteToken =
        RemoteToken({
            token: vm.createWallet("remoteToken").addr,
            tokenManager: vm.createWallet("remoteTokenManager").addr,
            chainId: 101
        });
    uint transferAmount = 10 ether;
    uint fees = 0.1 ether;

    MintAndBurnTokenManagerUpgradeableV2 tokenManagerV2;
    MintAndBurnTokenManagerUpgradeableV3 tokenManagerV3;
    BridgedToken bridgedToken;

    function setUp() external {
        vm.startPrank(deployer);
        tokenManagerV2 = deployMintAndBurnTokenManagerV2(chainGateway, fees);

        // Deploy new token
        bridgedToken = tokenManagerV2.deployToken(
            "USDZ",
            "Zilliqa USD",
            remoteToken.token,
            remoteToken.tokenManager,
            remoteToken.chainId
        );
        // Premint some tokens for user testing
        vm.startPrank(address(tokenManagerV2));
        bridgedToken.mint(user, transferAmount);
        assertEq(bridgedToken.balanceOf(user), transferAmount);

        vm.startPrank(deployer);
        // Carry out upgrade v3
        address implementationV3 = address(
            new MintAndBurnTokenManagerUpgradeableV3()
        );
        tokenManagerV2.upgradeToAndCall(implementationV3, "");
        tokenManagerV3 = MintAndBurnTokenManagerUpgradeableV3(
            address(tokenManagerV2)
        );
        vm.stopPrank();
    }

    function test_feesOnTransfer() external {
        startHoax(user);

        bridgedToken.approve(address(tokenManagerV3), transferAmount);

        vm.mockCall(
            chainGateway,
            abi.encodeCall(
                IRelayer.relayWithMetadata,
                (
                    remoteToken.chainId,
                    remoteToken.tokenManager,
                    ITokenManager.accept.selector,
                    abi.encode(
                        AcceptArgs(remoteToken.token, user, transferAmount)
                    ),
                    1_000_000
                )
            ),
            abi.encode(0)
        );
        tokenManagerV3.transfer{value: fees}(
            address(bridgedToken),
            remoteToken.chainId,
            user,
            transferAmount
        );
        assertEq(bridgedToken.balanceOf(address(tokenManagerV3)), 0);
        assertEq(bridgedToken.balanceOf(user), 0);
        assertEq(address(tokenManagerV3).balance, fees);

        vm.stopPrank();
    }

    function test_extraFeesOnTransfer() external {
        startHoax(user);

        bridgedToken.approve(address(tokenManagerV3), transferAmount);

        vm.mockCall(
            chainGateway,
            abi.encodeCall(
                IRelayer.relayWithMetadata,
                (
                    remoteToken.chainId,
                    remoteToken.tokenManager,
                    ITokenManager.accept.selector,
                    abi.encode(
                        AcceptArgs(remoteToken.token, user, transferAmount)
                    ),
                    1_000_000
                )
            ),
            abi.encode(0)
        );
        tokenManagerV3.transfer{value: fees * 2}(
            address(bridgedToken),
            remoteToken.chainId,
            user,
            transferAmount
        );
        assertEq(bridgedToken.balanceOf(address(tokenManagerV3)), 0);
        assertEq(bridgedToken.balanceOf(user), 0);
        assertEq(address(tokenManagerV3).balance, fees * 2);

        vm.stopPrank();
    }

    function test_RevertWhenNoFeesProvidedOnTransfer() external {
        startHoax(user);

        bridgedToken.approve(address(tokenManagerV3), transferAmount);

        vm.expectRevert(
            abi.encodeWithSelector(
                ITokenManagerFees.InsufficientFees.selector,
                0,
                fees
            )
        );
        tokenManagerV3.transfer{value: 0}(
            address(bridgedToken),
            remoteToken.chainId,
            user,
            transferAmount
        );

        assertEq(bridgedToken.balanceOf(address(tokenManagerV3)), 0);
        assertEq(bridgedToken.balanceOf(user), transferAmount);
        assertEq(address(tokenManagerV3).balance, 0);

        vm.stopPrank();
    }

    function test_RevertWhenInsufficientFeesProvidedOnTransfer() external {
        startHoax(user);
        uint halfFees = fees / 2;

        bridgedToken.approve(address(tokenManagerV3), transferAmount);

        vm.expectRevert(
            abi.encodeWithSelector(
                ITokenManagerFees.InsufficientFees.selector,
                halfFees,
                fees
            )
        );
        tokenManagerV3.transfer{value: halfFees}(
            address(bridgedToken),
            remoteToken.chainId,
            user,
            transferAmount
        );

        assertEq(bridgedToken.balanceOf(address(tokenManagerV3)), 0);
        assertEq(bridgedToken.balanceOf(user), transferAmount);
        assertEq(address(tokenManagerV3).balance, 0);

        vm.stopPrank();
    }

    function test_RevertTransferWhenPaused() external {
        vm.prank(deployer);
        vm.expectEmit(address(tokenManagerV3));
        emit IPausable.Paused(deployer);
        tokenManagerV3.pause();

        assertEq(tokenManagerV3.paused(), true);

        startHoax(user);
        uint halfFees = fees / 2;

        bridgedToken.approve(address(tokenManagerV3), transferAmount);

        vm.expectRevert(PausableUpgradeable.EnforcedPause.selector);
        tokenManagerV3.transfer{value: halfFees}(
            address(bridgedToken),
            remoteToken.chainId,
            user,
            transferAmount
        );

        assertEq(bridgedToken.balanceOf(address(tokenManagerV3)), 0);
        assertEq(bridgedToken.balanceOf(user), transferAmount);
        assertEq(address(tokenManagerV3).balance, 0);

        vm.stopPrank();
    }

    function test_RevertNonOwnerAttemptPause() external {
        address randomUser = vm.createWallet("randomUser").addr;

        vm.prank(randomUser);
        vm.expectRevert(
            abi.encodeWithSelector(
                OwnableUpgradeable.OwnableUnauthorizedAccount.selector,
                randomUser
            )
        );
        tokenManagerV3.pause();

        assertEq(tokenManagerV3.paused(), false);
    }

    function test_withdrawFees() external {
        uint withdrawAmount = 1 ether;
        assertEq(address(tokenManagerV3).balance, 0);
        assertEq(deployer.balance, 0);

        vm.deal(address(tokenManagerV3), withdrawAmount);
        assertEq(address(tokenManagerV3).balance, withdrawAmount);

        vm.prank(deployer);
        vm.expectEmit(address(tokenManagerV3));
        emit ITokenManagerFeesEvents.FeesWithdrawn(withdrawAmount);
        tokenManagerV3.withdrawFees(payable(deployer));

        assertEq(address(deployer).balance, withdrawAmount);
    }

    function test_withdrawFeesWhenNoFees() external {
        assertEq(address(tokenManagerV3).balance, 0);
        assertEq(deployer.balance, 0);

        vm.prank(deployer);
        vm.expectEmit(address(tokenManagerV3));
        emit ITokenManagerFeesEvents.FeesWithdrawn(0);
        tokenManagerV3.withdrawFees(payable(deployer));

        assertEq(address(deployer).balance, 0);
    }

    function test_setNewFeesAndTransfer() external {
        uint newFees = 1 ether;

        // Test setting fees
        vm.prank(deployer);
        vm.expectEmit(address(tokenManagerV3));
        emit ITokenManagerFeesEvents.FeesUpdated(fees, newFees);
        tokenManagerV3.setFees(newFees);

        assertEq(tokenManagerV3.getFees(), newFees);

        // Test transfer
        startHoax(user);

        bridgedToken.approve(address(tokenManagerV3), transferAmount);
        vm.mockCall(
            chainGateway,
            abi.encodeCall(
                IRelayer.relayWithMetadata,
                (
                    remoteToken.chainId,
                    remoteToken.tokenManager,
                    ITokenManager.accept.selector,
                    abi.encode(
                        AcceptArgs(remoteToken.token, user, transferAmount)
                    ),
                    1_000_000
                )
            ),
            abi.encode(0)
        );
        tokenManagerV3.transfer{value: newFees}(
            address(bridgedToken),
            remoteToken.chainId,
            user,
            transferAmount
        );

        assertEq(bridgedToken.balanceOf(address(tokenManagerV3)), 0);
        assertEq(bridgedToken.balanceOf(user), 0);
        assertEq(address(tokenManagerV3).balance, newFees);

        vm.stopPrank();
    }

    function test_RevertNonOwnerSetFees() external {
        uint newFees = 1 ether;
        address randomUser = vm.createWallet("randomUser").addr;

        // Test setting fees
        vm.prank(randomUser);
        vm.expectRevert(
            abi.encodeWithSelector(
                OwnableUpgradeable.OwnableUnauthorizedAccount.selector,
                randomUser
            )
        );
        tokenManagerV3.setFees(newFees);

        assertEq(tokenManagerV3.getFees(), fees);
    }

    function test_RevertNonOwnerWithdrawFees() external {
        uint balance = 2 ether;
        vm.deal(address(tokenManagerV3), balance);
        address randomUser = vm.createWallet("randomUser").addr;

        // Test setting fees
        vm.prank(randomUser);
        vm.expectRevert(
            abi.encodeWithSelector(
                OwnableUpgradeable.OwnableUnauthorizedAccount.selector,
                randomUser
            )
        );
        tokenManagerV3.withdrawFees(payable(randomUser));

        assertEq(address(tokenManagerV3).balance, balance);
    }

    function test_RevertNonOwnerUnpause() external {
        address randomUser = vm.createWallet("randomUser").addr;

        // Pause first
        vm.prank(deployer);
        tokenManagerV3.pause();

        assertEq(tokenManagerV3.paused(), true);

        // Test setting fees
        vm.prank(randomUser);
        vm.expectRevert(
            abi.encodeWithSelector(
                OwnableUpgradeable.OwnableUnauthorizedAccount.selector,
                randomUser
            )
        );
        tokenManagerV3.unpause();

        assertEq(tokenManagerV3.paused(), true);
    }

    function test_TokenOwnershipTransferred() external {
        address newOwner = vm.addr(200);
        vm.expectEmit(address(tokenManagerV3));
        emit ITokenManagerEvents.TokenRemoved(
            address(bridgedToken),
            remoteToken.chainId
        );
        vm.prank(deployer);
        tokenManagerV3.transferTokenOwnership(
            address(bridgedToken),
            remoteToken.chainId,
            newOwner
        );
        ITokenManagerStructs.RemoteToken memory newRemoteToken = tokenManagerV3
            .getRemoteTokens(address(bridgedToken), remoteToken.chainId);
        ITokenManagerStructs.RemoteToken memory expected;
        // Verify if owner has been updated
        assertEq(bridgedToken.owner(), newOwner);
        // Verify if remoteToken has been deleted
        assertEq(abi.encode(newRemoteToken), abi.encode(expected));
    }

    function test_transferOwneship2Step() external {
        address newOwner = vm.createWallet("newOwner").addr;

        vm.prank(deployer);
        tokenManagerV3.transferOwnership(newOwner);
        // Ownership should only be transferred after newOwner accepts
        assertEq(tokenManagerV3.owner(), deployer);

        vm.prank(newOwner);
        tokenManagerV3.acceptOwnership();
        assertEq(tokenManagerV3.owner(), newOwner);
    }
}
