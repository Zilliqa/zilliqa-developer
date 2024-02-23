// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {Script} from "forge-std/Script.sol";
import {Ownable2StepUpgradeable} from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import "forge-std/console.sol";

contract Update is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY_OWNER");
        address owner = vm.addr(deployerPrivateKey);
        console.log("Deployer is %s", owner);

        address newOwner = 0xA5EF26F343e393676110080aFe9f8557118155c9; // VERIFY
        console.log("New owner is %s", newOwner);

        address tokenManagerAddress = 0xF391A1Ee7b3ccad9a9451D2B7460Ac646F899f23;
        address validatorManagerAddress = 0x936feD44EC4F46CE08158B536Df2f864c30C4b5F;
        address chainGatewayAddress = 0x3967f1a272Ed007e6B6471b942d655C802b42009;

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
