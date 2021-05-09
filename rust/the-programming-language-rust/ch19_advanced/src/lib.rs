// 19.1 안전하지 않은 러스트
// 현재까지 작성한 코드들은 컴파일 타임에 강제되는 러스트의 메모리 안전성 보장을 갖는다.
// 그러나, 러스트는 이러한 메모리 안전성 보장을 강제하지 않는 숨겨진 내부의 두번째 언어를 갖는다.
// 이것을 안전하지 않은 러스트(unsafe Rust)라고 부른다

// 19.1.1 안전하지 않은 슈퍼파워
// 안전하지 않은 러스트로 전환하기 위해서는 unsafe 키워드를 사용한다.
// 그 다음 안전하지 않은 코드를 감싸주는 새 블록을 시작한다.

// 안전하지 않은 러스트에서 제공하는 4가지 기능
// 1. 로우 포인터(raw pointer)를 역참조하기
// 2. 안전하지 않은 함수 혹은 메소드 호출하기
// 3. 가변 정적 변수(mutable static variable)의 접근 혹은 수정하기
// 4. 안전하지 않은 트레잇 구현하기

// unsafe가 대여 검사기나, 러스트의 안전성 검사 기능을 끄는 것은 아니다.
// 단지, 위에 언급된 4개의 기능을 추가적으로 허용해주는 것이다.

// 19.1.2 로우 포인터를 역참조하기
// 러스트에서는 참조자들이 언제나 유효함을 컴파일러가 보장한다.
// 안전하지 않은 러스트는 로우 포인터(raw pointer)라고 불리는 참조자와 유사한 두 가지 새로운 타입을 갖습니다.
// 참조자를 이용하는 것처럼 로우 포인터도 불변 혹은 가변이 될 수 있으며,
// 각각 *const T(불변 로우 포인터)와 *mut T(가변 로우 포인터)라고 씁니다.

// 참조자나 스마트 포인터와는 다르게, 아래와 같은 로우 포인터의 성질을 명심하세요.
// 로우 포인터는 빌림 규칙 무시가 허용되어, 불변 및 가변 로우 포인터를 동시에 가질 수 있다.
// 로우 포인터는 유효한 메모리를 가리키고 있음을 보장하지 않는다.
// 로우 포인터는 널이 될 수 있다.
// 로우 포인터는 자동 메모리 정리가 구현되어 있지 않다.

#[test]
fn test_create_raw_pointer() {
    let mut num = 5;

    // 이미 유효성이 보장된 참조자로부터 직접 로우 포인터를 만들었다.
    // 따라서, 이러한 로우 포인터는 유효성이 보장된다
    // 로우 포인터는 불변 참조와 가변 참조를 동시에 생성했음에도 컴파일이 된다. (대여 규칙 무시)
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // 안전한 코드 내에서는 로우 포인터를 역참조하여 메모리에 저장된 데이터를 읽을 수 없다.
    // 따라서, 로우 포인터를 역참조할 때는, 항상 unsafe 영역 내에서만 가능하다.
    unsafe {
        println!("r1 is : {:?}", r1);
        println!("r2 is : {:?}", r2)
    }
}

#[test]
fn test_create_unsafe_raw_pointer() {
    let address = 0x012345usize;
    let r = address as *const i32; // 임의의 메모리 주소를 가르키는 로우 포인터
}

// 로우 포인터를 왜 사용할까?
// 1. C코드와의 상호작용을 할 때
// 2. 대여 검사기가 이해하지 못하는 안전한 추상화를 만들 때

// 안전하지 않은 함수 혹은 메서드 호출하기
#[test]
fn test_create_unsafe_func() {
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
}

// 안전하지 않은 코드 상에 안전한 추상화 생성하기
// split_at_mut 메서드는 사실 내부적으로 안전하지 않은 코드를 담고 있다.
// 가변 참조를 동시에 2개나 생성하기 때문에 안전한 러스트에서는 컴파일이 되지 않는다.
#[test]
fn test_split_mut() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

use std::slice;
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr(); // as_mut_ptr()은 *mut i32 로우 포인터를 반환
    assert!(mid <= len);

    unsafe {
        (
            // from_raw_parts_mut 메서드는 로우 포인터를 인자로 사용하고
            // 이 포인터가 유효함을 반드시 믿어야 하므로 안전하지 않습니다.
            // offset 메서드 또한, 안전하지 않다.
            // 하지만 위에서 수행한 assert!(mid <= len) 검사로 인해 안전서이 보장됨
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
        )
    }
}

// extern 함수를 사용하여 외부 코드 호출하기
#[test]
fn test_extrn() {
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

// 가변 정적 변수의 접근 혹은 수정하기
// 안전한 러스트는 소유권 규칙으로 인해 가변 전역 변수를 다루기 어렵다.
// 두 스레드가 동일한 가변 전역변수에 접근하면 데이터 레이스를 야기할 수 있다.
// 러스트에서 전역 변수는 정적(static) 변수라고 불립니다.

static HELLO_WORLD: &str = "Hello, world!";
#[test]
fn test_static_variable() {
    println!("{}", HELLO_WORLD);
}
// 정적 변수는 상수와 유사하지만 조금 다른 특징을 갖는다.
// 먼저, 정적 변수는 SCREAMING_SNAKE_CASE 형식을 따르며, 반드시 변수의 타입을 명시해야한다.
// 또한, 정적 변수는 'static 수명을 갖는다.
// 정적 변수는 항상 메모리 내의 고정된 주소 값을 갖는 반면, 상수는 사용될 때마다 데이터가 복사되는 것을 허용한다.

// 상수와 정적 변수 간의 또다른 차이점은 정적 변수가 가변일 수 있다는 점이다.
// 가변 정적 변수에 접근하고 수정하는 것은 '안전하지 않다'.

fn test_mut_static_var() {
    static mut COUNTER: u32 = 0;

    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }
    add_to_count(3);

    unsafe {
        println!("Counter: {}", COUNTER);
    }
}

// 안전하지 않은 트레잇 구현하기
// unsafe에서만 동작하는 마지막 기능은 안전하지 않은 트레잇 구현이다.
// 트레잇은 적어도 메소드 중 하나가 컴파일러가 검사할 수 없는
// 몇몇 불변성(invariant)을 갖고 있을 때 안전하지 않다.

// Sync와 Send 마커 트레잇을 상기해보자
// 우리의 타입이 전체적으로 Send되고, Sync한 타입으로 구성되어 있다면
// 컴파일러는 이 트레잇을 자동적으로 구현한다
// 만일 우리가 로우 포인터와 같이, Send되지 않거나 Sync하지 않은 타입을 포함한 타입을 구현한다면
// unsafe를 활용해야 한다.

// 고급 라이프타임
// 모든 참조자는 수명을 갖지만, 거의 대부분의 경우 러스트가 라이프타임을 생략하도록 해준다

// 라이프 타임의 세가지 고급 기능
// 1. 라이프타임 서브타이핑(subtyping): 한 라이프타임이 다른 라이프타임보다 오래 사는 것을 보장
// 2. 라이프타임 바운드: 제네릭 타입을 가르키는 참조자를 위한 라이프타임 명시하기
// 3. 트레잇 객체 라이프타임 추론: 컴파일러가 어떻게 트레잇 객체의 라이프타임을 추론하며, 언제 명시해야 할지

// 라이프타임 서브타이핑은 하나의 라이프타임이 다른 것보다 오래 사는 것을 보장합니다.
// 라이프타임 서브타이핑은 하나의 라이프타임이 다른 라이프타임보다 오래 살아야 함을 명시합니다.

fn test_lifetime_subtyping() {
    struct Context<'s>(&'s str);

    struct Parser<'c, 's: 'c> {
        // 's가 최소한 'c 보다는 오래 산다는 의미의 애노테이션
        context: &'c Context<'s>, // Context 구조체 참조자에 대한 수명을 알 수 없다..
    }

    impl<'c, 's> Parser<'c, 's> {
        fn parse(&self) -> Result<&'c str, ()> {
            Ok(&self.context.0[1..])
        }
    }

    fn parse_context<'c, 's: 'c>(context: &'c Context<'s>) -> Result<&'c str, ()> {
        Parser { context: &context }.parse()
        
       
        // context 변수의 소유권이 parse_context 함수에게 넘어가고 Parser는 context 변수의 소유권을 가져간다.
        // 그런데, Parser와 Parser의 parse 메서드가 반환한 값은
        //  Context 구조체와 동일한 수명을 가지기 때문에, parse_context 메서드를 벗어나면 모두 죽어버린다.
        //
        // Parse 구조체의 parse 메서드에서 반환된 값은,
        // Parser나 Context 구조체 보다 오래 살아야 한다.
        // 즉, parse_context의 반환 값은 context 구조체의 문자열 슬라이스의 수명과 일치해야 한다.
    }
}

// 제네릭 타입에 대한 참조자 상의 라이프타임 바운드
// T 제네릭 타입은 어떠한 타입도 될 수 있으며, 최소한 'a 라이프 타임보다는 오래 살 수 있다.
struct Ref<'a, T: 'a>(&'a T);
struct StaticRef<T: 'static>(&'static T);

// 트레잇 객체 라이프타임 추론
// 트레잇 객체는 동적 디스패치를 이용할 수 있게 해준다.
trait Red {}

struct Ball<'a> {
    diameter: &'a i32,
}

impl<'a> Red for Ball<'a> {}

// 트레잇 객체의 기본 라이프타임은 'static
// 'a Trait 혹은 'a mut Trait을 쓴 경우, 기본 라이프타임은 'a
// 단일 T:'a 구절을 쓴 경우, 트레잇 객체의 기본 라이프타임은 'a 입니다.

#[test]
fn test_trait_instance_life_time() {
    let num = 5;
    let obj = Box::new(Ball { diameter: &num }) as Box<Red>; // 동적 디스패칭
}

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
