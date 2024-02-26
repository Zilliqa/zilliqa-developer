// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";
import {Script} from "forge-std/Script.sol";
import {ChainGateway} from "contracts/core/ChainGateway.sol";
import {ValidatorManager} from "contracts/core/ValidatorManager.sol";

contract Dispatch is Script {
    using MessageHashUtils for bytes;

    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY_TESTNET");
        address chainGatewayAddress = 0x10917A34FE60eE8364a401a6b1d3adaf80D84eb6;

        // Check these fields on every call
        uint sourceChainId = 97;
        address target = 0x1509988c41f02014aA59d455c6a0D67b5b50f129; // Contract to be called EDIT
        // bytes EDIT, make sure to remove 0x
        bytes
            memory call = hex"1a90748a0000000000000000000000000000000000000000000000000000000000000061000000000000000000000000a6d73210af20a59832f264fbd991d2abf28401d0000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000600000000000000000000000008618d39a8276d931603c6bc7306af6a53ad2f1f3000000000000000000000000b494d016f2cf329224e2db445aa748cf96c18c29000000000000000000000000000000000000000000000000000000000000000a";
        uint gasLimit = 1_000_000;
        uint nonce = 2; // uint EDIT

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
