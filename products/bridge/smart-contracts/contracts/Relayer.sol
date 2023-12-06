// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";
import "@openzeppelin/contracts/utils/Create2.sol";
import "./ValidatorManager.sol";
import "./Bridged.sol";

using ECDSA for bytes32;
using MessageHashUtils for bytes;

interface IRelayerEvents {
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
}

interface IRelayerErrors {
    error FailedDeploymentInitialization();
    error InvalidSignatures();
    error NoSupermajority();
    error NonContractCaller();
    error AlreadyResumed();
    error AlreadyDispatched();
    /**
     * Occurs when a resume is attempted with a nonce that is greater than the
     * current nonce for the caller. Trying to resume a call not yet initialised
     */
    error IllegalResumeNonce();
    error InsufficientMinFeeDeposit();
}

interface IRelayer is IRelayerEvents, IRelayerErrors {}

contract Relayer is IRelayer {
    ValidatorManager public validatorManager;
    // caller => nonce
    mapping(address => uint) public nonces;
    // sourceChainId => caller => dispatched
    mapping(uint => mapping(address => mapping(uint => bool)))
        public dispatched;
    // caller => resumed
    mapping(address => mapping(uint => bool)) public resumed;
    mapping(address => uint) public feeDeposit;
    mapping(address => uint) public feeRefund;

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
    ) internal view {
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

    function depositFee() external payable {
        feeDeposit[msg.sender] += msg.value;
    }

    function withdrawFee(uint amount) external {
        feeDeposit[msg.sender] -= amount;
        payable(msg.sender).transfer(amount);
    }

    function refundFee() external {
        uint amount = feeRefund[msg.sender];
        // TODO: keep it 1 for saving gas
        feeRefund[msg.sender] = 0;
        payable(msg.sender).transfer(amount);
    }

    modifier meterFee(address caller) {
        uint feeStart = gasleft() * tx.gasprice;
        // 44703 = 21000 + 3 + 6600 + 17100
        // 17100 = init storage cost (worst case)
        // 6600 = operations related to gas tracking
        // 21000 = fixed cost of transaction
        uint feeOffset = (44703 + 16 * (msg.data.length - 4)) * tx.gasprice;
        // Should reject if insuficient to pay for the offset
        if (feeDeposit[caller] < feeOffset) {
            revert InsufficientMinFeeDeposit();
        }
        feeStart += feeOffset;
        // It will still take fees even if insufficient fee deposit is provided
        if (feeDeposit[caller] >= feeStart) {
            _;
        }
        uint spent = feeStart - gasleft() * tx.gasprice;
        feeDeposit[caller] -= spent;
        feeRefund[msg.sender] += spent;
    }

    function dispatch(
        uint sourceChainId,
        address caller,
        address target,
        bytes calldata call,
        bytes4 callback,
        uint nonce,
        bytes[] calldata signatures
    ) external meterFee(caller) onlyContractCaller(caller) {
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
    ) external meterFee(caller) onlyContractCaller(caller) {
        if (nonce > nonces[caller]) {
            revert IllegalResumeNonce();
        }
        if (resumed[caller][nonce]) {
            revert AlreadyResumed();
        }
        resumed[caller][nonce] = true;

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
        // TODO: Specifiy gas
        (bool success2, bytes memory response2) = caller.call{gas: 1_000_000}(
            call
        );

        emit Resumed(targetChainId, caller, call, success2, response2, nonce);
    }
}
