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
        address chainGatewayAddress = 0x3fa391E5a4c1b55D04A1b164fDC67ECEb312B93d;

        // Check these fields on every call
        uint sourceChainId = 32769;
        address target = 0xf42aa5b0D9B14f37c5de088178DA68DF841879E1; // Contract to be called EDIT
        // bytes EDIT, make sure to remove 0x
        bytes
            memory call = hex"1a90748a00000000000000000000000000000000000000000000000000000000000080010000000000000000000000003be6e686397f04901be15e3e02edc0c7565e4b130000000000000000000000000000000000000000000000000000000000000060000000000000000000000000000000000000000000000000000000000000006000000000000000000000000037595dc4dde8c43a5c80541c3cef7c6cc9a89867000000000000000000000000aaf33a7e4756d097b2158551a25374daf600c49d0000000000000000000000000000000000000000000000000000000005f5e100";
        uint gasLimit = 1_000_000;
        uint nonce = 0; // uint EDIT

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
