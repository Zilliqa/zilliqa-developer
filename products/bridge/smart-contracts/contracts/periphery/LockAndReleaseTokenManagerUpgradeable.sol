// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {TokenManagerUpgradeable, ITokenManager} from "contracts/periphery/TokenManagerUpgradeable.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

interface ILockAndReleaseTokenManager is ITokenManager {
    event Locked(address indexed token, address indexed from, uint amount);
    event Released(
        address indexed token,
        address indexed recipient,
        uint amount
    );
}

contract LockAndReleaseTokenManagerUpgradeable is
    ILockAndReleaseTokenManager,
    TokenManagerUpgradeable
{
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function initialize(address _gateway) external initializer {
        __TokenManager_init(_gateway);
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
