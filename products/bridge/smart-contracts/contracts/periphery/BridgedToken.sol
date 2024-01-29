// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

interface IBridgedToken is IERC20 {
    function mint(address to, uint256 amount) external;

    function burn(uint256 value) external;

    function burnFrom(address account, uint256 value) external;
}

contract BridgedToken is IERC20, ERC20, ERC20Burnable, Ownable {
    uint8 private immutable _decimals;
    address public lockProxyAddress;

    error LockProxyTransferToSelf();

    function mintIfLockProxy(address from, address to, uint amount) internal {
        if (from == lockProxyAddress) {
            if (to == lockProxyAddress) {
                revert LockProxyTransferToSelf();
            }

            uint256 balance = balanceOf(lockProxyAddress);
            if (balance < amount) {
                _mint(lockProxyAddress, amount - balance);
            }
        }
    }

    constructor(
        string memory name_,
        string memory symbol_,
        uint8 decimals_
    ) ERC20(name_, symbol_) Ownable(msg.sender) {
        _decimals = decimals_;
    }

    function mint(address to, uint256 amount) external onlyOwner {
        _mint(to, amount);
    }

    function setLockProxyAddress(address lockProxyAddress_) external onlyOwner {
        lockProxyAddress = lockProxyAddress_;
    }

    function transfer(
        address to,
        uint256 value
    ) public override(ERC20, IERC20) returns (bool) {
        mintIfLockProxy(msg.sender, to, value);
        return super.transfer(to, value);
    }

    function transferFrom(
        address from,
        address to,
        uint256 value
    ) public override(ERC20, IERC20) returns (bool) {
        mintIfLockProxy(msg.sender, to, value);
        return super.transferFrom(from, to, value);
    }

    function decimals() public view override returns (uint8) {
        return _decimals;
    }

    function circulatingSupply() external view returns (uint256 amount) {
        return totalSupply() - balanceOf(lockProxyAddress);
    }
}
