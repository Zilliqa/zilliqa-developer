// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {Script, console} from "forge-std/Script.sol";
import {NFToken} from "../../src/playground/erc721.sol";
import {BurnScillaAndMintEVMNFTSwap} from "../../src/burnScillaAndMintEVMNFTSwap.sol";
import {DeployBurnScillaAndMintEVMNFTSwap} from "../../script/deployBurnScillaAndMintEVMNFTSwap.sol";
import {CreateNewNFTsForSwapContract} from "../../script/playground/createNewNFTsForSwapContract.sol";

contract DeployAndConfigureNftSwapPlayground is Script {
    NFToken public playgroundEvmNFT;
    BurnScillaAndMintEVMNFTSwap public swapContract;

    function setUp() public {}

    function run() public {
        address scillaNFTAddress = vm.envAddress("SCILLA_NFT_ADDRESS");
        address initialOwner = msg.sender;

        vm.startBroadcast();

        playgroundEvmNFT = new NFToken();

        console.log("NFToken deployed to:", address(playgroundEvmNFT));
        console.log("NFToken owner:", playgroundEvmNFT.owner());

        // Call the DeployBurnScillaAndMintEVMNFTSwap.sol runWithParameters() to deploy swap contract
        swapContract = new DeployBurnScillaAndMintEVMNFTSwap().runWithParameters(scillaNFTAddress, address(playgroundEvmNFT), initialOwner);
        console.log("Swap contract deployed to:", address(swapContract));

        configureFirstBatchOfNfts(
            initialOwner,
            address(playgroundEvmNFT),
            address(swapContract),
            10
        );

        vm.stopBroadcast();
    }

    function configureFirstBatchOfNfts(
        address recipient,
        address evmNftAddress,
        address swapAddress,
        uint256 count
    ) public {
        NFToken nft = NFToken(evmNftAddress);
        BurnScillaAndMintEVMNFTSwap swap = BurnScillaAndMintEVMNFTSwap(swapAddress);

        uint256[] memory actualIdsToMint = new uint256[](count);
        for (uint256 i = 0; i < count; i++) {
            actualIdsToMint[i] = i;
        }

        nft.batchMint(recipient, actualIdsToMint);
        swap.setNftSwapMappings(actualIdsToMint, actualIdsToMint);
    }
}
