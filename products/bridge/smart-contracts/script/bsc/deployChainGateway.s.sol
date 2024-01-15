// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {Script} from "forge-std/Script.sol";
import {Relayer} from "contracts/core/Relayer.sol";
import {ValidatorManager} from "contracts/core/ValidatorManager.sol";
import {ChainGateway} from "contracts/core/ChainGateway.sol";
import "forge-std/console.sol";

contract Deployment is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY_TEST");
        address[] memory validators = new address[](1);

        validators[0] = vm.addr(deployerPrivateKey);

        vm.startBroadcast(deployerPrivateKey);

        ValidatorManager validatorManager = new ValidatorManager{
            salt: "zilliqa-bridge-uccb"
        }(validators[0]);
        validatorManager.initialize(validators);
        // address validatorManager = 0x462777dC056b3835d486f5f1Dd806195A569487F;

        new ChainGateway{salt: "zilliqa-bridge-uccb"}(
            address(validatorManager)
        );

        vm.stopBroadcast();
    }
}
