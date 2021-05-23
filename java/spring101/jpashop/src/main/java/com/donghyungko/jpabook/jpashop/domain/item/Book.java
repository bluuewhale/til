package com.donghyungko.jpabook.jpashop.domain.item;

import lombok.Getter;
import lombok.Setter;

import javax.persistence.DiscriminatorValue;
import javax.persistence.Entity;
import javax.persistence.GeneratedValue;

@Entity
@DiscriminatorValue("Book")
@Getter @Setter
public class Book extends Item {
    private String author;
    private String isbn;
}
