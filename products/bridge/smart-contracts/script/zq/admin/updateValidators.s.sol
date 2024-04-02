// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {Script} from "forge-std/Script.sol";
import {ValidatorManager} from "contracts/core/ValidatorManager.sol";
import "forge-std/console.sol";

contract Update is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY_OWNER");
        address validatorManagerAddress = 0x71f3AD7cA177818399C9d79d74A6b284E4BEAAc9;
        address currentValidator = 0x5807b970DB344d9d2386BbF5c3ec4cDA5CCdF1C8;
        address newValidator = 0x250572Ed005BaD64Ff24FbDc0d41875dAF58944f;

        vm.startBroadcast(deployerPrivateKey);

        ValidatorManager validatorManager = ValidatorManager(
            validatorManagerAddress
        );

        validatorManager.removeValidator(currentValidator);

        validatorManager.addValidator(newValidator);

        console.log(
            "Is current validator %s",
            validatorManager.isValidator(currentValidator)
        );
        console.log(
            "Is new validator %s",
            validatorManager.isValidator(newValidator)
        );

        vm.stopBroadcast();
    }
}
