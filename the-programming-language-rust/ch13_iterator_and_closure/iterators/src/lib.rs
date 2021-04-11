// 13.2 반복자를 이용해 일련의 아이템 처리하기
// 반복자 패턴을 이용하면 어떤 작업을 일련의 아이템을 대상으로 차례로 수행할 수 있다.
// 반복자는 아이템을 순회하면서 마지막 아이템에 도달하는 때를 판단한다.

// 러스트에서 반복자는 지연(lazy) 특성이 있다.
// 반복자는 실제로 사용하는 메서드를 호출하기 전까지는 아무런 일도 일어나지 않는다.
// python의 제너레이터와 유사

pub fn create_iterator() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
}

// 13.2.1 Iterator 트레이트와 next 메서드
// 모든 반복자는 표준 라이브러리에 정의된 Iterator라는 트레이트를 구현한다.
// Iterator 트레이트는 다음과 유사하게 선언되어 있다.
pub trait Iterator2 {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

pub fn call_next_from_iterator() {
    let v1 = vec![1, 2, 3];
    // mut으로 선언한 이유
    // 반복자의 next 메서드를 호출하면, 이미 리턴한 값을 추적하려고 반복자 내부의 상태가 변경됨
    // 즉, next 메서드는 반복자를 '소비(consume)'한다.
    // 위 언급한 Iterator 트레이트의 시그니처를 참고하면,
    // 매개변수가 &mut self로 선언되어 있음
    let mut v1_iter = v1.iter();

    // next로 얻어온 값은, 벡터 안에 저장된 값에 대한 불변 참조다.
    // 소유한 값을 리턴하는 반복자를 생성하려면 iter_into 메서드를 호출해야 한다.
    // 마찬가지로, 가변 참조를 순회하려면 iter_mut 메서드를 호출해야 한다.

    // 정리하면,
    // iter() : 불변 참조
    // iter_mut() : 가변 참조
    // iter_into() : 소유
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

// 13.2.2 반복자를 소비하는 메서드
// Iterator 트레이트는 여러 메서드를 제공한다
// 일부 메서드는 next 메서드를 호출하므로,
// Iterator 트레이트를 구현하려면 next 메서드를 반드시 구현해야 한다.

// next 메서드를 호출하는 메서드는 내부적으로 반복자를 소비하므로
// '소비 어댑터(consuming adaptors'라고 불린다.
// 일례로 sum 메서드는 반복자에 대한 소유권을 가지고(iter_into)
// next 메서드를 계속 호출해서 아이템을 순회한다.

pub fn test_consuming_adaptor() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

// 13.2.3 다른 반복자를 생성하는 메서드
// '반복자 어댑터(iterator adaptors)'라고도 불리는 이 메서드들은
// 반복자를 다른 종류의 반복자로 변경한다.
// 변경된 반복자도 지연(lazy) 특성을 갖고 있으므로,
// 반복자 어댑터를 호출한 이후, 소비 어댑터를 호출하여야 한다.

pub fn test_iterator_adaptor() {
    let v1: Vec<i32> = vec![1, 2, 3];
    // 반복자 어댑터(map)에 클로저를 매개변수로
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}

// 13.2.4 환경을 캡처하는 클로저의 활용
// 보편적으로 filter 반복자 어댑터를 이용해
// 환경을 캡처하는 클로저의 보편적인 사용 방법을 알아보자.
// filter 반복자 어댑터는 각 아이템을 bool 값을 리턴하는 클로저에 전달한다.

use std::fmt::{Debug, Display};
#[derive(Debug, PartialEq)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[test]
fn test_shoes_in_my_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ]
    );
}

// 13.2.5 Iterator 트레이트를 이용해 직접 반복자 구현하기

struct Counter {
    cnt: u32,
}

impl Counter {
    fn new() -> Self {
        return Self { cnt: 0 };
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.cnt += 1;

        if self.cnt < 6 {
            return Some(self.cnt);
        } else {
            return None;
        }
    }
}

#[test]
fn test_counter() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

// 13.2.7 Iterator 트레이트의 다른 메서드 활용하기
// 표준 라이브러리의 Iterator 트레이트를 구현하면
// Itrator 트레이트가 구현해 제공하는 메서드를 활용할 수 있다.
#[test]
fn using_default_methods_in_iteator_trait() {
    // zip은 두 반복자를 묶어주는 튜플의 형태로 묶어주는 역할을 한다.
    // 단, 어느 한쪽이라도 None을 반환하면 None을 반환하여 생성되지 않는다.

    // 작업 흐름
    // zip():  (0, 1), (1, 2), (2, 3), (3, 4)
    // map(): 0, 2, 6, 12
    // filter(): 6, 12
    // sum(): 18
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();

    assert_eq!(18, sum);
}

// 13.3.2 리턴된 반복자를 직접 사용하는 방법
// 반복자를 벡터로 변환하여 매개변수로 주는 것이 아니라,
// 소유권을 갖고 있는 반복자를 매개변수로 직접 전달

// 13.3.3 인덱스 대신 Iterator 트레이트의 메서드를 활용
// for문을 통해 반복자를 하나씩 소비하는 것이 아니라,
// '소비 어댑터'와 '반복자 어댑터'를 적극 활용하자 => 함수형 프로그래밍과 유사

// 반복자는 러스트의 무비용 추상화(zero-cost abstractions) 기능 중 하나다.
// 즉, 추상화를 사용한다고 해서 추가적인 런타임 오버헤드가 발생하지 않는다.

// 러스트는 반복자 어댑터를 통해, 몆 개의 아이템을 순회해야 한다는 것을 미리 알고 루프를 '풀어낸다'.
// '풀어낸다'는 의미는 루프를 제어하는 코드의 오버헤드를 없애기 위해
// 루프를 제거하고 루프 안에서 실행되던 코드를 필요한 횟수만큼 반복하는 코드를 생성한다.

// 클로저와 반복자의 구현은 런타임 성능에 거의 영향을 미치지 않는다.
// 러스트의 목표 중 하나인 무비용 추상화를 달성하기 위한 노력의 결과물!
// 마음껏 사용하자.
