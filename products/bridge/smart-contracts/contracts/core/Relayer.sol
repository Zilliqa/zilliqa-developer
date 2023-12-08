// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

interface IRelayerEvents {
    event Relayed(
        uint indexed targetChainId,
        address target,
        bytes call,
        uint gasLimit,
        uint nonce
    );
}

interface IRelayer is IRelayerEvents {
    function nonce() external returns (uint);

    function relay(
        uint targetChainId,
        address target,
        bytes calldata call,
        uint gasLimit
    ) external returns (uint);
}

contract Relayer is IRelayer {
    uint public nonce;

    function relay(
        uint targetChainId,
        address target,
        bytes calldata call,
        uint gasLimit
    ) external returns (uint) {
        emit Relayed(targetChainId, target, call, gasLimit, nonce);
        return nonce++;
    }
}
