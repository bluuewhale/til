package com.donghyungko.hellospring.service;

import com.donghyungko.hellospring.repository.MemberRepository;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

import javax.persistence.EntityManager;
import javax.persistence.PersistenceContext;
import javax.sql.DataSource;

/**
 * 자바 코드를 활용하여 Bean을 직접 등록하는 방법
 * 대부분의 경우, 컴포넌트 스캔을 활용하는 것이 편리하다
 * 그러나, 상황에 따라 구현 클래스를 변경(ex, MemoryMemberRepository -> MysqlMemberRepository)해야 하는 경우,
 * 변경지점을 최소화할 수 있다.
 */

@Configuration
public class SpringConfig {

    private final MemberRepository memberRepository;

    public SpringConfig(MemberRepository memberRepository) {
        this.memberRepository = memberRepository;
    }

    @Bean
    public MemberService memberService() {
        return new MemberService(memberRepository);
    }
}