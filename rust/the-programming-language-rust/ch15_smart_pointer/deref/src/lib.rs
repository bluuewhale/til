// 15.2 Deref 트레이트를 이용해 스마트 포인터를 참조처럼 취급하기
// Deref 트레이트를 구현하면 역참조 연산자(*)의 동작을 변경할 수 있다.
// 또한, 스마트 포인터를 참조처럼 취급할 수 있어
// 참조를 사용하는 코드를 그대로 스마트 포인터에도 적용할 수 있다.

// 먼저, 역참조 연산자가 참조를 어떻게 처리하는지부터 살펴보자.
// 그런 다음 Box<T>처럼 동작하는 사용자 정의 타입을 선언하고,
// 이 타입에 대해서 역참조 연선자가 제대로 동작하지 않는 이유에 대해서 살펴본다.
// 그리고 스마트 포인터가 참조와 같은 방식으로 동작하도록 Deref 트레이트를 구현한 후,
// 러스트의 역참조 강제 기능을 통해 스마트 포인터를 똑같이 취급하는 방법을 설명한다.

// 15.2.1 역참조 연산자를 이용해 포인터가 가르키는 값 읽어오기
// 보통 사용하는 참조는 일종의 포인터로 어딘가에 저장된 값을 가르키는 이정표다.

#[test]
fn test_pointer() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

// 15.2.2 Box<T>를 참조처럼 사용하기
// 위 예제에서 참조를 Box<T>로 대체할 수 있다.
#[test]
fn test_replace_ref_with_box_t() {
    let x = 5;
    let y = Box::new(x); // 참조와 유사

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

// 15.2.3 직접 구현하는 스마트 포인터
// 표준 라이브러리가 지원하는 Box<T>와 유사한 스마트 포인터를 직접 구현하면서
// 스마트 포인터가 참조와 어떻게 다른지 알아보자.
// 그런 후, 역참조 연산자를 지원하기 위한 기능을 추가(Deref 트레이트 구현)하자.

use std::ops::Deref;
struct MyBox<T>(T); // 하나의 원소를 갖는 튜플 구조체
impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        return Self(x);
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
#[test]
fn test_my_box() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, *y);
    assert_eq!(5, *(y.deref()));
}

// 15.2.5 함수와 메서드에 묵시적인 Deref 강제하기
// 역참조 강제(Deref coercion)는 러스트가 함수와 메서드의 인수에 대해 수행하는 편의 장치다.
// 역참조 강제는 Deref 트레이트를 구현하는 타입의 참조를 Deref 트레이트가 변환하는
// 원래의 타입에 대한 참조로 변환한다. ex) &String  -> &str

#[test]
fn test_deref_coercion() {
    fn hello(name: &str) {
        println!("안녕하세요 {}!", name);
    }
    let my_box = MyBox::new(String::from("Rust"));
    hello(&my_box); // &string -> &str 자동 변환
}

// 15.2.6 역참조 강제와 가변성
// Deref 트레이트가 불변 참조에 대한 * 연산자의 동작을 재정의하는 것처럼
// DerefMut 트레이트는 가변 참조에 대한 * 연산자의 동작을 재정의한다.
// Deref: 불변 참조에 대한 역참조
// DerefMut: 가변 참조에 대한 역참조
