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
}

contract Target {
    function test(uint num) external pure returns (uint) {
        require(num < 1000, "Too large");
        return num + 1;
    }
}
