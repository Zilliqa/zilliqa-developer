// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import "./ValidatorManager.sol";

contract Collector {
    ValidatorManager private _validatorManager;

    event Echoed(bytes32 indexed hash, bytes signature);

    error InvalidSignature();

    constructor(ValidatorManager validatorManager_) {
        _validatorManager = validatorManager_;
    }

    function echo(bytes32 hash, bytes memory signature) public {
        if (!_validatorManager.validateSignature(hash, signature)) {
            revert InvalidSignature();
        }
        emit Echoed(hash, signature);
    }

    function validatorManager() public view returns (ValidatorManager) {
        return _validatorManager;
    }
}
