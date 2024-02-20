// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {Script} from "forge-std/Script.sol";
import {ValidatorManager} from "contracts/core/ValidatorManager.sol";
import "forge-std/console.sol";

contract Update is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY_TEST");
        address validatorManagerAddress = 0x936feD44EC4F46CE08158B536Df2f864c30C4b5F;
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
