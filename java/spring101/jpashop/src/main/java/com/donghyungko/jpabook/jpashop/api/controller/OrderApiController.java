package com.donghyungko.jpabook.jpashop.api.controller;

import com.donghyungko.jpabook.jpashop.api.util.BaseResponse;
import com.donghyungko.jpabook.jpashop.api.util.SuccessResponse;
import com.donghyungko.jpabook.jpashop.domain.Order;
import com.donghyungko.jpabook.jpashop.domain.OrderItem;
import com.donghyungko.jpabook.jpashop.domain.OrderSearch;
import com.donghyungko.jpabook.jpashop.repository.OrderRepository;
import com.donghyungko.jpabook.jpashop.service.OrderService;
import lombok.AllArgsConstructor;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.List;

@RestController
@AllArgsConstructor
public class OrderApiController {

    private OrderService orderService;
    private OrderRepository orderRepository;

    @GetMapping("/api/v1/orders")
    public BaseResponse ordersV1() {
        List<Order> orders = orderRepository.findAllByString(new OrderSearch());
        for (Order order: orders) { // enforce query
            order.getMember().getName();
            order.getDelivery().getAddress();
            order.getOrderItems().stream().forEach(
                    (OrderItem o) -> o.getItem().getName()
            );
        }
        return new SuccessResponse(orders);
    }
}
