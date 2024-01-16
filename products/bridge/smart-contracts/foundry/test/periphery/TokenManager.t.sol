// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity 0.8.20;

import {Tester} from "foundry/test/Tester.sol";
import {MintAndBurnTokenManagerUpgradeable} from "contracts/periphery/MintAndBurnTokenManagerUpgradeable.sol";
import {TokenManagerUpgradeable} from "contracts/periphery/TokenManagerUpgradeable.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades/Upgrades.sol";

contract TokenManagerV1 is TokenManagerUpgradeable {
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function initialize(address _gateway) external initializer {
        __TokenManager_init(_gateway);
    }

    function _handleTransfer(
        address token,
        address from,
        uint amount
    ) internal override {}

    function _handleAccept(
        address token,
        address recipient,
        uint amount
    ) internal override {}
}

contract TokenManagerTest is Tester {
    address deployer = vm.addr(1);
    address chainGateway = vm.addr(102);

    MintAndBurnTokenManagerUpgradeable tokenManager;

    function setUp() external {
        vm.startPrank(deployer);
        address implementation = address(
            new MintAndBurnTokenManagerUpgradeable()
        );
        address proxy = address(
            new ERC1967Proxy(
                implementation,
                abi.encodeCall(
                    MintAndBurnTokenManagerUpgradeable.initialize,
                    chainGateway
                )
            )
        );
        tokenManager = MintAndBurnTokenManagerUpgradeable(proxy);

        vm.stopPrank();
    }

    function test_upgrade() external TODO {}
}
