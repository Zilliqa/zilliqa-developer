// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import "hardhat/console.sol";

import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";
import "@openzeppelin/contracts/utils/Create2.sol";
import "./ValidatorManager.sol";
import "./Bridged.sol";

using ECDSA for bytes32;
using MessageHashUtils for bytes;

contract Relayer {
    ValidatorManager private validatorManager;

    event TwinDeployment(address indexed twin);

    function deployTwin(
        bytes32 salt,
        bytes calldata bytecode,
        bytes calldata initCall
    ) public returns (address) {
        address bridgedContract = Create2.deploy(0, salt, bytecode);
        (bool success, ) = bridgedContract.call(initCall);
        require(success, "initialization failed");
        emit TwinDeployment(bridgedContract);
        return bridgedContract;
    }

    constructor(ValidatorManager _validatorManager) {
        validatorManager = _validatorManager;
    }

    // targetChainId => caller => nonce
    mapping(uint => mapping(address => uint)) nonces;
    // sourceChainId => caller => dispatched
    mapping(uint => mapping(address => mapping(uint => bool))) dispatched;
    // sourceChainId => caller => resumed
    mapping(uint => mapping(address => mapping(uint => bool))) resumed;

    event Relayed(
        uint indexed targetChainId,
        address caller,
        address target,
        bytes call,
        bool readonly,
        bytes4 callback,
        uint nonce
    );

    function relay(
        uint targetChainId,
        address target,
        bytes memory call,
        bool readonly,
        bytes4 callback
    ) public returns (uint) {
        emit Relayed(
            targetChainId,
            msg.sender,
            target,
            call,
            readonly,
            callback,
            nonces[targetChainId][msg.sender]
        );
        nonces[targetChainId][msg.sender]++;
        return nonces[targetChainId][msg.sender];
    }

    event Dispatched(
        uint indexed sourceChainId,
        address indexed caller,
        bytes4 callback,
        bool success,
        bytes response,
        uint indexed nonce
    );

    function validateRequest(
        bytes memory encodedMessage,
        bytes[] memory signatures
    ) private view {
        bytes32 hash = encodedMessage.toEthSignedMessageHash();
        require(
            validatorManager.validateUniqueSignatures(hash, signatures),
            "Invalid signatures"
        );
        require(
            validatorManager.hasSupermajority(signatures.length),
            "No supermajority"
        );
    }

    function dispatch(
        uint sourceChainId,
        address caller,
        address target,
        bytes memory call,
        bytes4 callback,
        uint nonce,
        bytes[] memory signatures
    ) public {
        require(
            !dispatched[sourceChainId][caller][nonce],
            "Already dispatched"
        );

        bytes memory message = abi.encode(
            sourceChainId,
            caller,
            target,
            call,
            false,
            callback,
            nonce
        );
        validateRequest(message, signatures);

        require(caller.code.length > 0, "code length");
        (bool success, bytes memory response) = Bridged(caller).dispatched(
            sourceChainId,
            target,
            call
        );
        emit Dispatched(
            sourceChainId,
            caller,
            callback,
            success,
            response,
            nonce
        );
        dispatched[sourceChainId][caller][nonce] = true;
    }

    function query(
        address caller,
        address target,
        bytes memory call
    ) public view returns (bool success, bytes memory response) {
        require(caller.code.length > 0, "code length");
        (success, response) = Bridged(caller).queried(target, call);
    }

    event Resumed(
        uint indexed targetChainId,
        address indexed caller,
        bytes call,
        bool success,
        bytes response,
        uint indexed nonce
    );

    // Ensure signatures are submitted in the order of their addresses
    function resume(
        uint targetChainId,
        address caller,
        bytes4 callback,
        bool success,
        bytes memory response,
        uint nonce,
        bytes[] memory signatures
    ) public payable {
        require(!resumed[targetChainId][caller][nonce], "Already resumed");
        bytes memory message = abi.encode(
            targetChainId,
            caller,
            callback,
            success,
            response,
            nonce
        );
        validateRequest(message, signatures);

        bytes memory call = abi.encodeWithSelector(
            callback,
            targetChainId,
            success,
            response,
            nonce
        );
        (bool success2, bytes memory response2) = caller.call{
            value: msg.value,
            gas: 100000
        }(call);

        emit Resumed(targetChainId, caller, call, success2, response2, nonce);
        resumed[targetChainId][caller][nonce] = true;
    }
}
