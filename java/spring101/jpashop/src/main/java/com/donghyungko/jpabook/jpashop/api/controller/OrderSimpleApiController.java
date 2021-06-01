package com.donghyungko.jpabook.jpashop.api.controller;

import com.donghyungko.jpabook.jpashop.api.util.BaseResponse;
import com.donghyungko.jpabook.jpashop.api.util.SuccessResponse;
import com.donghyungko.jpabook.jpashop.domain.Address;
import com.donghyungko.jpabook.jpashop.domain.Order;
import com.donghyungko.jpabook.jpashop.domain.OrderSearch;
import com.donghyungko.jpabook.jpashop.domain.OrderStatus;
import com.donghyungko.jpabook.jpashop.repository.OrderRepository;
import lombok.*;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

import java.time.LocalDateTime;
import java.util.List;
import java.util.stream.Collectors;

@RestController
@RequiredArgsConstructor
public class OrderSimpleApiController {

    private final OrderRepository orderRepository;

    @GetMapping("/api/v1/simple-orders")
    public List<Order> getOrdersV1() {
        return orderRepository.findAllByString(new OrderSearch());
    }

    @GetMapping("/api/v2/simple-orders")
    public BaseResponse getOrdersV2() {
        List<Order> orders = orderRepository.findAllByString(new OrderSearch());
        List<OrderQueryDto> orderQueryDtoList = orders.stream()
                .map(OrderQueryDto::fromOrder)
                .collect(Collectors.toList());

        return new SuccessResponse(orderQueryDtoList);
    }

    @GetMapping("/api/v3/simple-orders")
    public BaseResponse getOrdersV3() {
        List<Order> orders = orderRepository.findAllWithMemberDelivery();
        List<OrderQueryDto> orderQueryDtoList = orders.stream()
                .map(OrderQueryDto::fromOrder)
                .collect(Collectors.toList());

        System.out.println(orderQueryDtoList.size());

        return new SuccessResponse(orderQueryDtoList);
    }

    @Data
    @NoArgsConstructor(access = AccessLevel.PROTECTED)
    static class OrderQueryDto {
        private Long orderId;
        private String memberName;
        private LocalDateTime orderDate;
        private OrderStatus orderStatus;
        private Address address;

        static public OrderQueryDto fromOrder(Order order) {
            OrderQueryDto orderQueryDto = new OrderQueryDto();
            orderQueryDto.setOrderId(order.getId()); // LAZY 초기화
            orderQueryDto.setMemberName(order.getMember().getName());
            orderQueryDto.setOrderDate(order.getOrderDate());
            orderQueryDto.setOrderStatus(order.getStatus());
            orderQueryDto.setAddress(order.getDelivery().getAddress()); // LAZY 초기화
            return orderQueryDto; // N + 1 문제가 발생함
        }
    }

}

