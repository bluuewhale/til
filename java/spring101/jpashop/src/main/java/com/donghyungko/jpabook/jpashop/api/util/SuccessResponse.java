package com.donghyungko.jpabook.jpashop.api.util;

import lombok.Data;

import java.util.List;

@Data
public class SuccessResponse<T> extends BaseResponse {
    private T data;
    private int count;

    public SuccessResponse (T data) {
        this.data = data;
        if (data instanceof List) {
            this.count = ((List<?>) data).size();
        } else {
            this.count = 1;
        }
    }
}
