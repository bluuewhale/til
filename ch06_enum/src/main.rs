fn main() {
    // 6.1 열거자 정의하기
    test_enum();
    test_enum2();
    test_enum3();

    // 6.2.2 Option<T>를 이용한 매칭
    test_option_match();
}

// 6.1 열거자 정의하기
// 열거자는 나열한 값 중에 반드시 하나만 사용 가능

enum IPAddrKind {
    V4, // 열것값(Variant)
    V6,
}

struct IPAddr {
    kind: IPAddrKind,
    address: String,
}

fn test_enum() {
    let home = IPAddr {
        kind: IPAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IPAddr {
        kind: IPAddrKind::V6,
        address: String::from("::1"),
    };
}

// 6.1.1 열거자의 값
// 열거값에 데이터를 직접 넣을 수 있다.
enum IPAddrEnum {
    V4(String),
    V6(String),
}

fn test_enum2() {
    let home = IPAddrEnum::V4(String::from("127.0.0.1"));
    let loopback = IPAddrEnum::V6(String::from("::1"));
}

enum Message {
    Quit,
    Move { x: i32, y: i32 }, // 익명의 구조체를 열것값으로 사용
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    // impl 예약어를 통한 열거자 확장
    fn call(&self) {
        // do something
    }
}

fn test_enum3() {
    let m = Message::Write(String::from("Hello"));
    m.call();
}

// 6.1.2 Option 열거자를 Null값 대신 사용
// 중요: Rust에는 null 개념이 없고, 유일하게 Option enum의 열것값으로 None이라는 값이 존재한다.
// 따라서, 어떤 값이 null인지 아닌지 확인하려면, Option 열거자의 열것값인 None과 비교하면 된다.

// Option 열거자
// 어떤 값의 존재 여부를 포현하는 열거자
// 프렐류드에 포함되어 잇어서, 명시적으로 namespace를 가져오지 않아도 된다. (접두어 필요 없음)
// <T>는 제네릭 타입 매개변수를 의미 (어떤 타입의 데이터도 다 수용 가능)

// 어떤 값이 None값을 가질 수 있도록 하려면 Option<T> 타입을 명시적으로 사용해야 한다.
// 그런 후, 이 값을 사용하고자 하면 이 값이 널인 경우를 반드시 명시적으로 처리해주어야 한다.

fn test_option_enum() {
    let some_number = Some(5);
    let some_string = Some("a string");

    //let absent_number: Option<{ unknown }> = None;
}

// 6.2 Match
// if문의 표현식은 반드시 bool type을 return 해야하지만,
// match는 어떤 타입의 값도 사용할 수 있다.
enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("행운의 페니!");
            return 1;
        }
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// 6.2.1 값을 바인딩하는 패턴
// match 표현식의 가지 표현식의 또 다른 장점은 패턴에 일치하는 값의 일부를 바인딩할 수 있다.
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin2 {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

fn value_in_cents2(coin: Coin2) -> u32 {
    match coin {
        Coin2::Penny => 1,
        Coin2::Nickle => 5,
        Coin2::Dime => 10,
        Coin2::Quarter(state) => {
            // Quarter의 열것값인 UsState가 state변수에 바인딩 됨!
            println!("State quarter from {:?}!", state);
            return 25;
        }
    }
}

// 6.2.2 Option<T>를 이용한 매칭
// Option<T> 타입이 Some(T) 값을 가질 때, 그 안에 저장된 T 값을 꺼내 쓸 수 있다.
// Option<T>는 마찬가지로 match 표현식에서도 사용할 수 있다.
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn test_option_match() {
    let five: Option<i32> = Option::Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

// 열거자와 match 표현식의 조합은 + 변수 바인딩은 유용하게 사용됨

// 6.2.3 match는 반드시 모든 경우를 처리해야 한다
// Rust에서는 match 표현식에서 누락된 패턴이 있는지 컴파일러가 엄격하게 검사한다.

// 6.2.4 자리지정자 "_"
// 모든 경우를 다 처리하고 싶지 않은 경우 "_"(placeholder)를 활용하여 예외처리가 가능하다.
// _ 는 나머지 모든 패턴과 일치함을 의미
fn test_placeholder() {
    let some_u8_value: u8 = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        _ => (),
    }
}

// 6.3 if let을 이용한 간결한 흐름 제어
fn test_if_let() {
    let some_u8_value: Option<u8> = Some(0u8);
    // 아래 match 식은 너무 장황함
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // if let 으로 간결하게 표현
    if let Some(3) = some_u8_value {
        println!("three");
    }
}
