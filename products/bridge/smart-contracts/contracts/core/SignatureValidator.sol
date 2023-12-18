// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import {EnumerableSet} from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";

interface ISignatureValidatorErrors {
    error NonUniqueOrUnorderedSignatures();
    error InvalidValidatorOrSignatures();
    error NoSupermajority();
}

library SignatureValidator {
    using ECDSA for bytes32;
    using EnumerableSet for EnumerableSet.AddressSet;

    function isSupermajority(
        EnumerableSet.AddressSet storage self,
        uint count
    ) internal view returns (bool) {
        return count * 3 > self.length() * 2;
    }

    /**
     * @dev Checks signatures are unique, ordered and valid against message hash
     * and forms a supermajority
     * errors [NonUniqueOrUnorderedSignatures, InvalidValidatorOrSignatures, NoSupermajority]
     */
    function validateSignaturesWithSupermajority(
        EnumerableSet.AddressSet storage self,
        bytes32 ethSignedMessageHash,
        bytes[] calldata signatures
    ) internal view {
        address lastSigner = address(0);

        for (uint i = 0; i < signatures.length; ++i) {
            address signer = ethSignedMessageHash.recover(signatures[i]);
            if (signer <= lastSigner) {
                revert ISignatureValidatorErrors
                    .NonUniqueOrUnorderedSignatures();
            }
            if (!self.contains(signer)) {
                revert ISignatureValidatorErrors.InvalidValidatorOrSignatures();
            }
            lastSigner = signer;
        }

        if (!isSupermajority(self, signatures.length)) {
            revert ISignatureValidatorErrors.NoSupermajority();
        }
    }
}
