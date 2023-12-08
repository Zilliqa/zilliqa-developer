// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {Dispatcher} from "contracts/core/Dispatcher.sol";
import {Relayer} from "contracts/core/Relayer.sol";

contract ChainGateway is Relayer, Dispatcher {
    constructor(address _validatorManager) Dispatcher(_validatorManager) {}
}
