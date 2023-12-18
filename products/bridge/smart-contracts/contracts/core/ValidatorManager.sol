// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {EnumerableSet} from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {SignatureValidator} from "contracts/core/SignatureValidator.sol";

contract ValidatorManager is Ownable {
    using EnumerableSet for EnumerableSet.AddressSet;
    using SignatureValidator for EnumerableSet.AddressSet;

    EnumerableSet.AddressSet private _validators;

    constructor(address[] memory validators) Ownable(msg.sender) {
        uint validatorsLength = validators.length;
        for (uint i = 0; i < validatorsLength; ++i) {
            addValidator(validators[i]);
        }
    }

    // TODO: add restriction
    // Ownership should then be trasnferred to the relayer
    function addValidator(address user) public onlyOwner returns (bool) {
        return _validators.add(user);
    }

    // TODO: add restriction
    // Ownership should then be trasnferred to the relayer
    function removeValidator(address user) external onlyOwner returns (bool) {
        return _validators.remove(user);
    }

    // Expensive function, avoid calling on-chain
    function getValidators() external view returns (address[] memory) {
        return _validators.values();
    }

    function isValidator(address user) external view returns (bool) {
        return _validators.contains(user);
    }

    function validatorsSize() external view returns (uint) {
        return _validators.length();
    }

    function validateMessageWithSupermajority(
        bytes32 ethSignedMessageHash,
        bytes[] calldata signatures
    ) external view {
        _validators.validateSignaturesWithSupermajority(
            ethSignedMessageHash,
            signatures
        );
    }
}
