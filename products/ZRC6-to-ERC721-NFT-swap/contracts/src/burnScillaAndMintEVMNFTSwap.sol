// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";
import "@openzeppelin/contracts/token/ERC721/IERC721.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import "@openzeppelin/contracts/utils/Pausable.sol";

/**
 * @title BurnScillaAndMintEVMNFTSwap
 * @dev This contract enables the burning of ZRC6 NFTs (through Zilliqa interop) and minting of new ERC721 NFTs in one transaction.
 * This contract handles a single specific pair of Scilla NFT collection and its corresponding EVM NFT collection.
 * 
 * The contract uses Zilliqa interop to call the Scilla NFT collection contract and allows users to call the 
 * swapZRC6NFTForErc721NFTByByrningZRC6 method using an EVM wallet. This method takes the owner's ZilPay wallet address,
 * a signature, and a list of NFT IDs to be burned and swapped.
 * 
 * The signature is provided as proof that the EVM wallet that calls the contract also owns the ZilPay address.
 */
contract BurnScillaAndMintEVMNFTSwap is Ownable, ReentrancyGuard, Pausable {
    using ECDSA for bytes32;
    using MessageHashUtils for bytes32;

    // Events
    event NFTSwapped(
        address indexed evmWallet,
        address indexed zilPayAddress,
        uint256[] tokenIds
    );

    // State variables - immutable since they're set only in constructor
    address public immutable scillaNFTAddress;
    address public immutable evmNFTAddress;
    
    // Mapping to track used signatures to prevent replay attacks
    mapping(bytes32 => bool) public usedSignatures;
    
    // Custom errors
    error InvalidSignature();
    error SignatureAlreadyUsed();
    error InvalidTokenIdsLength();
    error ZeroAddress();

    /**
     * @dev Constructor sets the addresses of the Scilla NFT collection and EVM NFT collection
     * @param _scillaNFTAddress Address of the Scilla NFT collection contract
     * @param _evmNFTAddress Address of the EVM NFT collection contract
     * @param _initialOwner Address of the initial owner of the contract
     */
    constructor(
        address _scillaNFTAddress,
        address _evmNFTAddress,
        address _initialOwner
    ) Ownable(_initialOwner) {
        if (_scillaNFTAddress == address(0) || _evmNFTAddress == address(0)) {
            revert ZeroAddress();
        }
        if (_initialOwner == address(0)) {
            revert ZeroAddress();
        }
        
        scillaNFTAddress = _scillaNFTAddress;
        evmNFTAddress = _evmNFTAddress;
    }

    /**
     * @dev Swaps ZRC6 NFTs for ERC721 NFTs by burning ZRC6 tokens and minting corresponding ERC721 tokens
     * @param zilPayAddress The ZilPay wallet address that currently owns the Scilla NFTs
     * @param signature Signature proving ownership of the ZilPay address, signed with ZilPay
     * @param tokenIds List of NFT IDs to be burned and swapped
     * 
     * Requirements:
     * - Contract must not be paused
     * - tokenIds array must not be empty
     * - signature must be valid and not previously used
     * - All specified NFTs must be owned by the zilPayAddress on the Scilla side
     * 
     * Effects:
     * - Burns all specified NFTs on the Scilla NFT collection (sets owner to zero address)
     * - Mints corresponding NFTs on the EVM NFT collection to the caller's address
     * - Marks the signature as used to prevent replay attacks
     */
    function swapZRC6NFTForErc721NFTByByrningZRC6(
        address zilPayAddress,
        bytes memory signature,
        uint256[] memory tokenIds
    ) external nonReentrant whenNotPaused {
        // Input validation
        if (tokenIds.length == 0) {
            revert InvalidTokenIdsLength();
        }
        
        // Verify contract addresses are set (they're immutable so this check is redundant but kept for clarity)
        // Note: Since addresses are now immutable and set in constructor, they cannot be zero at runtime
        
        // Create message hash for signature verification
        bytes32 messageHash = keccak256(abi.encodePacked(zilPayAddress, msg.sender, block.chainid));
        bytes32 ethSignedMessageHash = messageHash.toEthSignedMessageHash();
        
        // Check if signature was already used
        if (usedSignatures[ethSignedMessageHash]) {
            revert SignatureAlreadyUsed();
        }
        
        // Verify signature
        address recoveredAddress = ethSignedMessageHash.recover(signature);
        if (recoveredAddress != zilPayAddress) {
            revert InvalidSignature();
        }
        
        // Mark signature as used
        usedSignatures[ethSignedMessageHash] = true;
        
        // TODO: Implement Zilliqa interop call to burn ZRC6 NFTs
        // This would involve calling the Scilla contract to transfer ownership to zero address
        _burnScillaNFTs(zilPayAddress, tokenIds);
        
        // TODO: Implement minting of ERC721 NFTs
        // This would involve calling the EVM NFT contract to mint tokens to msg.sender
        _mintEvmNFTs(msg.sender, tokenIds);
        
        emit NFTSwapped(msg.sender, zilPayAddress, tokenIds);
    }

    /**
     * @dev Burns Scilla NFTs by transferring ownership to zero address through interop
     * @param owner Current owner of the NFTs on Scilla side
     * @param tokenIds Array of token IDs to burn
     * 
     * NOTE: This is a placeholder implementation. The actual implementation would use
     * Zilliqa interop to call the Scilla contract methods.
     */
    function _burnScillaNFTs(address owner, uint256[] memory tokenIds) internal {
        // TODO: Implement actual Zilliqa interop call
        // This would involve:
        // 1. Calling the Scilla contract through interop
        // 2. Verifying ownership of tokens
        // 3. Transferring ownership to zero address
        
        // Placeholder - in actual implementation this would be an interop call
        // Example: ScillaContract(scillaNFTAddress).burnTokens(owner, tokenIds);
    }

    /**
     * @dev Mints EVM NFTs to the specified recipient
     * @param to Address to mint the NFTs to
     * @param tokenIds Array of token IDs to mint
     * 
     * NOTE: This is a placeholder implementation. The actual implementation would call
     * the EVM NFT contract's minting function.
     */
    function _mintEvmNFTs(address to, uint256[] memory tokenIds) internal {
        // TODO: Implement actual EVM NFT minting
        // This would involve calling the EVM NFT contract's mint function
        // The contract should have this contract address as an authorized minter
        
        // Placeholder - in actual implementation this would be:
        // for (uint256 i = 0; i < tokenIds.length; i++) {
        //     IERC721Mintable(evmNFTAddress).mint(to, tokenIds[i]);
        // }
    }

    /**
     * @dev Pauses the contract, preventing swaps
     * 
     * Requirements:
     * - Only callable by owner
     */
    function pause() external onlyOwner {
        _pause();
    }

    /**
     * @dev Unpauses the contract, allowing swaps
     * 
     * Requirements:
     * - Only callable by owner
     */
    function unpause() external onlyOwner {
        _unpause();
    }

    /**
     * @dev Checks if a signature has been used
     * @param zilPayAddress The ZilPay address
     * @param evmAddress The EVM address
     * @return bool True if signature has been used
     */
    function isSignatureUsed(address zilPayAddress, address evmAddress) external view returns (bool) {
        bytes32 messageHash = keccak256(abi.encodePacked(zilPayAddress, evmAddress, block.chainid));
        bytes32 ethSignedMessageHash = messageHash.toEthSignedMessageHash();
        return usedSignatures[ethSignedMessageHash];
    }
}