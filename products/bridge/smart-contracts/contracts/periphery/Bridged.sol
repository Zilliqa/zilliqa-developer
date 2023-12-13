// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts/utils/Create2.sol";
import "contracts/core/ChainGateway.sol";

abstract contract Bridged is Initializable {
    ChainGateway public relayer;

    error NotRelayer(address relayer);

    modifier onlyRelayer() {
        if (msg.sender != address(relayer)) {
            revert NotRelayer(msg.sender);
        }
        _;
    }

    function __Bridged_init(ChainGateway _relayer) public onlyInitializing {
        relayer = _relayer;
    }

    // function relay(
    //     uint targetChainId,
    //     address target,
    //     bytes memory call,
    //     uint gasLimit
    // ) internal returns (uint nonce) {
    //     nonce = relayer.relay(targetChainId, target, call, gasLimit);
    // }

    function _dispatched(
        address target,
        bytes calldata call
    ) internal onlyRelayer returns (bool success, bytes memory response) {
        (success, response) = target.call{gas: 1_000_000}(call);
    }

    function dispatched(
        uint sourceChainId,
        address target,
        bytes calldata call
    )
        external
        virtual
        onlyRelayer
        returns (bool success, bytes memory response)
    {
        (success, response) = _dispatched(target, call);
    }

    function depositFee() external payable virtual {
        relayer.depositFee{value: msg.value}();
    }
}

abstract contract BridgedTwin is Initializable, Bridged {
    uint public twinChainId;

    error InvalidChainId(uint chainId);
    error NotTwinChain(uint chainId);

    modifier onlyTwinChain(uint _twinChainId) {
        if (twinChainId != _twinChainId) {
            revert NotTwinChain(_twinChainId);
        }
        _;
    }

    function __BridgedTwin_init(uint _twinChainId) public onlyInitializing {
        if (_twinChainId == 0 || _twinChainId == block.chainid) {
            revert InvalidChainId(_twinChainId);
        }
        twinChainId = _twinChainId;
    }
}

interface ITwinFactory {
    error FailedDeploymentInitialization();
    event TwinDeployment(address indexed twin);
}

contract TwinFactory is ITwinFactory {
    function deployTwin(
        bytes32 salt,
        bytes calldata bytecode,
        bytes calldata initCall
    ) external returns (address) {
        address bridgedContract = Create2.deploy(0, salt, bytecode);
        (bool success, ) = bridgedContract.call(initCall);
        if (!success) {
            revert FailedDeploymentInitialization();
        }
        emit TwinDeployment(bridgedContract);
        return bridgedContract;
    }
}
