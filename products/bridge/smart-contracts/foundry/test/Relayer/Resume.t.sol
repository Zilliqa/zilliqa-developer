// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity 0.8.20;

import {RelayerTestFixture, MessageHashUtils, BridgeTarget, IReentrancy} from "./Helpers.sol";
import {IRelayerEvents, IRelayerErrors} from "contracts/Relayer.sol";

contract Resume is RelayerTestFixture, IRelayerEvents {
    using MessageHashUtils for bytes;

    BridgeTarget immutable bridgeTarget = new BridgeTarget();

    function setUp() public {
        // Deposit gas from the bridge to the relayer
        bridge.depositFee{value: 1 ether}();
    }

    function getResumeArgs(
        bool _success
    )
        public
        returns (
            uint targetChainId,
            address caller,
            bytes4 callback,
            bool success,
            bytes memory response,
            uint nonce,
            bytes memory call,
            bytes[] memory signatures
        )
    {
        targetChainId = 1;
        caller = address(bridgeTarget);
        callback = bridgeTarget.finish.selector;
        success = _success;
        nonce = 1;
        response = "worked";
        call = abi.encodeWithSelector(callback, success, response, nonce);
        // Generate signatures
        signatures = signResume(
            targetChainId,
            caller,
            callback,
            success,
            response,
            nonce
        );
    }

    function getResumeArgs(
        bytes4 _callback
    )
        public
        returns (
            uint targetChainId,
            address caller,
            bytes4 callback,
            bool success,
            bytes memory response,
            uint nonce,
            bytes memory call,
            bytes[] memory signatures
        )
    {
        targetChainId = 1;
        caller = address(bridgeTarget);
        callback = _callback;
        success = true;
        nonce = 1;
        response = "worked";
        call = abi.encodeWithSelector(callback, success, response, nonce);
        // Generate signatures
        signatures = signResume(
            targetChainId,
            caller,
            callback,
            success,
            response,
            nonce
        );
    }

    function signResume(
        uint targetChainId,
        address caller,
        bytes4 callback,
        bool success,
        bytes memory response,
        uint nonce
    ) public returns (bytes[] memory signatures) {
        bytes32 hashedMessage = abi
            .encode(
                block.chainid,
                targetChainId,
                caller,
                callback,
                success,
                response,
                nonce
            )
            .toEthSignedMessageHash();

        signatures = multiSign(sort(validators), hashedMessage);
    }

    function test_happyPath() external {
        (
            uint targetChainId,
            address caller,
            bytes4 callback,
            bool success,
            bytes memory response,
            uint nonce,
            bytes memory call,
            bytes[] memory signatures
        ) = getResumeArgs(true);

        vm.expectCall(address(bridgeTarget), call);
        vm.expectEmit(address(relayer));
        emit IRelayerEvents.Resumed(
            targetChainId,
            caller,
            call,
            true,
            hex"",
            nonce
        );
        relayer.resume(
            targetChainId,
            caller,
            callback,
            success,
            response,
            nonce,
            signatures
        );
    }

    function testRevert_badSignature() external {
        (
            uint targetChainId,
            address caller,
            bytes4 callback,
            bool success,
            bytes memory response,
            uint nonce,
            ,
            bytes[] memory signatures
        ) = getResumeArgs(true);
        uint dispatchedNonce = nonce + 1;

        vm.expectRevert(IRelayerErrors.InvalidSignatures.selector);
        relayer.resume(
            targetChainId,
            caller,
            callback,
            success,
            response,
            dispatchedNonce,
            signatures
        );
    }

    function testRevert_replay() external {
        (
            uint targetChainId,
            address caller,
            bytes4 callback,
            bool success,
            bytes memory response,
            uint nonce,
            ,
            bytes[] memory signatures
        ) = getResumeArgs(true);
        relayer.resume(
            targetChainId,
            caller,
            callback,
            success,
            response,
            nonce,
            signatures
        );

        vm.expectRevert(IRelayerErrors.AlreadyResumed.selector);
        relayer.resume(
            targetChainId,
            caller,
            callback,
            success,
            response,
            nonce,
            signatures
        );
    }

    function test_failedCallback() external {
        bytes4 _callback = bridgeTarget.finishRevert.selector;
        (
            uint targetChainId,
            address caller,
            bytes4 callback,
            bool success,
            bytes memory response,
            uint nonce,
            bytes memory call,
            bytes[] memory signatures
        ) = getResumeArgs(_callback);

        vm.expectCall(address(bridgeTarget), call);
        vm.expectEmit(address(relayer));
        emit IRelayerEvents.Resumed(
            targetChainId,
            caller,
            call,
            false,
            hex"",
            nonce
        );
        relayer.resume(
            targetChainId,
            caller,
            callback,
            success,
            response,
            nonce,
            signatures
        );
    }

    function test_failedCall() external {
        (
            uint targetChainId,
            address caller,
            bytes4 callback,
            bool success,
            bytes memory response,
            uint nonce,
            bytes memory call,
            bytes[] memory signatures
        ) = getResumeArgs(false);

        vm.expectCall(address(bridgeTarget), call);
        vm.expectEmit(address(relayer));
        emit IRelayerEvents.Resumed(
            targetChainId,
            caller,
            call,
            true,
            hex"",
            nonce
        );
        relayer.resume(
            targetChainId,
            caller,
            callback,
            success,
            response,
            nonce,
            signatures
        );
    }

    function testRevert_nonContractCaller() external {
        (
            uint targetChainId,
            ,
            bytes4 callback,
            bool success,
            bytes memory response,
            uint nonce,
            ,
            bytes[] memory signatures
        ) = getResumeArgs(true);

        vm.expectRevert(IRelayerErrors.NonContractCaller.selector);
        relayer.resume(
            targetChainId,
            vm.addr(1001),
            callback,
            success,
            response,
            nonce,
            signatures
        );
    }

    function test_outOfGasCallback() external {
        bytes4 _callback = bridgeTarget.infiniteLoop.selector;
        (
            uint targetChainId,
            address caller,
            bytes4 callback,
            bool success,
            bytes memory response,
            uint nonce,
            bytes memory call,
            bytes[] memory signatures
        ) = getResumeArgs(_callback);

        vm.expectCall(address(bridgeTarget), call);
        vm.expectEmit(address(relayer));
        emit IRelayerEvents.Resumed(
            targetChainId,
            caller,
            call,
            false,
            hex"",
            nonce
        );
        relayer.resume(
            targetChainId,
            caller,
            callback,
            success,
            response,
            nonce,
            signatures
        );
    }

    function test_reentrancy() external {
        bytes4 _callback = bridgeTarget.reentrancy.selector;
        (
            uint targetChainId,
            address caller,
            bytes4 callback,
            bool success,
            bytes memory response,
            uint nonce,
            bytes memory call,
            bytes[] memory signatures
        ) = getResumeArgs(_callback);

        bridgeTarget.setReentrancyConfig(
            address(relayer),
            abi.encodeWithSelector(
                relayer.resume.selector,
                targetChainId,
                caller,
                callback,
                success,
                response,
                nonce,
                signatures
            )
        );

        bytes memory expectedError = abi.encodeWithSelector(
            IReentrancy.ReentrancySafe.selector
        );
        vm.expectEmit(address(relayer));
        emit IRelayerEvents.Resumed(
            targetChainId,
            caller,
            call,
            false,
            expectedError,
            nonce
        );
        relayer.resume(
            targetChainId,
            caller,
            callback,
            success,
            response,
            nonce,
            signatures
        );
    }
}
