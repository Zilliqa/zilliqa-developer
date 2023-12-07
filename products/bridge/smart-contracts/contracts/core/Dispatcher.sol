// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";
import "contracts/core/ValidatorManager.sol";
import "contracts/core/FeeTracker.sol";

interface IDispatcherEvents {
    event Dispatched(
        uint indexed sourceChainId,
        address indexed target,
        bool success,
        bytes response,
        uint indexed nonce
    );
}

interface IDispatcherErrors {
    error InvalidSignatures();
    error NoSupermajority();
    error NonContractCaller();
    error AlreadyDispatched();
    error NotValidator();
}

interface IDispatcher is IDispatcherEvents, IDispatcherErrors {
    function dispatched(uint sourceChainId, uint nonce) external returns (bool);

    function dispatch(
        uint sourceChainId,
        address target,
        bytes calldata call,
        uint gasLimit,
        uint nonce,
        bytes[] calldata signatures
    ) external;
}

// Cross-chain only
contract Dispatcher is IDispatcher, FeeTracker {
    using MessageHashUtils for bytes;

    ValidatorManager public validatorManager;
    // sourceChainId => nonce => isDispatched
    mapping(uint => mapping(uint => bool)) public dispatched;

    modifier onlyContract(address c) {
        if (c.code.length == 0) {
            revert NonContractCaller();
        }
        _;
    }

    modifier onlyValidator() {
        if (!validatorManager.isValidator(msg.sender)) {
            revert NotValidator();
        }
        _;
    }

    modifier replayDispatchGuard(uint sourceChainId, uint nonce) {
        if (dispatched[sourceChainId][nonce]) {
            revert AlreadyDispatched();
        }
        dispatched[sourceChainId][nonce] = true;
        _;
    }

    constructor(ValidatorManager _validatorManager) {
        validatorManager = _validatorManager;
    }

    function validateRequest(
        bytes memory encodedMessage,
        bytes[] memory signatures
    ) internal view {
        bytes32 hash = encodedMessage.toEthSignedMessageHash();
        if (!validatorManager.validateUniqueSignatures(hash, signatures)) {
            revert InvalidSignatures();
        }
        if (!validatorManager.hasSupermajority(signatures.length)) {
            revert NoSupermajority();
        }
    }

    function dispatch(
        uint sourceChainId,
        address target,
        bytes calldata call,
        uint gasLimit,
        uint nonce,
        bytes[] calldata signatures
    )
        external
        meterFee(target)
        onlyValidator
        onlyContract(target)
        replayDispatchGuard(sourceChainId, nonce)
    {
        bytes memory message = abi.encode(
            sourceChainId,
            block.chainid,
            target,
            call,
            gasLimit,
            nonce
        );
        validateRequest(message, signatures);

        (bool success, bytes memory response) = (target).call{gas: gasLimit}(
            call
        );

        emit Dispatched(sourceChainId, target, success, response, nonce);
    }
}
