// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity 0.8.20;

import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {LockAndReleaseTokenManagerUpgradeable} from "contracts/periphery/LockAndReleaseTokenManagerUpgradeable.sol";
import {LockAndReleaseTokenManagerUpgradeableV2} from "contracts/periphery/TokenManagerV2/LockAndReleaseTokenManagerUpgradeableV2.sol";
import {LockAndReleaseTokenManagerUpgradeableV3} from "contracts/periphery/TokenManagerV3/LockAndReleaseTokenManagerUpgradeableV3.sol";

abstract contract LockAndReleaseTokenManagerDeployer {
    function deployLockAndReleaseTokenManagerV1(
        address chainGateway
    ) public returns (LockAndReleaseTokenManagerUpgradeable) {
        address implementation = address(
            new LockAndReleaseTokenManagerUpgradeable()
        );
        // Deploy proxy and attach v1
        address proxy = address(
            new ERC1967Proxy(
                implementation,
                abi.encodeCall(
                    LockAndReleaseTokenManagerUpgradeable.initialize,
                    chainGateway
                )
            )
        );

        return LockAndReleaseTokenManagerUpgradeable(proxy);
    }

    function deployLockAndReleaseTokenManagerV2(
        address chainGateway,
        uint fees
    ) public returns (LockAndReleaseTokenManagerUpgradeableV2) {
        LockAndReleaseTokenManagerUpgradeable proxy = deployLockAndReleaseTokenManagerV1(
                chainGateway
            );

        address newImplementation = address(
            new LockAndReleaseTokenManagerUpgradeableV2()
        );

        bytes memory encodedInitializerCall = abi.encodeCall(
            LockAndReleaseTokenManagerUpgradeableV2.reinitialize,
            fees
        );
        proxy.upgradeToAndCall(newImplementation, encodedInitializerCall);

        return LockAndReleaseTokenManagerUpgradeableV2(address(proxy));
    }

    function deployLockAndReleaseTokenManagerV3(
        address chainGateway,
        uint fees
    ) public returns (LockAndReleaseTokenManagerUpgradeableV3) {
        LockAndReleaseTokenManagerUpgradeableV2 proxy = deployLockAndReleaseTokenManagerV2(
                chainGateway,
                fees
            );

        address newImplementation = address(
            new LockAndReleaseTokenManagerUpgradeableV3()
        );

        proxy.upgradeToAndCall(newImplementation, "");

        return LockAndReleaseTokenManagerUpgradeableV3(address(proxy));
    }

    function deployLatestLockAndReleaseTokenManager(
        address chainGateway,
        uint fees
    ) public returns (LockAndReleaseTokenManagerUpgradeableV3) {
        return deployLockAndReleaseTokenManagerV3(chainGateway, fees);
    }
}
