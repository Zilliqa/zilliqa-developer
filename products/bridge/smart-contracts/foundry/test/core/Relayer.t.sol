// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity 0.8.20;

import {Tester} from "foundry/test/Tester.sol";
import {Relayer, IRelayerEvents} from "contracts/core/Relayer.sol";

contract RelayerTests is Tester, IRelayerEvents {
    Relayer relayer = new Relayer();

    function test_happyPath() external {
        uint nonce = 0;
        uint targetChainId = 1;
        address target = address(0x1);
        bytes memory call = abi.encodeWithSignature("foo()");
        uint gasLimit = 100_000;

        vm.expectEmit(address(relayer));
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

    function test_identicalConsecutiveCallsHaveDifferentNonce() external {
        uint nonce = 0;
        uint targetChainId = 1;
        address target = address(0x1);
        bytes memory call = abi.encodeWithSignature("foo()");
        uint gasLimit = 100_000;

        vm.expectEmit(address(relayer));
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
        result = relayer.relay(targetChainId, target, call, gasLimit);
        assertEq(result, nonce);
        assertEq(relayer.nonce(), nonce + 1);
    }
}
