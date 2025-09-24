// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Script.sol";
import "../../src/playground/erc721.sol";
import "../../src/burnScillaAndMintEVMNFTSwap.sol";

contract CreateNewNFTsForSwapContract is Script {
    function run() external {
        vm.startBroadcast();

        address evmNftAddress = vm.envAddress("EVM_NFT_ADDRESS");
        address swapAddress = vm.envAddress("SWAP_CONTRACT_ADDRESS");
        uint256 count = vm.envUint("NFT_COUNT");

        runWithParameters(evmNftAddress, swapAddress, count);

        vm.stopBroadcast();
    }

    function runWithParameters(
        address evmNftAddress,
        address swapAddress,
        uint256 count
    ) public {
        NFToken nft = NFToken(evmNftAddress);
        BurnScillaAndMintEVMNFTSwap swap = BurnScillaAndMintEVMNFTSwap(swapAddress);

        // Collect exactly 'count' available NFT IDs
        address recipient = msg.sender;
        uint256[] memory idsToMint = new uint256[](count);
        uint256 mintedCount = 0;
        uint256 currentId = 1;
        uint256 maxCheckId = 10000; // Safety limit to prevent infinite loop

        while (mintedCount < count && currentId <= maxCheckId) {
            // Check if NFT already exists
            try nft.ownerOf(currentId) returns (address) {
                // NFT already exists, skip
            } catch {
                // NFT doesn't exist, collect it for batch mint
                idsToMint[mintedCount] = currentId;
                mintedCount++;
            }
            currentId++;
        }

        // Ensure we found exactly 'count' available NFTs
        require(mintedCount == count, "Could not find the required number of available NFT IDs within the ID range");

        // Create the actual array for batch mint
        uint256[] memory actualIdsToMint = new uint256[](mintedCount);
        for (uint256 i = 0; i < mintedCount; i++) {
            actualIdsToMint[i] = idsToMint[i];
        }

        // Batch mint the NFTs
        nft.batchMint(recipient, actualIdsToMint);

        // Set mintedIds for the swap mappings
        uint256[] memory mintedIds = actualIdsToMint;
        swap.setNftSwapMappings(mintedIds, mintedIds);
    }
}
