// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity 0.8.20;

import {ValidatorManager} from "contracts/core/ValidatorManager.sol";
import {Tester, Vm} from "foundry/test/Tester.sol";

contract TransferReentrancyTester {
    address target;
    bytes data;
    bool public alreadyEntered = false;

    function reentrancyAttack(
        address _target,
        bytes calldata _data
    ) external returns (bool) {
        target = _target;
        data = _data;

        (bool success, ) = target.call(data);
        return success;
    }

    receive() external payable {
        if (address(target).balance > 0) {
            (bool success, ) = target.call(data);
            success;
        }
    }
}

interface IReentrancy {
    error ReentrancyVulnerability();
    error ReentrancySafe();
}

contract Target is IReentrancy {
    uint public c = 0;

    function depositFee(uint amount) external payable {
        amount;
    }

    function work(uint num_) external pure returns (uint) {
        require(num_ < 1000, "Too large");
        return num_ + 1;
    }

    function infiniteLoop() public {
        while (true) {
            c = c + 1;
        }
    }

    function finish(bool success, bytes calldata res, uint nonce) external {}

    function finishRevert(
        bool success,
        bytes calldata res,
        uint nonce
    ) external pure {
        success;
        res;
        nonce;
        revert();
    }

    bool public alreadyEntered = false;
    bytes public reentrancyCalldata;
    address public reentrancyTarget;

    function setReentrancyConfig(address target, bytes calldata data) external {
        reentrancyTarget = target;
        reentrancyCalldata = data;
    }

    function reentrancy() external {
        if (alreadyEntered) {
            revert IReentrancy.ReentrancyVulnerability();
        }
        alreadyEntered = true;
        (bool success, ) = reentrancyTarget.call(reentrancyCalldata);
        if (success) {
            revert IReentrancy.ReentrancyVulnerability();
        }
        revert IReentrancy.ReentrancySafe();
    }
}

abstract contract ValidatorManagerFixture is Tester {
    uint constant VALIDATOR_COUNT = 10;

    ValidatorManager validatorManager;
    Vm.Wallet[] public validators = new Vm.Wallet[](VALIDATOR_COUNT);

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
        ) = generateValidatorManager(VALIDATOR_COUNT);
        validators = _validators;
        validatorManager = _validatorManager;
    }
}
