// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {OwnableUpgradeable, Ownable2StepUpgradeable} from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

import {RegistryUpgradeable, IRegistry} from "contracts/core-upgradeable/RegistryUpgradeable.sol";

interface IRelayerEvents {
    event Relayed(
        uint indexed targetChainId,
        address target,
        bytes call,
        uint gasLimit,
        uint nonce
    );
}

interface IRelayer is IRelayerEvents, IRegistry {
    struct CallMetadata {
        uint sourceChainId;
        address sender;
    }

    function nonce(uint chainId) external view returns (uint);

    function relayWithMetadata(
        uint targetChainId,
        address target,
        bytes4 callSelector,
        bytes calldata callData,
        uint gasLimit
    ) external returns (uint);

    function relay(
        uint targetChainId,
        address target,
        bytes calldata call,
        uint gasLimit
    ) external returns (uint);
}

abstract contract RelayerUpgradeable is
    IRelayer,
    Initializable,
    Ownable2StepUpgradeable,
    RegistryUpgradeable
{
    /// @custom:storage-location erc7201:zilliqa.storage.Relayer
    struct RelayerStorage {
        // TargetChainId => Nonce
        mapping(uint => uint) nonce;
    }

    // keccak256(abi.encode(uint256(keccak256("zilliqa.storage.Relayer")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant RELAYER_STORAGE_POSITION =
        0x814fccf6b0465c7c83d1a86cf4c4cdd0d8463969cbd4702358f5ae439f30a000;

    function _getRelayerStorage()
        private
        pure
        returns (RelayerStorage storage $)
    {
        assembly {
            $.slot := RELAYER_STORAGE_POSITION
        }
    }

    function __Relayer_init(address _owner) internal onlyInitializing {
        __Ownable_init(_owner);
        __Relayer_init_unchained();
    }

    function __Relayer_init_unchained() internal onlyInitializing {}

    function nonce(uint chainId) external view returns (uint) {
        RelayerStorage storage $ = _getRelayerStorage();
        return $.nonce[chainId];
    }

    function _relay(
        uint targetChainId,
        address target,
        bytes memory call,
        uint gasLimit
    ) internal isRegistered(_msgSender()) returns (uint) {
        RelayerStorage storage $ = _getRelayerStorage();
        uint _nonce = ++$.nonce[targetChainId];

        emit Relayed(targetChainId, target, call, gasLimit, _nonce);
        return _nonce;
    }

    function relay(
        uint targetChainId,
        address target,
        bytes calldata call,
        uint gasLimit
    ) external returns (uint) {
        return _relay(targetChainId, target, call, gasLimit);
    }

    // Use this function to relay a call with metadata. This is useful for calling surrogate contracts.
    // Ensure the surrogate implements this interface
    function relayWithMetadata(
        uint targetChainId,
        address target,
        bytes4 callSelector,
        bytes calldata callData,
        uint gasLimit
    ) external returns (uint) {
        return
            _relay(
                targetChainId,
                target,
                abi.encodeWithSelector(
                    callSelector,
                    CallMetadata(block.chainid, _msgSender()),
                    callData
                ),
                gasLimit
            );
    }

    function register(address newTarget) external override onlyOwner {
        _register(newTarget);
    }

    function unregister(address removeTarget) external override onlyOwner {
        _unregister(removeTarget);
    }
}
