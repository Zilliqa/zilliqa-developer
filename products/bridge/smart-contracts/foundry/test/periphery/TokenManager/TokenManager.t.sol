// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity 0.8.20;

import {Tester} from "foundry/test/Tester.sol";
import {UUPSUpgrader} from "foundry/test/Helpers.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {Upgrades, Options} from "openzeppelin-foundry-upgrades/Upgrades.sol";
import {ITokenManagerStructs} from "contracts/periphery/TokenManagerUpgradeable.sol";
import {TokenManagerTestV1} from "./TokenManagerTestV1.sol";
import {TokenManagerTestV2} from "./TokenManagerTestV2.sol";
import {TokenManagerTestV3} from "./TokenManagerTestV3.sol";
import {TokenManagerTestV4} from "./TokenManagerTestV4.sol";
import {TokenManagerTestV5} from "./TokenManagerTestV5.sol";
import {TokenManagerTestV6} from "./TokenManagerTestV6.sol";
import {TokenManagerTestV7} from "./TokenManagerTestV7.sol";
import {TokenManagerTestV8} from "./TokenManagerTestV8.sol";
import {PausableUpgradeable} from "@openzeppelin/contracts-upgradeable/utils/PausableUpgradeable.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";

contract TokenManagerTest is Tester {
    address deployer = vm.addr(1);
    address mockChainGateway = vm.addr(102);
    address mockToken = vm.addr(103);
    ITokenManagerStructs.RemoteToken mockRemoteToken =
        ITokenManagerStructs.RemoteToken({
            token: vm.addr(104),
            tokenManager: vm.addr(105),
            chainId: 10
        });

    address tokenManagerProxy;

    function setUp() external {
        vm.startPrank(deployer);

        tokenManagerProxy = UUPSUpgrader.deploy(
            "TokenManagerTestV1.sol",
            abi.encodeCall(TokenManagerTestV1.initialize, mockChainGateway)
        );

        TokenManagerTestV1(tokenManagerProxy).registerToken(
            mockToken,
            mockRemoteToken
        );

        vm.stopPrank();
    }

    function test_baseline() external {
        ITokenManagerStructs.RemoteToken memory result = TokenManagerTestV1(
            tokenManagerProxy
        ).getRemoteTokens(mockToken, mockRemoteToken.chainId);
        assertEq(abi.encode(result), abi.encode(mockRemoteToken));
    }

    function test_upgrade_with_new_field_and_respective_getters() external {
        uint count = 5;

        UUPSUpgrader.upgrade(
            tokenManagerProxy,
            "TokenManagerTestV2.sol",
            abi.encodeCall(TokenManagerTestV2.reinitialize, count),
            deployer
        );

        TokenManagerTestV2 tokenManagerV2 = TokenManagerTestV2(
            tokenManagerProxy
        );

        assertEq(tokenManagerV2.getCounter(), count);

        tokenManagerV2.incrementCounter();
        assertEq(tokenManagerV2.getCounter(), count + 1);

        tokenManagerV2.setCounter(4);
        assertEq(tokenManagerV2.getCounter(), 4);
    }

    function test_upgrade_with_invalid_owner() external {
        address newImplementation = makeAddr("New Implementation");
        address invalidOwner = makeAddr("Invalid Owner");

        vm.prank(invalidOwner);
        vm.expectRevert(
            abi.encodeWithSelector(
                OwnableUpgradeable.OwnableUnauthorizedAccount.selector,
                invalidOwner
            )
        );
        TokenManagerTestV1(tokenManagerProxy).upgradeToAndCall(
            newImplementation,
            ""
        );
    }

    function test_upgrade_new_getter_for_existing_variable() external {
        UUPSUpgrader.upgrade(
            tokenManagerProxy,
            "TokenManagerTestV2.sol",
            abi.encodeCall(TokenManagerTestV2.reinitialize, 1),
            deployer
        );

        TokenManagerTestV2 tokenManagerV2 = TokenManagerTestV2(
            tokenManagerProxy
        );

        address remoteTokenAddress = tokenManagerV2.getRemoteTokenAddress(
            mockToken,
            mockRemoteToken.chainId
        );
        assertEq(remoteTokenAddress, mockRemoteToken.token);
    }

    function test_upgrade_with_field_removed_and_replaced_with_gap() external {
        UUPSUpgrader.upgrade(
            tokenManagerProxy,
            "TokenManagerTestV3.sol",
            abi.encodeWithSelector(TokenManagerTestV3.reinitialize.selector),
            deployer
        );

        TokenManagerTestV3 tokenManagerV3 = TokenManagerTestV3(
            tokenManagerProxy
        );

        ITokenManagerStructs.RemoteToken memory remoteToken = tokenManagerV3
            .getRemoteTokens(mockToken, mockRemoteToken.chainId);

        assertEq(abi.encode(remoteToken), abi.encode(mockRemoteToken));
    }

    function test_upgrade_inherit_new_erc7201_compliant_contract_to_end()
        external
    {
        UUPSUpgrader.upgrade(
            tokenManagerProxy,
            "TokenManagerTestV4.sol",
            abi.encodeWithSelector(TokenManagerTestV4.reinitialize.selector),
            deployer
        );

        TokenManagerTestV4 tokenManagerV4 = TokenManagerTestV4(
            tokenManagerProxy
        );

        // Verify all state is good
        address gateway = tokenManagerV4.getGateway();
        assertEq(gateway, mockChainGateway);

        ITokenManagerStructs.RemoteToken memory remoteToken = tokenManagerV4
            .getRemoteTokens(mockToken, mockRemoteToken.chainId);

        assertEq(abi.encode(remoteToken), abi.encode(mockRemoteToken));
        assertEq(tokenManagerV4.pausableFunction(), true);
        // Verify if pausing is working accordingly
        vm.prank(deployer);
        tokenManagerV4.pause();

        vm.expectRevert(PausableUpgradeable.EnforcedPause.selector);
        tokenManagerV4.pausableFunction();
    }

    function test_upgrade_inherit_new_erc7201_compliant_contract_to_middle()
        external
    {
        UUPSUpgrader.upgrade(
            tokenManagerProxy,
            "TokenManagerTestV5.sol",
            abi.encodeWithSelector(TokenManagerTestV5.reinitialize.selector),
            deployer
        );

        TokenManagerTestV5 tokenManagerV5 = TokenManagerTestV5(
            tokenManagerProxy
        );

        // Verify all state is good
        address gateway = tokenManagerV5.getGateway();
        assertEq(gateway, mockChainGateway);

        ITokenManagerStructs.RemoteToken memory remoteToken = tokenManagerV5
            .getRemoteTokens(mockToken, mockRemoteToken.chainId);

        assertEq(abi.encode(remoteToken), abi.encode(mockRemoteToken));
        assertEq(tokenManagerV5.pausableFunction(), true);
        // Verify if pausing is working accordingly
        vm.prank(deployer);
        tokenManagerV5.pause();

        vm.expectRevert(PausableUpgradeable.EnforcedPause.selector);
        tokenManagerV5.pausableFunction();
    }

    function test_upgrade_remove_erc7201_compliant_contract_inheritance_in_middle()
        external
    {
        UUPSUpgrader.upgrade(
            tokenManagerProxy,
            "TokenManagerTestV5.sol",
            abi.encodeWithSelector(TokenManagerTestV5.reinitialize.selector),
            deployer
        );

        // Upgrade again to remove contract in the middle
        UUPSUpgrader.upgrade(
            tokenManagerProxy,
            "TokenManagerTestV6.sol",
            abi.encodeWithSelector(TokenManagerTestV6.reinitialize.selector),
            deployer
        );

        TokenManagerTestV6 tokenManagerV6 = TokenManagerTestV6(
            tokenManagerProxy
        );

        // Verify all state is good
        address gateway = tokenManagerV6.getGateway();
        assertEq(gateway, mockChainGateway);

        ITokenManagerStructs.RemoteToken memory remoteToken = tokenManagerV6
            .getRemoteTokens(mockToken, mockRemoteToken.chainId);

        assertEq(abi.encode(remoteToken), abi.encode(mockRemoteToken));
    }

    function test_upgrade_add_erc7201_non_compliant_contract_inheritance_to_end()
        external
    {
        uint number = 201;
        address randomAddress = makeAddr("Address Value");
        // Uses new reinitialize function to initialise the new values
        UUPSUpgrader.upgrade(
            tokenManagerProxy,
            "TokenManagerTestV7.sol",
            abi.encodeCall(
                TokenManagerTestV7.reinitialize,
                (number, randomAddress)
            ),
            deployer
        );
        TokenManagerTestV7 tokenManagerV7 = TokenManagerTestV7(
            tokenManagerProxy
        );

        // Verify all state is good
        address gateway = tokenManagerV7.getGateway();
        assertEq(gateway, mockChainGateway);

        ITokenManagerStructs.RemoteToken memory remoteToken = tokenManagerV7
            .getRemoteTokens(mockToken, mockRemoteToken.chainId);

        assertEq(abi.encode(remoteToken), abi.encode(mockRemoteToken));

        // Verify new state is good
        assertEq(tokenManagerV7.number(), number);
        assertEq(tokenManagerV7.addressValue(), randomAddress);
    }

    // Test changing order of inheritance
    function test_upgrade_change_order_of_inheritance() external {
        // Upgrade again to remove contract in the middle
        UUPSUpgrader.upgrade(
            tokenManagerProxy,
            "TokenManagerTestV8.sol",
            abi.encodeWithSelector(TokenManagerTestV8.reinitialize.selector),
            deployer
        );

        TokenManagerTestV8 tokenManagerV8 = TokenManagerTestV8(
            tokenManagerProxy
        );

        // Verify all state is good
        address gateway = tokenManagerV8.getGateway();
        assertEq(gateway, mockChainGateway);

        ITokenManagerStructs.RemoteToken memory remoteToken = tokenManagerV8
            .getRemoteTokens(mockToken, mockRemoteToken.chainId);

        assertEq(abi.encode(remoteToken), abi.encode(mockRemoteToken));

        // Verify that ownable state is good
        address invalidOwner = makeAddr("Invalid Owner");
        vm.prank(invalidOwner);
        vm.expectRevert(
            abi.encodeWithSelector(
                OwnableUpgradeable.OwnableUnauthorizedAccount.selector,
                invalidOwner
            )
        );
        tokenManagerV8.setGateway(makeAddr("New Gateway"));
    }
}
