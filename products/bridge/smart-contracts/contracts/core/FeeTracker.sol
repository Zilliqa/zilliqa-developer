// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";

interface IFeeTrackerErrors {
    error InsufficientMinFeeDeposit();
}

interface IFeeTracker is IFeeTrackerErrors {
    function feeDeposit(address sponsor) external returns (uint);

    function feeRefund(address caller) external returns (uint);

    function depositFee() external payable;

    function withdrawFee(uint amount) external;

    function refundFee() external;
}

abstract contract FeeTracker is IFeeTracker {
    mapping(address => uint) public feeDeposit;
    mapping(address => uint) public feeRefund;

    modifier meterFee(address sponsor) {
        uint feeStart = gasleft() * tx.gasprice;
        // 44703 = 21000 + 3 + 6600 + 17100
        // 17100 = init storage cost (worst case)
        // 6600 = operations related to gas tracking
        // 21000 = fixed cost of transaction
        uint feeOffset = (44703 + 16 * (msg.data.length - 4)) * tx.gasprice;
        // Should reject if insuficient to pay for the offset
        if (feeDeposit[sponsor] < feeOffset) {
            revert InsufficientMinFeeDeposit();
        }
        feeStart += feeOffset;
        // It will still take fees even if insufficient fee deposit is provided
        if (feeDeposit[sponsor] >= feeStart) {
            _;
        }
        uint spent = feeStart - gasleft() * tx.gasprice;
        feeDeposit[sponsor] -= spent;
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
