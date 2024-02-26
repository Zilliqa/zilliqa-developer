// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

interface IDispatchReplayCheckerErrors {
    error AlreadyDispatched();
}

interface IDispatchReplayChecker is IDispatchReplayCheckerErrors {
    function dispatched(
        uint sourceChainId,
        uint nonce
    ) external view returns (bool);
}

abstract contract DispatchReplayCheckerUpgradeable is IDispatchReplayChecker {
    /// @custom:storage-location erc7201:zilliqa.storage.DispatchReplayChecker
    struct DispatchReplayCheckerStorage {
        // sourceChainId => nonce => isDispatched
        mapping(uint => mapping(uint => bool)) dispatched;
    }

    // keccak256(abi.encode(uint256(keccak256("zilliqa.storage.DispatchReplayChecker")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant DISPATCH_REPLAY_CHECKER_STORAGE_POSITION =
        0xf0d7858cd36fafa025d5af5f0a6a6196668a9b0994a77eee7583c69fc18dfb00;

    function _getDispatchReplayCheckerStorage()
        private
        pure
        returns (DispatchReplayCheckerStorage storage $)
    {
        assembly {
            $.slot := DISPATCH_REPLAY_CHECKER_STORAGE_POSITION
        }
    }

    function dispatched(
        uint sourceChainId,
        uint nonce
    ) external view returns (bool) {
        DispatchReplayCheckerStorage
            storage $ = _getDispatchReplayCheckerStorage();
        return $.dispatched[sourceChainId][nonce];
    }

    function _replayDispatchCheck(uint sourceChainId, uint nonce) internal {
        DispatchReplayCheckerStorage
            storage $ = _getDispatchReplayCheckerStorage();

        if ($.dispatched[sourceChainId][nonce]) {
            revert AlreadyDispatched();
        }
        $.dispatched[sourceChainId][nonce] = true;
    }

    modifier replayDispatchGuard(uint sourceShardId, uint nonce) {
        _replayDispatchCheck(sourceShardId, nonce);
        _;
    }
}
