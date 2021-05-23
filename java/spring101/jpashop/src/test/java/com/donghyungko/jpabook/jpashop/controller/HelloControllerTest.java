package com.donghyungko.jpabook.jpashop.controller;

import org.junit.jupiter.api.Test;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.stereotype.Controller;

import static org.junit.jupiter.api.Assertions.*;

@SpringBootTest
class HelloControllerTest {

    @Autowired
    private HelloController helloController;

    @Test
    public void loadContext() {
        assertNotNull(this.helloController);
    }
}