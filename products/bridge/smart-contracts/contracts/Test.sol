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
            twinChainId,
            target,
            abi.encodeWithSignature("test(uint256)", num),
            readonly,
            this.finish.selector
        );
    }

    event Succeeded(uint);
    event Failed(string);

    function finish(
        bool success,
        bytes calldata res,
        uint nonce
    ) external onlyRelayer {
        if (success) {
            uint num = abi.decode(res, (uint));
            emit Succeeded(num);
        } else {
            bytes4 sig = bytes4(res[:4]);
            bytes memory err = bytes(res[4:]);
            emit Failed(abi.decode(err, (string)));
        }
    }

    function startSum(address target, uint num, bool readonly) external {
        relay(
            twinChainId,
            target,
            abi.encodeWithSignature("testSum(uint256)", num),
            readonly,
            this.finishSum.selector
        );
    }

    function finishSum(
        bool success,
        bytes calldata res,
        uint nonce
    ) external onlyRelayer {
        if (success) {
            uint num = abi.decode(res, (uint));
            emit Succeeded(num);
        } else {
            bytes4(res[:4]);
            bytes memory err = bytes(res[4:]);
            emit Failed(abi.decode(err, (string)));
        }
    }

    function startNoReturn(address target, uint num, bool readonly) external {
        relay(
            twinChainId,
            target,
            abi.encodeWithSignature("testNoReturn(uint256)", num),
            readonly,
            this.finishNoReturn.selector
        );
    }

    event SucceededNoReturn();

    function finishNoReturn(
        bool success,
        bytes calldata res,
        uint nonce
    ) external onlyRelayer {
        if (success) {
            emit SucceededNoReturn();
        } else {
            bytes4(res[:4]);
            bytes memory err = bytes(res[4:]);
            emit Failed(abi.decode(err, (string)));
        }
    }

    function startMultipleReturn(
        address target,
        uint num,
        bool readonly
    ) external {
        relay(
            twinChainId,
            target,
            abi.encodeWithSignature("testMultipleReturn(uint256)", num),
            readonly,
            this.finishMultipleReturn.selector
        );
    }

    event SucceededMultipleReturn(uint, uint, uint);

    function finishMultipleReturn(
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
            bytes4(res[:4]);
            bytes memory err = bytes(res[4:]);
            emit Failed(abi.decode(err, (string)));
        }
    }
}

contract Target {
    uint private _num = 1;

    event TestNoReturn(uint num);
    event TestSum(uint num);

    function test(uint num_) external pure returns (uint) {
        require(num_ < 1000, "Too large");
        return num_ + 1;
    }

    function testSum(uint num_) external returns (uint) {
        _num += num_;
        emit TestSum(_num);
        return _num;
    }

    function num() external view returns (uint) {
        return _num;
    }

    function testNoReturn(uint num_) external {
        emit TestNoReturn(num_ + 1);
    }

    function testMultipleReturn(
        uint num_
    ) external pure returns (uint, uint, uint) {
        return (num_ + 1, num_ + 2, num_ + 3);
    }
}
