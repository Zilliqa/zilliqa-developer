// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity 0.8.20;

import "contracts/ValidatorManager.sol";
import "contracts/Relayer.sol";
import "./Tester.sol";

using ECDSA for bytes32;
using MessageHashUtils for bytes;

contract BridgeTarget {
    uint public c = 0;

    function work(uint num_) external pure returns (uint) {
        require(num_ < 1000, "Too large");
        return num_ + 1;
    }

    function infiniteLoop() public {
        while (true) {
            c = c + 1;
        }
    }
}

contract SimpleBridge is Bridged {
    function initialize(Relayer relayer) public initializer {
        __Bridged_init(relayer);
    }
}

abstract contract RelayerTestFixture is Tester, IRelayer {
    ValidatorManager validatorManager;
    Relayer relayer;
    uint constant validatorCount = 10;
    Vm.Wallet[] validators = new Vm.Wallet[](validatorCount);
    SimpleBridge immutable bridge = new SimpleBridge();

    constructor() {
        // Setup validator manager
        address[] memory validatorAddresses = new address[](validatorCount);
        for (uint i = 0; i < validatorCount; ++i) {
            validators[i] = vm.createWallet(i + 1);
            validatorAddresses[i] = validators[i].addr;
        }
        validatorManager = new ValidatorManager(validatorAddresses);
        // Setup relayer
        relayer = new Relayer(validatorManager);
        // Initialise bridge
        bridge.initialize(relayer);
    }
}

contract Dispatch is RelayerTestFixture {
    BridgeTarget immutable bridgeTarget = new BridgeTarget();

    function setUp() public {
        // Deposit gas from the bridge to the relayer
        bridge.depositFee{value: 1 ether}();
    }

    function getDispatchArgs()
        public
        returns (
            uint sourceChainId,
            address caller,
            address target,
            bytes memory call,
            bytes4 callback,
            uint nonce,
            bytes[] memory signatures
        )
    {
        sourceChainId = 1;
        caller = address(bridge);
        call = abi.encodeWithSelector(bridgeTarget.work.selector, uint(1));
        target = address(bridgeTarget);
        callback = bytes4(0);
        nonce = 1;
        // Generate signatures
        signatures = signDispatch(
            sourceChainId,
            caller,
            target,
            call,
            callback,
            nonce
        );
    }

    function getDispatchArgs(
        bytes memory call
    )
        public
        returns (
            uint sourceChainId,
            address caller,
            address target,
            bytes4 callback,
            uint nonce,
            bytes[] memory signatures
        )
    {
        sourceChainId = 1;
        caller = address(bridge);
        target = address(bridgeTarget);
        callback = bytes4(0);
        nonce = 1;
        // Generate signatures
        signatures = signDispatch(
            sourceChainId,
            caller,
            target,
            call,
            callback,
            nonce
        );
    }

    function signDispatch(
        uint sourceChainId,
        address caller,
        address target,
        bytes memory call,
        bytes4 callback,
        uint nonce
    ) public returns (bytes[] memory signatures) {
        bytes32 hashedMessage = abi
            .encode(
                sourceChainId,
                block.chainid,
                caller,
                target,
                call,
                false,
                callback,
                nonce
            )
            .toEthSignedMessageHash();

        signatures = multiSign(sort(validators), hashedMessage);
    }

    function test_happyPath() external {
        (
            uint sourceChainId,
            address caller,
            address target,
            bytes memory call,
            bytes4 callback,
            uint nonce,
            bytes[] memory signatures
        ) = getDispatchArgs();

        vm.expectCall(address(bridgeTarget), call);
        vm.expectEmit(address(relayer));
        emit IRelayerEvents.Dispatched(
            sourceChainId,
            caller,
            callback,
            true,
            abi.encode(uint(2)),
            nonce
        );
        relayer.dispatch(
            sourceChainId,
            caller,
            target,
            call,
            callback,
            nonce,
            signatures
        );
        // Check nonce increased
    }

    function testRevert_badSignature() external {
        // Prepare call
        (
            uint sourceChainId,
            address caller,
            address target,
            bytes memory call,
            bytes4 callback,
            uint signedNonce,
            bytes[] memory signatures
        ) = getDispatchArgs();
        uint dispatchedNonce = signedNonce + 1;

        vm.expectRevert(InvalidSignatures.selector);
        // Dispatch
        relayer.dispatch(
            sourceChainId,
            caller,
            target,
            call,
            callback,
            dispatchedNonce,
            signatures
        );
    }

    function testRevert_replay() external {
        // Prepare call
        (
            uint sourceChainId,
            address caller,
            address target,
            bytes memory call,
            bytes4 callback,
            uint nonce,
            bytes[] memory signatures
        ) = getDispatchArgs();

        // Dispatch
        relayer.dispatch(
            sourceChainId,
            caller,
            target,
            call,
            callback,
            nonce,
            signatures
        );
        // Replay
        vm.expectRevert(AlreadyDispatched.selector);
        relayer.dispatch(
            sourceChainId,
            caller,
            target,
            call,
            callback,
            nonce,
            signatures
        );
    }

    function test_unsuccessfulCalls() external {
        uint num = 1000;
        bytes memory failedCall = abi.encodeWithSelector(
            bridgeTarget.work.selector,
            num
        );
        (
            uint sourceChainId,
            address caller,
            address target,
            bytes4 callback,
            uint nonce,
            bytes[] memory signatures
        ) = getDispatchArgs(failedCall);

        // Dispatch
        vm.expectCall(address(bridgeTarget), failedCall);

        bytes memory expectedError = abi.encodeWithSignature(
            "Error(string)",
            "Too large"
        );
        vm.expectEmit(address(relayer));
        emit IRelayerEvents.Dispatched(
            sourceChainId,
            caller,
            callback,
            false,
            expectedError,
            nonce
        );
        relayer.dispatch(
            sourceChainId,
            caller,
            target,
            failedCall,
            callback,
            nonce,
            signatures
        );
    }

    function testRevert_nonContractCaller() external {
        (
            uint sourceChainId,
            ,
            address target,
            bytes memory call,
            bytes4 callback,
            uint nonce,
            bytes[] memory signatures
        ) = getDispatchArgs();

        // Dispatch
        vm.expectRevert(NonContractCaller.selector);
        relayer.dispatch(
            sourceChainId,
            vm.addr(1001),
            target,
            call,
            callback,
            nonce,
            signatures
        );
    }

    function testRevert_overspentgas() external {
        bytes memory call = abi.encodeWithSelector(
            bridgeTarget.infiniteLoop.selector
        );
        (
            uint sourceChainId,
            address caller,
            address target,
            bytes4 callback,
            uint nonce,
            bytes[] memory signatures
        ) = getDispatchArgs(call);

        // Dispatch
        vm.expectCall(address(bridgeTarget), call);
        vm.expectEmit(address(relayer));
        emit IRelayerEvents.Dispatched(
            sourceChainId,
            caller,
            callback,
            false,
            hex"", // denotes out of gas
            nonce
        );
        relayer.dispatch(
            sourceChainId,
            caller,
            target,
            call,
            callback,
            nonce,
            signatures
        );

        assertEq(bridgeTarget.c(), uint(0));
    }
}

contract Resume is Tester {}

contract SignatureValidation is Tester {
    function test_happyPath() external TODO {}

    function testRevert_noSignatures() external TODO {}

    function testRevert_emptyMessage() external TODO {}

    function testRevert_noMajority() external TODO {}

    function testRevert_invalidSignature() external TODO {}

    function testRevert_unorderedSignatures() external TODO {}

    function testRevert_repeatedSigners() external TODO {}
}

contract Fees is Tester {
    function test_feesRefundedToValidator() external TODO {}

    function testRevert_insufficientFees() external TODO {}

    function test_enoughRefundWhenSingleValidator() external TODO {}

    function test_enoughRefundWhenManyValidators() external TODO {}

    function test_enoughRefundWhenNoArgumentCall() external TODO {}

    function test_enoughRefundWhenManyArgumentCall() external TODO {}

    function testFuzz_varyValidators() external TODO {}

    function testFuzz_varyGasPrice() external TODO {}

    function test_validatorFeeRefund() external TODO {}

    function test_validatorFeeRefundWhenMultipleCalls() external TODO {}

    function test_validatorFeeRefundWhenNoFees() external TODO {}

    function test_depositFee() external TODO {}
}

contract TwinDeployment is Tester {
    function test_happyPath() external TODO {}
}

contract Relay is Tester {
    function test_happyPath() external TODO {}
}
