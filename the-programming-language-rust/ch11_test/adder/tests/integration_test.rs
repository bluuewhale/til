// 11.3.2 통합 테스트
// 러스트에서 통합 테스트는 완전히 라이브러리 영역의 바깥에서 진행된다.
// 라이브러리의 공개 API만 호출 가능
// 통합 테스트는 tests 디렉토리를 생성해야 한다.

// 카고는 tests 디렉토리를 특별히 다루며,
// #[cfg(test)] 특성을 지정하지 않아도 cargo test 명령을 실행할 때만 컴파일한다.
use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}

// 통합 테스트의 서브 모듈
// 통합 테스트의 양이 늘어나면 tests 디렉토리에 하나 이상의 파일을 생성하여 관리
// 기능 별로 그룹화하면 유용하다.
// tests 디렉토리의 각 파일은 별개의 크레이트로 컴파일된다.

// tests/common.rs 파일을 생성하고 setup() 함수를 선언한 후,
// 다른 테스트 파일에서 setup 함수를 호출하여 통합할 수도 있다.

// common 폴더에 mod.rs 파일에 setup 함수를 정의하면
// 카고는 common 모듈을 통합 테스트 파일로 간주하지 않고, 임포트 하여 사용 가능

mod common;
#[test]
fn it_adds_two2() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}

// 바이너리 크레이트(src/main.rs)에 정의된 함수를 가져와 테스트 할 수는 없다.
// 바이너리 크레이트는 코드를 직접 실행할 뿐이다.
