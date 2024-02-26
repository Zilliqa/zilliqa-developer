// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

interface ITokenManagerFeesEvents {
    event FeesUpdated(uint feesBefore, uint feesAfter);
    event FeesWithdrawn(uint amount);
}

interface ITokenManagerFees is ITokenManagerFeesEvents {
    error InsufficientFees(uint received, uint expected);

    function getFees() external view returns (uint);

    function setFees(uint newFees) external;

    function withdrawFees(address payable to) external;
}

abstract contract TokenManagerFees is ITokenManagerFees {
    /// @custom:storage-location erc7201:zilliqa.storage.TokenManagerFees
    struct TokenManagerFeeStorage {
        uint fees;
    }

    // keccak256(abi.encode(uint256(keccak256("zilliqa.storage.TokenManagerFees")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant Token_Manager_Fees_Storage_Location =
        0x160d773a327af1b4c116f10b1dde265dd32ef2539dc09d78f0d75883ba9f9a00;

    modifier checkFees() {
        uint expectedFees = getFees();
        if (msg.value < expectedFees) {
            revert InsufficientFees(msg.value, expectedFees);
        }
        _;
    }

    function _getTokenManagerFeeStorage()
        private
        pure
        returns (TokenManagerFeeStorage storage $)
    {
        assembly {
            $.slot := Token_Manager_Fees_Storage_Location
        }
    }

    function getFees() public view returns (uint) {
        TokenManagerFeeStorage storage $ = _getTokenManagerFeeStorage();
        return $.fees;
    }

    function _setFees(uint newFees) internal {
        TokenManagerFeeStorage storage $ = _getTokenManagerFeeStorage();
        emit FeesUpdated($.fees, newFees);
        $.fees = newFees;
    }

    function setFees(uint newFees) external virtual;

    function _withdrawFees(address payable to) internal {
        uint amount = address(this).balance;
        emit FeesWithdrawn(amount);
        to.transfer(amount);
    }

    function withdrawFees(address payable to) external virtual;
}
