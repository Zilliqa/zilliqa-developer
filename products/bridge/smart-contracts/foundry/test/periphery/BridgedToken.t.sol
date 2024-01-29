// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity 0.8.20;

import {Tester} from "foundry/test/Tester.sol";
import {BridgedToken} from "contracts/periphery/BridgedToken.sol";
import {IERC20Errors} from "@openzeppelin/contracts/interfaces/draft-IERC6093.sol";

interface ERC20Events {
    event Transfer(address indexed from, address indexed to, uint256 value);

    event Approval(
        address indexed owner,
        address indexed spender,
        uint256 value
    );
}

contract BridgedTokenTests is Tester, ERC20Events {
    address bridge = vm.createWallet("bridge").addr;
    address zilBridge = vm.createWallet("zilBridge").addr;
    address user = vm.createWallet("user").addr;
    uint mintedAmount = 10 ether;

    BridgedToken bridgedToken;

    function setUp() external {
        vm.prank(bridge);
        bridgedToken = new BridgedToken("Gold", "GLD", 18);
    }

    function mint(address target) internal {
        vm.prank(bridge);
        bridgedToken.mint(target, mintedAmount);
    }

    function migrateToZilBridge(address owner) internal {
        vm.startPrank(owner);
        bridgedToken.setLockProxyAddress(zilBridge);
        bridgedToken.renounceOwnership();
        vm.stopPrank();
    }

    function test_migrateOwnershipToZilBridge() external {
        migrateToZilBridge(bridge);

        assertEq(bridgedToken.lockProxyAddress(), zilBridge);
        assertEq(bridgedToken.owner(), address(0));
    }

    function test_transfer_asZilBridge_mintWhenNoLockedTokens() external {
        address target = vm.createWallet("target").addr;
        uint bridgeAmount = 100 ether;
        migrateToZilBridge(bridge);
        assertEq(bridgedToken.balanceOf(zilBridge), 0);
        assertEq(bridgedToken.balanceOf(target), 0);

        // Mint event
        vm.expectEmit(address(bridgedToken));
        emit ERC20Events.Transfer(address(0), zilBridge, bridgeAmount);
        // Transfer event
        vm.expectEmit(address(bridgedToken));
        emit ERC20Events.Transfer(zilBridge, target, bridgeAmount);
        vm.prank(zilBridge);
        bridgedToken.transfer(target, bridgeAmount);

        assertEq(bridgedToken.balanceOf(zilBridge), 0);
        assertEq(bridgedToken.balanceOf(target), bridgeAmount);
        assertEq(bridgedToken.circulatingSupply(), bridgeAmount);
        assertEq(bridgedToken.totalSupply(), bridgeAmount);
    }

    function test_transfer_asZilBridge_transferWithLockedTokens() external {
        address target = vm.createWallet("target").addr;
        mint(zilBridge);
        migrateToZilBridge(bridge);
        assertEq(bridgedToken.circulatingSupply(), 0);
        assertEq(bridgedToken.totalSupply(), mintedAmount);
        assertEq(bridgedToken.balanceOf(zilBridge), mintedAmount);
        assertEq(bridgedToken.balanceOf(target), 0);

        // Transfer event
        vm.expectEmit(address(bridgedToken));
        emit ERC20Events.Transfer(zilBridge, target, mintedAmount);
        vm.prank(zilBridge);
        bridgedToken.transfer(target, mintedAmount);

        assertEq(bridgedToken.balanceOf(zilBridge), 0);
        assertEq(bridgedToken.balanceOf(target), mintedAmount);
        assertEq(bridgedToken.circulatingSupply(), mintedAmount);
        assertEq(bridgedToken.totalSupply(), mintedAmount);
    }

    function test_transferFrom_asZilBridge_mintWhenNoLockedTokens() external {
        address target = vm.createWallet("target").addr;
        uint bridgeAmount = 100 ether;
        migrateToZilBridge(bridge);
        assertEq(bridgedToken.balanceOf(zilBridge), 0);
        assertEq(bridgedToken.balanceOf(target), 0);

        vm.prank(zilBridge);
        bridgedToken.approve(target, bridgeAmount);

        // Mint event
        vm.expectEmit(address(bridgedToken));
        emit ERC20Events.Transfer(address(0), zilBridge, bridgeAmount);
        // Transfer event
        vm.expectEmit(address(bridgedToken));
        emit ERC20Events.Transfer(zilBridge, target, bridgeAmount);
        vm.prank(target);
        bridgedToken.transferFrom(zilBridge, target, bridgeAmount);

        assertEq(bridgedToken.balanceOf(zilBridge), 0);
        assertEq(bridgedToken.balanceOf(target), bridgeAmount);
        assertEq(bridgedToken.circulatingSupply(), bridgeAmount);
        assertEq(bridgedToken.totalSupply(), bridgeAmount);
    }

    function test_transferFrom_asZilBridge_transferWithLockedTokens() external {
        address target = vm.createWallet("target").addr;
        mint(zilBridge);
        migrateToZilBridge(bridge);
        assertEq(bridgedToken.circulatingSupply(), 0);
        assertEq(bridgedToken.totalSupply(), mintedAmount);
        assertEq(bridgedToken.balanceOf(zilBridge), mintedAmount);
        assertEq(bridgedToken.balanceOf(target), 0);

        vm.prank(zilBridge);
        bridgedToken.approve(target, mintedAmount);

        // Transfer event
        vm.expectEmit(address(bridgedToken));
        emit ERC20Events.Transfer(zilBridge, target, mintedAmount);
        vm.prank(target);
        bridgedToken.transferFrom(zilBridge, target, mintedAmount);

        assertEq(bridgedToken.balanceOf(zilBridge), 0);
        assertEq(bridgedToken.balanceOf(target), mintedAmount);
        assertEq(bridgedToken.circulatingSupply(), mintedAmount);
        assertEq(bridgedToken.totalSupply(), mintedAmount);
    }

    function test_transfer_revertUserWithoutTokens() external {
        address target = vm.createWallet("target").addr;
        uint amount = 20 ether;
        migrateToZilBridge(bridge);

        vm.prank(user);
        vm.expectRevert(
            abi.encodeWithSelector(
                IERC20Errors.ERC20InsufficientBalance.selector,
                user,
                0,
                amount
            )
        );
        bridgedToken.transfer(target, amount);
    }

    function test_transferFrom_revertUserWithoutTokens() external TODO {
        address target = vm.createWallet("target").addr;
        uint amount = 20 ether;
        migrateToZilBridge(bridge);

        vm.prank(user);
        bridgedToken.approve(target, amount);

        vm.expectRevert(
            abi.encodeWithSelector(
                IERC20Errors.ERC20InsufficientBalance.selector,
                user,
                0,
                amount
            )
        );
        bridgedToken.transferFrom(user, target, amount);
    }

    function test_transfer_UserWithTokens() external TODO {
        mint(user);
        migrateToZilBridge(bridge);
    }

    function test_transferFrom_UserWithTokens() external TODO {
        mint(user);
        migrateToZilBridge(bridge);
    }

    function test_mint_revertsAfterMigration() external TODO {
        migrateToZilBridge(bridge);
    }

    function test_burn_revertsAfterMigration() external TODO {
        migrateToZilBridge(bridge);
    }
}
