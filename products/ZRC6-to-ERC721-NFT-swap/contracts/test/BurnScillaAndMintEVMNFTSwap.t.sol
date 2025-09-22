// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test, console} from "forge-std/Test.sol";
import {BurnScillaAndMintEVMNFTSwap} from "../src/burnScillaAndMintEVMNFTSwap.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract BurnScillaAndMintEVMNFTSwapTest is Test {
    BurnScillaAndMintEVMNFTSwap implementation;
    ERC1967Proxy proxy;
    BurnScillaAndMintEVMNFTSwap swapContract;

    address scillaNFTAddress = address(0x1000000000000000000000000000000000000001);
    address evmNFTAddress = address(0x2000000000000000000000000000000000000002);
    address initialOwner = address(this);

    function setUp() public {
        // Deploy implementation
        implementation = new BurnScillaAndMintEVMNFTSwap();
    }

    function testInitializeGas() public {
        // Prepare initData
        bytes memory initData = abi.encodeWithSelector(
            BurnScillaAndMintEVMNFTSwap.initialize.selector,
            scillaNFTAddress,
            evmNFTAddress,
            initialOwner
        );

        // Measure gas for proxy deployment without initData
        uint256 gasStartProxyOnly = gasleft();
        ERC1967Proxy proxyOnly = new ERC1967Proxy(address(implementation), "");
        uint256 gasForProxyOnly = gasStartProxyOnly - gasleft();

        // Measure gas for proxy deployment with initData
        uint256 gasStartProxyWithInit = gasleft();
        ERC1967Proxy proxyWithInit = new ERC1967Proxy(address(implementation), initData);
        uint256 gasForProxyWithInit = gasStartProxyWithInit - gasleft();

        // Calculate gas used for initialize (approximate)
        uint256 gasForInitialize = gasForProxyWithInit - gasForProxyOnly;

        console.log("Gas used for proxy only:", gasForProxyOnly);
        console.log("Gas used for proxy with init:", gasForProxyWithInit);
        console.log("Estimated gas used for initialize:", gasForInitialize);

        // Verify the contract
        swapContract = BurnScillaAndMintEVMNFTSwap(address(proxyWithInit));
        assertEq(swapContract.scillaNFTAddress(), scillaNFTAddress);
        assertEq(swapContract.evmNFTAddress(), evmNFTAddress);
        assertEq(swapContract.owner(), initialOwner);
    }
}
