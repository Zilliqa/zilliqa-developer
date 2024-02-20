// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {Script} from "forge-std/Script.sol";
import {Ownable2StepUpgradeable} from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import "forge-std/console.sol";

contract Update is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY_TEST");
        address owner = vm.addr(deployerPrivateKey);
        console.log("Deployer is %s", owner);

        address newOwner = 0x7975dc05A1D96c6c099537b71e3a6dB7DE0dBA06; // VERIFY
        console.log("New owner is %s", newOwner);

        address tokenManagerAddress = 0x6D61eFb60C17979816E4cE12CD5D29054E755948;
        address validatorManagerAddress = 0x71f3AD7cA177818399C9d79d74A6b284E4BEAAc9;
        address chainGatewayAddress = 0xbA44BC29371E19117DA666B729A1c6e1b35DDb40;

        vm.startBroadcast(deployerPrivateKey);
        Ownable2StepUpgradeable tokenManager = Ownable2StepUpgradeable(
            tokenManagerAddress
        );
        console.log(
            "TokenManager %s, pendingOwner: %s",
            tokenManagerAddress,
            tokenManager.pendingOwner()
        );
        tokenManager.transferOwnership(newOwner);
        Ownable2StepUpgradeable validatorManager = Ownable2StepUpgradeable(
            validatorManagerAddress
        );
        validatorManager.transferOwnership(newOwner);
        console.log(
            "ValidatorManager %s, pendingOwner: %s",
            validatorManagerAddress,
            validatorManager.pendingOwner()
        );
        Ownable2StepUpgradeable chainGateway = Ownable2StepUpgradeable(
            chainGatewayAddress
        );
        chainGateway.transferOwnership(newOwner);
        console.log(
            "ChainGateway %s, pendingOwner: %s",
            chainGatewayAddress,
            chainGateway.pendingOwner()
        );

        vm.stopBroadcast();
    }
}
