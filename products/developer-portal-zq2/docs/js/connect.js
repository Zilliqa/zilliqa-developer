async function connectZilliqaChain(
  api_endpoint,
  chain_id,
  chain_name,
  explorer,
  tok_name,
  tok_sym
) {
  try {
    await window.ethereum.request({
      method: "wallet_switchEthereumChain",
      params: [{ chainId: chain_id }],
    });
  } catch (switchError) {
    // This error code indicates that the chain has not been added to MetaMask.
    if (switchError.code === 4902) {
      try {
        await window.ethereum.request({
          method: "wallet_addEthereumChain",
          params: [
            {
              chainId: chain_id,
              chainName: chain_name,
              rpcUrls: [api_endpoint],
              blockExplorerUrls: [explorer],
              nativeCurrency: {
                decimals: 18,
                name: tok_name,
                symbol: tok_sym,
              },
            },
          ],
        });
      } catch (addError) {
        alert("Failed to add chain - " + addError);
      }
    }
  }
}
