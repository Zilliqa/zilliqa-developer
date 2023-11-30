// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity 0.8.20;

import {RelayerTestFixture, Vm, ValidatorManager, MessageHashUtils} from "./Helpers.sol";
import {IValidatorManager} from "contracts/ValidatorManager.sol";

contract SignatureValidation is RelayerTestFixture {
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
        bytes memory message = "Hello world";
        bytes[] memory signatures = multiSign(
            sort(validators),
            message.toEthSignedMessageHash()
        );
        // If it works does not do anything
        relayer.exposed_validateRequest(message, signatures);
    }

    function test_exactMajoritySign() external {
        bytes memory message = "Hello world";
        uint exactSupermajoritySize = exactSupermajority(validators.length);
        Vm.Wallet[] memory exactSupermajorityValidators = getValidatorSubset(
            validators,
            exactSupermajoritySize
        );
        bytes[] memory signatures = multiSign(
            sort(exactSupermajorityValidators),
            message.toEthSignedMessageHash()
        );
        // If it works does not do anything
        relayer.exposed_validateRequest(message, signatures);
    }

    function testRevert_lessThanSupermajoritySign() external {
        bytes memory message = "Hello world";
        uint exactSupermajoritySize = exactSupermajority(validators.length) - 1;
        Vm.Wallet[] memory exactSupermajorityValidators = getValidatorSubset(
            validators,
            exactSupermajoritySize
        );
        bytes[] memory signatures = multiSign(
            sort(exactSupermajorityValidators),
            message.toEthSignedMessageHash()
        );
        // If it works does not do anything
        vm.expectRevert(NoSupermajority.selector);
        relayer.exposed_validateRequest(message, signatures);
    }

    function testRevert_noSignatures() external {
        bytes memory message = "Hello world";
        bytes[] memory signatures = new bytes[](0);
        vm.expectRevert(NoSupermajority.selector);
        relayer.exposed_validateRequest(message, signatures);
    }

    function test_emptyMessage() external {
        bytes memory message;
        bytes[] memory signatures = multiSign(
            sort(validators),
            message.toEthSignedMessageHash()
        );
        relayer.exposed_validateRequest(message, signatures);
    }

    function testRevert_invalidSignature() external {
        bytes memory message = "Hello world";
        bytes[] memory signatures = multiSign(
            sort(validators),
            message.toEthSignedMessageHash()
        );
        // Manipulate one of the bytes in the first signature
        signatures[0][0] = 0;
        vm.expectRevert(InvalidSignatures.selector);
        relayer.exposed_validateRequest(message, signatures);
    }

    function testRevert_unorderedSignatures() external {
        bytes memory message = "Hello world";
        // Don't sort the validators by address
        bytes[] memory signatures = multiSign(
            validators,
            message.toEthSignedMessageHash()
        );
        vm.expectRevert(
            IValidatorManager.NonUniqueOrUnorderedSignatures.selector
        );
        relayer.exposed_validateRequest(message, signatures);
    }

    function testRevert_repeatedSigners() external {
        bytes memory message = "Hello world";
        // Don't sort the validators by address
        bytes[] memory signatures = multiSign(
            sort(validators),
            message.toEthSignedMessageHash()
        );
        // Repeat first and second validator
        signatures[0] = signatures[1];
        vm.expectRevert(
            IValidatorManager.NonUniqueOrUnorderedSignatures.selector
        );
        relayer.exposed_validateRequest(message, signatures);
    }

    function testFuzz_message(bytes memory message) external {
        bytes[] memory signatures = multiSign(
            sort(validators),
            message.toEthSignedMessageHash()
        );

        // Should work regardless of validators
        relayer.exposed_validateRequest(message, signatures);
    }

    function test_largeValidatorSet() external {
        uint validatorSize = 25_000;

        (
            Vm.Wallet[] memory _validators,
            ValidatorManager _validatorManager
        ) = generateValidatorManager(validatorSize);
        relayer.workaround_updateValidatorManager(_validatorManager);

        bytes memory message = "Hello World";
        bytes[] memory signatures = multiSign(
            sort(_validators),
            message.toEthSignedMessageHash()
        );

        // Should work regardless of validators
        relayer.exposed_validateRequest(message, signatures);
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
        relayer.workaround_updateValidatorManager(_validatorManager);
        Vm.Wallet[] memory validatorSubset = getValidatorSubset(
            _validators,
            signaturesCount
        );

        bytes memory message = "Hello World";

        bytes[] memory signatures = multiSign(
            sort(validatorSubset),
            message.toEthSignedMessageHash()
        );

        // Should work regardless of validators
        relayer.exposed_validateRequest(message, signatures);
    }
}
