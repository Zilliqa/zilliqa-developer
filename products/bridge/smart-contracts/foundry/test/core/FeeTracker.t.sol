// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity 0.8.20;

import {TransferReentrancyTester} from "foundry/test/Helpers.sol";
import {FeeTracker, IFeeTrackerErrors} from "contracts/core/FeeTracker.sol";
import {stdStorage, StdStorage} from "forge-std/Test.sol";
import {Tester} from "foundry/test/Tester.sol";

contract FeeTrackerTester is FeeTracker {
    bool public writeSuccessful;
    address public sponsor;

    function setSponsor(address _sponsor) external {
        sponsor = _sponsor;
    }

    function write(address _sponsor) external meterFee(_sponsor) {
        writeSuccessful = true;
    }

    function write() external meterFee(sponsor) {
        writeSuccessful = true;
    }

    function write(
        address _sponsor,
        bytes[] calldata data
    ) external meterFee(_sponsor) {
        data;
        writeSuccessful = true;
    }

    function write(
        address _sponsor,
        uint[] calldata data
    ) external meterFee(_sponsor) {
        data;
        writeSuccessful = true;
    }
}

contract FeeTrackerTests is Tester {
    using stdStorage for StdStorage;

    FeeTrackerTester feeTrackerTester;
    address sponsor1 = vm.addr(1000);
    address sender1 = vm.addr(2000);

    function setUp() external {
        feeTrackerTester = new FeeTrackerTester();
    }

    function verifyFeeInvariant(
        uint initialFeeDeposit,
        uint gasSpent
    ) internal {
        // feeDeposit + feeRefund = initial deposit
        assertEq(
            feeTrackerTester.feeDeposit(sponsor1),
            initialFeeDeposit - feeTrackerTester.feeRefund(sender1),
            "Invariant violated: feeDeposit + feeRefund = initial deposit"
        );
        assertGe(
            feeTrackerTester.feeRefund(sender1),
            gasSpent,
            "Invariant violated: Sender should be refunded more than the gas spent"
        );
    }

    function test_meterFee_happyPath() external {
        vm.txGasPrice(10 gwei);
        uint initialFeeDeposit = 100_000 ether;

        hoax(sponsor1);
        feeTrackerTester.depositFee{value: initialFeeDeposit}();

        vm.prank(sender1);
        uint gasStart = gasleft();
        feeTrackerTester.write{gas: 100_000}(sponsor1);
        uint feeSpent = (gasStart - gasleft()) * tx.gasprice;

        verifyFeeInvariant(initialFeeDeposit, feeSpent);
        assertTrue(feeTrackerTester.writeSuccessful());
    }

    function testRevert_meterFee_insufficientMinFeeDeposit() external {
        vm.txGasPrice(10 gwei);
        uint initialFeeDeposit = 1 gwei;

        hoax(sponsor1);
        feeTrackerTester.depositFee{value: initialFeeDeposit}();

        vm.prank(sender1);
        vm.expectRevert(IFeeTrackerErrors.InsufficientMinFeeDeposit.selector);
        feeTrackerTester.write{gas: 1_000_000}(sponsor1);

        assertFalse(feeTrackerTester.writeSuccessful());
        assertEq(
            feeTrackerTester.feeDeposit(sponsor1),
            initialFeeDeposit,
            "Sender should not make transaction when insufficient fee deposit is provided"
        );
    }

    function test_meterFee_insufficientFeeDeposit() external {
        vm.txGasPrice(10 gwei);
        uint initialFeeDeposit = 9_000_000 gwei;

        hoax(sponsor1);
        feeTrackerTester.depositFee{value: initialFeeDeposit}();

        vm.prank(sender1);
        uint gasStart = gasleft();
        feeTrackerTester.write{gas: 1_000_000}(sponsor1);
        uint feeSpent = (gasStart - gasleft()) * tx.gasprice;

        assertFalse(feeTrackerTester.writeSuccessful());
        verifyFeeInvariant(initialFeeDeposit, feeSpent);
    }

    function test_meterFee_enoughRefundWhenNoArgumentCall() external {
        vm.txGasPrice(10 gwei);
        feeTrackerTester.setSponsor(sponsor1);

        uint initialFeeDeposit = 1 ether;
        hoax(sponsor1);
        feeTrackerTester.depositFee{value: initialFeeDeposit}();

        vm.prank(sender1);
        uint gasStart = gasleft();
        feeTrackerTester.write{gas: 1_000_000}();
        uint feeSpent = (gasStart - gasleft()) * tx.gasprice;

        verifyFeeInvariant(initialFeeDeposit, feeSpent);
        assertTrue(feeTrackerTester.writeSuccessful());
    }

    function test_meterFee_enoughRefundWhenManyZeroArgumentCall() external {
        vm.txGasPrice(10 gwei);
        bytes[] memory data = new bytes[](100);

        uint initialFeeDeposit = 1 ether;
        hoax(sponsor1);
        feeTrackerTester.depositFee{value: initialFeeDeposit}();

        vm.prank(sender1);
        uint gasStart = gasleft();
        feeTrackerTester.write{gas: 1_000_000}(sponsor1, data);
        uint feeSpent = (gasStart - gasleft()) * tx.gasprice;

        verifyFeeInvariant(initialFeeDeposit, feeSpent);
        assertTrue(feeTrackerTester.writeSuccessful());
    }

    function test_meterFee_enoughRefundWhenManyNonzeroArgumentCall() external {
        vm.txGasPrice(10 gwei);
        uint[] memory data = new uint[](100);
        for (uint i = 0; i < data.length; ++i) {
            data[i] = type(uint).max;
        }

        uint initialFeeDeposit = 1 ether;
        hoax(sponsor1);
        feeTrackerTester.depositFee{value: initialFeeDeposit}();

        vm.prank(sender1);
        uint gasStart = gasleft();
        feeTrackerTester.write{gas: 1_000_000}(sponsor1, data);
        uint feeSpent = (gasStart - gasleft()) * tx.gasprice;

        verifyFeeInvariant(initialFeeDeposit, feeSpent);
        assertTrue(feeTrackerTester.writeSuccessful());
    }

    function test_meterFee_consecutiveCalls() external {
        vm.txGasPrice(10 gwei);
        uint[] memory data = new uint[](100);
        for (uint i = 0; i < data.length; ++i) {
            data[i] = type(uint).max;
        }

        uint initialFeeDeposit = 1 ether;
        hoax(sponsor1);
        feeTrackerTester.depositFee{value: initialFeeDeposit}();

        // First Call
        vm.prank(sender1);
        uint gasStart1 = gasleft();
        feeTrackerTester.write{gas: 1_000_000}(sponsor1, data);
        uint feeSpent1 = (gasStart1 - gasleft()) * tx.gasprice;

        verifyFeeInvariant(initialFeeDeposit, feeSpent1);
        assertTrue(feeTrackerTester.writeSuccessful());

        // Second Call

        vm.prank(sender1);
        uint gasStart2 = gasleft();
        feeTrackerTester.write{gas: 1_000_000}(sponsor1, data);
        uint feeSpent2 = (gasStart2 - gasleft()) * tx.gasprice;

        verifyFeeInvariant(initialFeeDeposit, feeSpent1 + feeSpent2);
        assertTrue(feeTrackerTester.writeSuccessful());
    }

    function testFuzz_meterFee_varyGasPrice(uint gasPrice) external {
        vm.txGasPrice(gasPrice % 1_000 gwei);

        uint initialFeeDeposit = 1_000 ether;
        hoax(sponsor1);
        feeTrackerTester.depositFee{value: initialFeeDeposit}();

        vm.prank(sender1);
        uint gasStart = gasleft();
        feeTrackerTester.write{gas: 1_000_000}(sponsor1);
        uint feeSpent = (gasStart - gasleft()) * tx.gasprice;

        verifyFeeInvariant(initialFeeDeposit, feeSpent);
        assertTrue(feeTrackerTester.writeSuccessful());
    }

    function test_refundFee_happyPath() external {
        uint amount = 1_000 gwei;
        vm.deal(address(sponsor1), amount);
        vm.deal(address(feeTrackerTester), amount);

        stdstore
            .target(address(feeTrackerTester))
            .sig(feeTrackerTester.feeRefund.selector)
            .with_key(sender1)
            .checked_write(amount);

        vm.prank(sender1);
        feeTrackerTester.refundFee();
        assertEq(feeTrackerTester.feeRefund(sender1), 0);
        assertEq(sender1.balance, amount);
        assertEq(address(feeTrackerTester).balance, 0);
    }

    function test_refundFee_reentrancy() external {
        TransferReentrancyTester reentrancyTester = new TransferReentrancyTester();
        uint amount = 1 ether;
        vm.deal(address(feeTrackerTester), amount * 2);

        bytes memory data = abi.encodeWithSelector(
            feeTrackerTester.refundFee.selector
        );

        stdstore
            .target(address(feeTrackerTester))
            .sig(feeTrackerTester.feeRefund.selector)
            .with_key(address(reentrancyTester))
            .checked_write(amount);

        // Expect revert on attempt to reenter: out of gas
        bool success = reentrancyTester.reentrancyAttack(
            address(feeTrackerTester),
            data
        );
        assertFalse(success, "Reentrancy should have failed");
        // No state should be changed
        assertEq(address(feeTrackerTester).balance, amount * 2);
        assertEq(address(reentrancyTester).balance, 0);
        assertEq(feeTrackerTester.feeRefund(address(reentrancyTester)), amount);
    }

    function test_depositFee_happyPath() external {
        uint amount = 1_000 gwei;

        hoax(sender1, amount);
        feeTrackerTester.depositFee{value: amount}();

        assertEq(feeTrackerTester.feeDeposit(sender1), amount);
        assertEq(address(feeTrackerTester).balance, amount);
        assertEq(sender1.balance, 0);
    }

    function test_withdrawFee_happyPath() external {
        uint amount = 1_000 gwei;
        startHoax(sender1, amount);

        feeTrackerTester.depositFee{value: amount}();
        assertEq(sender1.balance, 0);

        feeTrackerTester.withdrawFee(amount);
        assertEq(feeTrackerTester.feeDeposit(sender1), 0);
        assertEq(sender1.balance, amount);
        assertEq(address(feeTrackerTester).balance, 0);
    }

    function test_withdrawFee_reentrancy() external {
        TransferReentrancyTester reentrancyTester = new TransferReentrancyTester();
        uint amount = 1 ether;
        vm.deal(address(feeTrackerTester), amount);

        bytes memory data = abi.encodeWithSelector(
            feeTrackerTester.withdrawFee.selector
        );

        hoax(address(reentrancyTester), amount);
        feeTrackerTester.depositFee{value: amount}();

        // Expect revert on attempt to reenter: out of gas
        bool success = reentrancyTester.reentrancyAttack(
            address(feeTrackerTester),
            data
        );
        assertFalse(success, "Reentrancy should have failed");
        // No state should be changed
        assertEq(address(feeTrackerTester).balance, amount * 2);
        assertEq(address(reentrancyTester).balance, 0);
        assertEq(
            feeTrackerTester.feeDeposit(address(reentrancyTester)),
            amount
        );
    }
}
