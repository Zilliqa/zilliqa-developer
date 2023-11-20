// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import "hardhat/console.sol";

import "@openzeppelin/contracts/proxy/utils/Initializable.sol";
import "./Relayer.sol";

abstract contract Bridged is Initializable {
    Relayer private _relayer;

    function __Bridged_init(Relayer relayer) public onlyInitializing {
        _relayer = relayer;
    }

    modifier onlyRelayer() {
        require(msg.sender == address(_relayer), "Must be called by relayer");
        _;
    }

    function dispatched(
        uint targetChainId,
        address target,
        bytes memory call
    )
        public
        payable
        virtual
        onlyRelayer
        returns (bool success, bytes memory response)
    {
        (success, response) = _dispatched(target, call);
    }

    function _dispatched(
        address target,
        bytes memory call
    ) internal onlyRelayer returns (bool success, bytes memory response) {
        console.log("dispatched()");
        (success, response) = target.call{value: msg.value, gas: 100000}(call);
    }

    function queried(
        address target,
        bytes memory call
    ) public view virtual returns (bool success, bytes memory response) {
        (success, response) = target.staticcall{gas: 100000}(call);
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
}

abstract contract BridgedTwin is Initializable, Bridged {
    uint private _twinChainId;

    function __BridgedTwin_init(uint twinChainId_) public onlyInitializing {
        require(
            twinChainId_ > 0 && twinChainId_ != block.chainid,
            "Invalid chain ID"
        );
        _twinChainId = twinChainId_;
    }

    function twinChainId() public view returns (uint) {
        return _twinChainId;
    }

    modifier onlyTwin(uint twinChainId_) {
        require(_twinChainId == twinChainId_, "Must be called by twin");
        _;
    }
}
