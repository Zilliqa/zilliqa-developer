package com.zilliqa.laksaj.transaction;

import lombok.Builder;
import lombok.Data;

@Data
@Builder
public class TxPending {
  private int code;
  private String TxnHash;
}
