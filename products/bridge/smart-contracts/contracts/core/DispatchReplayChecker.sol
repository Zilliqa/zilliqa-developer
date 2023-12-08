// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

interface IDispatchReplayErrors {
    error AlreadyDispatched();
}

abstract contract DispatchReplayChecker is IDispatchReplayErrors {
    // sourceChainId => nonce => isDispatched
    mapping(uint => mapping(uint => bool)) public dispatched;

    function _replayDispatchCheck(uint sourceShardId, uint nonce) internal {
        if (dispatched[sourceShardId][nonce]) {
            revert AlreadyDispatched();
        }
        dispatched[sourceShardId][nonce] = true;
    }

    modifier replayDispatchGuard(uint sourceShardId, uint nonce) {
        _replayDispatchCheck(sourceShardId, nonce);
        _;
    }
}
