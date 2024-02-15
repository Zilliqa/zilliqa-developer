// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

interface IFeeTrackerErrors {
    error InsufficientMinFeeDeposit();
}

abstract contract FeeTracker is IFeeTrackerErrors {
    mapping(address => uint) public feeDeposit;
    mapping(address => uint) public feeRefund;

    modifier meterFee(address patron) {
        uint feeStart = gasleft() * tx.gasprice;
        // 44703 = 21000 + 3 + 6600 + 17100
        // 17100 = init storage cost (worst case)
        // 6600 = operations related to gas tracking
        // 21000 = fixed cost of transaction
        uint feeOffset = (44703 + 16 * (msg.data.length - 4)) * tx.gasprice;
        // Should reject if insuficient to pay for the offset
        if (feeDeposit[patron] < feeOffset) {
            revert InsufficientMinFeeDeposit();
        }
        feeStart += feeOffset;
        // It will still take fees even if insufficient fee deposit is provided
        if (feeDeposit[patron] >= feeStart) {
            _;
        }
        uint spent = feeStart - gasleft() * tx.gasprice;
        feeDeposit[patron] -= spent;
        feeRefund[msg.sender] += spent;
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
}
