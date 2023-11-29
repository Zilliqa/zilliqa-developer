// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";
import "@openzeppelin/contracts/utils/Create2.sol";
import "./ValidatorManager.sol";
import "./Bridged.sol";

using ECDSA for bytes32;
using MessageHashUtils for bytes;

contract Relayer {
    ValidatorManager public validatorManager;
    // targetChainId => caller => nonce
    mapping(address => uint) public nonces;
    // sourceChainId => caller => dispatched
    mapping(uint => mapping(address => mapping(uint => bool)))
        public dispatched;
    // targetChainId => caller => resumed
    mapping(address => mapping(uint => bool)) public resumed;
    mapping(address => uint) public gasDeposit;
    mapping(address => uint) public gasRefund;

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
            nonces[msg.sender]
        );
        return ++nonces[msg.sender];
    }

    function depositGas() external payable {
        gasDeposit[msg.sender] += msg.value;
    }

    function refundGas() external {
        uint amount = gasRefund[msg.sender];
        gasRefund[msg.sender] = 0;
        payable(msg.sender).transfer(amount);
    }

    modifier trackGas(address caller) {
        uint gasStart = gasleft();
        require(
            gasDeposit[caller] >= block.gaslimit * tx.gasprice,
            "Insufficient gas to cover limit"
        );
        _;
        // 44703 = 21000 + 3 + 6600 + 17100
        // 17100 = init storage cost (worst case)
        // 6600 = operations related to gas tracking
        // 21000 = fixed cost of transaction
        gasStart += 44703 + 16 * (msg.data.length - 4);
        uint spent = (gasStart - gasleft()) * tx.gasprice;
        gasDeposit[caller] -= spent;
        gasRefund[msg.sender] += spent;
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
        // TODO: Only validator
        if (dispatched[sourceChainId][caller][nonce]) {
            revert AlreadyDispatched();
        }
        dispatched[sourceChainId][caller][nonce] = true;

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
    ) external payable trackGas(caller) {
        if (resumed[caller][nonce]) {
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
        resumed[caller][nonce] = true;
    }
}
