// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity 0.8.20;

import {RelayerTestFixture} from "./Helpers.sol";

contract Fees is RelayerTestFixture {
    function test_feesRefundedToValidator() external TODO {
        // feeDeposit + feeRefund = initial deposit
    }

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
