// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {Script, console} from "forge-std/Script.sol";
import {NFToken} from "../src/testEnvContracts/erc721.sol";

contract NFTokenScript is Script {
    NFToken public nfToken;

    function setUp() public {}

    function run() public {
        vm.startBroadcast();

        // Deploy the NFToken contract
        nfToken = new NFToken();
        
        console.log("NFToken deployed to:", address(nfToken));
        console.log("Contract owner:", nfToken.owner());

        vm.stopBroadcast();
    }
}
