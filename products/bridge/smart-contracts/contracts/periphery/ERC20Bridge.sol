// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "./Bridged.sol";

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "@openzeppelin/contracts/proxy/utils/Initializable.sol";

contract BridgedERC20 is ERC20, ERC20Burnable {
    address public bridge;

    error NotBridge(address);

    modifier onlyBridge() {
        if (msg.sender != bridge) {
            revert NotBridge(msg.sender);
        }
        _;
    }

    constructor(
        string memory _name,
        string memory _symbol,
        address _bridge
    ) ERC20(_name, _symbol) {
        bridge = _bridge;
        _mint(msg.sender, 1000);
    }

    function mint(address to, uint256 amount) external onlyBridge {
        _mint(to, amount);
    }

    function burn(address from, uint256 amount) external onlyBridge {
        burnFrom(from, amount);
    }
}

contract MyToken is BridgedERC20 {
    constructor(address bridge_) BridgedERC20("MyToken", "MTK", bridge_) {}
}

contract ERC20Bridge is Initializable, Bridged, BridgedTwin {
    event Started(address, address, uint);
    event Succeeded();
    event Failed(string);

    function initialize(
        ChainGateway _relayer,
        uint _twinChainId
    ) public initializer {
        __Bridged_init(_relayer);
        __BridgedTwin_init(_twinChainId);
    }

    // This might be unecessary as bridge and exit will already restrict these calls
    function dispatched(
        uint sourceChainId,
        address target,
        bytes calldata call
    )
        external
        override
        onlyTwinChain(sourceChainId)
        returns (bool success, bytes memory response)
    {
        (success, response) = _dispatched(target, call);
    }

    function bridge(
        address token,
        address owner,
        uint value
    ) external returns (uint nonce) {
        BridgedERC20(token).transferFrom(owner, address(this), value);
        nonce = relay(
            twinChainId,
            token,
            abi.encodeWithSelector(BridgedERC20.mint.selector, owner, value),
            1_000_000
        );
        emit Started(token, owner, value);
    }

    function exit(
        address token,
        address owner,
        uint value
    ) external returns (uint nonce) {
        BridgedERC20(token).burn(owner, value);
        nonce = relay(
            twinChainId,
            token,
            abi.encodeWithSelector(IERC20.transfer.selector, owner, value),
            1_000_000
        );
        emit Started(token, owner, value);
    }

    function finish(
        bool success,
        bytes calldata res,
        uint nonce
    ) external onlyRelayer {
        if (success) {
            emit Succeeded();
        } else {
            bytes4 sig = bytes4(res[:4]);
            bytes memory err = bytes(res[4:]);
            emit Failed(abi.decode(err, (string)));
        }
    }
}
