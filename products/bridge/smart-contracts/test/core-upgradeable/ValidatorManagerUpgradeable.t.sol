// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity 0.8.20;

import {Vm, Tester} from "test/Tester.sol";
import {ValidatorManagerUpgradeable} from "contracts/core-upgradeable/ValidatorManagerUpgradeable.sol";

import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";

contract ValidatorManagerUpgradeableTests is Tester {
    using ECDSA for bytes32;
    using MessageHashUtils for bytes;

    Vm.Wallet ownerWallet = vm.createWallet("Owner");
    address owner = ownerWallet.addr;
    address validator1 = vm.createWallet("Validator1").addr;
    address validator2 = vm.createWallet("Validator2").addr;
    ValidatorManagerUpgradeable validatorManager;

    function setUp() external {
        address[] memory validators = new address[](1);
        validators[0] = validator1;

        address implementation = address(new ValidatorManagerUpgradeable());
        address proxy = address(
            new ERC1967Proxy(
                implementation,
                abi.encodeCall(
                    ValidatorManagerUpgradeable.initialize,
                    (owner, validators)
                )
            )
        );

        validatorManager = ValidatorManagerUpgradeable(proxy);
    }

    function test_addValidator() external {
        vm.startPrank(owner);
        validatorManager.addValidator(validator2);

        assertEq(validatorManager.validatorsSize(), 2);
        assertEq(validatorManager.isValidator(validator1), true);
        assertEq(validatorManager.isValidator(validator2), true);
        vm.stopPrank();
    }

    function test_removeValidator() external {
        vm.startPrank(owner);
        validatorManager.removeValidator(validator1);

        assertEq(validatorManager.validatorsSize(), 0);
        assertEq(validatorManager.isValidator(validator1), false);
        vm.stopPrank();
    }

    function test_revertAddValidatorIfNotOwner() external {
        address nonOwner = vm.createWallet("NonOwner").addr;

        vm.prank(nonOwner);
        vm.expectRevert(
            abi.encodeWithSelector(
                OwnableUpgradeable.OwnableUnauthorizedAccount.selector,
                nonOwner
            )
        );
        validatorManager.addValidator(validator2);
    }

    function test_revertRemoveValidatorIfNotOwner() external {
        address nonOwner = vm.createWallet("NonOwner").addr;

        vm.prank(nonOwner);
        vm.expectRevert(
            abi.encodeWithSelector(
                OwnableUpgradeable.OwnableUnauthorizedAccount.selector,
                nonOwner
            )
        );
        validatorManager.removeValidator(validator2);
    }

    function test_validateMessageWithSupermajority() external {}

    function test_transferOwnership() external {
        address newOwner = vm.createWallet("NewOwner").addr;

        vm.prank(owner);
        validatorManager.transferOwnership(newOwner);
        // Ownership should only be transferred after newOwner accepts
        assertEq(validatorManager.owner(), owner);

        vm.prank(newOwner);
        validatorManager.acceptOwnership();
        assertEq(validatorManager.owner(), newOwner);
    }
}
