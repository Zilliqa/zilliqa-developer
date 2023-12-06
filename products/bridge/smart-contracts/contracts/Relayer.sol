// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

interface IRelayerEvents {
    event Relayed(
        uint indexed targetChainId,
        address target,
        bytes call,
        uint nonce
    );
}

interface IRelayer is IRelayerEvents {
    function nonces(address target) external returns (uint);

    function relay(
        uint targetChainId,
        address target,
        bytes calldata call
    ) external returns (uint);
}

contract Relayer is IRelayer {
    // target => nonce
    mapping(address => uint) public nonces;

    function relay(
        uint targetChainId,
        address target,
        bytes calldata call
    ) external returns (uint) {
        emit Relayed(targetChainId, target, call, nonces[msg.sender]);
        return nonces[msg.sender]++;
    }
}
