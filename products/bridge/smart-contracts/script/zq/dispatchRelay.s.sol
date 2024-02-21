// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";
import {Script} from "forge-std/Script.sol";
import {IChainGateway} from "contracts/core-upgradeable/ChainGatewayUpgradeable.sol";

contract Dispatch is Script {
    using MessageHashUtils for bytes;

    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY_VALIDATOR");
        address chainGatewayAddress = 0xbA44BC29371E19117DA666B729A1c6e1b35DDb40;

        /* Parameters gathered from the Relayed event */
        // Likely fixed. Only 2 chains currently
        uint sourceChainId = 56;
        // Likely fixed. Only targets tokenManagers
        address target = 0x6D61eFb60C17979816E4cE12CD5D29054E755948;
        // EDIT: calldata. ensure to remove 0x. Will change on every relay
        bytes
            memory call = hex"1a90748a0000000000000000000000000000000000000000000000000000000000000038000000000000000000000000f391a1ee7b3ccad9a9451d2b7460ac646f899f2300000000000000000000000000000000000000000000000000000000000000600000000000000000000000000000000000000000000000000000000000000060000000000000000000000000241c677d9969419800402521ae87c411897a029f0000000000000000000000005807b970db344d9d2386bbf5c3ec4cda5ccdf1c8000000000000000000000000000000000000000000000000000000e8d4a51000";
        // Fixed for all x-brige txns
        uint gasLimit = 1_000_000;
        // EDIT: nonce. Will change on every relay
        uint nonce = 0;

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

        IChainGateway(chainGatewayAddress).dispatch(
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
