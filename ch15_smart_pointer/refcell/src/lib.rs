// 15.5 RefCell<T> 타입과 내부 가변성 패턴
// 내부 가변성(interior mutability)은 러스트가 데이터의 불변 참조를 이용해서
// 데이터를 가공할 수 있도록 지원하는 디자인 패턴이다.

// 내부 가변성은 데이터 구조 안에 unsafe 코드를 사용해서 값의 가공과 대여를 관장하는 러스트의 규칙을 우회한다.

// 15.5.1 RefCell<T> 타입을 이용해 런타임에 대여 규칙 강제하기
// Rc<T>와 달리 RefCell<T> 타입은 자신이 보유한 데이터에 대한 단일 소유권을 표현한다.

// 그렇다면 RefCell<T> 타입과 Box<T> 타입의 차이점은 뭘까?
// 제 4장에서 설명했던 대여 규칙을 다시 살펴보자.
// - 어느 한 시점에 하나의 가변 참조나 여러 개의 불변 참조를 가질 수 있지만, 둘 모두를 가질 수는 없다
// - 참조는 항상 유효해야 한다.

// 참조와 Box<T> 타입의 경우 대여 규칙의 불변성질은 컴파일타임에 평가된다.
// 반면, RefCell<T> 타입은 이런 불변성질이 런타임에 적용된다.

// 러스트 컴파일러는 컴파일타임에 일부 분석을 수행할 수 없으므로,
// 러스트는 코드가 소유권 규칙을 준수하는지를 확인할 수 없어 올바른 프로그램이라도 컴파일을 수행하지 못할 수 있다.
// RefCell<T> 타입은 코드가 대여 규칙을 준수하는 것이 확실하지만 컴파일러가 이를 보장하지 못할 때 유용하다.

// Rec<T> 타입과 마찬가지로 RefCell<T>는 단일 스레드 환경에서만 사용해야 하며,
// 다중 스레드 환경에서 사용하려 하면 컴파일 에러가 발생한다.

// Rc<T>는 같은 데이터에 대한 다중 소유권을 허용한다.
// Box<T>와 RefCell<T> 타입은 단일 소유권만 지원한다.

// Box<T>는 컴파일타임에 가변 또는 불변 대여를 검사한다
// Rc<T>는 컴파일 타임에 불변 대여만을 검사한다.
// RefCell<T>는 런타임에 가변 또는 불변 대여를 검사한다.

// RefCell<T>는 런타임에 가변 대여를 검사하므로 RefCell<T>가 불변이더라도 그 안에 저장된 값을 변경할 수 있다.

// 15.5.2 내부 가변성: 불변 값에 대한 가변 대여
// 대여 규칙에 따르면 불변 값을 변경하기 위해 대여할 수 없다
// 따라서 아래의 코드는 컴파일되지 않는다
pub fn cannot_mut_borrow_immutable_var() {
    let x = 5;
    // let y = &mut x;
}

// 하지만 때로는 이 값을 불변으로 유지하면서도 값이 제공하는 메서드를 통해 값을 변경해야 할 수도 있다.
// 물론, 값의 메서드 외부의 코드는 그 값을 변경할 수 없다.
// 이때 내부 가변성을 지원하는 RefCell<T> 타입을 이용하는 것도 한 방법이다.
// 하지만, RefCell<T> 타입은 대여 규칙을 우회하는 것이 아니다.
// 컴파일러의 대여 검사기는 이런 내부 가변성을 허용하며 대여 규칙의 검사를 런타임에 실행한다.

// (1) 내부 가변성의 활용 예: 모조 객체
// 테스트 더블(test double)은 테스트 과정에서 어떤 타입의 역할을 대신하는 타입을 일컫는 프로그래밍 용어다.
// 모조 객체(Mock objects)는 테스트 더블의 구체적인 형태 중 하나로 테스트 중에 일어났던 일들을 기록해서
// 의도했던 동작이 이루어졌는지를 확인하기 위한 용도로 사용하는 객체다.

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
    warning: f64,
    alert: f64,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> Self {
        Self {
            messenger,
            value: 0,
            max,
            warning: 0.75,
            alert: 0.9,
        }
    }
    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let pct_of_max = self.value as f64 / self.max as f64;

        if pct_of_max > self.warning && pct_of_max < self.alert {
            self.messenger.send("경고: 최대값의 75%를 사용했습니다.");
        } else if pct_of_max > self.alert && pct_of_max < 1.0 {
            self.messenger
                .send("긴급 경고: 최댓값의 90%를 사용했습니다.");
        } else if pct_of_max > 1.0 {
            self.messenger.send("에러: 최댓값을 초과했습니다.")
        }
    }
}

// sent_messages 필드를 RefCell 타입으로 선언함

// Message Trait의 send 메서드는 불변참조
// 그러나, LimitTracker의 set_value 메서드는
// 내부적으로 Messenger Trait를 구현한 MockMessenger의 send 메서드를 호출하는데
// MockMessenger의 send 매서드는 내부적으로 sent_messages라는 필드의 값을 수정한다.
// 따라서, 가변 참조로 대여해야 하는데, Messenger 트레이트의 인터페이스는 불변 참조를 원칙으로 한다.
// 불변 참조를 유지하면서 가변 참조로 대여하여 내부적으로 값을 수정할 수 있도록 하려면, 해당 필드를 RefCell 타입으로 선언해야 한다!
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> Self {
            Self {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message)); // self는 불변참조 대여 했지만 필드는 가변 참조
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }

    #[test]
    #[should_panic]
    fn two_mut_borrow_sholud_panic() {
        impl MockMessenger {
            fn send2(&self, message: &str) {
                // 두 개의 가변 참조를 생성하였으므로, 런타임에 패닉이 발생
                let mut one_borrow = self.sent_messages.borrow_mut();
                let mut two_borrow = self.sent_messages.borrow_mut();

                one_borrow.push(String::from(message));
                two_borrow.push(String::from(message));
            }
        }
        let mock_messenger = MockMessenger::new();
        mock_messenger.send2("panic!");
    }
}

// 15.5.3 Rc<T>와 RefCell<T>를 조합해서 가변 데이터에 다중 소유권 적용하기
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}
use std::cell::RefCell;
use std::rc::Rc;
use List::{Cons, Nil};
#[test]
fn rc_with_refcell() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a 수정 후 = {:?}", a);
    println!("b 수정 후 = {:?}", b);
    println!("c 수정 후 = {:?}", c);
}
