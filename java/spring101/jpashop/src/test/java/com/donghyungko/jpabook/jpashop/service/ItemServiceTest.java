package com.donghyungko.jpabook.jpashop.service;

import com.donghyungko.jpabook.jpashop.domain.Category;
import com.donghyungko.jpabook.jpashop.domain.item.Book;
import com.donghyungko.jpabook.jpashop.domain.item.Item;
import org.junit.jupiter.api.Test;
import org.junit.runner.RunWith;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.test.context.junit4.SpringRunner;
import org.springframework.transaction.annotation.Transactional;

import java.util.Arrays;

import static org.junit.jupiter.api.Assertions.*;
import static org.assertj.core.api.Assertions.*;

@RunWith(SpringRunner.class)
@SpringBootTest
@Transactional
class ItemServiceTest {

    @Autowired
    ItemService itemService;

    @Test
    public void 아이템_저장() throws Exception {
        //given
        Book book = new Book();
        book.setName("the great adventure");
        book.setPrice(10000);
        book.setIsbn("AAAAAA");
        book.setAuthor("Donghyung Ko");
        book.setStockQuantity(100);

        //when
        itemService.saveItem(book);
        Item findResult = itemService.findItem(book.getId()).get();

        //then
        assertThat(findResult).isEqualTo(book);

    }

}