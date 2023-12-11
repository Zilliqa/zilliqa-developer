// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {RelayerTestFixture, RelayerHarness, ValidatorManager, Test, TransferReentrancyTester} from "./Helpers.sol";
import {IRelayerErrors} from "contracts/Relayer.sol";

contract RelayerFeeTester is RelayerHarness {
    bool public writeSuccessful;
    address public sponsor;

    constructor(
        ValidatorManager validatorManger
    ) RelayerHarness(validatorManager) {}

    function test(address _sponsor) external meterFee(_sponsor) {
        writeSuccessful = true;
    }

    function setSponsor(address _sponsor) external {
        sponsor = _sponsor;
    }

    function test() external meterFee(sponsor) {
        writeSuccessful = true;
    }

    function test(
        address _sponsor,
        bytes[] calldata data
    ) external meterFee(_sponsor) {
        data;
        writeSuccessful = true;
    }

    function test(
        address _sponsor,
        uint[] calldata data
    ) external meterFee(_sponsor) {
        data;
        writeSuccessful = true;
    }
}

contract Fees is RelayerTestFixture {
    using stdStorage for StdStorage;

    RelayerFeeTester relayerTester;
    address sponsor1 = vm.addr(1000);
    address sender1 = vm.addr(2000);

    function setUp() public {
        relayerTester = new RelayerFeeTester(validatorManager);
    }

    function verifyFeeInvariant(
        uint initialFeeDeposit,
        uint gasSpent
    ) internal {
        // feeDeposit + feeRefund = initial deposit
        assertEq(
            relayerTester.feeDeposit(sponsor1),
            initialFeeDeposit - relayerTester.feeRefund(sender1),
            "Invariant violated: feeDeposit + feeRefund = initial deposit"
        );
        assertGe(
            relayerTester.feeRefund(sender1),
            gasSpent,
            "Invariant violated: Sender should be refunded more than the gas spent"
        );
    }

    function test_meterFee_happyPath() external {
        vm.txGasPrice(10 gwei);
        uint initialFeeDeposit = 100000 ether;

        hoax(sponsor1);
        relayerTester.depositFee{value: initialFeeDeposit}();

        vm.prank(sender1);
        uint gasStart = gasleft();
        relayerTester.test{gas: 100_000}(sponsor1);
        uint feeSpent = (gasStart - gasleft()) * tx.gasprice;

        relayerTester.verifyFeeInvariant(
            initialFeeDeposit,
            feeSpent,
            sponsor1,
            sender1
        );
        assertTrue(relayerTester.writeSuccessful());
    }

    function testRevert_meterFee_insufficientMinFeeDeposit() external {
        vm.txGasPrice(10 gwei);
        uint initialFeeDeposit = 1 gwei;

        hoax(sponsor1);
        relayerTester.depositFee{value: initialFeeDeposit}();

        vm.prank(sender1);
        vm.expectRevert(IRelayerErrors.InsufficientMinFeeDeposit.selector);
        relayerTester.test{gas: 1_000_000}(sponsor1);

        assertFalse(relayerTester.writeSuccessful());
        assertEq(
            relayerTester.feeDeposit(sponsor1),
            initialFeeDeposit,
            "Sender should not make transaction when insufficient fee deposit is provided"
        );
    }

    function test_meterFee_insufficientFeeDeposit() external {
        vm.txGasPrice(10 gwei);
        uint initialFeeDeposit = 9_000_000 gwei;

        hoax(sponsor1);
        relayerTester.depositFee{value: initialFeeDeposit}();

        vm.prank(sender1);
        uint gasStart = gasleft();
        relayerTester.test{gas: 1_000_000}(sponsor1);
        uint feeSpent = (gasStart - gasleft()) * tx.gasprice;

        assertFalse(relayerTester.writeSuccessful());
        relayerTester.verifyFeeInvariant(
            initialFeeDeposit,
            feeSpent,
            sponsor1,
            sender1
        );
    }

    function test_meterFee_enoughRefundWhenNoArgumentCall() external {
        vm.txGasPrice(10 gwei);
        relayerTester.setSponsor(sponsor1);

        uint initialFeeDeposit = 1 ether;
        hoax(sponsor1);
        relayerTester.depositFee{value: initialFeeDeposit}();

        vm.prank(sender1);
        uint gasStart = gasleft();
        relayerTester.test{gas: 1_000_000}();
        uint feeSpent = (gasStart - gasleft()) * tx.gasprice;

        relayerTester.verifyFeeInvariant(
            initialFeeDeposit,
            feeSpent,
            sponsor1,
            sender1
        );
        assertTrue(relayerTester.writeSuccessful());
    }

    function test_meterFee_enoughRefundWhenManyZeroArgumentCall() external {
        vm.txGasPrice(10 gwei);
        bytes[] memory data = new bytes[](100);

        uint initialFeeDeposit = 1 ether;
        hoax(sponsor1);
        relayerTester.depositFee{value: initialFeeDeposit}();

        vm.prank(sender1);
        uint gasStart = gasleft();
        relayerTester.test{gas: 1_000_000}(sponsor1, data);
        uint feeSpent = (gasStart - gasleft()) * tx.gasprice;

        relayerTester.verifyFeeInvariant(
            initialFeeDeposit,
            feeSpent,
            sponsor1,
            sender1
        );
        assertTrue(relayerTester.writeSuccessful());
    }

    function test_meterFee_enoughRefundWhenManyNonzeroArgumentCall() external {
        vm.txGasPrice(10 gwei);
        uint[] memory data = new uint[](100);
        for (uint i = 0; i < data.length; ++i) {
            data[i] = type(uint).max;
        }

        uint initialFeeDeposit = 1 ether;
        hoax(sponsor1);
        relayerTester.depositFee{value: initialFeeDeposit}();

        vm.prank(sender1);
        uint gasStart = gasleft();
        relayerTester.test{gas: 1_000_000}(sponsor1, data);
        uint feeSpent = (gasStart - gasleft()) * tx.gasprice;

        relayerTester.verifyFeeInvariant(
            initialFeeDeposit,
            feeSpent,
            sponsor1,
            sender1
        );
        assertTrue(relayerTester.writeSuccessful());
    }

    function test_meterFee_consecutiveCalls() external {
        vm.txGasPrice(10 gwei);
        uint[] memory data = new uint[](100);
        for (uint i = 0; i < data.length; ++i) {
            data[i] = type(uint).max;
        }

        uint initialFeeDeposit = 1 ether;
        hoax(sponsor1);
        relayerTester.depositFee{value: initialFeeDeposit}();

        // First Call
        vm.prank(sender1);
        uint gasStart1 = gasleft();
        relayerTester.test{gas: 1_000_000}(sponsor1, data);
        uint feeSpent1 = (gasStart1 - gasleft()) * tx.gasprice;

        relayerTester.verifyFeeInvariant(
            initialFeeDeposit,
            feeSpent1,
            sponsor1,
            sender1
        );
        assertTrue(relayerTester.writeSuccessful());

        // Second Call

        vm.prank(sender1);
        uint gasStart2 = gasleft();
        relayerTester.test{gas: 1_000_000}(sponsor1, data);
        uint feeSpent2 = (gasStart2 - gasleft()) * tx.gasprice;

        relayerTester.verifyFeeInvariant(
            initialFeeDeposit,
            feeSpent1 + feeSpent2,
            sponsor1,
            sender1
        );
        assertTrue(relayerTester.writeSuccessful());
    }

    function testFuzz_meterFee_varyGasPrice(uint gasPrice) external {
        vm.txGasPrice(gasPrice % 1_000 gwei);

        uint initialFeeDeposit = 1_000 ether;
        hoax(sponsor1);
        relayerTester.depositFee{value: initialFeeDeposit}();

        vm.prank(sender1);
        uint gasStart = gasleft();
        relayerTester.test{gas: 1_000_000}(sponsor1);
        uint feeSpent = (gasStart - gasleft()) * tx.gasprice;

        relayerTester.verifyFeeInvariant(
            initialFeeDeposit,
            feeSpent,
            sponsor1,
            sender1
        );
        assertTrue(relayerTester.writeSuccessful());
    }

    function test_refundFee_happyPath() external {
        uint amount = 1000 gwei;
        vm.deal(address(relayerTester), amount);

        stdstore
            .target(address(relayerTester))
            .sig(relayerTester.feeRefund.selector)
            .with_key(sender1)
            .checked_write(amount);

        vm.prank(sender1);
        relayerTester.refundFee();
        assertEq(relayerTester.feeRefund(sender1), 0);
        assertEq(sender1.balance, amount);
        assertEq(address(relayerTester).balance, 0);
    }

    function test_refundFee_reentrancy() external {
        TransferReentrancyTester reentrancyTester = new TransferReentrancyTester();
        uint amount = 1 ether;
        vm.deal(address(relayerTester), amount * 2);

        bytes memory data = abi.encodeWithSelector(
            relayerTester.refundFee.selector
        );

        stdstore
            .target(address(relayerTester))
            .sig(relayerTester.feeRefund.selector)
            .with_key(address(reentrancyTester))
            .checked_write(amount);

        // Expect revert on attempt to reenter: out of gas
        bool success = reentrancyTester.testVulnerability(
            address(relayerTester),
            data
        );
        assertFalse(success, "Reentrancy should have failed");
        // No state should be changed
        assertEq(address(relayerTester).balance, amount * 2);
        assertEq(address(reentrancyTester).balance, 0);
        assertEq(relayerTester.feeRefund(address(reentrancyTester)), amount);
    }

    function test_depositFee_happyPath() external {
        uint amount = 1000 gwei;

        hoax(sender1, amount);
        relayerTester.depositFee{value: amount}();

        assertEq(relayerTester.feeDeposit(sender1), amount);
        assertEq(address(relayerTester).balance, amount);
        assertEq(sender1.balance, 0);
    }

    function test_withdrawFee_happyPath() external {
        uint amount = 1000 gwei;
        startHoax(sender1, amount);

        relayerTester.depositFee{value: amount}();
        assertEq(sender1.balance, 0);

        relayerTester.withdrawFee(amount);
        assertEq(relayerTester.feeDeposit(sender1), 0);
        assertEq(sender1.balance, amount);
        assertEq(address(relayerTester).balance, 0);
    }

    function test_withdrawFee_reentrancy() external {
        TransferReentrancyTester reentrancyTester = new TransferReentrancyTester();
        uint amount = 1 ether;
        vm.deal(address(relayerTester), amount);

        bytes memory data = abi.encodeWithSelector(
            relayerTester.withdrawFee.selector
        );

        hoax(address(reentrancyTester), amount);
        relayerTester.depositFee{value: amount}();

        // Expect revert on attempt to reenter: out of gas
        bool success = reentrancyTester.testVulnerability(
            address(relayerTester),
            data
        );
        assertFalse(success, "Reentrancy should have failed");
        // No state should be changed
        assertEq(address(relayerTester).balance, amount * 2);
        assertEq(address(reentrancyTester).balance, 0);
        assertEq(relayerTester.feeDeposit(address(reentrancyTester)), amount);
    }
}
