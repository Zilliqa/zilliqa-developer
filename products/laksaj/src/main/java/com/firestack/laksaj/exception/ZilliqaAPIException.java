package com.firestack.laksaj.exception;

import lombok.Data;
import lombok.EqualsAndHashCode;

@Data
@EqualsAndHashCode(callSuper=true)
public class ZilliqaAPIException extends Exception {
    private String message;
    private int code;

    public ZilliqaAPIException(String message, int code) {
        this.message = message;
        this.code = code;
    }
}
