package com.donghyungko.jpabook.jpashop.service;

import com.donghyungko.jpabook.jpashop.domain.Member;
import com.donghyungko.jpabook.jpashop.repository.MemberRepository;
import lombok.AllArgsConstructor;
import lombok.RequiredArgsConstructor;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.beans.factory.annotation.Required;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import java.util.List;
import java.util.Optional;

@Service
@Transactional(readOnly = true)
@RequiredArgsConstructor
public class MemberService {

    private final MemberRepository memberRepository;

    /**
     * 회원 가입
     * 같은 이름의 회원은 중복 등록 금지
     * @param member
     * @return
     * @throws Exception
     */
    @Transactional
    public Long join(Member member) throws Exception {
        validateDuplicateMember(member);
        memberRepository.save(member);
        return member.getId();
    }

    /**
     * 전체 회원 목록 조회
     * @param
     * @return List<Member>
     */
    public List<Member> findMembers() {
        return memberRepository.findAll();
    }

    /**
     * ID로 회원 조회
     * @param id
     * @return
     */
    public Optional<Member> findOne(Long id) {
        return memberRepository.findById(id);
    }

    /**
     * 이름으로 회원 목록 조회
     * @param name
     * @return
     */
    public List<Member> findByName(String name) {
        return memberRepository.findByName(name);
    }

    // 회원명 중복 여부 확인
    private void validateDuplicateMember(Member member) {
        if (!(memberRepository.findByName(member.getName()).isEmpty())) {
            throw new IllegalStateException("Duplicated entry for member");
        }
    }
}
