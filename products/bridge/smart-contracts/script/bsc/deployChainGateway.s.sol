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

        // ValidatorManager validatorManager = new ValidatorManager{
        //     salt: "zilliqa-uccb"
        // }(validators[0]);
        // validatorManager.initialize(validators);

        new ChainGateway{salt: "zilliqa"}(
            address(0x5EDE85Ee7B2b4aefA88505Aa3893c1628FCeB0CE),
            validators[0]
        );

        vm.stopBroadcast();
    }
}
