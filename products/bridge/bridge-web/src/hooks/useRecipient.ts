import { fromBech32Address, toBech32Address } from "@zilliqa-js/crypto";
import { validation } from "@zilliqa-js/util";
import { useEffect, useState } from "react";
import { useAccount } from "wagmi";

export default function useRecipient() {
  const [recipient, setRecipient] = useState<string>();
  const { address: account } = useAccount();

  const validBech32Address = recipient && validation.isBech32(recipient);
  const validEthAddress = recipient && validation.isAddress(recipient);

  const isAddressValid = !recipient || validBech32Address !== validEthAddress;

  const recipientEth = validBech32Address
    ? (fromBech32Address(recipient) as `0x${string}`)
    : (recipient as `0x${string}` | undefined);

  useEffect(() => {
    account && setRecipient(account);
  }, [account]);

  function toggleAddress() {
    setRecipient((_recipient) => {
      if (!_recipient) {
        return _recipient;
      }
      if (validation.isBech32(_recipient)) {
        return fromBech32Address(_recipient!);
      }
      if (validation.isAddress(_recipient)) {
        return toBech32Address(_recipient);
      }
    });
  }

  function handleUpdateAddress(newAddress: string) {
    setRecipient(newAddress);
  }

  return {
    recipient,
    recipientEth,
    toggleAddress,
    handleUpdateAddress,
    isAddressValid,
  };
}
