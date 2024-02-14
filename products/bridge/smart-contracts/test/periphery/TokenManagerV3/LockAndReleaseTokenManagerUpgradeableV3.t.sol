// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity 0.8.20;

import {Tester} from "test/Tester.sol";
import {LockAndReleaseTokenManagerUpgradeable} from "contracts/periphery/LockAndReleaseTokenManagerUpgradeable.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {LockAndReleaseTokenManagerUpgradeableV2} from "contracts/periphery/TokenManagerV2/LockAndReleaseTokenManagerUpgradeableV2.sol";
import {LockAndReleaseTokenManagerUpgradeableV3} from "contracts/periphery/TokenManagerV3/LockAndReleaseTokenManagerUpgradeableV3.sol";
import {ITokenManager, ITokenManagerFees, ITokenManagerStructs, ITokenManagerEvents} from "contracts/periphery/TokenManagerV2/TokenManagerUpgradeableV2.sol";
import {ITokenManagerFeesEvents} from "contracts/periphery/TokenManagerV2/TokenManagerFees.sol";
import {IRelayer, CallMetadata} from "contracts/core/Relayer.sol";
import {PausableUpgradeable} from "@openzeppelin/contracts-upgradeable/utils/PausableUpgradeable.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {TestToken} from "test/Helpers.sol";

interface IPausable {
    event Paused(address account);
    event Unpaused(address account);
}

contract LockAndReleaseTokenManagerUpgradeableV3Tests is
    Tester,
    ITokenManagerEvents,
    ITokenManagerStructs,
    ITokenManagerFeesEvents,
    IPausable
{
    address deployer = vm.addr(1);
    address chainGateway = vm.addr(102);
    address user = vm.createWallet("user").addr;

    RemoteToken remoteToken =
        RemoteToken({
            token: vm.createWallet("remoteToken").addr,
            tokenManager: vm.createWallet("remoteTokenManager").addr,
            chainId: 101
        });
    uint transferAmount = 10 ether;
    uint fees = 0.1 ether;

    LockAndReleaseTokenManagerUpgradeable tokenManager;
    LockAndReleaseTokenManagerUpgradeableV2 tokenManagerV2;
    LockAndReleaseTokenManagerUpgradeableV3 tokenManagerV3;
    TestToken token;

    function setUp() external {
        vm.startPrank(deployer);
        address implementation = address(
            new LockAndReleaseTokenManagerUpgradeable()
        );
        address proxy = address(
            new ERC1967Proxy(
                implementation,
                abi.encodeCall(
                    LockAndReleaseTokenManagerUpgradeable.initialize,
                    chainGateway
                )
            )
        );
        tokenManager = LockAndReleaseTokenManagerUpgradeable(proxy);

        assertEq(tokenManager.getGateway(), chainGateway);

        // Deploy new token
        token = new TestToken(transferAmount);
        tokenManager.registerToken(address(token), remoteToken);

        // Check
        RemoteToken memory actualRemoteToken = tokenManager.getRemoteTokens(
            address(token),
            remoteToken.chainId
        );
        assertEq(abi.encode(remoteToken), abi.encode(actualRemoteToken));

        // Carry out upgrade V2
        address implementationV2 = address(
            new LockAndReleaseTokenManagerUpgradeableV2()
        );
        bytes memory encodedInitializerCall = abi.encodeCall(
            LockAndReleaseTokenManagerUpgradeableV2.reinitialize,
            fees
        );
        tokenManager.upgradeToAndCall(implementationV2, encodedInitializerCall);
        tokenManagerV2 = LockAndReleaseTokenManagerUpgradeableV2(
            address(tokenManager)
        );
        // Check new fees introduced
        assertEq(tokenManagerV2.getFees(), fees);

        // Then check existing data is still intact
        assertEq(tokenManagerV2.getGateway(), chainGateway);
        actualRemoteToken = tokenManagerV2.getRemoteTokens(
            address(token),
            remoteToken.chainId
        );
        assertEq(abi.encode(remoteToken), abi.encode(actualRemoteToken));

        // Premint some tokens for user testing
        token.transfer(user, transferAmount);
        assertEq(token.balanceOf(user), transferAmount);
        assertEq(token.balanceOf(deployer), 0);

        // Carry out upgrade v3
        address implementationV3 = address(
            new LockAndReleaseTokenManagerUpgradeableV3()
        );
        tokenManagerV2.upgradeToAndCall(implementationV3, "");
        tokenManagerV3 = LockAndReleaseTokenManagerUpgradeableV3(
            address(tokenManager)
        );

        vm.stopPrank();
    }

    function test_feesOnTransfer() external {
        startHoax(user);

        token.approve(address(tokenManagerV3), transferAmount);

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
            address(token),
            remoteToken.chainId,
            user,
            transferAmount
        );
        assertEq(token.balanceOf(address(tokenManagerV3)), transferAmount);
        assertEq(token.balanceOf(user), 0);
        assertEq(address(tokenManagerV3).balance, fees);

        vm.stopPrank();
    }

    function test_extraFeesOnTransfer() external {
        startHoax(user);

        token.approve(address(tokenManagerV3), transferAmount);

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
            address(token),
            remoteToken.chainId,
            user,
            transferAmount
        );
        assertEq(token.balanceOf(address(tokenManagerV3)), transferAmount);
        assertEq(token.balanceOf(user), 0);
        assertEq(address(tokenManagerV3).balance, fees * 2);

        vm.stopPrank();
    }

    function test_RevertWhenNoFeesProvidedOnTransfer() external {
        startHoax(user);

        token.approve(address(tokenManagerV3), transferAmount);

        vm.expectRevert(
            abi.encodeWithSelector(
                ITokenManagerFees.InsufficientFees.selector,
                0,
                fees
            )
        );
        tokenManagerV3.transfer{value: 0}(
            address(token),
            remoteToken.chainId,
            user,
            transferAmount
        );

        assertEq(token.balanceOf(address(tokenManagerV3)), 0);
        assertEq(token.balanceOf(user), transferAmount);
        assertEq(address(tokenManagerV3).balance, 0);

        vm.stopPrank();
    }

    function test_RevertWhenInsufficientFeesProvidedOnTransfer() external {
        startHoax(user);
        uint halfFees = fees / 2;

        token.approve(address(tokenManagerV3), transferAmount);

        vm.expectRevert(
            abi.encodeWithSelector(
                ITokenManagerFees.InsufficientFees.selector,
                halfFees,
                fees
            )
        );
        tokenManagerV3.transfer{value: halfFees}(
            address(token),
            remoteToken.chainId,
            user,
            transferAmount
        );

        assertEq(token.balanceOf(address(tokenManagerV3)), 0);
        assertEq(token.balanceOf(user), transferAmount);
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

        token.approve(address(tokenManagerV3), transferAmount);

        vm.expectRevert(PausableUpgradeable.EnforcedPause.selector);
        tokenManagerV3.transfer{value: halfFees}(
            address(token),
            remoteToken.chainId,
            user,
            transferAmount
        );

        assertEq(token.balanceOf(address(tokenManagerV3)), 0);
        assertEq(token.balanceOf(user), transferAmount);
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

        token.approve(address(tokenManagerV3), transferAmount);
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
            address(token),
            remoteToken.chainId,
            user,
            transferAmount
        );

        assertEq(token.balanceOf(address(tokenManagerV3)), transferAmount);
        assertEq(token.balanceOf(user), 0);
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
