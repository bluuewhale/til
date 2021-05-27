package com.donghyungko.jpabook.jpashop.api.controller;


import com.donghyungko.jpabook.jpashop.api.util.BaseResponse;
import com.donghyungko.jpabook.jpashop.api.util.ErrorResponse;
import com.donghyungko.jpabook.jpashop.api.util.SuccessResponse;
import com.donghyungko.jpabook.jpashop.domain.Member;
import com.donghyungko.jpabook.jpashop.service.MemberService;
import com.fasterxml.jackson.databind.ser.Serializers;
import lombok.*;
import org.hibernate.sql.Update;
import org.springframework.web.bind.annotation.*;

import javax.validation.Valid;
import javax.validation.constraints.NotEmpty;
import java.util.List;
import java.util.Optional;
import java.util.stream.Collectors;

@RestController
@AllArgsConstructor
public class MemberApiController {

    private MemberService memberService;


    @GetMapping("/api/v1/members")
    public BaseResponse getMembersV1() throws Exception {
        List<Member> members = memberService.findMembers();
        return new SuccessResponse(members);
    }

    @PostMapping("/api/v1/members")
    public BaseResponse saveMemberV1(@RequestBody @Valid Member member) throws Exception {
        Long id = memberService.join(member);
        CreateMemberResponse response = new CreateMemberResponse(id);
        return new SuccessResponse(response);
    }

    @GetMapping("/api/v2/members")
    public BaseResponse getMembersV2() throws Exception {
        List<MemberDto> memberDtoList = memberService.findMembers()
            .stream()
            .map((Member m) -> MemberDto.builder()
                .id(m.getId())
                .name(m.getName())
                .build()
            ).collect(Collectors.toList());

        return new SuccessResponse(memberDtoList);
    }

    @PostMapping("/api/v2/members")
    public BaseResponse saveMemberV2(@RequestBody @Valid CreateMemberRequest request) throws Exception {

        Member member= new Member();
        member.setName(request.getName());

        Long id = memberService.join(member);
        CreateMemberResponse response = new CreateMemberResponse(id);
        return new SuccessResponse(response);
    }

    @GetMapping("/api/v2/members/{id}")
    public BaseResponse getMemberV2(
            @PathVariable("id") Long id,
            @RequestBody @Valid GetMemberRequest request
    ) {
        Optional<Member> result = memberService.findOne(id);
        if (result.isEmpty()) {
            return new ErrorResponse("존재하지 않는 회원입니다.");
        }

        Member member = result.get();
        MemberDto memberDto = MemberDto.builder()
                .id(member.getId())
                .name(member.getName())
                .build();

        return new SuccessResponse(memberDto);
    }

    @PutMapping("/api/v2/members/{id}")
    public BaseResponse updateMemberV2(
            @PathVariable("id") Long id,
            @RequestBody @Valid UpdateMemberRequest request
    ) {
        memberService.update(id, request.getName());

        Optional<Member> result = memberService.findOne(id);
        if (result.isEmpty()) {
            return new ErrorResponse("존재하지 않는 회원입니다.");
        }

        String name = result.get().getName();
        UpdateMemberResponse response = UpdateMemberResponse.builder()
                .id(id)
                .name(name)
                .build();

        return new SuccessResponse(response);
    }

    @Data
    static class CreateMemberRequest {
        private String name;
    }

    @Data
    @AllArgsConstructor
    static class CreateMemberResponse {
        private Long id;
    }

    @Data
    static class UpdateMemberRequest {
        private String name;
    }

    @Data
    @Builder
    static class UpdateMemberResponse {
        private Long id;
        private String name;
    }

    @Data
    static class GetMemberRequest {
    }

    @Data
    @Builder
    static class MemberDto {
        private Long id;
        private String name;
    }
}
