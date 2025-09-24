// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.20;

library ScillaConnector {
    uint private constant CALL_SCILLA_WITH_THE_SAME_SENDER = 1;
    uint private constant SCILLA_CALL_PRECOMPILE_ADDRESS = 0x5a494c53;

    /**
     * @dev Calls BatchBurn on a ZRC6 contract
     * @param target The address of the ZRC6 contract
     * @param tokenIds The list of token IDs to burn
     */
    function callBatchBurn(
        address target,
        uint256[] memory tokenIds
    ) internal {
        // Check that all tokenIds fit in uint128
        for (uint256 i = 0; i < tokenIds.length; i++) {
            require(tokenIds[i] <= type(uint128).max, "Token ID too large");
        }
        bytes memory encodedArgs = abi.encode(
            target,
            "BatchBurn",
            CALL_SCILLA_WITH_THE_SAME_SENDER,
            tokenIds
        );
        uint256 argsLength = encodedArgs.length;

        bool success;
        assembly {
            success := call(
                gas(),
                SCILLA_CALL_PRECOMPILE_ADDRESS,
                0,
                add(encodedArgs, 0x20),
                argsLength,
                0x20,
                0
            )
        }
        require(success);
    }
}
