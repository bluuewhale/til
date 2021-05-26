package com.donghyungko.jpabook.jpashop.form;

import lombok.Getter;
import lombok.Setter;

import javax.validation.constraints.Min;
import javax.validation.constraints.NotBlank;

@Getter @Setter
public class BookForm {

    private Long id;

    @NotBlank(message = "제품명을 입력해주세요")
    private String name;

    @Min(value = 0, message = "올바른 값을 입력해주세요")
    private int price;

    @Min(value = 0, message = "올바른 값을 입력해주세요")
    private int stockQuantity;

    private String author;
    private String isbn;
}
