// 11.3 테스트의 조직화
// 단위 테스트
// 작고 집중적이며 한 번에 하나의 모듈을 독립적으로 테스트
// 비공개 인터페이스도 테스트 가능

// 통합 테스트
// 완전히 라이브러리 이외의 것을 테스트
// 외부 코드와 같은 방법으로 테스트 코드를 사용
// 공개 인터페이스만 테스트 가능
// 하나의 테스트가 여러 개의 모듈을 활용 가능

// 11.3.1 단위 테스트
// 보통 단위 테스트는 src 디렉토리의 각 파일에 테스트할 코드를 함께 작성한다.
// 각 파일에 tests라는 모듈을 cfg(test)라는 특성과 함께 선언하고, 그 안에 테스트 코드 작성

// (1) Tests 모듈과 #[cfg(test)] 특성
// tests 모듈의 #[cfg(test)] 특성은 러스트에게 이 모듈은
// cargo test 명령을 실행할 때만 코드를 컴파일하고 실행하라고 지시하는 것이다.
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// (2) 비공개 함수의 테스트
pub fn add<'a>(a: &'a i32, b: &'a i32) -> i32 {
    return internal_adder(a, b);
}

fn internal_adder<'a>(a: &'a i32, b: &'a i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests_add {
    use super::{add, internal_adder};

    #[test]
    fn test_add() {
        assert_eq!(2 + 2, add(&2, &2));
    }

    fn test_internal_adder() {
        assert_eq!(2 + 2, internal_adder(&2, &2));
    }
}
