// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {Relayer, CallMetadata} from "contracts/core/Relayer.sol";
import {TokenManager} from "contracts/periphery/TokenManager.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

contract LockAndReleaseTokenManager is TokenManager, Ownable {
    event Locked(address token, address recipient, uint amount);
    event Released(address token, address recipient, uint amount);

    // TODO: deploy counterfactually
    constructor(address _gateway) TokenManager(_gateway) Ownable(msg.sender) {}

    function addToken(
        address token,
        address remoteToken,
        address tokenManager,
        uint chainId
    ) external onlyGateway {
        _registerToken(token, remoteToken, tokenManager, chainId);
    }

    // Outgoing
    function _handleTransfer(
        address token,
        address recipient,
        uint amount
    ) internal override {
        IERC20(token).transferFrom(msg.sender, address(this), amount);
        emit Locked(token, recipient, amount);
    }

    function _handleAccept(
        address token,
        address recipient,
        uint amount
    ) internal override {
        IERC20(token).transfer(recipient, amount);
        emit Released(token, recipient, amount);
    }
}
