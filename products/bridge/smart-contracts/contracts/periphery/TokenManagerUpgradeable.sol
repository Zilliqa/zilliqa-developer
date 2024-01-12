// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {Relayer, CallMetadata} from "contracts/core/Relayer.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";

struct AcceptArgs {
    address token;
    address recipient;
    uint amount;
}

struct RemoteToken {
    address token;
    address tokenManager;
    uint chainId;
}

interface ITokenManager {
    error InvalidSourceChainId();
    error InvalidTokenManager();
    error NotGateway();

    event TokenRegistered(
        address indexed token,
        address remoteToken,
        address remoteTokenManager,
        uint remoteChainId
    );

    function getRemoteTokens(
        address token,
        uint remoteChainId
    ) external view returns (RemoteToken memory);

    function registerToken(
        address token,
        address remoteToken,
        address remoteTokenManager,
        uint remoteChainId
    ) external;

    function transfer(
        address token,
        uint remoteChainId,
        address remoteRecipient,
        uint amount
    ) external;

    function accept(
        CallMetadata calldata metadata,
        AcceptArgs calldata args
    ) external;

    function setGateway(address _gateway) external;
}

abstract contract TokenManagerUpgradeable is
    ITokenManager,
    Initializable,
    OwnableUpgradeable
{
    Relayer gateway;
    // localTokenAddress => remoteChainId => RemoteToken
    mapping(address => mapping(uint => RemoteToken)) internal remoteTokens;

    modifier onlyGateway() {
        if (msg.sender != address(gateway)) {
            revert NotGateway();
        }
        _;
    }

    function getRemoteTokens(
        address token,
        uint remoteChainId
    ) external view returns (RemoteToken memory) {
        return remoteTokens[token][remoteChainId];
    }

    function __TokenManager_init(address _gateway) internal onlyInitializing {
        __Ownable_init(msg.sender);
        _setGateway(_gateway);
    }

    function _registerToken(
        address localToken,
        address remoteToken,
        address remoteTokenManager,
        uint remoteChainId
    ) internal {
        remoteTokens[localToken][remoteChainId] = RemoteToken(
            remoteToken,
            remoteTokenManager,
            remoteChainId
        );
    }

    function registerToken(
        address token,
        address remoteToken,
        address remoteTokenManager,
        uint remoteChainId
    ) external virtual onlyOwner {
        _registerToken(token, remoteToken, remoteTokenManager, remoteChainId);
        emit TokenRegistered(
            token,
            remoteToken,
            remoteTokenManager,
            remoteChainId
        );
    }

    function _handleTransfer(
        address token,
        address recipient,
        uint amount
    ) internal virtual;

    function _handleAccept(
        address token,
        address recipient,
        uint amount
    ) internal virtual;

    // Outgoing
    function transfer(
        address token,
        uint remoteChainId,
        address remoteRecipient,
        uint amount
    ) external virtual {
        RemoteToken memory remoteToken = remoteTokens[token][remoteChainId];

        _handleTransfer(token, msg.sender, amount);

        gateway.relayWithMetadata(
            remoteToken.chainId,
            remoteToken.tokenManager,
            this.accept.selector,
            abi.encode(AcceptArgs(remoteToken.token, remoteRecipient, amount)),
            1_000_000
        );
    }

    // Incoming
    function accept(
        CallMetadata calldata metadata,
        AcceptArgs calldata args
    ) external virtual onlyGateway {
        if (
            metadata.sourceChainId !=
            remoteTokens[args.token][metadata.sourceChainId].chainId
        ) {
            revert InvalidSourceChainId();
        }
        if (
            metadata.sender !=
            remoteTokens[args.token][metadata.sourceChainId].tokenManager
        ) {
            revert InvalidTokenManager();
        }

        _handleAccept(args.token, args.recipient, args.amount);
    }

    function _setGateway(address _gateway) internal {
        gateway = Relayer(_gateway);
    }

    function setGateway(address _gateway) external onlyOwner {
        _setGateway(_gateway);
    }
}
