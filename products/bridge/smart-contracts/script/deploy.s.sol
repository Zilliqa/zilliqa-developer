// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {Script} from "forge-std/Script.sol";
import {Relayer} from "contracts/core/Relayer.sol";
import {ValidatorManager} from "contracts/core/ValidatorManager.sol";
import {ChainGateway} from "contracts/core/ChainGateway.sol";
import {Target} from "foundry/test/Target.sol";
import "forge-std/console.sol";

// Relayer Address: 0x46242abc24c6ba2d6B91a2a2e18008eeCac5eD71
// ValidatorManager Address: 0xb228aa0a543204988C1A4f3fd10FEe6551f7A379
// ChainGateway Address: 0x4DF88A0dF446b2cb14Ed57d12F48255758DE842a
// Target Address: 0x287b0F2491653E5Cb93981AcF7fb30576480015D

contract Deployment is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        uint256 validator2 = vm.envUint("PRIVATE_KEY2");
        // uint256 validator3 = vm.envUint("PRIVATE_KEY3");
        address[] memory validators = new address[](2);
        // address target = 0xAEace9DC4125f5E7c6bC1D77D515fa64f69f7dEC;

        validators[0] = vm.addr(deployerPrivateKey);
        validators[1] = vm.addr(validator2);
        // validators[2] = vm.addr(validator3);

        vm.startBroadcast(deployerPrivateKey);

        ValidatorManager validatorManager = new ValidatorManager{salt: "salt"}(
            validators[0]
        );
        validatorManager.initialize(validators);

        ChainGateway gateway = new ChainGateway{salt: "salt"}(
            address(validatorManager),
            validators[0]
        );

        new Target{salt: "salt"}(address(gateway));

        vm.stopBroadcast();
    }
}
