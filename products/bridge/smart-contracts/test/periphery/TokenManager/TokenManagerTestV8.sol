// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {IRelayer, CallMetadata} from "contracts/core/Relayer.sol";
import {ITokenManager} from "contracts/periphery/TokenManagerUpgradeable.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";

/**
 * Change the order of inheritance between OwnableUpgradeable and UUPSUpgradeable
 */
abstract contract TokenManagerUpgradeableTestV8 is
    ITokenManager,
    Initializable,
    OwnableUpgradeable,
    UUPSUpgradeable
{
    /// @custom:storage-location erc7201:zilliqa.storage.TokenManager
    struct TokenManagerStorage {
        address gateway;
        // localTokenAddress => remoteChainId => RemoteToken
        mapping(address => mapping(uint => RemoteToken)) remoteTokens;
    }

    // keccak256(abi.encode(uint256(keccak256("zilliqa.storage.TokenManager")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant Token_Manager_Storage_Location =
        0x4a6c2e6a7e6518c249bdcd1d934ea16ea5325bbae105af814eb678f5f49f3400;

    function _getTokenManagerStorage()
        private
        pure
        returns (TokenManagerStorage storage $)
    {
        assembly {
            $.slot := Token_Manager_Storage_Location
        }
    }

    function getGateway() public view returns (address) {
        TokenManagerStorage storage $ = _getTokenManagerStorage();
        return $.gateway;
    }

    function _setGateway(address _gateway) internal {
        TokenManagerStorage storage $ = _getTokenManagerStorage();
        $.gateway = _gateway;
    }

    function setGateway(address _gateway) external onlyOwner {
        _setGateway(_gateway);
    }

    function getRemoteTokens(
        address token,
        uint remoteChainId
    ) public view returns (RemoteToken memory) {
        TokenManagerStorage storage $ = _getTokenManagerStorage();
        return $.remoteTokens[token][remoteChainId];
    }

    modifier onlyGateway() {
        if (msg.sender != address(getGateway())) {
            revert NotGateway();
        }
        _;
    }

    function __TokenManager_init(address _gateway) internal onlyInitializing {
        __Ownable_init(msg.sender);
        _setGateway(_gateway);
    }

    function _authorizeUpgrade(address) internal virtual override onlyOwner {}

    function _removeToken(address localToken, uint remoteChainId) internal {
        TokenManagerStorage storage $ = _getTokenManagerStorage();
        delete $.remoteTokens[localToken][remoteChainId];
        emit TokenRemoved(localToken, remoteChainId);
    }

    function _registerToken(
        address localToken,
        RemoteToken memory remoteToken
    ) internal {
        TokenManagerStorage storage $ = _getTokenManagerStorage();
        $.remoteTokens[localToken][remoteToken.chainId] = remoteToken;
        emit TokenRegistered(
            localToken,
            remoteToken.token,
            remoteToken.tokenManager,
            remoteToken.chainId
        );
    }

    // Token Overrides
    function registerToken(
        address token,
        RemoteToken memory remoteToken
    ) external virtual onlyOwner {
        _registerToken(token, remoteToken);
    }

    // TO OVERRIDE – Incoming
    function _handleTransfer(
        address token,
        address from,
        uint amount
    ) internal virtual;

    // TO OVERRIDE – Outgoing
    function _handleAccept(
        address token,
        address recipient,
        uint amount
    ) internal virtual;

    function transfer(
        address token,
        uint remoteChainId,
        address remoteRecipient,
        uint amount
    ) external virtual {}

    // Incoming
    function accept(
        CallMetadata calldata metadata,
        bytes calldata _args
    ) external virtual onlyGateway {}
}

contract TokenManagerTestV8 is TokenManagerUpgradeableTestV8 {
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function reinitialize() public reinitializer(2) {
        __TokenManager_init(getGateway());
    }

    function _handleTransfer(
        address token,
        address from,
        uint amount
    ) internal override {}

    function _handleAccept(
        address token,
        address recipient,
        uint amount
    ) internal override {}
}
