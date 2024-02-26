// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {TokenManagerUpgradeableV3, ITokenManager} from "contracts/periphery/TokenManagerV3/TokenManagerUpgradeableV3.sol";
import {BridgedToken} from "contracts/periphery/BridgedToken.sol";

interface IMintAndBurnTokenManager {
    event Minted(address indexed token, address indexed recipient, uint amount);
    event Burned(address indexed token, address indexed from, uint amount);
    event BridgedTokenDeployed(
        address token,
        address remoteToken,
        address remoteTokenManager,
        uint remoteChainId
    );
}

contract MintAndBurnTokenManagerUpgradeableV3 is
    IMintAndBurnTokenManager,
    TokenManagerUpgradeableV3
{
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
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
        RemoteToken memory remoteTokenStruct = RemoteToken(
            remoteToken,
            tokenManager,
            remoteChainId
        );

        _registerToken(address(bridgedToken), remoteTokenStruct);

        emit BridgedTokenDeployed(
            address(bridgedToken),
            remoteToken,
            tokenManager,
            remoteChainId
        );

        return bridgedToken;
    }

    function transferTokenOwnership(
        address localToken,
        uint remoteChainId,
        address newOwner
    ) external onlyOwner {
        BridgedToken(localToken).transferOwnership(newOwner);
        _removeToken(localToken, remoteChainId);
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
