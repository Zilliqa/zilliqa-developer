// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Script, console} from "forge-std/Script.sol";
import {BurnScillaAndMintEVMNFTSwap} from "../src/burnScillaAndMintEVMNFTSwap.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract DeployBurnScillaAndMintEVMNFTSwap is Script {
    function run() external {
        address scillaNFTAddress = vm.envAddress("SCILLA_NFT_ADDRESS");
        address evmNFTAddress = vm.envAddress("EVM_NFT_ADDRESS");
        address initialOwner = msg.sender;

        vm.startBroadcast();
        runWithParameters(scillaNFTAddress, evmNFTAddress, initialOwner);
        vm.stopBroadcast();
    }

    function runWithParameters(
        address scillaNFTAddress,
        address evmNFTAddress,
        address initialOwner
    ) public returns (BurnScillaAndMintEVMNFTSwap) {
        // Deploy the implementation contract
        BurnScillaAndMintEVMNFTSwap implementation = new BurnScillaAndMintEVMNFTSwap();
        console.log("Implementation deployed to:", address(implementation));

        // Prepare the initialization data
        bytes memory initData = abi.encodeWithSelector(
            BurnScillaAndMintEVMNFTSwap.initialize.selector,
            scillaNFTAddress,
            evmNFTAddress,
            initialOwner
        );

        // Deploy the proxy
        ERC1967Proxy proxy = new ERC1967Proxy(address(implementation), initData);
        console.log("Proxy deployed to:", address(proxy));
        
        // Cast the proxy to the contract interface
        BurnScillaAndMintEVMNFTSwap swapContract = BurnScillaAndMintEVMNFTSwap(address(proxy));
        
        console.log("BurnScillaAndMintEVMNFTSwap contract deployed at:", address(swapContract));
        console.log("Scilla NFT Address:", swapContract.scillaNFTAddress());
        console.log("EVM NFT Address:", swapContract.evmNFTAddress());
        console.log("Contract Owner:", swapContract.owner());
        console.log("Contract Paused:", swapContract.paused());

        return swapContract;
    }
}