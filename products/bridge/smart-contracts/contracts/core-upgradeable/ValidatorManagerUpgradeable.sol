// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {ISignatureValidatorErrors, SignatureValidator} from "contracts/core/SignatureValidator.sol";

import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import {OwnableUpgradeable, Ownable2StepUpgradeable} from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import {EnumerableSet} from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";

interface IValidatorManager is ISignatureValidatorErrors {
    function addValidator(address user) external returns (bool);

    function removeValidator(address user) external returns (bool);

    function getValidators() external view returns (address[] memory);

    function isValidator(address user) external view returns (bool);

    function validatorsSize() external view returns (uint);

    function validateMessageWithSupermajority(
        bytes32 ethSignedMessageHash,
        bytes[] calldata signatures
    ) external view;
}

contract ValidatorManagerUpgradeable is
    IValidatorManager,
    Initializable,
    UUPSUpgradeable,
    Ownable2StepUpgradeable
{
    using EnumerableSet for EnumerableSet.AddressSet;
    using SignatureValidator for EnumerableSet.AddressSet;

    /// @custom:storage-location erc7201:zilliqa.storage.ValidatorManager
    struct ValidatorManagerStorage {
        EnumerableSet.AddressSet validators;
    }

    // keccak256(abi.encode(uint256(keccak256("zilliqa.storage.ValidatorManager")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant VALIDATOR_MANAGER_STORAGE_POSITION =
        0x7accde04f7b3831ef9580fa40c18d71adaa2564f23664e60f2464dcc899c5400;

    function _getValidatorManagerStorage()
        private
        pure
        returns (ValidatorManagerStorage storage $)
    {
        assembly {
            $.slot := VALIDATOR_MANAGER_STORAGE_POSITION
        }
    }

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function initialize(
        address _owner,
        address[] calldata validators
    ) external initializer {
        __Ownable_init(_owner);

        uint validatorsLength = validators.length;
        for (uint i = 0; i < validatorsLength; ++i) {
            _addValidator(validators[i]);
        }
    }

    function _authorizeUpgrade(address) internal virtual override onlyOwner {}

    function _validators()
        internal
        view
        returns (EnumerableSet.AddressSet storage)
    {
        ValidatorManagerStorage storage $ = _getValidatorManagerStorage();
        return $.validators;
    }

    function _addValidator(address user) internal returns (bool) {
        return _validators().add(user);
    }

    // Ownership should then be transferred to the relayer
    function addValidator(address user) public onlyOwner returns (bool) {
        return _addValidator(user);
    }

    // Ownership should then be transferred to the relayer
    function removeValidator(address user) external onlyOwner returns (bool) {
        return _validators().remove(user);
    }

    // Expensive function, avoid calling on-chain
    function getValidators() external view returns (address[] memory) {
        return _validators().values();
    }

    function isValidator(address user) external view returns (bool) {
        return _validators().contains(user);
    }

    function validatorsSize() external view returns (uint) {
        return _validators().length();
    }

    function validateMessageWithSupermajority(
        bytes32 ethSignedMessageHash,
        bytes[] calldata signatures
    ) external view {
        _validators().validateSignaturesWithSupermajority(
            ethSignedMessageHash,
            signatures
        );
    }
}
