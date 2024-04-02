// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {TokenManagerUpgradeable} from "contracts/periphery/TokenManagerUpgradeable.sol";

/**
 * Same as the original contract, with just a new reinitializer on the non-abstract contract
 */
contract TokenManagerTestV6 is TokenManagerUpgradeable {
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function reinitialize() public reinitializer(3) {
        __TokenManager_init(getGateway());
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
