// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity 0.8.20;

import {Tester} from "foundry/test/Tester.sol";
import {TokenManagerUpgradeable} from "contracts/periphery/TokenManagerUpgradeable.sol";
import {TokenManagerV1Test} from "foundry/test/periphery/contracts/TokenManagerV1Test.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades/Upgrades.sol";

contract TokenManagerTest is Tester {
    address deployer = vm.addr(1);
    address chainGateway = vm.addr(102);

    TokenManagerV1Test tokenManager;

    function setUp() external {
        vm.startPrank(deployer);

        tokenManager = TokenManagerV1Test(
            Upgrades.deployUUPSProxy(
                "TokenManagerV1Test.sol",
                abi.encodeCall(TokenManagerV1Test.initialize, chainGateway)
            )
        );

        vm.stopPrank();
    }

    function test_upgrade() external TODO {}
}
