// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity 0.8.20;

import {Tester} from "foundry/test/Tester.sol";
import {BridgedToken} from "contracts/periphery/BridgedToken.sol";
import {IERC20Errors} from "@openzeppelin/contracts/interfaces/draft-IERC6093.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

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

    function preMint(address target) internal {
        vm.prank(bridge);
        bridgedToken.mint(target, mintedAmount);
    }

    function migrateToZilBridge() internal {
        vm.startPrank(bridge);
        bridgedToken.setLockProxyAddress(zilBridge);
        vm.stopPrank();

        vm.startPrank(zilBridge);
        uint originalAmount = bridgedToken.totalSupply();
        bridgedToken.transfer(bridge, originalAmount);
        assertEq(bridgedToken.balanceOf(bridge), originalAmount);
        assertEq(bridgedToken.totalSupply(), originalAmount * 2);
        vm.stopPrank();

        vm.startPrank(bridge);
        bridgedToken.burn(bridgedToken.balanceOf(bridge));

        bridgedToken.renounceOwnership();
        vm.stopPrank();
    }

    function test_migrateOwnershipToZilBridge() external {
        preMint(user);
        assertEq(bridgedToken.totalSupply(), mintedAmount);
        assertEq(bridgedToken.circulatingSupply(), mintedAmount);

        migrateToZilBridge();

        assertEq(bridgedToken.totalSupply(), mintedAmount);
        assertEq(bridgedToken.circulatingSupply(), mintedAmount);
        assertEq(bridgedToken.lockProxyAddress(), zilBridge);
        assertEq(bridgedToken.owner(), address(0));
    }

    function test_transfer_asZilBridge_mintWhenNoLockedTokens() external {
        address target = vm.createWallet("target").addr;
        uint bridgeAmount = 100 ether;
        migrateToZilBridge();
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
        preMint(user);
        migrateToZilBridge();

        vm.prank(user);
        bridgedToken.transfer(zilBridge, mintedAmount);
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

    function test_transfer_asZilBridge_transferWithPartiallyLockedTokens()
        external
    {
        address target = vm.createWallet("target").addr;
        preMint(user);
        migrateToZilBridge();

        vm.prank(user);
        bridgedToken.transfer(zilBridge, mintedAmount);

        assertEq(bridgedToken.circulatingSupply(), 0);
        assertEq(bridgedToken.totalSupply(), mintedAmount);
        assertEq(bridgedToken.balanceOf(zilBridge), mintedAmount);
        assertEq(bridgedToken.balanceOf(target), 0);

        uint transferAmount = mintedAmount * 3;

        // Transfer event
        vm.expectEmit(address(bridgedToken));
        emit ERC20Events.Transfer(zilBridge, target, transferAmount);
        vm.prank(zilBridge);
        bridgedToken.transfer(target, transferAmount);

        assertEq(bridgedToken.balanceOf(zilBridge), 0);
        assertEq(bridgedToken.balanceOf(target), transferAmount);
        assertEq(bridgedToken.circulatingSupply(), transferAmount);
        assertEq(bridgedToken.totalSupply(), transferAmount);
    }

    function test_transferFrom_asZilBridge_mintWhenNoLockedTokens() external {
        address target = vm.createWallet("target").addr;
        uint bridgeAmount = 100 ether;
        migrateToZilBridge();
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
        preMint(user);

        migrateToZilBridge();

        vm.prank(user);
        bridgedToken.transfer(zilBridge, mintedAmount);
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
        migrateToZilBridge();

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

    function test_transferFrom_revertUserWithoutTokens() external {
        address target = vm.createWallet("target").addr;
        uint amount = 20 ether;
        migrateToZilBridge();

        vm.prank(user);
        bridgedToken.approve(target, amount);

        vm.prank(target);
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

    function test_transfer_UserWithTokens() external {
        address target = vm.createWallet("target").addr;
        preMint(user);
        migrateToZilBridge();
        assertEq(bridgedToken.balanceOf(target), 0);
        assertEq(bridgedToken.balanceOf(user), mintedAmount);
        assertEq(bridgedToken.circulatingSupply(), mintedAmount);
        assertEq(bridgedToken.totalSupply(), mintedAmount);

        // Transfer event
        vm.expectEmit(address(bridgedToken));
        emit ERC20Events.Transfer(user, target, mintedAmount);
        vm.prank(user);
        bridgedToken.transfer(target, mintedAmount);

        assertEq(bridgedToken.balanceOf(user), 0);
        assertEq(bridgedToken.balanceOf(target), mintedAmount);
        assertEq(bridgedToken.circulatingSupply(), mintedAmount);
        assertEq(bridgedToken.totalSupply(), mintedAmount);
    }

    function test_transferFrom_UserWithTokens() external {
        address target = vm.createWallet("target").addr;
        preMint(user);
        migrateToZilBridge();
        assertEq(bridgedToken.balanceOf(target), 0);
        assertEq(bridgedToken.balanceOf(user), mintedAmount);
        assertEq(bridgedToken.circulatingSupply(), mintedAmount);
        assertEq(bridgedToken.totalSupply(), mintedAmount);

        // Approve
        vm.prank(user);
        bridgedToken.approve(target, mintedAmount);

        // Transfer event
        vm.expectEmit(address(bridgedToken));
        emit ERC20Events.Transfer(user, target, mintedAmount);
        vm.prank(target);
        bridgedToken.transferFrom(user, target, mintedAmount);

        assertEq(bridgedToken.balanceOf(user), 0);
        assertEq(bridgedToken.balanceOf(target), mintedAmount);
        assertEq(bridgedToken.circulatingSupply(), mintedAmount);
        assertEq(bridgedToken.totalSupply(), mintedAmount);
    }

    function test_mint_revertsAfterMigration() external {
        migrateToZilBridge();

        vm.prank(bridge);
        vm.expectRevert(
            abi.encodeWithSelector(
                Ownable.OwnableUnauthorizedAccount.selector,
                bridge
            )
        );
        bridgedToken.mint(user, 10 ether);
    }

    function test_burn_revertsAfterMigration() external {
        migrateToZilBridge();

        vm.prank(bridge);
        vm.expectRevert(
            abi.encodeWithSelector(
                Ownable.OwnableUnauthorizedAccount.selector,
                bridge
            )
        );
        bridgedToken.burn(10 ether);
    }

    function test_burnFrom_revertsAfterMigration() external {
        migrateToZilBridge();

        vm.prank(bridge);
        vm.expectRevert(
            abi.encodeWithSelector(
                Ownable.OwnableUnauthorizedAccount.selector,
                bridge
            )
        );
        bridgedToken.burn(10 ether);
    }

    function test_mint_beforeMigration() external {
        uint amount = 20 ether;
        assertEq(bridgedToken.balanceOf(bridge), 0);
        assertEq(bridgedToken.circulatingSupply(), 0);
        assertEq(bridgedToken.totalSupply(), 0);

        vm.prank(bridge);
        bridgedToken.mint(user, amount);

        assertEq(bridgedToken.balanceOf(bridge), 0);
        assertEq(bridgedToken.balanceOf(user), amount);
        assertEq(bridgedToken.circulatingSupply(), amount);
        assertEq(bridgedToken.totalSupply(), amount);
    }

    function test_burn_beforeMigration() external {
        preMint(bridge);
        assertEq(bridgedToken.balanceOf(bridge), mintedAmount);
        assertEq(bridgedToken.circulatingSupply(), mintedAmount);
        assertEq(bridgedToken.totalSupply(), mintedAmount);

        vm.prank(bridge);
        bridgedToken.burn(mintedAmount);

        assertEq(bridgedToken.balanceOf(bridge), 0);
        assertEq(bridgedToken.balanceOf(user), 0);
        assertEq(bridgedToken.circulatingSupply(), 0);
        assertEq(bridgedToken.totalSupply(), 0);
    }

    function test_burnFrom_beforeMigration() external {
        preMint(user);
        assertEq(bridgedToken.balanceOf(bridge), 0);
        assertEq(bridgedToken.balanceOf(user), mintedAmount);
        assertEq(bridgedToken.circulatingSupply(), mintedAmount);
        assertEq(bridgedToken.totalSupply(), mintedAmount);

        vm.prank(user);
        bridgedToken.approve(bridge, mintedAmount);

        vm.prank(bridge);
        bridgedToken.burnFrom(user, mintedAmount);

        assertEq(bridgedToken.balanceOf(bridge), 0);
        assertEq(bridgedToken.balanceOf(user), 0);
        assertEq(bridgedToken.circulatingSupply(), 0);
        assertEq(bridgedToken.totalSupply(), 0);
    }

    function test_transfer_RevertsIfNoFundsBeforeMigration() external {
        address target = vm.createWallet("target").addr;
        uint amount = 10 ether;
        assertEq(bridgedToken.balanceOf(user), 0);

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

    function test_transferFrom_RevertsIfNoFundsBeforeMigration() external {
        address target = vm.createWallet("target").addr;
        uint amount = 10 ether;
        assertEq(bridgedToken.balanceOf(user), 0);

        vm.prank(user);
        bridgedToken.approve(target, amount);

        vm.prank(target);
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
}
