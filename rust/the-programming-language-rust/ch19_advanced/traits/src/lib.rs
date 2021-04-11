// 고급 트레잇

// 연관 타입은 트레잇 정의 내에서 플레이스홀더 타입을 명시합니다.
// 연관 타입(associated type)은 타입 플레이스홀더와 트레잇을 연결하여
// 트레잇 메소드 정의를 할 때, 이 플레이스홀더 타입을 시그니처 내에서 사용할 수 있도록 합니다.
// 트레잇을 구현하는 사람은 이 빈칸의 타입이 특정 구현을 위해 사용될 수 있도록 구체 타입을 명시하게 됩니다.

// 이를 통해, 우리는 트레잇이 구현되기 전까지 어떠한 타입이 필요한지 정확하게 알 필요 없이
// 임의의 타입을 사용하는 트레잇을 정의할 수 있습니다.

// 연관 타입을 가진 트레잇의 한 예는 표준 라이브러리에서 제공하는 Iterator 트레잇입니다.
// Item이 정확히 어떤 타입인지는, Iterator 트레잇을 구현할 때 정하면 된다!
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

// 연관 타입 vs 제네릭
// 연관 타입은 트레잇을 정의할 때, 어떤 타입이든 다룰 수 있도록 한다는 점에서
// 제네릭 타입과 유사한 모습을 보입니다.
// 제네릭을 이용하면, 우리는 각 구현마다 타입을 명시해야 하지만, Iterator에 대한 복수 구현이 가능하다
// impl Iterator<String> for Counter {...}
// impl Iterator<u32> for Counter {...}
// 반면, 연관 타입을 사용하면 Item의 타입이 무엇이 될지를 한번만 선택할 수 있다.

// 기본 제네릭 타입 파라미터와 연산자 오버라이팅
// 우리가 제네릭 타입 파라미터를 사용할 때, 해당 제네릭 타입에 대한 기본 구체 타입을 명시할 수 있다.
// 이는 기본 타입이 동작할 경우, 트레잇을 구현한 사람이 구체 타입을 명시해야 하는 수고를 덜어줍니다.
// 제네릭 타입에 대한 기본 타입의 명시 문법은 제네릭 타입을 선언할 때, <PlaceholderType=ConcreteType> 꼴입니다.

// 이 테크닉이 유용한 경우 중 좋은 예가 바로 연산자 오버로딩과 함께 쓰이는 경우입니다.
// 연산자 오버로딩(operator overloading)은 특정산 상황에서(+ 같은) 연산자의 동작을 커스터마이징 하는 것입니다.

// 러스트는 여러분 만의 연산자를 만들거나 임의의 연산자를 오버로딩하는 것을 허용하지 않습니다.
// 하지만, 여러분은 std::ops에 나열되어 있는 연산자와 연관된 구현하는 것으로서
// 연산자 및 관련된 트레잇을 오버로딩 할 수 있습니다.

use std::cmp::PartialEq;
use std::fmt::Debug;
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[test]
fn test_overload_add() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    )
}

// Add 트레잇의 기본 구조
trait Add2<RHS = Self> {
    // 제네릭 타입의 기본 파라미터는 Self
    type Output;
    fn add(self, rhs: RHS) -> Self::Output;
}

// 만약 서로 다른 타입끼리 더하고 싶다면, 기본 제네릭 타입을 오버로드 해야함

fn test_overload_basic_generic_type_parameter() {
    struct Milimeters(u32);
    struct Meters(u32);

    impl Add<Meters> for Milimeters {
        type Output = Milimeters;

        fn add(self, other: Meters) -> Milimeters {
            Milimeters(self.0 + (other.0 * 1000))
        }
    }
}

// 모호성 방지를 위한 완전 정규화(fully qualified) 문법: 동일한 이름의 메소드 호출하기
// 트레이트의 메스드가 동일한 이름을 갖는 경우 발생하는 문제
trait Pilot {
    fn fly(&self);
}

trait Wizzard {
    fn fly(&self);
}
struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("Pilot!")
    }
}
impl Wizzard for Human {
    fn fly(&self) {
        println!("Wizzard!")
    }
}

impl Human {
    fn fly(&self) {
        println!("Human!")
    }
}

#[test]
fn test_duplicated_trait_method() {
    let human = Human {};

    human.fly();
    Pilot::fly(&human);
    Wizzard::fly(&human);
}

// 만약 트레이트 메서드의 일부가 self 파라미터를 갖고 있지 않는 경우에는..?
trait Animal {
    fn name() -> String;
}

struct Dog;

impl Dog {
    fn name() -> String {
        String::from("Dog")
    }
}

impl Animal for Dog {
    fn name() -> String {
        String::from("Dog is Animal!")
    }
}

#[test]
fn test_fully_qualifed_syntax() {
    println!("Name: {}", Dog::name());
    println!("Name: {}", <Dog as Animal>::name());
}

// 슈퍼트레잇(supertrait)을 사용하여 어떤 트레잇 내에서 다른 트레잇의 기능 요구하기
// 예를 들어, 어떤 값을 애스터리스크(*)로 감싸서 출력하는 outline_print 라는 메서드를 가지고 있는
// OutlinePrint라는 트레이트를 만든다고 하자.
// OutlinePrint 트레이트를 구현하는 구조체는 물론 Display 트레이트도 함께 구현하여야 한다.
use std::fmt::{self, Display};
trait OutlinePrint: Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point2 {
    x: i32,
    y: i32,
}

impl Display for Point2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl OutlinePrint for Point2 {}

#[test]
fn test_supertrait() {
    let point = Point2 { x: 3, y: 5 };
    point.outline_print();
}

// 외부 타입에 대해, 외부 트레잇을 구현하기 위한 뉴타입 패턴
// 예를 들어, Vec 타입에 Display 트레잇을 구현하기 위해 VecDisplayWrapper 구조체를 만든다.

use std::ops::Deref;
#[derive(Debug)]
struct VecDisplayWrapper<'a>(&'a Vec<String>);
impl<'a> Display for VecDisplayWrapper<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
// NewType으로 외부 트레잇을 구현하는 경우의 단점은 내부 값에 있는 메서드를 호출할 수 없다는 것이다.
// 이러한 단점은 Deref 트레잇을 구현하여 해결할 수 있다.

impl<'a> Deref for VecDisplayWrapper<'a> {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[test]
fn test_newtype_pattern() {
    let vec = vec![String::from("Hello"), String::from("World")];
    let vec_display = VecDisplayWrapper(&vec);
    println!("vector: {}", vec_display);

    for v in vec_display.iter() {
        println!("v: {}", v);
    }
}
