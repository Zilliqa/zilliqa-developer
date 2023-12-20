// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {Relayer, CallMetadata} from "contracts/core/Relayer.sol";

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

abstract contract TokenManager {
    Relayer gateway;
    mapping(address => RemoteToken) public remoteTokens;

    error InvalidSourceChainId();
    error InvalidTokenManager();
    error NotGateway();

    modifier onlyGateway() {
        if (msg.sender != address(gateway)) {
            revert NotGateway();
        }
        _;
    }

    constructor(address _gateway) {
        gateway = Relayer(_gateway);
    }

    function _registerToken(
        address token,
        address remoteToken,
        address tokenManager,
        uint chainId
    ) internal {
        remoteTokens[token] = RemoteToken(remoteToken, tokenManager, chainId);
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
        address remoteRecipient,
        uint amount
    ) public virtual {
        RemoteToken memory remoteToken = remoteTokens[token];

        _handleTransfer(token, msg.sender, amount);

        gateway.relayWithMetadata(
            remoteToken.chainId,
            remoteToken.tokenManager,
            this.accept.selector,
            abi.encode(AcceptArgs(remoteToken.token, remoteRecipient, amount)),
            1_000_000
        );
    }

    function transfer(address token, uint amount) external {
        transfer(token, msg.sender, amount);
    }

    // Incoming
    function accept(
        CallMetadata calldata metadata,
        AcceptArgs calldata args
    ) external virtual onlyGateway {
        if (metadata.sourceChainId != remoteTokens[args.token].chainId) {
            revert InvalidSourceChainId();
        }
        if (metadata.sender == remoteTokens[args.token].tokenManager) {
            revert InvalidTokenManager();
        }
        _handleAccept(args.token, args.recipient, args.amount);
    }
}
