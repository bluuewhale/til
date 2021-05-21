package com.donghyungko.hellospring.service;

import com.donghyungko.hellospring.domain.Member;
import com.donghyungko.hellospring.repository.MemberRepsitory;
import com.donghyungko.hellospring.repository.MemoryMemberRepository;

import java.util.List;
import java.util.Optional;

public class MemberService {

    private final MemberRepsitory memberRepository ;

    public MemberService(MemberRepsitory memberRepository) {
        this.memberRepository = memberRepository;
    }


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
