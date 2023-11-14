// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "./Bridged.sol";

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";

contract BridgedERC20 is ERC20, ERC20Burnable {
    address _bridge;

    constructor(
        string memory name_,
        string memory symbol_,
        address bridge_
    ) ERC20(name_, symbol_) {
        _bridge = bridge_;
        _mint(msg.sender, 1000);
    }

    modifier onlyBridge() {
        require(msg.sender == _bridge, "Not the bridge");
        _;
    }

    function mint(address to, uint256 amount) public onlyBridge {
        _mint(to, amount);
    }

    function burn(address from, uint256 amount) public onlyBridge {
        burnFrom(from, amount);
    }
}

contract MyToken is BridgedERC20 {
    constructor(address bridge_) BridgedERC20("MyToken", "MTK", bridge_) {}
}

contract ERC20Bridge is Bridged {
    event Started(address, address, uint);

    function bridge(
        address token,
        address owner,
        uint value
    ) public returns (uint nonce) {
        MyToken(token).transferFrom(owner, address(this), value);
        nonce = relay(
            token,
            abi.encodeWithSignature("mint(address,uint256)", owner, value),
            false,
            this.finish.selector
        );
        emit Started(token, owner, value);
    }

    function exit(
        address token,
        address owner,
        uint value
    ) public returns (uint nonce) {
        MyToken(token).burn(owner, value);
        nonce = relay(
            token,
            abi.encodeWithSignature("transfer(address,uint256)", owner, value),
            false,
            this.finish.selector
        );
        emit Started(token, owner, value);
    }

    event Succeeded();
    event Failed(string);

    function finish(
        bool success,
        bytes calldata res,
        uint nonce
    ) public onlyRelayer {
        if (success) {
            emit Succeeded();
        } else {
            bytes4 sig = bytes4(res[:4]);
            bytes memory err = bytes(res[4:]);
            emit Failed(abi.decode(err, (string)));
        }
    }
}
