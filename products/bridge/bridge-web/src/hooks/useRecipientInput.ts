import { fromBech32Address, toBech32Address } from "@zilliqa-js/crypto";
import { validation } from "@zilliqa-js/util";
import { useEffect } from "react";
import { useAccount } from "wagmi";
import { create } from "zustand";

interface RecipientState {
  recipient?: string;
  setRecipient: (newRecipient: string) => void;
  toggleAddress: () => void;
}

const useRecipientStore = create<RecipientState>((set) => ({
  setRecipient: (newRecipient: string) =>
    set(() => ({ recipient: newRecipient })),
  toggleAddress: () =>
    set(({ recipient: _recipient }) => {
      if (_recipient) {
        if (validation.isBech32(_recipient)) {
          return { recipient: fromBech32Address(_recipient!) };
        }
        if (validation.isAddress(_recipient)) {
          return { recipient: toBech32Address(_recipient) };
        }
      }

      return { recipient: _recipient };
    }),
}));

export default function useRecipientInput() {
  const { recipient, setRecipient, toggleAddress } = useRecipientStore(
    (state) => state
  );

  const { address: account } = useAccount();

  const validBech32Address = recipient && validation.isBech32(recipient);
  const validEthAddress = recipient && validation.isAddress(recipient);

  const isAddressValid = !recipient || validBech32Address !== validEthAddress;

  const recipientEth = validBech32Address
    ? (fromBech32Address(recipient) as `0x${string}`)
    : (recipient as `0x${string}` | undefined);

  useEffect(() => {
    account && setRecipient(account);
  }, [account, setRecipient]);

  function handleUpdateRecipient(newAddress: string) {
    setRecipient(newAddress);
  }

  return {
    recipient,
    recipientEth,
    toggleAddress,
    handleUpdateRecipient,
    isAddressValid,
  };
}
