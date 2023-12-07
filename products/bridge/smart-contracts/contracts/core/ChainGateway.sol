// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import "contracts/core/Dispatcher.sol";
import "contracts/core/Relayer.sol";

contract ChainGateway is Relayer, Dispatcher {
    constructor(
        ValidatorManager _validatorManager
    ) Dispatcher(_validatorManager) {}
}
