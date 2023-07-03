package com.zilliqa.laksaj.blockchain;

import lombok.Builder;
import lombok.Data;

/** Represents what comes back from a getTransaction() call; this contains the transaction fields and
 *  receipt
 */
@Data
@Builder
public class TransactionData {
    private String ID;
    private String version;
    private String nonce;
    private String amount;
    private String gasPrice;
    private String gasLimit;
    private String signature;
    private TransactionReceipt receipt;
    private String senderPubKey;
    private String toAddr;
    private String code;
    private String data;
}
