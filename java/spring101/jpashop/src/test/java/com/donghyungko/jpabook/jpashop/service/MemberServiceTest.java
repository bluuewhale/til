package com.donghyungko.jpabook.jpashop.service;

import com.donghyungko.jpabook.jpashop.domain.Member;
import org.assertj.core.api.Assertions;
import org.junit.jupiter.api.Test;
import org.junit.runner.RunWith;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.test.context.junit4.SpringRunner;
import org.springframework.test.context.junit4.statements.SpringRepeat;
import org.springframework.transaction.annotation.Transactional;

import java.util.Arrays;
import java.util.List;

import static org.assertj.core.api.Assertions.*;
import static org.junit.jupiter.api.Assertions.*;

@RunWith(SpringRunner.class)
@SpringBootTest
@Transactional
class MemberServiceTest {

    @Autowired MemberService memberService;

    @Test
    public void 회원가입() throws Exception {
        //given
        Member member = new Member();
        member.setName("A");
        
        //when
        Long id = memberService.join(member);
        Member findResult = memberService.findOne(id).get();
        
        //then
        assertThat(findResult).isEqualTo(member);
    }

    @Test
    public void 중복된_이름_회원가입_예외() throws Exception {
        //given
        Member member1 = new Member();
        member1.setName("A");

        Member member2 = new Member();
        member2.setName("A");

        //when
        memberService.join(member1);
        try {
            memberService.join(member2); // This must throw Exception
        } catch (IllegalStateException e) {
            return ;
        }

        Assertions.fail("This must throw Exception");
    }

    @Test
    public void findMembers() throws Exception {
        //given
        Member member1 = new Member();
        member1.setName("A");

        Member member2 = new Member();
        member2.setName("B");

        //when
        memberService.join(member1);
        memberService.join(member2);
        List<Member> findResult = memberService.findMembers();

        //then
        assertThat(findResult.size()).isEqualTo(2);
        assertThat(findResult.stream().map((m) -> m.getName())).isEqualTo(
                Arrays.asList(new String[] {"A", "B"})
        );
    }

    @Test
    public void testFindOne() throws Exception {
        //given
        Member member = new Member();
        member.setName("A");

        //when
        Long id = memberService.join(member);
        Member findResult = memberService.findOne(id).get();

        //then
        assertThat(findResult).isEqualTo(member);
    }

    @Test
    public void testFindByName() throws Exception {
        //given
        Member member1 = new Member();
        member1.setName("A");

        //when
        memberService.join(member1);
        List<Member> findResult = memberService.findByName("A");

        //then
        assertThat(findResult.size()).isEqualTo(1);
        assertThat(findResult.stream().map((m) -> m.getName())).isEqualTo(
                Arrays.asList(new String[] {"A"})
        );
    }

}