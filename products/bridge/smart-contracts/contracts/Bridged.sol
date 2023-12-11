// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import "hardhat/console.sol";

import "@openzeppelin/contracts/proxy/utils/Initializable.sol";
import "./Relayer.sol";

abstract contract Bridged is Initializable {
    Relayer public relayer;

    error NotRelayer(address relayer);

    modifier onlyRelayer() {
        if (msg.sender != address(relayer)) {
            revert NotRelayer(msg.sender);
        }
        _;
    }

    function __Bridged_init(Relayer relayer_) public onlyInitializing {
        relayer = relayer_;
    }

    function relay(
        uint targetChainId,
        address target,
        bytes memory call,
        bool readonly,
        bytes4 callback
    ) internal returns (uint nonce) {
        nonce = relayer.relay(targetChainId, target, call, readonly, callback);
    }

    function _dispatched(
        address target,
        bytes calldata call
    ) internal onlyRelayer returns (bool success, bytes memory response) {
        (success, response) = target.call{gas: 1_000_000}(call);
    }

    function dispatched(
        uint sourceChainId,
        address target,
        bytes calldata call
    )
        external
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

    function depositFee() external payable virtual {
        relayer.depositFee{value: msg.value}();
    }
}

abstract contract BridgedTwin is Initializable, Bridged {
    uint public twinChainId;

    error InvalidChainId(uint chainId);
    error NotTwinChain(uint chainId);

    modifier onlyTwinChain(uint _twinChainId) {
        if (twinChainId != _twinChainId) {
            revert NotTwinChain(_twinChainId);
        }
        _;
    }

    function __BridgedTwin_init(uint _twinChainId) public onlyInitializing {
        if (_twinChainId == 0 || _twinChainId == block.chainid) {
            revert InvalidChainId(_twinChainId);
        }
        twinChainId = _twinChainId;
    }
}
