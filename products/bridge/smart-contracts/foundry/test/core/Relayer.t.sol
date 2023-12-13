// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity 0.8.20;

import {Tester} from "foundry/test/Tester.sol";
import {Relayer, IRelayerEvents, CallMetadata} from "contracts/core/Relayer.sol";

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
    Relayer relayer = new Relayer();

    function test_relay_happyPath() external {
        uint nonce = 0;
        uint targetChainId = 1;
        address target = address(0x1);
        bytes memory call = abi.encodeWithSelector(ITest.foo.selector);
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

    function test_relay_identicalConsecutiveCallsHaveDifferentNonce() external {
        uint nonce = 0;
        uint targetChainId = 1;
        address target = address(0x1);
        bytes memory call = abi.encodeWithSelector(ITest.foo.selector);
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

    function test_relayWithMetadata_happyPath() external {
        uint nonce = 0;
        uint targetChainId = 1;
        address target = address(0x1);
        bytes4 callSelector = ITest.foo.selector;
        bytes memory callData = abi.encode(Args(1));
        uint gasLimit = 100_000;

        bytes memory expectedCall = abi.encodeWithSelector(
            callSelector,
            abi.encode(CallMetadata(block.chainid, address(this))),
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
}
