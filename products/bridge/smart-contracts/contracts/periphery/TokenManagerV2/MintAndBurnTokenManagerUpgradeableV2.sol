// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {TokenManagerUpgradeableV2, ITokenManager} from "contracts/periphery/TokenManagerV2/TokenManagerUpgradeableV2.sol";
import {BridgedToken} from "contracts/periphery/BridgedToken.sol";
import {IMintAndBurnTokenManager} from "contracts/periphery/MintAndBurnTokenManagerUpgradeable.sol";

contract MintAndBurnTokenManagerUpgradeableV2 is
    IMintAndBurnTokenManager,
    TokenManagerUpgradeableV2
{
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function reinitialize(uint fees) external reinitializer(2) {
        _setFees(fees);
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
