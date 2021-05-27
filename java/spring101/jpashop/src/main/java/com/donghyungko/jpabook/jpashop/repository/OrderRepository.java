package com.donghyungko.jpabook.jpashop.repository;

import com.donghyungko.jpabook.jpashop.domain.Order;
import com.donghyungko.jpabook.jpashop.domain.OrderSearch;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Repository;
import org.springframework.util.StringUtils;

import javax.persistence.EntityManager;
import javax.persistence.TypedQuery;
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

    public List<Order> findAllByString(OrderSearch orderSearch) {
        //language=JPAQL
        String jpql = "select o From Order o join o.member m";
        boolean isFirstCondition = true;

        //주문 상태 검색
        if (orderSearch.getOrderStatus() != null) {
            if (isFirstCondition) {
                jpql += " where";
                isFirstCondition = false;
            } else {
                jpql += " and";
            }
            jpql += " o.status = :status";
        }

        //회원 이름 검색
        if (StringUtils.hasText(orderSearch.getMemberName())) {
            if (isFirstCondition) {
                jpql += " where";
                isFirstCondition = false;
            } else {
                jpql += " and";
            }
            jpql += " m.name like :name";
        }
        TypedQuery<Order> query = em.createQuery(jpql, Order.class).setMaxResults(1000); //최대 1000건
        if (orderSearch.getOrderStatus() != null) {
            query = query.setParameter("status", orderSearch.getOrderStatus());
        }
        if (StringUtils.hasText(orderSearch.getMemberName())) {
            query = query.setParameter("name", orderSearch.getMemberName());
        }
        return query.getResultList();
    }

    public List<Order> findAllWithMemberDelivery() {
        return em.createQuery(
        "select o from Order o" +
                " join fetch o.member m" +
                " join fetch o.delivery d",
                Order.class
        ).getResultList();
    }
}