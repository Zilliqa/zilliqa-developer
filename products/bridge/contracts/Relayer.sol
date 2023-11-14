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
        bytes calldata bytecode
    ) public returns (address) {
        address bridgedContract = Create2.deploy(0, salt, bytecode);
        Bridged(bridgedContract).initialize(this);
        emit TwinDeployment(bridgedContract);
        return bridgedContract;
    }

    constructor(ValidatorManager _validatorManager) {
        validatorManager = _validatorManager;
    }

    mapping(address => uint) nonces;
    mapping(address => mapping(uint => bool)) dispatched;
    mapping(address => mapping(uint => bool)) resumed;

    event Relayed(
        address caller,
        address target,
        bytes call,
        bool readonly,
        bytes4 callback,
        uint nonce
    );

    function relay(
        address target,
        bytes memory call,
        bool readonly,
        bytes4 callback
    ) public returns (uint) {
        emit Relayed(
            msg.sender,
            target,
            call,
            readonly,
            callback,
            nonces[msg.sender]
        );
        nonces[msg.sender]++;
        return nonces[msg.sender];
    }

    event Dispatched(
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
        address caller,
        address target,
        bytes memory call,
        bytes4 callback,
        uint nonce,
        bytes[] memory signatures
    ) public {
        require(!dispatched[caller][nonce], "Already dispatched");

        bytes memory message = abi.encode(
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
            target,
            call
        );
        emit Dispatched(caller, callback, success, response, nonce);
        dispatched[caller][nonce] = true;
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
        address indexed caller,
        bytes call,
        bool success,
        bytes response,
        uint indexed nonce
    );

    // Ensure signatures are submitted in the order of their addresses
    function resume(
        address caller,
        bytes4 callback,
        bool success,
        bytes memory response,
        uint nonce,
        bytes[] memory signatures
    ) public payable {
        require(!resumed[caller][nonce], "Already resumed");
        bytes memory message = abi.encode(
            caller,
            callback,
            success,
            response,
            nonce
        );
        validateRequest(message, signatures);

        bytes memory call = abi.encodeWithSelector(
            callback,
            success,
            response,
            nonce
        );
        (bool success2, bytes memory response2) = caller.call{
            value: msg.value,
            gas: 100000
        }(call);

        emit Resumed(caller, call, success2, response2, nonce);
        resumed[caller][nonce] = true;
    }
}
