// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;

import "./Bridge.sol";

contract Twin is Bridged {
    function start(address target, uint num, bool readonly) public {
        uint nonce = relay(
            target,
            abi.encodeWithSignature("test(uint256)", num),
            readonly,
            this.finish.selector
        );
        console.log("start()", nonce);
    }

    event Succeeded(uint);
    event Failed(string);

    function finish(
        bool success,
        bytes calldata res,
        uint nonce
    ) public onlyRelayer {
        console.log("finish()", nonce);
        if (success) {
            uint num = abi.decode(res, (uint));
            emit Succeeded(num);
        } else {
            bytes4 sig = bytes4(res[:4]);
            bytes memory err = bytes(res[4:]);
            emit Failed(abi.decode(err, (string)));
        }
    }
}

contract Target {
    function test(uint num) public pure returns (uint) {
        console.log("test()");
        require(num < 1000, "Too large");
        return num + 1;
    }
}
