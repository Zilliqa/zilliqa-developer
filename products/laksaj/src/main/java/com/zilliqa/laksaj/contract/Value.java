package com.zilliqa.laksaj.contract;

import lombok.Builder;
import lombok.Data;

@Data
@Builder
public class Value {
    private String vname;
    private String type;
    private Object value;
}

