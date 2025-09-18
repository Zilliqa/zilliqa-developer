// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/utils/ReentrancyGuardUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/utils/PausableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import "@openzeppelin/contracts/token/ERC721/IERC721.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";


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
 * 
 * @custom:security-contact security@zilliqa.com
 */
contract BurnScillaAndMintEVMNFTSwap is 
    Initializable, 
    OwnableUpgradeable, 
    ReentrancyGuardUpgradeable, 
    PausableUpgradeable,
    UUPSUpgradeable 
{
    using ECDSA for bytes32;
    using MessageHashUtils for bytes32;

    // Events
    event NFTSwapped(
        address indexed evmWallet,
        string zilPayAddress,
        uint256[] tokenIds
    );

    // State variables - no longer immutable since they need to be set in initializer
    address public scillaNFTAddress;
    address public evmNFTAddress;
    
    // Mapping to link Scilla ZRC-6 token IDs to EVM ERC-721 token IDs
    mapping(uint256 => uint256) private nftSwapMapping;

    // Custom errors
    error InvalidSignature();
    error InvalidTokenIdsLength();
    error ZeroAddress();
    error AlreadyInitialized();
    error InvalidMapping();
    error InvalidLength();

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @dev Initializes the contract with the addresses of the Scilla NFT collection and EVM NFT collection
     * @param _scillaNFTAddress Address of the Scilla NFT collection contract
     * @param _evmNFTAddress Address of the EVM NFT collection contract
     * @param _initialOwner Address of the initial owner of the contract
     */
    function initialize(
        address _scillaNFTAddress,
        address _evmNFTAddress,
        address _initialOwner
    ) public initializer {
        if (_scillaNFTAddress == address(0) || _evmNFTAddress == address(0)) {
            revert ZeroAddress();
        }
        if (_initialOwner == address(0)) {
            revert ZeroAddress();
        }
        
        __Ownable_init(_initialOwner);
        __ReentrancyGuard_init();
        __Pausable_init();
        __UUPSUpgradeable_init();
        
        scillaNFTAddress = _scillaNFTAddress;
        evmNFTAddress = _evmNFTAddress;
    }

    /**
     * @dev Swaps ZRC6 NFTs for ERC721 NFTs by burning ZRC6 tokens and minting corresponding ERC721 tokens
     * @param scillaAddress The ZilPay wallet address that currently owns the Scilla NFTs
     * @param scillaNftIdsToSwap List of NFT IDs to be burned and swapped
     * @param signature Signature proving ownership of the ZilPay address, signed with ZilPay
     * 
     * Requirements:
     * - Contract must not be paused
     * - scillaNftIdsToSwap array must not be empty
     * - signature must be valid
     * - All specified NFTs must have a mapping
     * - All specified NFTs must be owned by the scillaAddress on the Scilla side
     * 
     * Effects:
     * - Burns all specified NFTs on the Scilla NFT collection (sets owner to zero address)
     * - Mints corresponding NFTs on the EVM NFT collection to the caller's address
     */
    function swapZRC6NFTForErc721NFTByByrningZRC6(
        string memory scillaAddress,
        uint256[] memory scillaNftIdsToSwap,
        bytes memory signature
    ) external nonReentrant whenNotPaused {
        // Input validation
        if (scillaNftIdsToSwap.length == 0) {
            revert InvalidTokenIdsLength();
        }
        
        // Verify contract addresses are set
        if (scillaNFTAddress == address(0) || evmNFTAddress == address(0)) {
            revert ZeroAddress();
        }
        
        // Create message hash for signature verification
        bytes32 messageHash = keccak256(abi.encodePacked(msg.sender));
        bytes32 ethSignedMessageHash = messageHash.toEthSignedMessageHash();
        
        // Verify signature
        address recoveredAddress = ethSignedMessageHash.recover(signature);
        if (keccak256(abi.encodePacked(recoveredAddress)) != keccak256(abi.encodePacked(scillaAddress))) {
            revert InvalidSignature();
        }
        
        // Mapping check
        for (uint256 i = 0; i < scillaNftIdsToSwap.length; i++) {
            if (nftSwapMapping[scillaNftIdsToSwap[i]] == 0) {
                revert InvalidMapping();
            }
        }
        
        // TODO: Implement Zilliqa interop call to burn ZRC6 NFTs
        // This would involve calling the Scilla contract to transfer ownership to zero address
        _burnScillaNFTs(scillaAddress, scillaNftIdsToSwap);
        
        // Transfer corresponding ERC721 NFTs
        uint256[] memory mappedIds = new uint256[](scillaNftIdsToSwap.length);
        for (uint256 i = 0; i < scillaNftIdsToSwap.length; i++) {
            mappedIds[i] = nftSwapMapping[scillaNftIdsToSwap[i]];
        }
        _transferEvmNFTs(msg.sender, mappedIds);
        
        emit NFTSwapped(msg.sender, scillaAddress, scillaNftIdsToSwap);
    }

    /**
     * @dev Burns Scilla NFTs by transferring ownership to zero address through interop
     * @param owner Current owner of the NFTs on Scilla side
     * @param tokenIds Array of token IDs to burn
     * 
     * NOTE: This is a placeholder implementation. The actual implementation would use
     * Zilliqa interop to call the Scilla contract methods.
     */
    function _burnScillaNFTs(string memory owner, uint256[] memory tokenIds) internal {
        // TODO: Implement Zilliqa interop call to burn ZRC6 NFTs
        // Verify ownership of tokens
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
    function _transferEvmNFTs(address to, uint256[] memory tokenIds) internal {
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
     * @dev Authorizes contract upgrades
     * 
     * Requirements:
     * - Only callable by owner
     */
    function _authorizeUpgrade(address newImplementation) internal override onlyOwner {}

    /**
     * @dev Sets the NFT swap mapping for a single pair
     * @param scillaId The Scilla NFT ID
     * @param evmId The corresponding EVM NFT ID
     */
    function setNftSwapMapping(uint256 scillaId, uint256 evmId) external onlyOwner {
        nftSwapMapping[scillaId] = evmId;
    }

    /**
     * @dev Sets multiple NFT swap mappings
     * @param scillaIds Array of Scilla NFT IDs
     * @param evmIds Array of corresponding EVM NFT IDs
     */
    function setNftSwapMappings(uint256[] memory scillaIds, uint256[] memory evmIds) external onlyOwner {
        if (scillaIds.length != evmIds.length) {
            revert InvalidLength();
        }
        for (uint256 i = 0; i < scillaIds.length; i++) {
            nftSwapMapping[scillaIds[i]] = evmIds[i];
        }
    }
}