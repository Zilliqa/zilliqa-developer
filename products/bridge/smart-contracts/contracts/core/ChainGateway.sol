// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {ChainDispatcher} from "contracts/core/ChainDispatcher.sol";
import {Relayer} from "contracts/core/Relayer.sol";

contract ChainGateway is Relayer, ChainDispatcher {
    constructor(address _validatorManager) ChainDispatcher(_validatorManager) {}
}
