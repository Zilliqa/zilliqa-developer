// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity 0.8.20;

import {Tester} from "foundry/test/Tester.sol";
import {DispatchReplayChecker, IDispatchReplayCheckerErrors} from "contracts/core/DispatchReplayChecker.sol";

contract DispatchReplayCheckerHarness is DispatchReplayChecker {
    function exposed_replayDispatchCheck(
        uint sourceShardId,
        uint nonce
    ) external {
        _replayDispatchCheck(sourceShardId, nonce);
    }
}

contract DispatchReplayCheckerTests is Tester {
    DispatchReplayCheckerHarness harness = new DispatchReplayCheckerHarness();

    function test_happyPath() external {
        uint sourceShardId = 0;
        uint nonce = 0;

        harness.exposed_replayDispatchCheck(sourceShardId, nonce);

        assertEq(
            harness.dispatched(sourceShardId, nonce),
            true,
            "should have marked dispatched"
        );
    }

    function testRevert_whenAlreadyDispatched() external {
        uint sourceShardId = 0;
        uint nonce = 0;

        harness.exposed_replayDispatchCheck(sourceShardId, nonce);
        assertEq(
            harness.dispatched(sourceShardId, nonce),
            true,
            "should have marked dispatched"
        );

        vm.expectRevert(
            IDispatchReplayCheckerErrors.AlreadyDispatched.selector
        );
        harness.exposed_replayDispatchCheck(sourceShardId, nonce);
    }

    function test_sameNonceDifferentSourceShard() external {
        uint chain1 = 0;
        uint chain2 = 1;
        uint nonce = 0;

        harness.exposed_replayDispatchCheck(chain1, nonce);
        assertEq(
            harness.dispatched(chain1, nonce),
            true,
            "should have marked dispatched"
        );

        harness.exposed_replayDispatchCheck(chain2, nonce);
        assertEq(
            harness.dispatched(chain2, nonce),
            true,
            "should have marked dispatched"
        );
    }
}
