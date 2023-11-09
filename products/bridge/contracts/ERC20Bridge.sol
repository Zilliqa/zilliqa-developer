// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;

import "./Bridge.sol";

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";

contract MyToken is ERC20, ERC20Burnable {
	address bridge;

    constructor(address _bridge) ERC20("MyToken", "MTK") {
		bridge = _bridge;
		_mint(msg.sender, 1000);
	}

    function mint(address to, uint256 amount) public {
		require(msg.sender == bridge, "Not the bridge");
        _mint(to, amount);
    }
	
    function burn(address from, uint256 amount) public {
		require(msg.sender == bridge, "Not the bridge");
        burnFrom(from, amount);
    }
}

contract ERC20Bridge is Bridged {

	event Started(address, address, uint);

	function bridge(address token, address owner, uint value) public {
		MyToken(token).transferFrom(owner, address(this), value);
		uint nonce = relay(token, abi.encodeWithSignature("mint(address,uint256)", owner, value), false, this.finish.selector);
		emit Started(token, owner, value);
	}

	function exit(address token, address owner, uint value) public {
		MyToken(token).burn(owner, value);
		uint nonce = relay(token, abi.encodeWithSignature("transfer(address,uint256)", owner, value), false, this.finish.selector);
		emit Started(token, owner, value);
	}

	event Succeeded();
	event Failed(string);
	
	function finish(bool success, bytes calldata res, uint nonce) public onlyRelayer {
		if (success) {
			emit Succeeded();
		} else {
			bytes4 sig = bytes4(res[:4]);
			bytes memory err = bytes(res[4:]);
			emit Failed(abi.decode(err, (string)));
		}
	}
}

