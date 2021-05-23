package com.donghyungko.hellospring.service;

import com.donghyungko.hellospring.domain.Member;
import com.donghyungko.hellospring.repository.MemberRepository;
import org.springframework.beans.factory.annotation.Autowired;

import javax.transaction.Transactional;
import java.util.List;
import java.util.Optional;

@Transactional
public class MemberService<T extends MemberRepository>{
    private final T memberRepository ;

    @Autowired
    public MemberService(T memberRepository) {
        this.memberRepository = memberRepository;
    }

    /***
     * 이름 중복 검사
     * 같은 이름의 회원이 있을 경우, IllegalStateExection을 발생시킨다.
     */
    private void validateDuplicateMember(Member member) {
        memberRepository.findByName(member.getName()).ifPresent(m -> {
            throw new IllegalStateException("이미 존재하는 회원입니다.");
        });
    }
    /***
     * 회원 가입
     * 같은 이름이 있는 중복 회원은 등록할 수 없다.
     */
    public Long join(Member member) {
        validateDuplicateMember(member); // 중복 회원 검증
        memberRepository.save(member);
        return member.getId();
    }

    /***
     * 전체 회원 조회
     */
    public List<Member> findMembers() {
        return memberRepository.findAll();
    }

    /***
     * 회원 한명 조회
     */
    public Optional<Member> findOne(Long memberId) {
        return memberRepository.findById(memberId);
    }

}
