// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Script, console} from "forge-std/Script.sol";
import {BurnScillaAndMintEVMNFTSwap} from "../src/burnScillaAndMintEVMNFTSwap.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract DeployBurnScillaAndMintEVMNFTSwap is Script {
    
    // Default addresses - these should be overridden via environment variables or constructor params
    address constant DEFAULT_SCILLA_NFT_ADDRESS = 0x1000000000000000000000000000000000000001;
    address constant DEFAULT_EVM_NFT_ADDRESS = 0x2000000000000000000000000000000000000002;
    
    function run() external {
        // Get deployment parameters from environment variables or use defaults
        address scillaNFTAddress = vm.envOr("SCILLA_NFT_ADDRESS", DEFAULT_SCILLA_NFT_ADDRESS);
        address evmNFTAddress = vm.envOr("EVM_NFT_ADDRESS", DEFAULT_EVM_NFT_ADDRESS);
        address initialOwner = vm.envOr("INITIAL_OWNER", msg.sender);
        
        vm.startBroadcast();
        
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
        
        vm.stopBroadcast();
    }
    
    /**
     * @dev Deploy with custom parameters
     * @param _scillaNFTAddress Address of the Scilla NFT collection contract
     * @param _evmNFTAddress Address of the EVM NFT collection contract
     * @param _initialOwner Address of the initial owner of the contract
     */
    function deployWithParams(
        address _scillaNFTAddress,
        address _evmNFTAddress,
        address _initialOwner
    ) external returns (address) {
        vm.startBroadcast();
        
        // Deploy the implementation contract
        BurnScillaAndMintEVMNFTSwap implementation = new BurnScillaAndMintEVMNFTSwap();
        console.log("Implementation deployed to:", address(implementation));
        
        // Prepare the initialization data
        bytes memory initData = abi.encodeWithSelector(
            BurnScillaAndMintEVMNFTSwap.initialize.selector,
            _scillaNFTAddress,
            _evmNFTAddress,
            _initialOwner
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
        
        vm.stopBroadcast();
        
        return address(swapContract);
    }
}