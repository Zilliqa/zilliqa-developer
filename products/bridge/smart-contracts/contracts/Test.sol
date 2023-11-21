// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/proxy/utils/Initializable.sol";
import "./Bridged.sol";

contract Twin is Initializable, Bridged, BridgedTwin {
    function initialize(Relayer relayer, uint twinChainId) public initializer {
        __Bridged_init(relayer);
        __BridgedTwin_init(twinChainId);
    }

    function start(address target, uint num, bool readonly) external {
        uint nonce = relay(
            twinChainId(),
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
        uint targetChainId,
        bool success,
        bytes calldata res,
        uint nonce
    ) external onlyRelayer {
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

    function startNoReturn(address target, uint num, bool readonly) external {
        uint nonce = relay(
            twinChainId(),
            target,
            abi.encodeWithSignature("testNoReturn(uint256)", num),
            readonly,
            this.finishNoReturn.selector
        );
        console.log("start()", nonce);
    }

    event SucceededNoReturn();

    function finishNoReturn(
        uint targetChainId,
        bool success,
        bytes calldata res,
        uint nonce
    ) external onlyRelayer {
        if (success) {
            emit SucceededNoReturn();
        } else {
            bytes4 sig = bytes4(res[:4]);
            bytes memory err = bytes(res[4:]);
            emit Failed(abi.decode(err, (string)));
        }
    }

    function startMultipleReturn(
        address target,
        uint num,
        bool readonly
    ) external {
        uint nonce = relay(
            twinChainId(),
            target,
            abi.encodeWithSignature("testMultipleReturn(uint256)", num),
            readonly,
            this.finishMultipleReturn.selector
        );
    }

    event SucceededMultipleReturn(uint, uint, uint);

    function finishMultipleReturn(
        uint targetChainId,
        bool success,
        bytes calldata res,
        uint nonce
    ) external onlyRelayer {
        if (success) {
            (uint num, uint num2, uint num3) = abi.decode(
                res,
                (uint, uint, uint)
            );
            emit SucceededMultipleReturn(num, num2, num3);
        } else {
            bytes4 sig = bytes4(res[:4]);
            bytes memory err = bytes(res[4:]);
            emit Failed(abi.decode(err, (string)));
        }
    }
}

contract Target {
    event TestNoReturn(uint num);

    function test(uint num) external pure returns (uint) {
        require(num < 1000, "Too large");
        return num + 1;
    }

    function testNoReturn(uint num) external {
        emit TestNoReturn(num + 1);
    }

    function testMultipleReturn(
        uint num
    ) external pure returns (uint, uint, uint) {
        return (num + 1, num + 2, num + 3);
    }
}
