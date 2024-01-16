// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity 0.8.20;

import {TokenManagerUpgradeable} from "contracts/periphery/TokenManagerUpgradeable.sol";

contract TokenManagerV1Test is TokenManagerUpgradeable {
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
