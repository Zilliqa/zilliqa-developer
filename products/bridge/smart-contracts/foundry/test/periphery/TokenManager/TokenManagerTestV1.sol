// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity 0.8.20;

import {TokenManagerUpgradeable} from "contracts/periphery/TokenManagerUpgradeable.sol";

/**
 * Implementation of the original abstract contract, used as the baseline for the tests
 */
contract TokenManagerTestV1 is TokenManagerUpgradeable {
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function initialize(address _gateway) external initializer {
        __TokenManager_init(_gateway);
    }

    function _handleTransfer(
        address token,
        address from,
        uint amount
    ) internal override {}

    function _handleAccept(
        address token,
        address recipient,
        uint amount
    ) internal override {}
}
