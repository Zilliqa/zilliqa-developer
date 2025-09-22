import React, { useState } from 'react';
import { mintZRC6NFT } from '../config/zilpay-wallet';
import { CONTRACT_ADDRESSES } from '../config/contracts';

interface MintNFTComponentProps {
  zilPayAccount: string;
}

interface MintResult {
  transactionId: string;
}

export default function MintNFTComponent({ zilPayAccount }: MintNFTComponentProps) {
  const [isMinting, setIsMinting] = useState(false);
  const [tokenUri, setTokenUri] = useState('');
  const [mintResult, setMintResult] = useState<MintResult | null>(null);
  const [error, setError] = useState<string | null>(null);

  const handleMint = async () => {
    if (!zilPayAccount) {
      setError('ZilPay account not available');
      return;
    }

    setIsMinting(true);
    setError(null);
    setMintResult(null);

    try {
      // Use the configured ZRC6 contract address for testnet
      const contractAddress = CONTRACT_ADDRESSES[33101].ZRC6;

      if (!contractAddress) {
        throw new Error('ZRC6 contract address not configured for testnet. Please update CONTRACT_ADDRESSES in config/contracts.ts');
      }

      const result = await mintZRC6NFT(
        contractAddress,
        zilPayAccount, // Mint to the connected ZilPay account
        tokenUri || '' // Use empty string if no URI provided
      );

      setMintResult(result as MintResult);
    } catch (err) {
      console.error('Minting failed:', err);
      setError(err instanceof Error ? err.message : 'Failed to mint NFT');
    } finally {
      setIsMinting(false);
    }
  };

  return (
    <div className="mt-4 p-4 bg-gray-50 rounded-lg border">
      <h4 className="text-lg font-semibold mb-3">Mint New Scilla NFT</h4>
      <p className="text-sm text-gray-600 mb-4">
        Mint a new ZRC6 NFT on the Zilliqa network. The NFT will be minted to your connected ZilPay wallet address.
      </p>

      <div className="space-y-3">
        <div>
          <label htmlFor="tokenUri" className="block text-sm font-medium text-gray-700 mb-1">
            Token URI (optional)
          </label>
          <input
            type="text"
            id="tokenUri"
            value={tokenUri}
            onChange={(e) => setTokenUri(e.target.value)}
            placeholder="https://example.com/metadata.json"
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
          />
          <p className="text-xs text-gray-500 mt-1">
            Leave empty to use the contract&apos;s base URI + token ID
          </p>
        </div>

        <button
          onClick={handleMint}
          disabled={isMinting}
          className="w-full px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:bg-gray-400 disabled:cursor-not-allowed transition font-medium"
        >
          {isMinting ? 'Minting...' : 'Mint NFT'}
        </button>

        {error && (
          <div className="p-3 bg-red-50 border border-red-200 rounded-md">
            <p className="text-sm text-red-700">{error}</p>
          </div>
        )}

        {mintResult && (
          <div className="p-3 bg-green-50 border border-green-200 rounded-md">
            <p className="text-sm text-green-700 font-medium">NFT Minted Successfully!</p>
            <div className="mt-2 text-xs text-green-600">
              <p>Transaction ID: {mintResult.transactionId}</p>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
