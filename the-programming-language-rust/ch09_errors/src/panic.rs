/* 9.1 panic! 매크로를 이용한 회복 불가능한 에러 처리

해결할 수 없는 에러가 발생한 경우 러스트는 panic! 매크로를 제공한다.
panic! 매크로를 실행하면 프로그램은 실패 메시지를 출력하고 스택을 모두 정리한 다음 종료한다.
 */

pub fn run_panic() {
    panic!("crash and burn");
}

// 9.1.1 panic! 역추적 사용하기
// 코드의 버그로 인해 panic! 매크로가 호출되는 경우
pub fn panic_occurs() -> i32 {
    let v = vec![1, 2, 3];

    return v[99]; // 일반적으로 타 언어에서는, buffer overread 가 발생
}
