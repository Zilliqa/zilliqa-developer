// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";

contract ValidatorManager {
    using ECDSA for bytes32;
    using MessageHashUtils for bytes;
    using EnumerableSet for EnumerableSet.AddressSet;

    EnumerableSet.AddressSet private _validators;

    constructor(address[] memory validators) {
        for (uint i = 0; i < validators.length; i++) {
            addValidator(validators[i]);
        }
    }

    // TODO: add restriction
    function addValidator(address user) public returns (bool) {
        return _validators.add(user);
    }

    // TODO: add restriction
    function removeValidator(address user) public returns (bool) {
        return _validators.remove(user);
    }

    // Expensive function, avoid calling on-chain
    function getValidators() public view returns (address[] memory) {
        return _validators.values();
    }

    function isValidator(address user) public view returns (bool) {
        return _validators.contains(user);
    }

    function validatorsCount() public view returns (uint) {
        return _validators.length();
    }

    function validateUniqueSignatures(
        bytes32 ethSignedMessageHash,
        bytes[] memory signatures
    ) public view returns (bool) {
        address lastSigner = address(0);

        for (uint i = 0; i < signatures.length; i++) {
            address signer = ethSignedMessageHash.recover(signatures[i]);
            require(
                signer > lastSigner,
                "Signatures must be unique and in increasing order"
            );
            if (!isValidator(signer)) {
                return false;
            }
            lastSigner = signer;
        }
        return true;
    }

    function hasSupermajority(uint count) public view returns (bool) {
        return count * 3 > validatorsCount() * 2;
    }

    function validateSignature(
        bytes32 ethSignedMessageHash,
        bytes memory signature
    ) public view returns (bool) {
        address signer = ethSignedMessageHash.recover(signature);
        return isValidator(signer);
    }
}
