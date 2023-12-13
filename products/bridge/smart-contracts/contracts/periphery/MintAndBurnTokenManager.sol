// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {Relayer, CallMetadata} from "contracts/core/Relayer.sol";

import {TokenManager, RemoteToken} from "contracts/periphery/TokenManager.sol";
import {IWrappedToken, WrappedToken} from "contracts/periphery/WrappedToken.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

contract MintAndBurnTokenManager is TokenManager, Ownable {
    event Minted(address token, address recipient, uint amount);
    event Burned(address token, address recipient, uint amount);

    // TODO: deployed counterfactually
    constructor(address _gateway) TokenManager(_gateway) Ownable(msg.sender) {}

    function deployToken(
        string calldata name,
        string calldata symbol,
        address remoteToken,
        address tokenManager,
        uint chainId
    ) external onlyOwner {
        // TODO: deployed counterfactually
        WrappedToken token = new WrappedToken(name, symbol);
        _registerToken(address(token), remoteToken, tokenManager, chainId);
    }

    // Outgoing
    function _handleTransfer(
        address token,
        address recipient,
        uint amount
    ) internal override {
        IWrappedToken(token).burnFrom(recipient, amount);
        emit Burned(token, recipient, amount);
    }

    function _handleAccept(
        address token,
        address recipient,
        uint amount
    ) internal override {
        IWrappedToken(token).mint(recipient, amount);
        emit Minted(token, recipient, amount);
    }
}
