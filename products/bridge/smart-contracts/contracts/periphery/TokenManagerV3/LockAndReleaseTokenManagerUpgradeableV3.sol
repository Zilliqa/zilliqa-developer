// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {TokenManagerUpgradeableV3, ITokenManager} from "contracts/periphery/TokenManagerV3/TokenManagerUpgradeableV3.sol";
import {IERC20} from "contracts/periphery/LockAndReleaseTokenManagerUpgradeable.sol";

interface ILockAndReleaseTokenManager {
    event Locked(address indexed token, address indexed from, uint amount);
    event Released(
        address indexed token,
        address indexed recipient,
        uint amount
    );
}

contract LockAndReleaseTokenManagerUpgradeableV3 is
    ILockAndReleaseTokenManager,
    TokenManagerUpgradeableV3
{
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    // Outgoing
    function _handleTransfer(
        address token,
        address from,
        uint amount
    ) internal override {
        IERC20(token).transferFrom(from, address(this), amount);
        emit Locked(token, from, amount);
    }

    // Incoming
    function _handleAccept(
        address token,
        address recipient,
        uint amount
    ) internal override {
        IERC20(token).transfer(recipient, amount);
        emit Released(token, recipient, amount);
    }
}
