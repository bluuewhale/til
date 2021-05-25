package com.donghyungko.jpabook.jpashop.service;

import com.donghyungko.jpabook.jpashop.domain.Address;
import com.donghyungko.jpabook.jpashop.domain.Member;
import com.donghyungko.jpabook.jpashop.domain.Order;
import com.donghyungko.jpabook.jpashop.domain.OrderStatus;
import com.donghyungko.jpabook.jpashop.domain.item.Book;
import com.donghyungko.jpabook.jpashop.domain.item.Item;
import com.donghyungko.jpabook.jpashop.exception.NotEnoughStockException;
import com.donghyungko.jpabook.jpashop.repository.OrderRepository;
import org.hibernate.boot.model.source.internal.hbm.XmlElementMetadata;
import org.assertj.core.api.Assertions.*;
import org.junit.jupiter.api.Test;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.transaction.annotation.Transactional;

import javax.persistence.EntityManager;
import javax.swing.plaf.metal.MetalMenuBarUI;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.InstanceOfAssertFactories.iterable;
import static org.junit.Assert.assertEquals;
import static org.junit.Assert.fail;

@SpringBootTest
@Transactional
class OrderServiceTest {

    @Autowired EntityManager em;
    @Autowired OrderService orderService;
    @Autowired OrderRepository orderRepository;



    @Test
    public void 주문_생성() throws Exception {
        // given
        Member member = createMember();
        Item book = createBook();
        int orderCount = 2;

        //when
        Long orderId = orderService.order(member.getId(), book.getId(), orderCount);

        //then
        Order findResult = orderRepository.findOne(orderId).get();
        assertEquals(
                "회원에 등록된 Order 필드가, orders 테이블에 저장된 정보와 일치해야 한다",
                member.getOrders().stream().findAny().get(),
                findResult
        );
        assertEquals("상품 주문시 상태는 ORDER", OrderStatus.ORDER, findResult.getStatus());
        assertEquals("주문한 상품 종류 수가 정확해야 한다", 1, findResult.getOrderItems().size());
        assertEquals("주문 가격은 상품 가격 * 상품 수량의 합이다", 10000 * 2, findResult.getTotalPrice());
        assertEquals("상품의 재고는 주문 수량만큼 감소해야 한다", 10 - 2, book.getStockQuantity());

    }
    
    @Test
    public void 주문_취소() throws Exception {
        // given
        Member member = createMember();
        Item book = createBook();
        int orderCount = 5;

        Long orderId = orderService.order(member.getId(), book.getId(), orderCount);

        //when
        orderService.cancelOrder(orderId);

        //then
        Order findResult = orderRepository.findOne(orderId).get();
        assertEquals("상품 취소시 상태는 CANCEL", OrderStatus.CANCEL, findResult.getStatus());
        assertEquals("주문한 상품 종류 수가 정확해야 한다", 1, findResult.getOrderItems().size());
        assertEquals("주문 가격은 상품 가격 * 상품 수량의 합이다", 10000 * 5, findResult.getTotalPrice());
        assertEquals("주문 취소 후에는 상품의 재고 수량이 원상복구 되어야 한다.", 10, book.getStockQuantity());
    }
    
    @Test
    public void 상품주문_재고수량초과() throws Exception {
        //given
        Member member = createMember();
        Item book = createBook();
        int orderCount = 20;

        //when
        try {
            orderService.order(member.getId(), book.getId(), orderCount); // this should throw error
        } catch (NotEnoughStockException e) {
            return;
        }

        //then
        fail("상품수량을 초과한 주문 수량은 에러를 반환하여야 합니다.");
    }

    private Member createMember() {
        Member member = new Member();
        member.setName("A");
        member.setAddress(new Address("서울", "경기", "123-123"));
        em.persist(member);
        return member;
    }

    private Book createBook() {
        Book book = new Book();
        book.setName("시골 JPA");
        book.setPrice(10000);
        book.setStockQuantity(10);
        em.persist(book);
        return book;
    }

}