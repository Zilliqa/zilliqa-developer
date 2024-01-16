// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {TokenManagerUpgradeable, ITokenManager} from "contracts/periphery/TokenManagerUpgradeable.sol";
import {BridgedToken} from "contracts/periphery/BridgedToken.sol";

interface IMintAndBurnTokenManager is ITokenManager {
    event Minted(address indexed token, address indexed recipient, uint amount);
    event Burned(address indexed token, address indexed from, uint amount);
    event BridgedTokenDeployed(
        address token,
        address remoteToken,
        address remoteTokenManager,
        uint remoteChainId
    );
}

contract MintAndBurnTokenManagerUpgradeable is
    IMintAndBurnTokenManager,
    TokenManagerUpgradeable
{
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function initialize(address _gateway) external initializer {
        __TokenManager_init(_gateway);
    }

    function deployToken(
        string calldata name,
        string calldata symbol,
        uint8 decimals,
        address remoteToken,
        address tokenManager,
        uint remoteChainId
    ) external returns (BridgedToken) {
        return
            _deployToken(
                name,
                symbol,
                decimals,
                remoteToken,
                tokenManager,
                remoteChainId
            );
    }

    function deployToken(
        string calldata name,
        string calldata symbol,
        address remoteToken,
        address tokenManager,
        uint remoteChainId
    ) external returns (BridgedToken) {
        return
            _deployToken(
                name,
                symbol,
                18,
                remoteToken,
                tokenManager,
                remoteChainId
            );
    }

    function _deployToken(
        string calldata name,
        string calldata symbol,
        uint8 decimals,
        address remoteToken,
        address tokenManager,
        uint remoteChainId
    ) internal onlyOwner returns (BridgedToken) {
        // TODO: deployed counterfactually
        BridgedToken bridgedToken = new BridgedToken(name, symbol, decimals);

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
        address from,
        uint amount
    ) internal override {
        BridgedToken(token).burnFrom(from, amount);
        emit Burned(token, from, amount);
    }

    // Incoming
    function _handleAccept(
        address token,
        address recipient,
        uint amount
    ) internal override {
        BridgedToken(token).mint(recipient, amount);
        emit Minted(token, recipient, amount);
    }
}
