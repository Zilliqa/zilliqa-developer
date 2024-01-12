// SPDX-License-Identifier: GPL-3.0-or-later

pragma solidity ^0.5.16;

import "./EIP20Interface.sol";
import "./SafeMath.sol";

contract EIP20ZRC2Proxy is EIP20Interface {
    using SafeMath for uint256;

    address public zrc2_address;
    string public symbol;
    string public name;
    uint8 public decimals;

    uint256 private constant _CALL_SCILLA_WITH_THE_SAME_SENDER = 1;
    uint256 private constant _NONEVM_CALL_PRECOMPILE_ADDRESS = 0x5a494c53;
    uint256 private constant _NONEVM_STATE_READ_PRECOMPILE_ADDRESS = 0x5a494c92;

    uint128 private constant _UINT8_MAX = 2 ** 8 - 1;
    uint128 private constant _UINT128_MAX = 2 ** 128 - 1;

    /**
     * @notice Constructs a new EIP20ZRC2Proxy contract
     * @param _zrc2_address The address of the underlying ZRC2 contract
     */
    constructor(address _zrc2_address) public {
        zrc2_address = _zrc2_address;

        symbol = _read_scilla_string("symbol");
        decimals = safeTrimFromUint32toUint8(_read_scilla_uint32("decimals"));
        name = _read_scilla_string("name");
    }

    /**
     * @notice Get the total supply of tokens
     * @return The total supply of tokens
     */
    function totalSupply() external view returns (uint256) {
        return _read_scilla_uint128("total_supply");
    }

    /**
     * @notice Get the token balance for a specific account
     * @param tokenOwner The address of the account
     * @return The balance of the account
     */
    function balanceOf(address tokenOwner) external view returns (uint256) {
        return _read_scilla_map_uint128("balances", tokenOwner);
    }

    /**
     * @notice Transfer tokens to a specified address
     * @param to The address to transfer to
     * @param tokens The amount of tokens to transfer
     * @return true if transfer was successful
     */
    function transfer(address to, uint256 tokens) external returns (bool) {
        _call_scilla_with_address_uint_args(
            "Transfer",
            to,
            safeTrimFromUint256ToUint128(tokens)
        );
        return true;
    }

    /**
     * @notice Transfer tokens from one address to another
     * @param from The address to transfer from
     * @param to The address to transfer to
     * @param tokens The amount of tokens to transfer
     * @return true if transfer was successful
     */
    function transferFrom(
        address from,
        address to,
        uint256 tokens
    ) external returns (bool) {
        _call_scilla_with_address_address_uint_args(
            "TransferFrom",
            from,
            to,
            safeTrimFromUint256ToUint128(tokens)
        );
        return true;
    }

    /**
     * @notice Check the amount of tokens that an owner has allowed a spender to use
     * @param tokenOwner The address of the token owner
     * @param spender The address of the spender
     * @return The amount of tokens remaining for the spender
     */
    function allowance(
        address tokenOwner,
        address spender
    ) external view returns (uint256) {
        return
            _read_scilla_nested_map_uint128("allowances", tokenOwner, spender);
    }

    /**
     * @notice Approve a spender to spend a certain amount of tokens
     * @param spender The address of the spender
     * @param new_allowance The new allowance for the spender
     * @return true if approval was successful
     */
    function approve(
        address spender,
        uint256 new_allowance
    ) external returns (bool) {
        uint256 current_allowance = this.allowance(msg.sender, spender);

        if (current_allowance >= new_allowance) {
            _call_scilla_with_address_uint_args(
                "DecreaseAllowance",
                spender,
                safeTrimFromUint256ToUint128(
                    current_allowance.sub(new_allowance)
                )
            );
        } else {
            _call_scilla_with_address_uint_args(
                "IncreaseAllowance",
                spender,
                safeTrimFromUint256ToUint128(
                    new_allowance.sub(current_allowance)
                )
            );
        }
        return true;
    }

    /**
     * @dev Asserts that a value is a valid 128 bit integer
     * @param value The value to be checked
     * @return The original value cast to a uint128
     */
    function safeTrimFromUint256ToUint128(
        uint256 value
    ) private pure returns (uint128) {
        require(value <= _UINT128_MAX, "value greater than uint128 max value");
        return uint128(value);
    }

    /**
     * @dev Asserts that a value is a valid 128 bit integer
     * @param value The value to be checked
     * @return The original value cast to a uint128
     */
    function safeTrimFromUint32toUint8(
        uint32 value
    ) private pure returns (uint8) {
        require(value <= _UINT8_MAX, "value greater than uint8 max value");
        return uint8(value);
    }

    // Private functions used for accessing ZRC2 contract

    /**
     * @dev Calls a ZRC2 contract function with two arguments
     * @param tran_name The name of the function to call
     * @param addressArg The first argument to the function
     * @param uintArg The second argument to the function
     */
    function _call_scilla_with_address_uint_args(
        string memory tran_name,
        address addressArg,
        uint128 uintArg
    ) private {
        bytes memory encodedArgs = abi.encode(
            zrc2_address,
            tran_name,
            _CALL_SCILLA_WITH_THE_SAME_SENDER,
            addressArg,
            uintArg
        );
        uint256 argsLength = encodedArgs.length;

        assembly {
            let alwaysSuccessForThisPrecompile := call(
                21000,
                _NONEVM_CALL_PRECOMPILE_ADDRESS,
                0,
                add(encodedArgs, 0x20),
                argsLength,
                0x20,
                0
            )
        }
    }

    /**
     * @dev Calls a ZRC2 contract function with three arguments
     * @param tran_name The name of the function to call on the ZRC2 contract
     * @param address1Arg The first argument to the function
     * @param address2Arg The second argument to the function
     * @param uintArg The third argument to the function
     */
    function _call_scilla_with_address_address_uint_args(
        string memory tran_name,
        address address1Arg,
        address address2Arg,
        uint128 uintArg
    ) private {
        bytes memory encodedArgs = abi.encode(
            zrc2_address,
            tran_name,
            _CALL_SCILLA_WITH_THE_SAME_SENDER,
            address1Arg,
            address2Arg,
            uintArg
        );
        uint256 argsLength = encodedArgs.length;

        assembly {
            let alwaysSuccessForThisPrecompile := call(
                21000,
                _NONEVM_CALL_PRECOMPILE_ADDRESS,
                0,
                add(encodedArgs, 0x20),
                argsLength,
                0x20,
                0
            )
        }
    }

    /**
     * @dev Reads a 128 bit integer from a ZRC2 contract
     * @param variable_name The name of the variable to read from the ZRC2 contract
     * @return The value of the variable
     */
    function _read_scilla_uint128(
        string memory variable_name
    ) private view returns (uint128) {
        bytes memory encodedArgs = abi.encode(zrc2_address, variable_name);
        uint256 argsLength = encodedArgs.length;
        bytes memory output = new bytes(36);

        assembly {
            let alwaysSuccessForThisPrecompile := staticcall(
                21000,
                _NONEVM_STATE_READ_PRECOMPILE_ADDRESS,
                add(encodedArgs, 0x20),
                argsLength,
                add(output, 0x20),
                32
            )
        }

        return abi.decode(output, (uint128));
    }

    /**
     * @dev Reads a 32 bit integer from a ZRC2 contract
     * @param variable_name The name of the variable to read from the ZRC2 contract
     * @return The value of the variable
     */
    function _read_scilla_uint32(
        string memory variable_name
    ) private view returns (uint32) {
        bytes memory encodedArgs = abi.encode(zrc2_address, variable_name);
        uint256 argsLength = encodedArgs.length;
        bytes memory output = new bytes(36);

        assembly {
            let alwaysSuccessForThisPrecompile := staticcall(
                21000,
                _NONEVM_STATE_READ_PRECOMPILE_ADDRESS,
                add(encodedArgs, 0x20),
                argsLength,
                add(output, 0x20),
                32
            )
        }

        return abi.decode(output, (uint32));
    }

    /**
     * @dev Reads a string from a ZRC2 contract
     * @param variable_name The name of the variable to read from the ZRC2 contract
     * @return The value of the variable
     */
    function _read_scilla_string(
        string memory variable_name
    ) public view returns (string memory retVal) {
        bytes memory encodedArgs = abi.encode(zrc2_address, variable_name);
        uint256 argsLength = encodedArgs.length;
        bool success;
        bytes memory output = new bytes(128);
        uint256 output_len = output.length - 4;
        assembly {
            success := staticcall(
                21000,
                _NONEVM_STATE_READ_PRECOMPILE_ADDRESS,
                add(encodedArgs, 0x20),
                argsLength,
                add(output, 0x20),
                output_len
            )
        }
        require(success);

        (retVal) = abi.decode(output, (string));
        return retVal;
    }

    /**
     * @dev Reads a 128 bit integer from a map in a ZRC2 contract
     * @param variable_name The name of the map in the ZRC2 contract
     * @param addressMapKey The key to the map
     * @return The value associated with the key in the map
     */
    function _read_scilla_map_uint128(
        string memory variable_name,
        address addressMapKey
    ) private view returns (uint128) {
        bytes memory encodedArgs = abi.encode(
            zrc2_address,
            variable_name,
            addressMapKey
        );
        uint256 argsLength = encodedArgs.length;
        bytes memory output = new bytes(36);

        assembly {
            let alwaysSuccessForThisPrecompile := staticcall(
                21000,
                _NONEVM_STATE_READ_PRECOMPILE_ADDRESS,
                add(encodedArgs, 0x20),
                argsLength,
                add(output, 0x20),
                32
            )
        }

        return abi.decode(output, (uint128));
    }

    /**
     * @dev Reads a 128 bit integer from a nested map in a ZRC2 contract
     * @param variable_name The name of the map in the ZRC2 contract
     * @param firstMapKey The first key to the map
     * @param secondMapKey The second key to the map
     * @return The value associated with the keys in the map
     */
    function _read_scilla_nested_map_uint128(
        string memory variable_name,
        address firstMapKey,
        address secondMapKey
    ) private view returns (uint128) {
        bytes memory encodedArgs = abi.encode(
            zrc2_address,
            variable_name,
            firstMapKey,
            secondMapKey
        );
        uint256 argsLength = encodedArgs.length;
        bytes memory output = new bytes(36);

        assembly {
            let alwaysSuccessForThisPrecompile := staticcall(
                21000,
                _NONEVM_STATE_READ_PRECOMPILE_ADDRESS,
                add(encodedArgs, 0x20),
                argsLength,
                add(output, 0x20),
                32
            )
        }

        return abi.decode(output, (uint128));
    }
}
