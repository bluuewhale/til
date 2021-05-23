package com.donghyungko.jpabook.jpashop.repository;

import com.donghyungko.jpabook.jpashop.domain.Member;
import org.junit.jupiter.api.Test;
import org.junit.runner.RunWith;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.test.annotation.Rollback;
import org.springframework.test.context.junit4.SpringRunner;

import javax.transaction.Transactional;

import static org.junit.jupiter.api.Assertions.*;
import static org.assertj.core.api.Assertions.*;

@SpringBootTest
@RunWith(SpringRunner.class)
class JpaMemberRepositoryTest {

    @Autowired JpaMemberRepository memberRepository;
    private JpaMemberRepository memberRepository1;


    @Test
    public void loadContext() throws Exception {
        assertNotNull(memberRepository);
    }

    @Test
    @Transactional
    public void 회원저장() throws Exception {
        //given
        Member member = new Member();
        member.setName("A");

        //when
        Long saveId = memberRepository.save(member);
        Member findResult = memberRepository.findById(saveId);

        //then
        assertThat(findResult.getId()).isEqualTo(member.getId());
        assertThat(findResult.getName()).isEqualTo(member.getName());
        assertThat(findResult).isEqualTo(member);
    }
}