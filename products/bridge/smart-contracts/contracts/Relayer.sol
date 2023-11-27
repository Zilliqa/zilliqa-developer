// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import "hardhat/console.sol";

import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";
import "@openzeppelin/contracts/utils/Create2.sol";
import "./ValidatorManager.sol";
import "./Bridged.sol";

using ECDSA for bytes32;
using MessageHashUtils for bytes;

contract Relayer {
    ValidatorManager private validatorManager;
    // targetChainId => caller => nonce
    mapping(address => uint) private _nonces;
    // sourceChainId => caller => dispatched
    mapping(uint => mapping(address => mapping(uint => bool)))
        private _dispatched;
    // targetChainId => caller => resumed
    mapping(address => mapping(uint => bool)) private _resumed;
    mapping(address => uint) private _gas;
    mapping(address => uint) private _refund;

    event TwinDeployment(address indexed twin);
    event Relayed(
        uint indexed targetChainId,
        address caller,
        address target,
        bytes call,
        bool readonly,
        bytes4 callback,
        uint nonce
    );
    event Dispatched(
        uint indexed sourceChainId,
        address indexed caller,
        bytes4 callback,
        bool success,
        bytes response,
        uint indexed nonce
    );
    event Resumed(
        uint indexed targetChainId,
        address indexed caller,
        bytes call,
        bool success,
        bytes response,
        uint indexed nonce
    );

    error FailedDeploymentInitialization();
    error InvalidSignatures();
    error NoSupermajority();
    error NonContractCaller();
    error AlreadyResumed();
    error AlreadyDispatched();

    modifier onlyContractCaller(address caller) {
        if (caller.code.length == 0) {
            revert NonContractCaller();
        }
        _;
    }

    constructor(ValidatorManager _validatorManager) {
        validatorManager = _validatorManager;
    }

    function validateRequest(
        bytes memory encodedMessage,
        bytes[] memory signatures
    ) private view {
        bytes32 hash = encodedMessage.toEthSignedMessageHash();
        if (!validatorManager.validateUniqueSignatures(hash, signatures)) {
            revert InvalidSignatures();
        }
        if (!validatorManager.hasSupermajority(signatures.length)) {
            revert NoSupermajority();
        }
    }

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

    function relay(
        uint targetChainId,
        address target,
        bytes calldata call,
        bool readonly,
        bytes4 callback
    ) external returns (uint) {
        emit Relayed(
            targetChainId,
            msg.sender,
            target,
            call,
            readonly,
            callback,
            _nonces[msg.sender]
        );
        return ++_nonces[msg.sender];
    }

    // TODO: remove later, used for testing
    function warmup() external {
        ++_refund[msg.sender];
    }

    function topUpGas(address target) external payable {
        _gas[target] += msg.value;
    }

    function refundGas() external {
        uint amount = _refund[msg.sender];
        _refund[msg.sender] = 0;
        payable(msg.sender).transfer(amount);
    }

    function refund(address x) external view {
        _refund[x];
    }

    function gas(address x) external view {
        _gas[x];
    }

    modifier trackGas(address caller) {
        uint gasStart = gasleft();
        require(
            _gas[caller] >= block.gaslimit * tx.gasprice,
            "Insufficient gas to cover limit"
        );
        _;
        // 27603 = 21000 + 3 + 6600
        gasStart += 27603 + 16 * (msg.data.length - 4);
        uint spent = (gasStart - gasleft()) * tx.gasprice;
        _gas[caller] -= spent;
        _refund[msg.sender] += spent;
    }

    function dispatch(
        uint sourceChainId,
        address caller,
        address target,
        bytes calldata call,
        bytes4 callback,
        uint nonce,
        bytes[] calldata signatures
    ) external trackGas(caller) onlyContractCaller(caller) {
        if (_dispatched[sourceChainId][caller][nonce]) {
            revert AlreadyDispatched();
        }

        bytes memory message = abi.encode(
            sourceChainId,
            block.chainid,
            caller,
            target,
            call,
            false,
            callback,
            nonce
        );
        validateRequest(message, signatures);

        (bool success, bytes memory response) = Bridged(caller).dispatched(
            sourceChainId,
            target,
            call
        );
        emit Dispatched(
            sourceChainId,
            caller,
            callback,
            success,
            response,
            nonce
        );
        _dispatched[sourceChainId][caller][nonce] = true;
    }

    function query(
        address caller,
        address target,
        bytes calldata call
    )
        external
        view
        onlyContractCaller(caller)
        returns (bool success, bytes memory response)
    {
        (success, response) = Bridged(caller).queried(target, call);
    }

    // Ensure signatures are submitted in the order of their addresses
    function resume(
        uint targetChainId,
        address caller,
        bytes4 callback,
        bool success,
        bytes calldata response,
        uint nonce,
        bytes[] calldata signatures
    ) external payable {
        if (_resumed[caller][nonce]) {
            revert AlreadyResumed();
        }
        bytes memory message = abi.encode(
            block.chainid,
            targetChainId,
            caller,
            callback,
            success,
            response,
            nonce
        );
        validateRequest(message, signatures);

        bytes memory call = abi.encodeWithSelector(
            callback,
            success,
            response,
            nonce
        );
        (bool success2, bytes memory response2) = caller.call{
            value: msg.value,
            gas: 100000
        }(call);

        emit Resumed(targetChainId, caller, call, success2, response2, nonce);
        _resumed[caller][nonce] = true;
    }
}
