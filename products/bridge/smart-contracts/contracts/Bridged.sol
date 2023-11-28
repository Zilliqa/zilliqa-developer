// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import "hardhat/console.sol";

import "@openzeppelin/contracts/proxy/utils/Initializable.sol";
import "./Relayer.sol";

abstract contract Bridged is Initializable {
    Relayer private _relayer;

    error NotRelayer(address relayer);

    modifier onlyRelayer() {
        if (msg.sender != address(_relayer)) {
            revert NotRelayer(msg.sender);
        }
        _;
    }

    function __Bridged_init(Relayer relayer_) public onlyInitializing {
        _relayer = relayer_;
    }

    function relayer() public view returns (Relayer) {
        return _relayer;
    }

    function relay(
        uint targetChainId,
        address target,
        bytes memory call,
        bool readonly,
        bytes4 callback
    ) internal returns (uint nonce) {
        nonce = _relayer.relay(targetChainId, target, call, readonly, callback);
    }

    function _dispatched(
        address target,
        bytes calldata call
    ) internal onlyRelayer returns (bool success, bytes memory response) {
        (success, response) = target.call{value: msg.value, gas: 100000}(call);
    }

    function dispatched(
        uint sourceChainId,
        address target,
        bytes calldata call
    )
        external
        payable
        virtual
        onlyRelayer
        returns (bool success, bytes memory response)
    {
        (success, response) = _dispatched(target, call);
    }

    function queried(
        address target,
        bytes calldata call
    ) external view virtual returns (bool success, bytes memory response) {
        (success, response) = target.staticcall{gas: 100000}(call);
    }
}

abstract contract BridgedTwin is Initializable, Bridged {
    uint private _twinChainId;

    error InvalidChainId(uint chainId);
    error NotTwinChain(uint chainId);

    modifier onlyTwinChain(uint twinChainId_) {
        if (_twinChainId != twinChainId_) {
            revert NotTwinChain(twinChainId_);
        }
        _;
    }

    function __BridgedTwin_init(uint twinChainId_) public onlyInitializing {
        if (twinChainId_ == 0 || twinChainId_ == block.chainid) {
            revert InvalidChainId(twinChainId_);
        }
        _twinChainId = twinChainId_;
    }

    function twinChainId() public view returns (uint) {
        return _twinChainId;
    }
}
