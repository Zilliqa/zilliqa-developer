// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import {Ownable2StepUpgradeable} from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";

import {IChainDispatcher, ChainDispatcherUpgradeable} from "contracts/core-upgradeable/ChainDispatcherUpgradeable.sol";
import {IRelayer, RelayerUpgradeable} from "contracts/core-upgradeable/RelayerUpgradeable.sol";

interface IChainGateway is IRelayer, IChainDispatcher {}

contract ChainGatewayUpgradeable is
    Initializable,
    UUPSUpgradeable,
    Ownable2StepUpgradeable,
    RelayerUpgradeable,
    ChainDispatcherUpgradeable
{
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function initialize(
        address _validatorManager,
        address _owner
    ) external initializer {
        __Ownable_init(_owner);
        __Relayer_init_unchained();
        __ChainDispatcher_init_unchained(_validatorManager);
    }

    function _authorizeUpgrade(address) internal virtual override onlyOwner {}
}
