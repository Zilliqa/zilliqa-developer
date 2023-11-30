// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity 0.8.20;

import {RelayerTestFixture, Vm, ValidatorManager, BridgeTarget, MessageHashUtils, IRelayerEvents} from "./Helpers.sol";

contract Dispatch is RelayerTestFixture {
    using MessageHashUtils for bytes;

    BridgeTarget immutable bridgeTarget = new BridgeTarget();

    function setUp() public {
        // Deposit gas from the bridge to the relayer
        bridge.depositFee{value: 1 ether}();
    }

    function getDispatchArgs()
        public
        returns (
            uint sourceChainId,
            address caller,
            address target,
            bytes memory call,
            bytes4 callback,
            uint nonce,
            bytes[] memory signatures
        )
    {
        sourceChainId = 1;
        caller = address(bridge);
        call = abi.encodeWithSelector(bridgeTarget.work.selector, uint(1));
        target = address(bridgeTarget);
        callback = bytes4(0);
        nonce = 1;
        // Generate signatures
        signatures = signDispatch(
            sourceChainId,
            caller,
            target,
            call,
            callback,
            nonce
        );
    }

    function getDispatchArgs(
        bytes memory call
    )
        public
        returns (
            uint sourceChainId,
            address caller,
            address target,
            bytes4 callback,
            uint nonce,
            bytes[] memory signatures
        )
    {
        sourceChainId = 1;
        caller = address(bridge);
        target = address(bridgeTarget);
        callback = bytes4(0);
        nonce = 1;
        // Generate signatures
        signatures = signDispatch(
            sourceChainId,
            caller,
            target,
            call,
            callback,
            nonce
        );
    }

    function signDispatch(
        uint sourceChainId,
        address caller,
        address target,
        bytes memory call,
        bytes4 callback,
        uint nonce
    ) public returns (bytes[] memory signatures) {
        bytes32 hashedMessage = abi
            .encode(
                sourceChainId,
                block.chainid,
                caller,
                target,
                call,
                false,
                callback,
                nonce
            )
            .toEthSignedMessageHash();

        signatures = multiSign(sort(validators), hashedMessage);
    }

    function test_happyPath() external {
        (
            uint sourceChainId,
            address caller,
            address target,
            bytes memory call,
            bytes4 callback,
            uint nonce,
            bytes[] memory signatures
        ) = getDispatchArgs();

        vm.expectCall(address(bridgeTarget), call);
        vm.expectEmit(address(relayer));
        emit IRelayerEvents.Dispatched(
            sourceChainId,
            caller,
            callback,
            true,
            abi.encode(uint(2)),
            nonce
        );
        relayer.dispatch(
            sourceChainId,
            caller,
            target,
            call,
            callback,
            nonce,
            signatures
        );
        // Check nonce increased
    }

    function testRevert_badSignature() external {
        // Prepare call
        (
            uint sourceChainId,
            address caller,
            address target,
            bytes memory call,
            bytes4 callback,
            uint signedNonce,
            bytes[] memory signatures
        ) = getDispatchArgs();
        uint dispatchedNonce = signedNonce + 1;

        vm.expectRevert(InvalidSignatures.selector);
        // Dispatch
        relayer.dispatch(
            sourceChainId,
            caller,
            target,
            call,
            callback,
            dispatchedNonce,
            signatures
        );
    }

    function testRevert_replay() external {
        // Prepare call
        (
            uint sourceChainId,
            address caller,
            address target,
            bytes memory call,
            bytes4 callback,
            uint nonce,
            bytes[] memory signatures
        ) = getDispatchArgs();

        // Dispatch
        relayer.dispatch(
            sourceChainId,
            caller,
            target,
            call,
            callback,
            nonce,
            signatures
        );
        // Replay
        vm.expectRevert(AlreadyDispatched.selector);
        relayer.dispatch(
            sourceChainId,
            caller,
            target,
            call,
            callback,
            nonce,
            signatures
        );
    }

    function test_unsuccessfulCalls() external {
        uint num = 1000;
        bytes memory failedCall = abi.encodeWithSelector(
            bridgeTarget.work.selector,
            num
        );
        (
            uint sourceChainId,
            address caller,
            address target,
            bytes4 callback,
            uint nonce,
            bytes[] memory signatures
        ) = getDispatchArgs(failedCall);

        // Dispatch
        vm.expectCall(address(bridgeTarget), failedCall);

        bytes memory expectedError = abi.encodeWithSignature(
            "Error(string)",
            "Too large"
        );
        vm.expectEmit(address(relayer));
        emit IRelayerEvents.Dispatched(
            sourceChainId,
            caller,
            callback,
            false,
            expectedError,
            nonce
        );
        relayer.dispatch(
            sourceChainId,
            caller,
            target,
            failedCall,
            callback,
            nonce,
            signatures
        );
    }

    function testRevert_nonContractCaller() external {
        (
            uint sourceChainId,
            ,
            address target,
            bytes memory call,
            bytes4 callback,
            uint nonce,
            bytes[] memory signatures
        ) = getDispatchArgs();

        // Dispatch
        vm.expectRevert(NonContractCaller.selector);
        relayer.dispatch(
            sourceChainId,
            vm.addr(1001),
            target,
            call,
            callback,
            nonce,
            signatures
        );
    }

    function testRevert_overspentgas() external {
        bytes memory call = abi.encodeWithSelector(
            bridgeTarget.infiniteLoop.selector
        );
        (
            uint sourceChainId,
            address caller,
            address target,
            bytes4 callback,
            uint nonce,
            bytes[] memory signatures
        ) = getDispatchArgs(call);

        // Dispatch
        vm.expectCall(address(bridgeTarget), call);
        vm.expectEmit(address(relayer));
        emit IRelayerEvents.Dispatched(
            sourceChainId,
            caller,
            callback,
            false,
            hex"", // denotes out of gas
            nonce
        );
        relayer.dispatch(
            sourceChainId,
            caller,
            target,
            call,
            callback,
            nonce,
            signatures
        );

        assertEq(bridgeTarget.c(), uint(0));
    }
}
