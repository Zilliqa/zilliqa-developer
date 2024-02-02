// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {TokenManagerUpgradeableV2, ITokenManager} from "contracts/periphery/TokenManagerV2/TokenManagerUpgradeableV2.sol";
import {ILockAndReleaseTokenManager, IERC20} from "contracts/periphery/LockAndReleaseTokenManagerUpgradeable.sol";

contract LockAndReleaseTokenManagerUpgradeableV2 is
    ILockAndReleaseTokenManager,
    TokenManagerUpgradeableV2
{
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function reinitialize(uint fees) external reinitializer(2) {
        _setFees(fees);
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
