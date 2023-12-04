// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity 0.8.20;

import {RelayerTestFixture, Vm, ValidatorManager, BridgeTarget, MessageHashUtils, IReentrancy} from "./Helpers.sol";
import {IRelayerEvents, IRelayerErrors} from "contracts/Relayer.sol";

struct DispatchArgs {
    uint sourceChainId;
    address caller;
    address target;
    bytes call;
    bytes4 callback;
    uint nonce;
}

library DispatchArgsBuilder {
    function instance(
        address caller,
        address target
    ) external pure returns (DispatchArgs memory args) {
        args.sourceChainId = 1;
        args.caller = caller;
        args.call = abi.encodeWithSelector(BridgeTarget.work.selector, uint(1));
        args.target = target;
        args.callback = bytes4(0);
        args.nonce = 1;
    }

    function withCall(
        DispatchArgs memory args,
        bytes calldata call
    ) external pure returns (DispatchArgs memory) {
        args.call = call;
        return args;
    }

    function withCaller(
        DispatchArgs memory args,
        address caller
    ) external pure returns (DispatchArgs memory) {
        args.caller = caller;
        return args;
    }
}

contract Dispatch is RelayerTestFixture, IRelayerEvents {
    using MessageHashUtils for bytes;
    using DispatchArgsBuilder for DispatchArgs;

    BridgeTarget immutable bridgeTarget = new BridgeTarget();

    function setUp() public {
        // Deposit gas from the bridge to the relayer
        bridge.depositFee{value: 1 ether}();
    }

    function signDispatch(
        DispatchArgs memory args
    ) public returns (bytes[] memory signatures) {
        bytes32 hashedMessage = abi
            .encode(
                args.sourceChainId,
                block.chainid,
                args.caller,
                args.target,
                args.call,
                false,
                args.callback,
                args.nonce
            )
            .toEthSignedMessageHash();

        signatures = multiSign(sort(validators), hashedMessage);
    }

    function test_happyPath() external {
        DispatchArgs memory args = DispatchArgsBuilder.instance(
            address(bridge),
            address(bridgeTarget)
        );
        bytes[] memory signatures = signDispatch(args);

        vm.expectCall(address(bridgeTarget), args.call);
        vm.expectEmit(address(relayer));
        emit IRelayerEvents.Dispatched(
            args.sourceChainId,
            args.caller,
            args.callback,
            true,
            abi.encode(uint(2)),
            args.nonce
        );
        relayer.dispatch(
            args.sourceChainId,
            args.caller,
            args.target,
            args.call,
            args.callback,
            args.nonce,
            signatures
        );
        assertEq(
            relayer.dispatched(args.sourceChainId, args.caller, args.nonce),
            true
        );
    }

    function testRevert_badSignature() external {
        // Prepare call
        DispatchArgs memory args = DispatchArgsBuilder.instance(
            address(bridge),
            address(bridgeTarget)
        );
        bytes[] memory signatures = signDispatch(args);
        uint badNonce = args.nonce + 1;

        vm.expectRevert(IRelayerErrors.InvalidSignatures.selector);
        // Dispatch
        relayer.dispatch(
            args.sourceChainId,
            args.caller,
            args.target,
            args.call,
            args.callback,
            badNonce,
            signatures
        );
    }

    function testRevert_replay() external {
        // Prepare call
        DispatchArgs memory args = DispatchArgsBuilder.instance(
            address(bridge),
            address(bridgeTarget)
        );
        bytes[] memory signatures = signDispatch(args);

        // Dispatch
        relayer.dispatch(
            args.sourceChainId,
            args.caller,
            args.target,
            args.call,
            args.callback,
            args.nonce,
            signatures
        );
        // Replay
        vm.expectRevert(IRelayerErrors.AlreadyDispatched.selector);
        relayer.dispatch(
            args.sourceChainId,
            args.caller,
            args.target,
            args.call,
            args.callback,
            args.nonce,
            signatures
        );
    }

    function test_failedCall() external {
        uint num = 1000;
        bytes memory failedCall = abi.encodeWithSelector(
            bridgeTarget.work.selector,
            num
        );
        DispatchArgs memory args = DispatchArgsBuilder
            .instance(address(bridge), address(bridgeTarget))
            .withCall(failedCall);
        bytes[] memory signatures = signDispatch(args);

        // Dispatch
        vm.expectCall(address(bridgeTarget), failedCall);

        bytes memory expectedError = abi.encodeWithSignature(
            "Error(string)",
            "Too large"
        );
        vm.expectEmit(address(relayer));
        emit IRelayerEvents.Dispatched(
            args.sourceChainId,
            args.caller,
            args.callback,
            false,
            expectedError,
            args.nonce
        );
        relayer.dispatch(
            args.sourceChainId,
            args.caller,
            args.target,
            args.call,
            args.callback,
            args.nonce,
            signatures
        );
    }

    function testRevert_nonContractCaller() external {
        DispatchArgs memory args = DispatchArgsBuilder
            .instance(address(bridge), address(bridgeTarget))
            .withCaller(vm.addr(1001));
        bytes[] memory signatures = signDispatch(args);

        // Dispatch
        vm.expectRevert(IRelayerErrors.NonContractCaller.selector);
        relayer.dispatch(
            args.sourceChainId,
            args.caller,
            args.target,
            args.call,
            args.callback,
            args.nonce,
            signatures
        );
    }

    function test_outOfGasCall() external {
        bytes memory call = abi.encodeWithSelector(
            bridgeTarget.infiniteLoop.selector
        );
        DispatchArgs memory args = DispatchArgsBuilder
            .instance(address(bridge), address(bridgeTarget))
            .withCall(call);
        bytes[] memory signatures = signDispatch(args);

        // Dispatch
        vm.expectCall(address(bridgeTarget), args.call);
        vm.expectEmit(address(relayer));
        emit IRelayerEvents.Dispatched(
            args.sourceChainId,
            args.caller,
            args.callback,
            false,
            hex"", // denotes out of gas
            args.nonce
        );
        relayer.dispatch(
            args.sourceChainId,
            args.caller,
            args.target,
            args.call,
            args.callback,
            args.nonce,
            signatures
        );

        assertEq(bridgeTarget.c(), uint(0));
    }

    function test_reentrancy() external {
        bytes memory call = abi.encodeWithSelector(
            bridgeTarget.reentrancy.selector
        );
        DispatchArgs memory args = DispatchArgsBuilder
            .instance(address(bridge), address(bridgeTarget))
            .withCall(call);
        bytes[] memory signatures = signDispatch(args);

        bridgeTarget.setReentrancyConfig(
            address(relayer),
            abi.encodeWithSelector(
                relayer.dispatch.selector,
                args.sourceChainId,
                args.caller,
                args.target,
                args.call,
                args.callback,
                args.nonce,
                signatures
            )
        );

        // Dispatch
        bytes memory expectedError = abi.encodeWithSelector(
            IReentrancy.ReentrancySafe.selector
        );
        vm.expectEmit(address(relayer));
        emit IRelayerEvents.Dispatched(
            args.sourceChainId,
            args.caller,
            args.callback,
            false,
            expectedError,
            args.nonce
        );
        relayer.dispatch(
            args.sourceChainId,
            args.caller,
            args.target,
            args.call,
            args.callback,
            args.nonce,
            signatures
        );
    }
}
