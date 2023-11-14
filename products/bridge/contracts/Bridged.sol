// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import "hardhat/console.sol";

import "@openzeppelin/contracts/proxy/utils/Initializable.sol";
import "./Relayer.sol";

abstract contract Bridged is Initializable {
    Relayer private _relayer;

    function initialize(Relayer relayer) public initializer {
        _relayer = relayer;
    }

    modifier onlyRelayer() {
        require(msg.sender == address(_relayer), "Must be called by relayer");
        _;
    }

    function dispatched(
        address target,
        bytes memory call
    ) public payable onlyRelayer returns (bool success, bytes memory response) {
        console.log("dispatched()");
        (success, response) = target.call{value: msg.value, gas: 100000}(call);
    }

    function queried(
        address target,
        bytes memory call
    ) public view onlyRelayer returns (bool success, bytes memory response) {
        console.log("queried()");
        (success, response) = target.staticcall{gas: 100000}(call);
    }

    function relay(
        address target,
        bytes memory call,
        bool readonly,
        bytes4 callback
    ) internal returns (uint nonce) {
        nonce = _relayer.relay(target, call, readonly, callback);
    }
}
