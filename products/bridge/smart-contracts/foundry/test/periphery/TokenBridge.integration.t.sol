// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity 0.8.20;

import {Tester, Vm} from "foundry/test/Tester.sol";
import {LockAndReleaseTokenManagerUpgradeable} from "contracts/periphery/LockAndReleaseTokenManagerUpgradeable.sol";
import {ITokenManagerStructs, TokenManagerUpgradeable} from "contracts/periphery/TokenManagerUpgradeable.sol";
import {MintAndBurnTokenManagerUpgradeable} from "contracts/periphery/MintAndBurnTokenManagerUpgradeable.sol";
import {BridgedToken} from "contracts/periphery/BridgedToken.sol";
import {CallMetadata, IRelayerEvents} from "contracts/core/Relayer.sol";
import {ValidatorManager} from "contracts/core/ValidatorManager.sol";
import {ChainGateway} from "contracts/core/ChainGateway.sol";
import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {TestToken} from "foundry/test/Helpers.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades/Upgrades.sol";

// Integration Tests combining the TokenManagers and ChainGateway
contract TokenBridgeTests is Tester, IRelayerEvents {
    using MessageHashUtils for bytes;

    // Gateway shared between the two chains
    Vm.Wallet validatorWallet = vm.createWallet(1);
    address validator = validatorWallet.addr;
    address[] validators = [validator];
    address sourceUser = vm.addr(2);
    address remoteUser = vm.addr(3);
    uint originalTokenSupply = 1000 ether;

    LockAndReleaseTokenManagerUpgradeable sourceTokenManager;
    TestToken originalToken;
    ChainGateway sourceChainGateway;
    ValidatorManager sourceValidatorManager;

    MintAndBurnTokenManagerUpgradeable remoteTokenManager;
    BridgedToken bridgedToken;
    ChainGateway remoteChainGateway;
    ValidatorManager remoteValidatorManager;

    function setUp() external {
        // Deploy Source Infra
        sourceValidatorManager = new ValidatorManager(validator);
        vm.prank(validator);
        sourceValidatorManager.initialize(validators);
        vm.prank(validator);
        sourceChainGateway = new ChainGateway(
            address(sourceValidatorManager),
            validator
        );

        // Deploy Target Infra
        remoteValidatorManager = new ValidatorManager(validator);
        vm.prank(validator);
        remoteValidatorManager.initialize(validators);
        vm.prank(validator);
        remoteChainGateway = new ChainGateway(
            address(remoteValidatorManager),
            validator
        );

        // Deploy LockAndReleaseTokenManagerUpgradeable
        address implementation = address(
            new LockAndReleaseTokenManagerUpgradeable()
        );
        address proxy = address(
            new ERC1967Proxy(
                implementation,
                abi.encodeCall(
                    LockAndReleaseTokenManagerUpgradeable.initialize,
                    address(sourceChainGateway)
                )
            )
        );
        sourceTokenManager = LockAndReleaseTokenManagerUpgradeable(proxy);

        // Deploy MintAndBurnTokenManagerUpgradeable
        implementation = address(new MintAndBurnTokenManagerUpgradeable());
        proxy = address(
            new ERC1967Proxy(
                implementation,
                abi.encodeCall(
                    MintAndBurnTokenManagerUpgradeable.initialize,
                    address(remoteChainGateway)
                )
            )
        );
        remoteTokenManager = MintAndBurnTokenManagerUpgradeable(proxy);

        // Register contracts to chaingateway
        vm.prank(validator);
        sourceChainGateway.register(address(sourceTokenManager));
        vm.prank(validator);
        remoteChainGateway.register(address(remoteTokenManager));

        // Deploy original ERC20
        originalToken = new TestToken(originalTokenSupply);
        originalToken.transfer(sourceUser, originalTokenSupply);

        // Deploy bridged ERC20
        bridgedToken = remoteTokenManager.deployToken(
            "USDZ",
            "Zilliqa USD",
            address(originalToken),
            address(sourceTokenManager),
            block.chainid
        );

        ITokenManagerStructs.RemoteToken
            memory remoteToken = ITokenManagerStructs.RemoteToken({
                token: address(bridgedToken),
                tokenManager: address(remoteTokenManager),
                chainId: block.chainid
            });

        // Register bridged token with original token
        sourceTokenManager.registerToken(address(originalToken), remoteToken);
    }

    function test_happyPath() external {
        vm.startPrank(sourceUser);
        uint amount = originalTokenSupply;
        uint sourceChainId = block.chainid;
        uint remoteChainId = block.chainid;
        assertEq(originalToken.balanceOf(sourceUser), amount);

        bytes memory data = abi.encodeWithSelector(
            TokenManagerUpgradeable.accept.selector,
            CallMetadata(sourceChainId, address(sourceTokenManager)), // From
            abi.encode(
                ITokenManagerStructs.AcceptArgs(
                    address(bridgedToken),
                    remoteUser,
                    amount
                )
            ) // To
        );

        // Approve and transfer
        originalToken.approve(address(sourceTokenManager), amount);
        vm.expectEmit(address(sourceChainGateway));
        emit IRelayerEvents.Relayed(
            remoteChainId,
            address(remoteTokenManager),
            data,
            1_000_000,
            0
        );
        sourceTokenManager.transfer(
            address(originalToken),
            remoteChainId,
            remoteUser,
            amount
        );

        // Make the bridge txn
        vm.startPrank(validator);
        bytes[] memory signatures = new bytes[](1);
        signatures[0] = sign(
            validatorWallet,
            abi
                .encode(
                    sourceChainId,
                    remoteChainId,
                    address(remoteTokenManager),
                    data,
                    1_000_000,
                    0
                )
                .toEthSignedMessageHash()
        );
        remoteChainGateway.dispatch(
            sourceChainId,
            address(remoteTokenManager),
            data,
            1_000_000,
            0,
            signatures
        );

        // Check balances
        assertEq(bridgedToken.balanceOf(remoteUser), amount);
        assertEq(bridgedToken.totalSupply(), amount);
        assertEq(originalToken.totalSupply(), amount);
        assertEq(originalToken.balanceOf(sourceUser), 0);

        // Now sending it back
        vm.startPrank(remoteUser);
        bridgedToken.approve(address(remoteTokenManager), amount);
        remoteTokenManager.transfer(
            address(bridgedToken),
            sourceChainId,
            sourceUser,
            amount
        );

        //Mock Call
        // Make the bridge txn
        vm.startPrank(validator);
        data = abi.encodeWithSelector(
            TokenManagerUpgradeable.accept.selector,
            CallMetadata(remoteChainId, address(remoteTokenManager)), // From
            abi.encode(
                ITokenManagerStructs.AcceptArgs(
                    address(originalToken),
                    sourceUser,
                    amount
                )
            ) // To
        );
        signatures[0] = sign(
            validatorWallet,
            abi
                .encode(
                    remoteChainId,
                    sourceChainId,
                    address(sourceTokenManager),
                    data,
                    1_000_000,
                    0
                )
                .toEthSignedMessageHash()
        );
        sourceChainGateway.dispatch(
            remoteChainId,
            address(sourceTokenManager),
            data,
            1_000_000,
            0,
            signatures
        );

        // Check balances back to normal
        assertEq(bridgedToken.balanceOf(remoteUser), 0);
        assertEq(originalToken.balanceOf(sourceUser), amount);
    }
}
