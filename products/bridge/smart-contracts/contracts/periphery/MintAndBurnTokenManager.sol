// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {Relayer, CallMetadata} from "contracts/core/Relayer.sol";

import {TokenManager, RemoteToken} from "contracts/periphery/TokenManager.sol";
import {IBridgedToken, BridgedToken} from "contracts/periphery/BridgedToken.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

contract MintAndBurnTokenManager is TokenManager {
    event Minted(address token, address recipient, uint amount);
    event Burned(address token, address recipient, uint amount);
    event BridgedTokenDeployed(
        address indexed token,
        address remoteToken,
        address remoteTokenManager,
        uint remoteChainId
    );

    // TODO: deployed counterfactually
    constructor(address _gateway) TokenManager(_gateway) {}

    function deployToken(
        string calldata name,
        string calldata symbol,
        address remoteToken,
        address tokenManager,
        uint remoteChainId
    ) external onlyOwner returns (BridgedToken) {
        // TODO: deployed counterfactually
        BridgedToken bridgedToken = new BridgedToken(name, symbol);

        _registerToken(
            address(bridgedToken),
            remoteToken,
            tokenManager,
            remoteChainId
        );

        emit BridgedTokenDeployed(
            address(bridgedToken),
            remoteToken,
            tokenManager,
            remoteChainId
        );

        return bridgedToken;
    }

    // Outgoing
    function _handleTransfer(
        address token,
        address recipient,
        uint amount
    ) internal override {
        IBridgedToken(token).burnFrom(recipient, amount);
        emit Burned(token, recipient, amount);
    }

    // Incoming
    function _handleAccept(
        address token,
        address recipient,
        uint amount
    ) internal override {
        IBridgedToken(token).mint(recipient, amount);
        emit Minted(token, recipient, amount);
    }
}
