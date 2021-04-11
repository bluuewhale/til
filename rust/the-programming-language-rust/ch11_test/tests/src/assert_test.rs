#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    // 의도적으로 실패하기
    #[test]
    fn panic() {
        panic!("테스트 실패!");
    }
}

// 11.1.2 assert! 매크로 이용해 결과 확인하기
// assert! 매크로는 불리언으로 평가되는 표현식을 인수로 전달하면 된다.

#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        return (self.length > other.length) && (self.width > other.length);
    }
}

#[cfg(test)]
mod rectangle_tests {
    use super::*;

    #[test]
    fn larget_can_hold_smaller() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 1,
        };

        assert!(!smaller.can_hold(&larger))
    }
}

// 11.1.3 assert_eq!와 assert_ne! 매크로를 이용한 동등 비교 테스트
// assert_eq!와 assert_ne! 매크로는 각각 ==와 != 연산자를 사용한다.

// 따라서, 매크로로 전달되는 값들은 PartialEq와 Debug 트레이트를 구현해야 한다.
// 직접 선언한 구조체나 열거자는 개발자가 직접 PartialEq 트레이트를 구현해야 한다.
// 또한 검증이 실패했을 때, 값을 올바르게 출력하려면 Debug 트레이트를 구현해야 한다.
// 두 트레이트는 상속이 가능하므로, 선언 시에 #[derive(PartialEq, Debug)] 애노테이션을 추가하기만 하면 된다.

// 11.1.4 사용자 정의 실패 메시지
// assert 매크로에 실패 메시지를 입력할 수 있는 선택형 인수가 있다.
pub fn greeting(name: &str) -> String {
    return format!("안녕하세요");
}

#[cfg(test)]
mod fail_msg_test {
    use super::*;

    #[test]
    fn custom_fail_msg() {
        let hi = greeting("캐롤");
        assert!(
            hi.contains("캐롤"),
            "greeting 함수의 결과물에 매개변수인 이름이 포함되어 있지 않음. 결괏값: {}",
            hi
        );
    }
}

// 11.1.5 should_panic 매크로를 이용해 패닉의 발생 여부 검증하기
pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Self {
        if value < 1 || value > 100 {
            panic!(
                "반드시 1과 100 사이의 값을 사용해야 합니다. 입력값: {}",
                value
            );
        }
        return Guess { value };
    }
}

// should_panic 매크로를 사용할 때는
// expected 매개변수를 통해 패닉 메시지가 일치하는지 여부를 검사할 수 있다.
#[cfg(test)]
mod should_panic_test {
    use super::Guess;

    #[test]
    #[should_panic(expected = "반드시 1과 100 사이의 값을 사용해야 합니다.")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
