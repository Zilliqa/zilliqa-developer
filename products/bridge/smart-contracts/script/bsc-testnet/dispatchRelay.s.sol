// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";
import {Script} from "forge-std/Script.sol";
import {ChainGateway} from "contracts/core/ChainGateway.sol";

contract Dispatch is Script {
    using MessageHashUtils for bytes;

    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY_TEST");
        address chainGatewayAddress = 0x2114e979b7CFDd8b358502e00f50Fd5f7787Fe63;

        // Check these fields on every call
        uint sourceChainId = 32769;
        address target = 0xF391A1Ee7b3ccad9a9451D2B7460Ac646F899f23; // Contract to be called EDIT
        // bytes EDIT, make sure to remove 0x
        bytes
            memory call = hex"1a90748a00000000000000000000000000000000000000000000000000000000000080010000000000000000000000006d61efb60c17979816e4ce12cd5d29054e75594800000000000000000000000000000000000000000000000000000000000000600000000000000000000000000000000000000000000000000000000000000060000000000000000000000000351da1e7500aba1d168b9435dce73415718d212f000000000000000000000000b34b88220fa1eaedba5d50b271af8c3ee14a24dd000000000000000000000000000000000000000000000001cc88cdada7568000";
        uint gasLimit = 1_000_000;
        uint nonce = 1; // uint EDIT

        bytes32 hashMessage = abi
            .encode(sourceChainId, block.chainid, target, call, gasLimit, nonce)
            .toEthSignedMessageHash();
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(
            deployerPrivateKey,
            hashMessage
        );
        bytes[] memory signatures = new bytes[](1);
        signatures[0] = abi.encodePacked(r, s, v);

        vm.startBroadcast(deployerPrivateKey);

        ChainGateway(chainGatewayAddress).dispatch(
            sourceChainId,
            target,
            call,
            gasLimit,
            nonce,
            signatures
        );

        vm.stopBroadcast();
    }
}
