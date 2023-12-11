// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity 0.8.20;

import {Test, Vm} from "forge-std/Test.sol";
import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";
import {Target, ValidatorManagerFixture, IReentrancy} from "foundry/test/Helpers.sol";

import {ChainDispatcher, IChainDispatcherEvents, IChainDispatcherErrors} from "contracts/core/ChainDispatcher.sol";
import {ValidatorManager} from "contracts/core/ValidatorManager.sol";
import {ISignatureValidatorErrors} from "contracts/core/SignatureValidator.sol";
import {IDispatchReplayCheckerErrors} from "contracts/core/DispatchReplayChecker.sol";

struct DispatchArgs {
    uint sourceChainId;
    address target;
    bytes call;
    uint gasLimit;
    uint nonce;
}

library DispatchArgsBuilder {
    function instance(
        address target
    ) external pure returns (DispatchArgs memory args) {
        args.sourceChainId = 1;
        args.target = target;
        args.call = abi.encodeWithSelector(Target.work.selector, uint(1));
        args.gasLimit = 1_000_000;
        args.nonce = 1;
    }

    function withCall(
        DispatchArgs memory args,
        bytes calldata call
    ) external pure returns (DispatchArgs memory) {
        args.call = call;
        return args;
    }
}

contract DispatcherHarness is ChainDispatcher, Test {
    constructor(address _validatorManager) ChainDispatcher(_validatorManager) {}

    function workaround_updateValidatorManager(
        ValidatorManager _validatorManager
    ) external {
        validatorManager = _validatorManager;
    }

    function verifyFeeInvariant(
        uint initialFeeDeposit,
        uint gasSpent,
        address sponsor1,
        address sender1
    ) external {
        // feeDeposit + feeRefund = initial deposit
        assertEq(
            feeDeposit[sponsor1],
            initialFeeDeposit - feeRefund[sender1],
            "Invariant violated: feeDeposit + feeRefund = initial deposit"
        );
        assertGe(
            feeRefund[sender1],
            gasSpent,
            "Invariant violated: Sender should be refunded more than the gas spent"
        );
    }
}

contract DispatcherFixture is ValidatorManagerFixture {
    using MessageHashUtils for bytes;
    using DispatchArgsBuilder for DispatchArgs;
    uint constant INITIAL_FEE_DEPOSIT = 1 ether;

    Target internal immutable target = new Target();
    DispatcherHarness internal immutable dispatcher;

    constructor() ValidatorManagerFixture() {
        // Deposit gas from the bridge to the dispatcher

        dispatcher = new DispatcherHarness(address(validatorManager));
        hoax(address(target));
        dispatcher.depositFee{value: INITIAL_FEE_DEPOSIT}();
    }

    function signDispatch(
        DispatchArgs memory args
    ) public returns (bytes[] memory signatures) {
        bytes32 hashedMessage = abi
            .encode(
                args.sourceChainId,
                block.chainid,
                args.target,
                args.call,
                args.gasLimit,
                args.nonce
            )
            .toEthSignedMessageHash();

        signatures = multiSign(sort(validators), hashedMessage);
    }
}

contract DispatcherTests is IChainDispatcherEvents, DispatcherFixture {
    using DispatchArgsBuilder for DispatchArgs;

    function setUp() external {
        vm.startPrank(address(validators[0].addr));
    }

    function test_happyPath() external {
        DispatchArgs memory args = DispatchArgsBuilder.instance(
            address(target)
        );
        bytes[] memory signatures = signDispatch(args);

        vm.expectCall(address(target), args.call);
        vm.expectEmit(address(dispatcher));
        emit Dispatched(
            args.sourceChainId,
            args.target,
            true,
            abi.encode(uint(2)),
            args.nonce
        );
        dispatcher.dispatch(
            args.sourceChainId,
            args.target,
            args.call,
            args.gasLimit,
            args.nonce,
            signatures
        );
        assertEq(dispatcher.dispatched(args.sourceChainId, args.nonce), true);
    }

    function testRevert_badSignature() external {
        // Prepare call
        DispatchArgs memory args = DispatchArgsBuilder.instance(
            address(target)
        );
        bytes[] memory signatures = signDispatch(args);
        uint badNonce = args.nonce + 1;

        vm.expectRevert(
            ISignatureValidatorErrors.InvalidValidatorOrSignatures.selector
        );
        // Dispatch
        dispatcher.dispatch(
            args.sourceChainId,
            args.target,
            args.call,
            args.gasLimit,
            badNonce,
            signatures
        );
    }

    function testRevert_replay() external {
        // Prepare call
        DispatchArgs memory args = DispatchArgsBuilder.instance(
            address(target)
        );
        bytes[] memory signatures = signDispatch(args);

        // Dispatch
        dispatcher.dispatch(
            args.sourceChainId,
            args.target,
            args.call,
            args.gasLimit,
            args.nonce,
            signatures
        );
        // Replay
        vm.expectRevert(
            IDispatchReplayCheckerErrors.AlreadyDispatched.selector
        );
        dispatcher.dispatch(
            args.sourceChainId,
            args.target,
            args.call,
            args.gasLimit,
            args.nonce,
            signatures
        );
    }

    function test_failedCall() external {
        uint num = 1000;
        bytes memory failedCall = abi.encodeWithSelector(
            target.work.selector,
            num
        );
        DispatchArgs memory args = DispatchArgsBuilder
            .instance(address(target))
            .withCall(failedCall);
        bytes[] memory signatures = signDispatch(args);

        // Dispatch
        vm.expectCall(address(target), failedCall);

        bytes memory expectedError = abi.encodeWithSignature(
            "Error(string)",
            "Too large"
        );
        vm.expectEmit(address(dispatcher));
        emit Dispatched(
            args.sourceChainId,
            args.target,
            false,
            expectedError,
            args.nonce
        );
        dispatcher.dispatch(
            args.sourceChainId,
            args.target,
            args.call,
            args.gasLimit,
            args.nonce,
            signatures
        );
    }

    function testRevert_nonContractCaller() external {
        DispatchArgs memory args = DispatchArgsBuilder.instance(vm.addr(1001));
        bytes[] memory signatures = signDispatch(args);

        // Dispatch
        vm.expectRevert(IChainDispatcherErrors.NonContractCaller.selector);
        dispatcher.dispatch(
            args.sourceChainId,
            args.target,
            args.call,
            args.gasLimit,
            args.nonce,
            signatures
        );
    }

    function test_outOfGasCall() external {
        bytes memory call = abi.encodeWithSelector(
            target.infiniteLoop.selector
        );
        DispatchArgs memory args = DispatchArgsBuilder
            .instance(address(target))
            .withCall(call);
        bytes[] memory signatures = signDispatch(args);

        // Dispatch
        vm.expectCall(address(target), args.call);
        vm.expectEmit(address(dispatcher));
        emit Dispatched(
            args.sourceChainId,
            args.target,
            false,
            hex"", // denotes out of gas
            args.nonce
        );
        dispatcher.dispatch(
            args.sourceChainId,
            args.target,
            args.call,
            args.gasLimit,
            args.nonce,
            signatures
        );

        assertEq(target.c(), uint(0));
    }

    function testRevert_whenNotValidatorSubmitting() public {
        // Prepare call
        DispatchArgs memory args = DispatchArgsBuilder.instance(
            address(target)
        );
        bytes[] memory signatures = signDispatch(args);

        // Dispatch
        vm.stopPrank();
        vm.expectRevert(IChainDispatcherErrors.NotValidator.selector);
        dispatcher.dispatch(
            args.sourceChainId,
            args.target,
            args.call,
            args.gasLimit,
            args.nonce,
            signatures
        );
    }

    function test_reentrancy() external {
        bytes memory call = abi.encodeWithSelector(target.reentrancy.selector);
        DispatchArgs memory args = DispatchArgsBuilder
            .instance(address(target))
            .withCall(call);
        bytes[] memory signatures = signDispatch(args);

        target.setReentrancyConfig(
            address(dispatcher),
            abi.encodeWithSelector(
                dispatcher.dispatch.selector,
                args.sourceChainId,
                args.target,
                args.call,
                args.gasLimit,
                args.nonce,
                signatures
            )
        );

        // Dispatch
        bytes memory expectedError = abi.encodeWithSelector(
            IReentrancy.ReentrancySafe.selector
        );
        vm.expectEmit(address(dispatcher));
        emit Dispatched(
            args.sourceChainId,
            args.target,
            false,
            expectedError,
            args.nonce
        );
        dispatcher.dispatch(
            args.sourceChainId,
            args.target,
            args.call,
            args.gasLimit,
            args.nonce,
            signatures
        );
    }

    function test_enoughFeeRefundWhenSingleValidator() external {
        // Setup single validator
        (
            Vm.Wallet[] memory _validators,
            ValidatorManager _validatorManager
        ) = generateValidatorManager(1);
        dispatcher.workaround_updateValidatorManager(_validatorManager);
        validators = _validators;
        address sender = validators[0].addr;

        // Fix gas price
        vm.txGasPrice(10 gwei);

        // Prepare call
        DispatchArgs memory args = DispatchArgsBuilder.instance(
            address(target)
        );
        bytes[] memory signatures = signDispatch(args);

        // Dispatch
        vm.expectCall(address(target), args.call);
        vm.expectEmit(address(dispatcher));
        emit Dispatched(
            args.sourceChainId,
            args.target,
            true,
            abi.encode(uint(2)),
            args.nonce
        );
        uint gasStart = gasleft();
        dispatcher.dispatch{gas: 1_000_000}(
            args.sourceChainId,
            args.target,
            args.call,
            args.gasLimit,
            args.nonce,
            signatures
        );
        uint feeSpent = (gasStart - gasleft()) * tx.gasprice;

        // Verify fee invariant
        dispatcher.verifyFeeInvariant(
            INITIAL_FEE_DEPOSIT,
            feeSpent,
            address(target),
            sender
        );
    }

    function test_enoughFeeRefundWhenManyValidators() external {
        // Setup single validator
        (
            Vm.Wallet[] memory _validators,
            ValidatorManager _validatorManager
        ) = generateValidatorManager(1000);
        dispatcher.workaround_updateValidatorManager(_validatorManager);
        validators = _validators;
        address sender = validators[0].addr;

        // Fix gas price
        vm.txGasPrice(10 gwei);

        // Prepare call
        DispatchArgs memory args = DispatchArgsBuilder.instance(
            address(target)
        );
        bytes[] memory signatures = signDispatch(args);

        // Dispatch
        vm.expectCall(address(target), args.call);
        vm.expectEmit(address(dispatcher));
        emit Dispatched(
            args.sourceChainId,
            args.target,
            true,
            abi.encode(uint(2)),
            args.nonce
        );
        uint gasStart = gasleft();
        dispatcher.dispatch{gas: 10_000_000}(
            args.sourceChainId,
            args.target,
            args.call,
            args.gasLimit,
            args.nonce,
            signatures
        );
        uint feeSpent = (gasStart - gasleft()) * tx.gasprice;

        // Verify fee invariant
        dispatcher.verifyFeeInvariant(
            INITIAL_FEE_DEPOSIT,
            feeSpent,
            address(target),
            sender
        );
    }
}
