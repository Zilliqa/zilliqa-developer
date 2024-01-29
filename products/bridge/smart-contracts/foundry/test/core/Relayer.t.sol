// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity 0.8.20;

import {Tester} from "foundry/test/Tester.sol";
import {Relayer, IRelayerEvents, CallMetadata} from "contracts/core/Relayer.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {Registry} from "contracts/core/Registry.sol";

struct Args {
    uint num;
}

interface ITest {
    function foo() external;

    function fooWithMetadata(
        CallMetadata calldata call,
        Args calldata data
    ) external;
}

contract RelayerTests is Tester, IRelayerEvents {
    Relayer relayer;
    address owner = vm.addr(100);
    address registered = vm.addr(101);

    function setUp() external {
        relayer = new Relayer(owner);
        vm.prank(owner);
        relayer.register(registered);
    }

    function test_relay_happyPath() external {
        uint nonce = 0;
        uint targetChainId = 1;
        address target = address(0x1);
        bytes memory call = abi.encodeWithSelector(ITest.foo.selector);
        uint gasLimit = 100_000;

        vm.expectEmit(address(relayer));
        vm.prank(registered);
        emit IRelayerEvents.Relayed(
            targetChainId,
            target,
            call,
            gasLimit,
            nonce
        );
        uint result = relayer.relay(targetChainId, target, call, gasLimit);

        assertEq(result, nonce);
        assertEq(relayer.nonce(), nonce + 1);
    }

    function test_relay_identicalConsecutiveCallsHaveDifferentNonce() external {
        uint nonce = 0;
        uint targetChainId = 1;
        address target = address(0x1);
        bytes memory call = abi.encodeWithSelector(ITest.foo.selector);
        uint gasLimit = 100_000;

        vm.expectEmit(address(relayer));
        vm.prank(registered);
        emit IRelayerEvents.Relayed(
            targetChainId,
            target,
            call,
            gasLimit,
            nonce
        );
        uint result = relayer.relay(targetChainId, target, call, gasLimit);

        assertEq(result, nonce);
        assertEq(relayer.nonce(), nonce + 1);

        nonce++;

        vm.expectEmit(address(relayer));
        emit IRelayerEvents.Relayed(
            targetChainId,
            target,
            call,
            gasLimit,
            nonce
        );
        vm.prank(registered);
        result = relayer.relay(targetChainId, target, call, gasLimit);
        assertEq(result, nonce);
        assertEq(relayer.nonce(), nonce + 1);
    }

    function test_relayWithMetadata_happyPath() external {
        uint nonce = 0;
        uint targetChainId = 1;
        address target = address(0x1);
        bytes4 callSelector = ITest.foo.selector;
        bytes memory callData = abi.encode(Args(1));
        uint gasLimit = 100_000;

        bytes memory expectedCall = abi.encodeWithSelector(
            callSelector,
            CallMetadata(block.chainid, registered),
            callData
        );

        vm.expectEmit(address(relayer));
        emit IRelayerEvents.Relayed(
            targetChainId,
            target,
            expectedCall,
            gasLimit,
            nonce
        );
        vm.prank(registered);
        uint result = relayer.relayWithMetadata(
            targetChainId,
            target,
            callSelector,
            callData,
            gasLimit
        );

        assertEq(result, nonce);
        assertEq(relayer.nonce(), nonce + 1);
    }

    function test_RevertNonRegisteredSender() external {
        uint targetChainId = 1;
        address target = address(0x1);
        bytes memory call = abi.encodeWithSelector(ITest.foo.selector);
        uint gasLimit = 100_000;
        address notRegisteredSender = vm.addr(10);

        vm.prank(notRegisteredSender);
        vm.expectRevert(
            abi.encodeWithSelector(
                Registry.NotRegistered.selector,
                notRegisteredSender
            )
        );
        relayer.relay(targetChainId, target, call, gasLimit);
    }

    function test_removeRegisteredSender() external {
        uint targetChainId = 1;
        address target = address(0x1);
        bytes memory call = abi.encodeWithSelector(ITest.foo.selector);
        uint gasLimit = 100_000;

        vm.prank(owner);
        relayer.unregister(registered);
        assertEq(relayer.registered(registered), false);

        vm.prank(registered);
        vm.expectRevert(
            abi.encodeWithSelector(Registry.NotRegistered.selector, registered)
        );
        relayer.relay(targetChainId, target, call, gasLimit);
    }

    function test_RevertUnauthorizedRegister() external {
        address notOwner = vm.createWallet("notOwner").addr;
        address newRegistrant = vm.createWallet("newRegistrant").addr;

        vm.prank(notOwner);
        vm.expectRevert(
            abi.encodeWithSelector(
                Ownable.OwnableUnauthorizedAccount.selector,
                notOwner
            )
        );
        relayer.register(newRegistrant);
    }

    function test_RevertUnauthorizedUnregister() external {
        address notOwner = vm.createWallet("notOwner").addr;

        vm.prank(notOwner);
        vm.expectRevert(
            abi.encodeWithSelector(
                Ownable.OwnableUnauthorizedAccount.selector,
                notOwner
            )
        );
        relayer.unregister(registered);
    }
}
