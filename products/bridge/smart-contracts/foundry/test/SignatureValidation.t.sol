// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity 0.8.20;

import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";
import {ValidatorManager} from "contracts/core/ValidatorManager.sol";
import {ISignatureValidatorErrors} from "contracts/core/SignatureValidator.sol";
import {IRelayerErrors} from "contracts/core/Relayer.sol";
import {Tester, Vm} from "foundry/test/Tester.sol";

abstract contract ValidatorManagerFixture is Tester {
    using MessageHashUtils for bytes;

    ValidatorManager validatorManager;
    uint constant validatorCount = 10;
    Vm.Wallet[] validators = new Vm.Wallet[](validatorCount);

    function generateValidatorManager(
        uint size
    ) internal returns (Vm.Wallet[] memory, ValidatorManager) {
        Vm.Wallet[] memory _validators = new Vm.Wallet[](size);
        address[] memory validatorAddresses = new address[](size);
        for (uint i = 0; i < size; ++i) {
            _validators[i] = vm.createWallet(i + 1);
            validatorAddresses[i] = _validators[i].addr;
        }
        ValidatorManager _validatorManager = new ValidatorManager(
            validatorAddresses
        );

        return (_validators, _validatorManager);
    }

    constructor() {
        // Setup validator manager
        (
            Vm.Wallet[] memory _validators,
            ValidatorManager _validatorManager
        ) = generateValidatorManager(validatorCount);
        validators = _validators;
        validatorManager = _validatorManager;
    }
}

contract SignatureValidation is ValidatorManagerFixture {
    using MessageHashUtils for bytes;

    function exactSupermajority(
        uint size
    ) private pure returns (uint supermajority) {
        supermajority = (size * 2) / 3 + 1;
    }

    function getValidatorSubset(
        Vm.Wallet[] memory _validators,
        uint size
    ) private pure returns (Vm.Wallet[] memory subset) {
        subset = new Vm.Wallet[](size);
        for (uint i = 0; i < size; ++i) {
            subset[i] = _validators[i];
        }
    }

    function test_allValidatorsSign() external {
        bytes32 messageHash = bytes("Hello world").toEthSignedMessageHash();
        bytes[] memory signatures = multiSign(sort(validators), messageHash);
        // If it works does not do anything
        validatorManager.validateMessageWithSupermajority(
            messageHash,
            signatures
        );
    }

    function test_exactMajoritySign() external {
        bytes32 messageHash = bytes("Hello world").toEthSignedMessageHash();
        uint exactSupermajoritySize = exactSupermajority(validators.length);
        Vm.Wallet[] memory exactSupermajorityValidators = getValidatorSubset(
            validators,
            exactSupermajoritySize
        );
        bytes[] memory signatures = multiSign(
            sort(exactSupermajorityValidators),
            messageHash
        );
        // If it works does not do anything
        validatorManager.validateMessageWithSupermajority(
            messageHash,
            signatures
        );
    }

    function testRevert_lessThanSupermajoritySign() external {
        bytes32 messageHash = bytes("Hello world").toEthSignedMessageHash();
        uint exactSupermajoritySize = exactSupermajority(validators.length) - 1;
        Vm.Wallet[] memory exactSupermajorityValidators = getValidatorSubset(
            validators,
            exactSupermajoritySize
        );
        bytes[] memory signatures = multiSign(
            sort(exactSupermajorityValidators),
            messageHash
        );
        // If it works does not do anything
        vm.expectRevert(ISignatureValidatorErrors.NoSupermajority.selector);
        validatorManager.validateMessageWithSupermajority(
            messageHash,
            signatures
        );
    }

    function testRevert_noSignatures() external {
        bytes32 messageHash = bytes("Hello world").toEthSignedMessageHash();
        bytes[] memory signatures = new bytes[](0);
        vm.expectRevert(ISignatureValidatorErrors.NoSupermajority.selector);
        validatorManager.validateMessageWithSupermajority(
            messageHash,
            signatures
        );
    }

    function test_emptyMessage() external {
        bytes32 messageHash;
        bytes[] memory signatures = multiSign(sort(validators), messageHash);
        validatorManager.validateMessageWithSupermajority(
            messageHash,
            signatures
        );
    }

    function testRevert_invalidSignature() external {
        bytes32 messageHash = bytes("Hello world").toEthSignedMessageHash();
        bytes[] memory signatures = multiSign(sort(validators), messageHash);
        // Manipulate one of the bytes in the first signature
        signatures[0][0] = 0;
        vm.expectRevert(
            ISignatureValidatorErrors.InvalidValidatorOrSignatures.selector
        );
        validatorManager.validateMessageWithSupermajority(
            messageHash,
            signatures
        );
    }

    function testRevert_unorderedSignatures() external {
        bytes32 messageHash = bytes("Hello world").toEthSignedMessageHash();
        // Don't sort the validators by address
        bytes[] memory signatures = multiSign(validators, messageHash);
        vm.expectRevert(
            ISignatureValidatorErrors.NonUniqueOrUnorderedSignatures.selector
        );
        validatorManager.validateMessageWithSupermajority(
            messageHash,
            signatures
        );
    }

    function testRevert_repeatedSigners() external {
        bytes32 messageHash = bytes("Hello world").toEthSignedMessageHash();
        // Don't sort the validators by address
        bytes[] memory signatures = multiSign(sort(validators), messageHash);
        // Repeat first and second validator
        signatures[0] = signatures[1];
        vm.expectRevert(
            ISignatureValidatorErrors.NonUniqueOrUnorderedSignatures.selector
        );
        validatorManager.validateMessageWithSupermajority(
            messageHash,
            signatures
        );
    }

    function testFuzz_message(bytes memory message) external {
        bytes32 messageHash = message.toEthSignedMessageHash();
        bytes[] memory signatures = multiSign(sort(validators), messageHash);

        // Should work regardless of validators
        validatorManager.validateMessageWithSupermajority(
            messageHash,
            signatures
        );
    }

    function test_largeValidatorSet() external {
        uint validatorSize = 25_000;

        (
            Vm.Wallet[] memory _validators,
            ValidatorManager _validatorManager
        ) = generateValidatorManager(validatorSize);

        bytes32 messageHash = bytes("Hello World").toEthSignedMessageHash();
        bytes[] memory signatures = multiSign(sort(_validators), messageHash);

        // Should work regardless of validators
        _validatorManager.validateMessageWithSupermajority(
            messageHash,
            signatures
        );
    }

    /// forge-config: default.fuzz.runs = 100
    function testFuzz_signatureCount(uint input) external {
        uint size = 200;
        uint exactSupermajoritySize = exactSupermajority(size);
        uint signaturesCount = exactSupermajoritySize +
            (input % (size - exactSupermajoritySize));

        (
            Vm.Wallet[] memory _validators,
            ValidatorManager _validatorManager
        ) = generateValidatorManager(size);
        Vm.Wallet[] memory validatorSubset = getValidatorSubset(
            _validators,
            signaturesCount
        );
        bytes32 messageHash = bytes("Hello World").toEthSignedMessageHash();

        bytes[] memory signatures = multiSign(
            sort(validatorSubset),
            messageHash
        );

        // Should work regardless of validators
        _validatorManager.validateMessageWithSupermajority(
            messageHash,
            signatures
        );
    }
}
