package com.donghyungko.jpabook.jpashop.controller;

import com.donghyungko.jpabook.jpashop.domain.Member;
import com.donghyungko.jpabook.jpashop.domain.OrderSearch;
import com.donghyungko.jpabook.jpashop.domain.item.Item;
import com.donghyungko.jpabook.jpashop.form.OrderForm;
import com.donghyungko.jpabook.jpashop.service.ItemService;
import com.donghyungko.jpabook.jpashop.service.MemberService;
import com.donghyungko.jpabook.jpashop.service.OrderService;
import lombok.RequiredArgsConstructor;
import org.aspectj.weaver.ast.Or;
import org.springframework.stereotype.Controller;
import org.springframework.ui.Model;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.ModelAttribute;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestParam;

import java.util.List;

@Controller
@RequiredArgsConstructor
public class OrderController {

    private final OrderService orderService;
    private final MemberService memberService;
    private final ItemService itemService;

    @GetMapping("/orders")
    public String list(@ModelAttribute("orderSearch")OrderSearch orderSearch, Model model) {
        return "redirect:/";
    }

    @GetMapping("/orders/new")
    public String createForm(Model model) {

        List<Member> members = memberService.findMembers();
        List<Item> items = itemService.findItems();

        model.addAttribute("members", members);
        model.addAttribute("items", items);

        return "/orders/createOrderForm";
    }

    @PostMapping("/orders/new")
    public String create(@RequestParam("memberId") Long memberId,
                         @RequestParam("itemId") Long itemId,
                         @RequestParam("count") int count
    ) {
        orderService.order(memberId, itemId, count);

        return "redirect:/";
    }
}
