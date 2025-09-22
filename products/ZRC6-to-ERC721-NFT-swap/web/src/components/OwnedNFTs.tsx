import React, { useState, useEffect } from 'react';
import { Zilliqa } from '@zilliqa-js/zilliqa';
import { appConfig } from '@/config';
import { CONTRACT_ADDRESSES } from '@/config/contracts';

interface OwnedNFTsProps {
  zilPayAccount: string;
}

const OwnedNFTs: React.FC<OwnedNFTsProps> = ({ zilPayAccount }) => {
  const [ownedTokenIds, setOwnedTokenIds] = useState<string[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!zilPayAccount) return;

    const fetchOwnedNFTs = async () => {
      setLoading(true);
      setError(null);

      try {
        const zilliqa = new Zilliqa(appConfig.zilliqaNodeUrl);
        const contractAddress = CONTRACT_ADDRESSES[33469].ZRC6;

        // Get the base16 address from ZilPay
        const base16Address = window.zilPay?.wallet?.defaultAccount?.base16?.toLowerCase();
        if (!base16Address) {
          throw new Error('Unable to get base16 address from ZilPay');
        }

        // Query the entire token_owners map
        const response = await zilliqa.blockchain.getSmartContractSubState(
          contractAddress,
          'token_owners',
          []
        );

        if (response.result && response.result.token_owners) {
          // Filter token IDs where the owner matches the user's base16 address
          const owned = Object.entries(response.result.token_owners)
            .filter(([tokenId, owner]) => (owner as string).toLowerCase() === base16Address)
            .map(([tokenId]) => tokenId);

          setOwnedTokenIds(owned);
        } else {
          setOwnedTokenIds([]);
        }
      } catch (err) {
        console.error('Error fetching owned NFTs:', err);
        setError(err instanceof Error ? err.message : 'Failed to fetch owned NFTs');
      } finally {
        setLoading(false);
      }
    };

    fetchOwnedNFTs();
  }, [zilPayAccount]);

  if (loading) {
    return (
      <div className="mt-4 p-4 bg-gray-50 rounded-lg">
        <p className="text-sm text-gray-600">Loading owned NFTs...</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="mt-4 p-4 bg-red-50 rounded-lg">
        <p className="text-sm text-red-600">Error: {error}</p>
      </div>
    );
  }

  return (
    <div className="mt-4 p-4 bg-gray-50 rounded-lg">
      <h4 className="text-lg font-semibold mb-2">Your Owned Scilla NFTs</h4>
      {ownedTokenIds.length === 0 ? (
        <p className="text-sm text-gray-600">No NFTs owned.</p>
      ) : (
        <div className="max-h-40 overflow-y-auto border rounded p-2 bg-white">
          <ul className="space-y-1">
            {ownedTokenIds.map((tokenId) => (
              <li key={tokenId} className="text-sm font-mono text-gray-700">
                Token ID: {tokenId}
              </li>
            ))}
          </ul>
        </div>
      )}
    </div>
  );
};

export default OwnedNFTs;
