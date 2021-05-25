package com.donghyungko.jpabook.jpashop.repository;

import com.donghyungko.jpabook.jpashop.domain.Order;
import com.donghyungko.jpabook.jpashop.domain.OrderSearch;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Repository;

import javax.persistence.EntityManager;
import java.util.List;
import java.util.Optional;

@Repository
@RequiredArgsConstructor
public class OrderRepository {

    private final EntityManager em;

    public void save(Order order) {
        em.persist(order);
    }

    public Optional<Order> findOne(Long id) {
        return Optional.ofNullable(em.find(Order.class, id));
    }


    public List<Order> findAll(OrderSearch orderSearch) {
        // 동적 쿼리
        String query = "" +
                "select o from Order o " +
                "join o.member m" +
                "where m.name = like :name" +
                "and o.status = :status";

        return em.createQuery(query, Order.class)
                .setParameter("name", orderSearch.getMemberName())
                .setParameter("status", orderSearch.getOrderStatus())
                .getResultList();

    }


}
