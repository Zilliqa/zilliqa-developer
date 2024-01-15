// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";
import {Script} from "forge-std/Script.sol";
import {Relayer, CallMetadata} from "contracts/core/Relayer.sol";
import {ITokenManager, AcceptArgs} from "contracts/periphery/TokenManagerUpgradeable.sol";
import {ValidatorManager} from "contracts/core/ValidatorManager.sol";
import {ChainGateway} from "contracts/core/ChainGateway.sol";
import "forge-std/console.sol";

contract Dispatch is Script {
    using MessageHashUtils for bytes;

    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY_TEST");
        address chainGatewayAddress = 0x5cE584e24f6703f3197Ca83d442807cB82474f8D;

        // Check these fields on every call
        uint sourceChainId = 33101;
        address target = 0xd10077bCE4A9D19068965dE519CED8a2fC1B096C; // Contract to be called EDIT
        // bytes EDIT, make sure to remove 0x
        bytes
            memory call = hex"1a90748a000000000000000000000000000000000000000000000000000000000000814d000000000000000000000000d10077bce4a9d19068965de519ced8a2fc1b096c000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000600000000000000000000000006d78c86d66dfe5be5f55fbaa8b1d3fd28edff396000000000000000000000000b494d016f2cf329224e2db445aa748cf96c18c2900000000000000000000000000000000000000000000000000005af3107a4000";
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
