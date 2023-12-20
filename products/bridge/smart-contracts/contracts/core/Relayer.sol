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

interface IRelayer is IRelayerEvents {}

struct CallMetadata {
    uint sourceChainId;
    address sender;
}

contract Relayer is IRelayer {
    uint public nonce;

    // Use this function to relay a call with metadata. This is useful for calling surrogate contracts.
    // Ensure the surrogate implements this interface
    function relayWithMetadata(
        uint targetChainId,
        address target,
        bytes4 callSelector,
        bytes calldata callData,
        uint gasLimit
    ) external returns (uint) {
        emit Relayed(
            targetChainId,
            target,
            abi.encodeWithSelector(
                callSelector,
                abi.encode(CallMetadata(block.chainid, msg.sender)),
                callData
            ),
            gasLimit,
            nonce
        );

        return nonce++;
    }

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
