/* 9.3 패닉에 빠질 것인가? 말 것인가?
panic! vs Result ??

panic!: 회복 불가능
Result: 회복 가능

*/

// 9.3.1 예제, 프로토타입 코드, 그리고 테스트
// 예제에서는 패닉을 발생시키는 unwrap, expect 매서드르 호출하자

// 9.3.2 컴파일러보다 개발자가 더 많은 정보를 가진 경우
// Ok 값을 리턴할 것이 확실하더라도 unwrap 매서드를 호출하는 것이 좋다.
use std::net::IpAddr;

pub fn unwrap_recommended() {
    // 하드코딩된 문자열일 파싱하는 것이므로, 실패할리가 없다.
    // 그럼에도 불구하고 컴파일러는 Result값이 Err인 경우를 처리하도록 강요한다.
    let home: IpAddr = "127.0.0.1".parse().unwrap();
    println!("{}", home);
}

// 9.3.3 에러 처리를 위한 조언
// 타입 검사는 컴파일러에게

use std::cmp::Ordering;
use std::io;

// 9.3.4 유효성 검사를 위한 커스텀 타입
pub fn validate() {
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("입력하신 숫자를 읽지 못했습니다.");

        let guess = match guess.trim().parse::<i32>() {
            Ok(n) => n,
            Err(_) => {
                continue;
            }
        };

        if guess < 1 || guess > 100 {
            println!("1에서 100 사이의 값을 입력해주세요.");
            continue;
        }
        break;
    }
}

// 개선된 방법
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!(
                "유추한 값은 반드시 1에서 100 사이의 값이어야 합니다. 입력한 값 {}",
                value
            );
        }

        return Guess { value };
    }

    pub fn value(&self) -> i32 {
        return self.value;
    }
}
