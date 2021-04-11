// 15.3 Drop 트레이트를 구현해서 메모리를 해제할 때 코드 실행하기
// 스마트 포인터에 있어 중요한 두 번째 트레이트는 Drop이다.
// Drop 트레이트는 어떤 타입에도 구현할 수 있으며, 파일이나 네트워크 연결 같은 자원을 해제하는 코드를 명시할 수 있다.
// 스마트 포인터를 구현할 때는 거의 항상 Drop 트레이트의 기능을 사용한다.

use std::fmt::Display;
struct MyPointer<T>
where
    T: Display,
{
    data: T,
}

impl<T> Drop for MyPointer<T>
where
    T: Display,
{
    fn drop(&mut self) {
        println!("MyPoint의 데이터 '{}'를 해제합니다.", self.data);
    }
}

#[test]
fn test_impl_drop_trait() {
    let hi = MyPointer {
        data: String::from("안녕하세요"),
    };
    let hello = MyPointer {
        data: String::from("반갑습니다"),
    };

    // hi.drop(); 명시적으로 drop 메서드를 호출하면 컴파일 에러가 발생함
}

use std::mem::drop;

#[test]
fn test_custom_drop() {
    let hi = MyPointer {
        data: String::from("안녕하세요"),
    };
    let hello = MyPointer {
        data: String::from("반갑습니다"),
    };
    drop(hi);
}
