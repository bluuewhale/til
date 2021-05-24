package com.donghyungko.jpabook.jpashop.repository;

import com.donghyungko.jpabook.jpashop.domain.Member;
import org.junit.jupiter.api.Test;
import org.junit.runner.RunWith;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.test.context.junit4.SpringRunner;

import javax.transaction.Transactional;

import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;

import static org.junit.jupiter.api.Assertions.*;
import static org.assertj.core.api.Assertions.*;

@SpringBootTest
@RunWith(SpringRunner.class)
class MemberRepositoryTest {

    @Autowired MemberRepository memberRepository;
    private MemberRepository memberRepository1;


    @Test
    public void loadContext() throws Exception {
        assertNotNull(memberRepository);
    }

    @Test
    @Transactional
    public void 회원저장_및_ID로_회원조회() throws Exception {
        //given
        Member member = new Member();
        member.setName("A");

        //when
        Long saveId = memberRepository.save(member);
        Member findResult = memberRepository.findById(saveId).get();

        //then
        assertThat(findResult.getId()).isEqualTo(member.getId());
        assertThat(findResult.getName()).isEqualTo(member.getName());
        assertThat(findResult).isEqualTo(member);
    }

    @Test
    @Transactional
    public void 이름으로_회원목록조회() throws Exception {
        //given
        Member member1 = new Member();
        member1.setName("A");

        Member member2 = new Member();
        member2.setName("B");

        //when
        memberRepository.save(member1);
        memberRepository.save(member2);
        List<Member> findResult = memberRepository.findByName("A");

        //then
        assertThat(findResult.size()).isEqualTo(1);
        assertThat(findResult.stream().map((m) -> m.getName()).collect(Collectors.toList())).isEqualTo(
                Arrays.asList(new String[] { "A" })
        );

    }

    @Test
    @Transactional
    public void 전체회원목록조회() throws Exception {
        //given
        Member member1 = new Member();
        member1.setName("A");

        Member member2 = new Member();
        member2.setName("B");

        //when
        Long saveId = memberRepository.save(member1);
        Long saveId2 = memberRepository.save(member2);
        List<Member> findResult = memberRepository.findAll();

        //then
        assertThat(findResult.size()).isEqualTo(2);
        assertThat(findResult.stream().map((m) -> m.getName()).collect(Collectors.toList())).isEqualTo(
            Arrays.asList(new String[] { "A", "B" })
        );
    }
}