// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {TokenManagerUpgradeableV2, ITokenManager} from "contracts/periphery/upgrade-tests/TokenManagerUpgradeableV2.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

/**
 * @notice Test V2 contracts not real ones used in production
 */
interface ILockAndReleaseTokenManager is ITokenManager {
    event Locked(address indexed token, address indexed from, uint amount);
    event Released(
        address indexed token,
        address indexed recipient,
        uint amount
    );
}

contract LockAndReleaseTokenManagerUpgradeableV2 is
    ILockAndReleaseTokenManager,
    TokenManagerUpgradeableV2
{
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    // function initialize(address _gateway) external initializer {
    //     __TokenManager_init(_gateway);
    // }

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
