// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

interface IRegistryErrors {
    error NotRegistered(address targetAddress);
}

interface IRegistryEvents {
    event ContractRegistered(address target);
    event ContractUnregistered(address target);
}

interface IRegistry is IRegistryErrors, IRegistryEvents {
    function registered(address target) external view returns (bool);

    function register(address newTarget) external;

    function unregister(address removeTarget) external;
}

abstract contract RegistryUpgradeable is IRegistry {
    /// @custom:storage-location erc7201:zilliqa.storage.Registry
    struct RegistryStorage {
        mapping(address => bool) registered;
    }

    // keccak256(abi.encode(uint256(keccak256("zilliqa.storage.Registry")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant REGISTRY_STORAGE_POSITION =
        0x4432bdf0e567007e5ad3c8ad839a7f885ef69723eaa659dd9f06e98a97274300;

    function _getRegistryStorage()
        private
        pure
        returns (RegistryStorage storage $)
    {
        assembly {
            $.slot := REGISTRY_STORAGE_POSITION
        }
    }

    modifier isRegistered(address target) {
        RegistryStorage storage $ = _getRegistryStorage();
        if (registered(target)) {
            revert NotRegistered(target);
        }
        _;
    }

    function registered(address target) public view returns (bool) {
        RegistryStorage storage $ = _getRegistryStorage();
        return $.registered[target];
    }

    function _register(address newTarget) internal {
        RegistryStorage storage $ = _getRegistryStorage();
        $.registered[newTarget] = true;
        emit ContractRegistered(newTarget);
    }

    function _unregister(address removeTarget) internal {
        RegistryStorage storage $ = _getRegistryStorage();
        $.registered[removeTarget] = false;
        emit ContractUnregistered(removeTarget);
    }
}
