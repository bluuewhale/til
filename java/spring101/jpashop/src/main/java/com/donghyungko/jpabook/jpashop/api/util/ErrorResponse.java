package com.donghyungko.jpabook.jpashop.api.util;

import lombok.Data;

@Data
public class ErrorResponse extends BaseResponse {
    private String errorMessage;
    private String errorStatus;

    public ErrorResponse(String errorMessage) {
        this.errorMessage = errorMessage;
        this.errorStatus = "404";
    }
    public ErrorResponse(String errorMessage, String errorStatus) {
        this.errorMessage = errorMessage;
        this.errorStatus = errorStatus;
    }
}
