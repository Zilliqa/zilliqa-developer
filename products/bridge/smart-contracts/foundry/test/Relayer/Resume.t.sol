// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {RelayerTestFixture, MessageHashUtils, BridgeTarget, IReentrancy} from "./Helpers.sol";
import {IRelayerEvents, IRelayerErrors} from "contracts/Relayer.sol";

struct ResumeArgs {
    uint targetChainId;
    address caller;
    bytes4 callback;
    bool success;
    bytes response;
    uint nonce;
    bytes call;
}

library ResumeArgsBuilder {
    function instance(
        address caller
    ) external pure returns (ResumeArgs memory args) {
        args.targetChainId = 1;
        args.caller = caller;
        args.callback = BridgeTarget.finish.selector;
        args.success = true;
        args.response = "worked";
        args.nonce = 1;
        args.call = encodeCall(args);
    }

    function withSuccess(
        ResumeArgs memory args,
        bool _success
    ) external pure returns (ResumeArgs memory) {
        args.success = _success;
        args.call = encodeCall(args);
        return args;
    }

    function withCallback(
        ResumeArgs memory args,
        bytes4 callback
    ) external pure returns (ResumeArgs memory) {
        args.callback = callback;
        args.call = encodeCall(args);
        return args;
    }

    function withNonce(
        ResumeArgs memory args,
        uint nonce
    ) external pure returns (ResumeArgs memory) {
        args.nonce = nonce;
        args.call = encodeCall(args);
        return args;
    }

    function withCaller(
        ResumeArgs memory args,
        address caller
    ) external pure returns (ResumeArgs memory) {
        args.caller = caller;
        args.call = encodeCall(args);
        return args;
    }

    function encodeCall(
        ResumeArgs memory args
    ) public pure returns (bytes memory) {
        return
            abi.encodeWithSelector(
                args.callback,
                args.success,
                args.response,
                args.nonce
            );
    }
}

contract Resume is RelayerTestFixture, IRelayerEvents {
    using MessageHashUtils for bytes;
    using stdStorage for StdStorage;
    using ResumeArgsBuilder for ResumeArgs;

    BridgeTarget immutable bridgeTarget = new BridgeTarget();

    function setUp() public {
        // Deposit gas from the bridge to the relayer
        bridge.depositFee{value: 1 ether}();
        // Set nonces to 1 for the bridge target, emulate that it has relayed a call
        stdstore
            .target(address(relayer))
            .sig(relayer.nonces.selector)
            .with_key(address(bridgeTarget))
            .checked_write(1);
    }

    function signResume(
        ResumeArgs memory args
    ) public returns (bytes[] memory signatures) {
        bytes32 hashedMessage = abi
            .encode(
                block.chainid,
                args.targetChainId,
                args.caller,
                args.callback,
                args.success,
                args.response,
                args.nonce
            )
            .toEthSignedMessageHash();

        signatures = multiSign(sort(validators), hashedMessage);
    }

    function test_happyPath() external {
        ResumeArgs memory args = ResumeArgsBuilder.instance(
            address(bridgeTarget)
        );
        bytes[] memory signatures = signResume(args);

        vm.expectCall(address(bridgeTarget), args.call);
        vm.expectEmit(address(relayer));
        emit IRelayerEvents.Resumed(
            args.targetChainId,
            args.caller,
            args.call,
            true,
            hex"",
            args.nonce
        );
        relayer.resume(
            args.targetChainId,
            args.caller,
            args.callback,
            args.success,
            args.response,
            args.nonce,
            signatures
        );
        assertEq(relayer.resumed(args.caller, args.nonce), true);
    }

    function testRevert_badSignature() external {
        ResumeArgs memory args = ResumeArgsBuilder.instance(
            address(bridgeTarget)
        );
        bytes[] memory signatures = signResume(args);
        uint badNonce = args.nonce - 1;

        vm.expectRevert(IRelayerErrors.InvalidSignatures.selector);
        relayer.resume(
            args.targetChainId,
            args.caller,
            args.callback,
            args.success,
            args.response,
            badNonce,
            signatures
        );
    }

    function testRevert_illegalResumeNonce() external {
        ResumeArgs memory args = ResumeArgsBuilder
            .instance(address(bridgeTarget))
            .withNonce(2);
        bytes[] memory signatures = signResume(args);

        vm.expectRevert(IRelayerErrors.IllegalResumeNonce.selector);
        relayer.resume(
            args.targetChainId,
            args.caller,
            args.callback,
            args.success,
            args.response,
            args.nonce,
            signatures
        );
    }

    function testRevert_replay() external {
        ResumeArgs memory args = ResumeArgsBuilder.instance(
            address(bridgeTarget)
        );
        bytes[] memory signatures = signResume(args);

        relayer.resume(
            args.targetChainId,
            args.caller,
            args.callback,
            args.success,
            args.response,
            args.nonce,
            signatures
        );

        vm.expectRevert(IRelayerErrors.AlreadyResumed.selector);
        relayer.resume(
            args.targetChainId,
            args.caller,
            args.callback,
            args.success,
            args.response,
            args.nonce,
            signatures
        );
    }

    function test_failedCallback() external {
        bytes4 callback = bridgeTarget.finishRevert.selector;
        ResumeArgs memory args = ResumeArgsBuilder
            .instance(address(bridgeTarget))
            .withCallback(callback);
        bytes[] memory signatures = signResume(args);

        vm.expectCall(address(bridgeTarget), args.call);
        vm.expectEmit(address(relayer));
        emit IRelayerEvents.Resumed(
            args.targetChainId,
            args.caller,
            args.call,
            false,
            hex"",
            args.nonce
        );
        relayer.resume(
            args.targetChainId,
            args.caller,
            args.callback,
            args.success,
            args.response,
            args.nonce,
            signatures
        );
    }

    function test_failedCall() external {
        ResumeArgs memory args = ResumeArgsBuilder
            .instance(address(bridgeTarget))
            .withSuccess(false);
        bytes[] memory signatures = signResume(args);

        vm.expectCall(address(bridgeTarget), args.call);
        vm.expectEmit(address(relayer));
        emit IRelayerEvents.Resumed(
            args.targetChainId,
            args.caller,
            args.call,
            true,
            hex"",
            args.nonce
        );
        relayer.resume(
            args.targetChainId,
            args.caller,
            args.callback,
            args.success,
            args.response,
            args.nonce,
            signatures
        );
    }

    function testRevert_nonContractCaller() external {
        ResumeArgs memory args = ResumeArgsBuilder
            .instance(address(bridgeTarget))
            .withCaller(vm.addr(1001));
        bytes[] memory signatures = signResume(args);

        vm.expectRevert(IRelayerErrors.NonContractCaller.selector);
        relayer.resume(
            args.targetChainId,
            args.caller,
            args.callback,
            args.success,
            args.response,
            args.nonce,
            signatures
        );
    }

    function test_outOfGasCallback() external {
        bytes4 callback = bridgeTarget.infiniteLoop.selector;
        ResumeArgs memory args = ResumeArgsBuilder
            .instance(address(bridgeTarget))
            .withCallback(callback);
        bytes[] memory signatures = signResume(args);

        vm.expectCall(address(bridgeTarget), args.call);
        vm.expectEmit(address(relayer));
        emit IRelayerEvents.Resumed(
            args.targetChainId,
            args.caller,
            args.call,
            false,
            hex"",
            args.nonce
        );
        relayer.resume(
            args.targetChainId,
            args.caller,
            args.callback,
            args.success,
            args.response,
            args.nonce,
            signatures
        );
    }

    function test_reentrancy() external {
        bytes4 callback = bridgeTarget.reentrancy.selector;
        ResumeArgs memory args = ResumeArgsBuilder
            .instance(address(bridgeTarget))
            .withCallback(callback);
        bytes[] memory signatures = signResume(args);

        bridgeTarget.setReentrancyConfig(
            address(relayer),
            abi.encodeWithSelector(
                relayer.resume.selector,
                args.targetChainId,
                args.caller,
                args.callback,
                args.success,
                args.response,
                args.nonce,
                signatures
            )
        );

        bytes memory expectedError = abi.encodeWithSelector(
            IReentrancy.ReentrancySafe.selector
        );
        vm.expectEmit(address(relayer));
        emit IRelayerEvents.Resumed(
            args.targetChainId,
            args.caller,
            args.call,
            false,
            expectedError,
            args.nonce
        );
        relayer.resume(
            args.targetChainId,
            args.caller,
            args.callback,
            args.success,
            args.response,
            args.nonce,
            signatures
        );
    }
}
