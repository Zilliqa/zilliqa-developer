// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity 0.8.20;

import {Tester, Vm} from "test/Tester.sol";
import {ITokenManagerStructs, TokenManagerUpgradeable} from "contracts/periphery/TokenManagerUpgradeable.sol";
import {LockAndReleaseTokenManagerUpgradeableV3} from "contracts/periphery/TokenManagerV3/LockAndReleaseTokenManagerUpgradeableV3.sol";
import {MintAndBurnTokenManagerUpgradeableV3} from "contracts/periphery/TokenManagerV3/MintAndBurnTokenManagerUpgradeableV3.sol";
import {BridgedToken} from "contracts/periphery/BridgedToken.sol";
import {CallMetadata, IRelayerEvents} from "contracts/core/Relayer.sol";
import {ValidatorManager} from "contracts/core/ValidatorManager.sol";
import {ChainGateway} from "contracts/core/ChainGateway.sol";
import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";
import {TestToken} from "test/Helpers.sol";
import {LockAndReleaseTokenManagerDeployer} from "test/periphery/TokenManagerDeployers/LockAndReleaseTokenManagerDeployer.sol";
import {MintAndBurnTokenManagerDeployer} from "test/periphery/TokenManagerDeployers/MintAndBurnTokenManagerDeployer.sol";

// Integration Tests combining the TokenManagers and ChainGateway
contract TokenBridgeIntegrationTests is
    Tester,
    IRelayerEvents,
    LockAndReleaseTokenManagerDeployer,
    MintAndBurnTokenManagerDeployer
{
    using MessageHashUtils for bytes;

    // Gateway shared between the two chains
    Vm.Wallet validatorWallet = vm.createWallet(1);
    address validator = validatorWallet.addr;
    address[] validators = [validator];
    address sourceUser = vm.addr(2);
    address remoteUser = vm.addr(3);
    uint originalTokenSupply = 1000 ether;
    uint fees = 0.1 ether;

    LockAndReleaseTokenManagerUpgradeableV3 sourceTokenManager;
    TestToken originalToken;
    ChainGateway sourceChainGateway;
    ValidatorManager sourceValidatorManager;

    MintAndBurnTokenManagerUpgradeableV3 remoteTokenManager;
    BridgedToken bridgedToken;
    ChainGateway remoteChainGateway;
    ValidatorManager remoteValidatorManager;

    function setUp() external {
        vm.startPrank(validator);
        // Deploy Source Infra
        sourceValidatorManager = new ValidatorManager(validator);
        sourceValidatorManager.initialize(validators);
        sourceChainGateway = new ChainGateway(
            address(sourceValidatorManager),
            validator
        );

        // Deploy Target Infra
        remoteValidatorManager = new ValidatorManager(validator);
        remoteValidatorManager.initialize(validators);
        remoteChainGateway = new ChainGateway(
            address(remoteValidatorManager),
            validator
        );

        // Deploy LockAndReleaseTokenManagerUpgradeable
        sourceTokenManager = deployLatestLockAndReleaseTokenManager(
            address(sourceChainGateway),
            fees
        );

        // Deploy MintAndBurnTokenManagerUpgradeable
        remoteTokenManager = deployLatestMintAndBurnTokenManager(
            address(remoteChainGateway),
            fees
        );

        // Register contracts to chaingateway
        sourceChainGateway.register(address(sourceTokenManager));
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

        vm.stopPrank();
    }

    function test_happyPath() external {
        startHoax(sourceUser);
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
        sourceTokenManager.transfer{value: fees}(
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
        startHoax(remoteUser);
        bridgedToken.approve(address(remoteTokenManager), amount);
        remoteTokenManager.transfer{value: fees}(
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
