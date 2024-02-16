// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

import {IValidatorManager} from "contracts/core/ValidatorManager.sol";
import {IDispatchReplayChecker, DispatchReplayCheckerUpgradeable} from "contracts/core-upgradeable/DispatchReplayCheckerUpgradeable.sol";

interface IChainDispatcherEvents {
    event Dispatched(
        uint indexed sourceChainId,
        address indexed target,
        bool success,
        bytes response,
        uint indexed nonce
    );
}

interface IChainDispatcherErrors {
    error NonContractCaller(address target);
}

interface IChainDispatcher is
    IChainDispatcherEvents,
    IChainDispatcherErrors,
    IDispatchReplayChecker
{
    function validatorManager() external view returns (address);

    function dispatch(
        uint sourceChainId,
        address target,
        bytes calldata call,
        uint gasLimit,
        uint nonce,
        bytes[] calldata signatures
    ) external;
}

// Cross-chain only
abstract contract ChainDispatcherUpgradeable is
    IChainDispatcher,
    Initializable,
    DispatchReplayCheckerUpgradeable
{
    using MessageHashUtils for bytes;

    /// @custom:storage-location erc7201:zilliqa.storage.ChainDispatcher
    struct ChainDispatcherStorage {
        IValidatorManager validatorManager;
    }

    // keccak256(abi.encode(uint256(keccak256("zilliqa.storage.ChainDispatcher")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant CHAIN_DISPATCHER_STORAGE_POSITION =
        0x8cff60b14f9f959be48079fe56fd2ddb283fd144e381f4bd805400fbf1d0d600;

    function _getChainDispatcherStorage()
        private
        pure
        returns (ChainDispatcherStorage storage $)
    {
        assembly {
            $.slot := CHAIN_DISPATCHER_STORAGE_POSITION
        }
    }

    function __ChainDispatcher_init(
        address _validatorManager
    ) internal onlyInitializing {
        ChainDispatcherStorage storage $ = _getChainDispatcherStorage();
        $.validatorManager = IValidatorManager(_validatorManager);
    }

    function validatorManager() external view returns (address) {
        ChainDispatcherStorage storage $ = _getChainDispatcherStorage();
        return address($.validatorManager);
    }

    function dispatch(
        uint sourceChainId,
        address target,
        bytes calldata call,
        uint gasLimit,
        uint nonce,
        bytes[] calldata signatures
    ) external replayDispatchGuard(sourceChainId, nonce) {
        ChainDispatcherStorage storage $ = _getChainDispatcherStorage();

        $.validatorManager.validateMessageWithSupermajority(
            abi
                .encode(
                    sourceChainId,
                    block.chainid,
                    target,
                    call,
                    gasLimit,
                    nonce
                )
                .toEthSignedMessageHash(),
            signatures
        );

        // If it is not a contract the call itself should not revert
        if (target.code.length == 0) {
            emit Dispatched(
                sourceChainId,
                target,
                false,
                abi.encodeWithSelector(NonContractCaller.selector, target),
                nonce
            );
            return;
        }

        (bool success, bytes memory response) = (target).call{gas: gasLimit}(
            call
        );

        emit Dispatched(sourceChainId, target, success, response, nonce);
    }
}
