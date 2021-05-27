package com.donghyungko.jpabook.jpashop.api.util;


import com.donghyungko.jpabook.jpashop.domain.*;
import com.donghyungko.jpabook.jpashop.domain.item.Book;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Component;
import org.springframework.transaction.annotation.Transactional;

import javax.annotation.PostConstruct;
import javax.persistence.EntityManager;

/**
 * 총 주문 2개
 * * userA
 *   * JPA1 BOOK
 *   * JPA2 BOOK
 * * userB
 *   * SPRING1 BOOK
 *   * SPRING2 BOOK
 */
@Component
@RequiredArgsConstructor
public class InitDb {

    private final InitService initService;

    @PostConstruct
    public void init() {
        initService.dbInit1();
        initService.dbInit2();
    }

    @Component
    @Transactional
    @RequiredArgsConstructor
    static class InitService {
        private final EntityManager em;
        private Member createMember(String name, String city, String street, String zipcode) {
            Member member = new Member();
            member.setName(name);
            member.setAddress(new Address(city, street, zipcode));
            return member;
        }

        @Transactional
        public void dbInit1() {
            Member member = createMember("userA", "Seoul", "1", "111-111");
            em.persist(member);

            Book book = createBook("JPA1 BOOK", 10000);
            Book book2 = createBook("JPA2 BOOK", 20000);
            em.persist(book);
            em.persist(book2);

            OrderItem orderItem1 = OrderItem.createOrderItem(book, 10000, 1);
            OrderItem orderItem2 = OrderItem.createOrderItem(book2, 20000, 1);

            Delivery delivery = createDelivery(member);

            Order order = Order.createOrder(member, delivery, orderItem1, orderItem2);
            em.persist(order);
        }

        @Transactional
        public void dbInit2() {
            Member member = createMember("userB", "Seoul", "1", "111-111");
            em.persist(member);

            Book book = createBook("SPRING1 BOOK", 10000);
            Book book2 = createBook("SPRING2 BOOK", 20000);
            em.persist(book);
            em.persist(book2);

            OrderItem orderItem1 = OrderItem.createOrderItem(book, 10000, 1);
            OrderItem orderItem2 = OrderItem.createOrderItem(book2, 20000, 1);

            Delivery delivery = createDelivery(member);

            Order order = Order.createOrder(member, delivery, orderItem1, orderItem2);
            em.persist(order);
        }

        private Delivery createDelivery(Member member) {
            Delivery delivery = new Delivery();
            delivery.setAddress(member.getAddress());
            return delivery;
        }

        private Book createBook(String name, int price) {
            Book book = new Book();
            book.setName(name);
            book.setPrice(price);
            book.setStockQuantity(100);
            return book;
        }
    }
}

