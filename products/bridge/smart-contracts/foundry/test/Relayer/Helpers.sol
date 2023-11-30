// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity 0.8.20;

import "contracts/ValidatorManager.sol";
import "contracts/Relayer.sol";
import "foundry/test/Tester.sol";

using ECDSA for bytes32;

contract BridgeTarget {
    uint public c = 0;

    function work(uint num_) external pure returns (uint) {
        require(num_ < 1000, "Too large");
        return num_ + 1;
    }

    function infiniteLoop() public {
        while (true) {
            c = c + 1;
        }
    }
}

contract SimpleBridge is Bridged {
    function initialize(Relayer relayer) public initializer {
        __Bridged_init(relayer);
    }
}

contract RelayerHarness is Relayer {
    constructor(ValidatorManager validatorManager) Relayer(validatorManager) {}

    function exposed_validateRequest(
        bytes memory encodedMessage,
        bytes[] memory signatures
    ) public view {
        return validateRequest(encodedMessage, signatures);
    }

    function workaround_updateValidatorManager(
        ValidatorManager validatorManager_
    ) external {
        validatorManager = validatorManager_;
    }
}

abstract contract RelayerTestFixture is Tester, IRelayer {
    using MessageHashUtils for bytes;

    ValidatorManager validatorManager;
    RelayerHarness relayer;
    uint constant validatorCount = 10;
    Vm.Wallet[] validators = new Vm.Wallet[](validatorCount);
    SimpleBridge immutable bridge = new SimpleBridge();

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

        // Setup relayer
        relayer = new RelayerHarness(validatorManager);
        // Initialise bridge
        bridge.initialize(relayer);
    }
}
