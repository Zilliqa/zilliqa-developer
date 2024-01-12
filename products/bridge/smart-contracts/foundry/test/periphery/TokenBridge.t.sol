// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity 0.8.20;

import {Tester, Vm} from "foundry/test/Tester.sol";
import {LockAndReleaseTokenManager} from "contracts/periphery/LockAndReleaseTokenManager.sol";
import {AcceptArgs, TokenManager} from "contracts/periphery/TokenManager.sol";
import {MintAndBurnTokenManager} from "contracts/periphery/MintAndBurnTokenManager.sol";
import {BridgedToken} from "contracts/periphery/BridgedToken.sol";
import {CallMetadata, Relayer} from "contracts/core/Relayer.sol";
import {ValidatorManager} from "contracts/core/ValidatorManager.sol";
import {ChainGateway} from "contracts/core/ChainGateway.sol";
import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";
import {TestToken} from "foundry/test/Helpers.sol";

contract TokenBridgeTests is Tester {
    using MessageHashUtils for bytes;

    // Gateway shared between the two chains
    Vm.Wallet validatorWallet = vm.createWallet(1);
    address validator = validatorWallet.addr;
    address[] validators = [validator];
    address sourceUser = vm.addr(2);
    address remoteUser = vm.addr(3);
    uint originalTokenSupply = 1000 ether;

    LockAndReleaseTokenManager sourceTokenManager;
    TestToken originalToken;
    ChainGateway sourceChainGateway;
    ValidatorManager sourceValidatorManager;

    MintAndBurnTokenManager remoteTokenManager;
    BridgedToken bridgedToken;
    ChainGateway remoteChainGateway;
    ValidatorManager remoteValidatorManager;

    function setUp() external {
        // Deploy Source Infra
        sourceValidatorManager = new ValidatorManager(validator);
        vm.prank(validator);
        sourceValidatorManager.initialize(validators);
        sourceChainGateway = new ChainGateway(address(sourceValidatorManager));

        // Deploy Target Infra
        remoteValidatorManager = new ValidatorManager(validator);
        vm.prank(validator);
        remoteValidatorManager.initialize(validators);
        remoteChainGateway = new ChainGateway(address(remoteValidatorManager));

        // Deploy LockAndReleaseTokenManager
        sourceTokenManager = new LockAndReleaseTokenManager(
            address(sourceChainGateway)
        );

        // Deploy MintAndBurnTokenManager
        remoteTokenManager = new MintAndBurnTokenManager(
            address(remoteChainGateway)
        );

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

        // Register bridged token with original token
        sourceTokenManager.registerToken(
            address(originalToken),
            address(bridgedToken),
            address(remoteTokenManager),
            block.chainid
        );
    }

    function test_happyPath() external {
        vm.startPrank(sourceUser);
        uint amount = originalTokenSupply;
        uint sourceChainId = block.chainid;
        uint remoteChainId = block.chainid;
        assertEq(originalToken.balanceOf(sourceUser), amount);

        // Approve and transfer
        originalToken.approve(address(sourceTokenManager), amount);
        sourceTokenManager.transfer(
            address(originalToken),
            remoteChainId,
            remoteUser,
            amount
        );

        // Make the bridge txn
        vm.startPrank(validator);
        bytes memory data = abi.encodeWithSelector(
            TokenManager.accept.selector,
            CallMetadata(sourceChainId, address(sourceTokenManager)), // From
            AcceptArgs(address(bridgedToken), remoteUser, amount) // To
        );
        bytes[] memory signatures = new bytes[](1);
        signatures[0] = sign(
            validatorWallet,
            abi
                .encode(
                    sourceChainId,
                    remoteChainId,
                    address(remoteTokenManager),
                    data,
                    10_000_000,
                    0
                )
                .toEthSignedMessageHash()
        );
        remoteChainGateway.dispatch(
            sourceChainId,
            address(remoteTokenManager),
            data,
            10_000_000,
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

        // //Mock Call
        // Make the bridge txn
        vm.startPrank(validator);
        data = abi.encodeWithSelector(
            TokenManager.accept.selector,
            CallMetadata(remoteChainId, address(remoteTokenManager)), // From
            AcceptArgs(address(originalToken), sourceUser, amount) // To
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

        // // Check balances back to normal
        assertEq(bridgedToken.balanceOf(remoteUser), 0);
        assertEq(originalToken.balanceOf(sourceUser), amount);
    }
}
