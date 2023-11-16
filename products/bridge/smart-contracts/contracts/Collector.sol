// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import "./ValidatorManager.sol";

contract Collector {
    ValidatorManager private validatorManager;
    event Echoed(bytes32 indexed hash, bytes signature);

    constructor(ValidatorManager _validatorManager) {
        validatorManager = _validatorManager;
    }

    function echo(bytes32 hash, bytes memory signature) public {
        require(
            validatorManager.validateSignature(hash, signature),
            "Wrong validator"
        );
        emit Echoed(hash, signature);
    }
}
