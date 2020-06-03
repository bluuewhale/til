// 10.3 트레이트: 공유 가능한 행위를 정의하는 방법
// 트레이트는 러스트 컴파일러에게 특정 타입이 어떤 기능을 실행할 수 있으며,
// 어떤 타입과 이 기능을 공유할 수 있는지를 알려주는 방법이다.

// 트레이트는 공유가능한 행위를 추상화된 방식으로 정의하는 방법이다.
// 트레이트에 제네릭을 결합해서 모든 타입에 특정 행위를 공유할 수도 있다.

// 트레이트는 다른 언어에서는 '인터페이스(interface)'라고 부르는 기능과 유사하다.

// 10.3.1 트레이트 선언하기
// 타입의 행위는 해당 타입에 대해 호출할 수 있는 매서드로 구성된다.
// 이때 다른 타입에 같은 매서드를 호출할 수 있다면, 그 행위는 타입 간에 공유가 가능하다.

// 트레이트의 정의는 어떤 목적을 이루는데 필요한 일련의 행위를 정의하고
// 여러 타입에 적용할 매서드 시그너처를 그룹화하는 방법이다.

pub trait Summary {
    fn summarize(&self) -> String; // abstract method와 유사
}

// 10.3.2 타입에 트레이트 구현하기
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        return format!("{}, by {}, ({})", self.headline, self.author, self.location);
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        // override
        return format!("{}: {}", self.username, self.content);
    }
}

pub fn impl_trait_within_struct() {
    let tweet = Tweet {
        username: String::from("Donghyung"),
        content: String::from("러스트 언어 공부를 시작했습니다"),
        reply: false,
        retweet: false,
    };

    println!("{}", tweet.summarize());
}

// 트레이트 구현에 있어 한 가지 제약은 트레이트나 트레이트를 구현할 타입이
// 현재 크레이트의 로컬 타입이어야 한다는 점이다.
// 즉, 외부 크레이트에 외부 트레이트를 구현할 수 없다

// 10.3.3 기본 구현
// 때로는 모든 타입이 트레이트에 선언된 모든 메서드를 구현하도록 하는 것보다는
// 일부 혹은 전체 메소드의 기본 동작을 구현해 주는 편이 유용할 때가 있다.
// >> 인터페이스로 공통의 메서드를 구현

pub trait Summary2 {
    fn summarize(&self) -> String {
        // inheritance
        return String::from("(계속 읽기)");
    }
}
pub struct NewsArticle2 {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary2 for NewsArticle2 {}

pub fn create_struct_with_default() {
    let article = NewsArticle2 {
        headline: String::from("대한민국, 러시아 월드컵 예선에서 독일에 이겼다."),
        location: String::from("카잔 아레나, 러시아"),
        author: String::from("위키백과"),
        content: String::from("2018년 6월 27일 .."),
    };

    println!("새로운 기사: {}", article.summarize());
}

// 10.3.4 트레이트 매개변수
// 제네릭을 활용하여 트레이트가 여러가지 타입을 받을 수 있도록 설정
pub fn notify(item: &impl Summary) {
    // Summary 트레이트를 구현한 타입만 매개변수로 받을 수 있음

    println!("속보: {}", item.summarize());
}

// (1) 트레이트 경계 문법
// impl Trait 문법은 비교적 간단한 경우에는 잘 작동하지만,
// 사실은 트레이트 경계라고 부르는 더 긴 문법을 간단히 표현하기 위한 장치다.
// 트레이트 경게는 다음과 같다.
pub fn notify2<T: Summary>(item: &T) {
    println!("속보! {}", item.summarize());
}
// 간단한 경우에는 impl Trait 문법이 유용하지만
// 복잡한 경우에는 트레이트 경계를 활용
pub fn notify3<T: Summary>(item1: &T, item2: &T) {
    println!("속보! {}", item1.summarize());
    println!("속보! {}", item2.summarize());
}

// (2) +문법으로 여러 트레이트 경계 정의하기
use std::fmt::{Debug, Display};
pub fn notify4(item: &(impl Summary + Display)) {}
pub fn notify5<T: Summary + Debug>(item: &T) {}

// (3) where 절을 이용해 트레이트 경계 정리하기
// where 절을 사용하지 않은 경우
pub fn some_function<T: Display + Debug, U: Summary + Display>(t: T, u: U) {}

// where 절을 사용한 경우
pub fn sume_function2<T, U>(t: T, u: U)
where
    T: Display + Debug,
    U: Summary + Display,
{
    println!("YEAH!")
}

// 10.3.5 트레이트를 구현하는 값 리턴하기
// impl Trait 문법은 특정 트레이트를 구현하는 타입을 리턴값으로 사용할 때도 활용할 수 있다.

// 리턴 타입이 구현해야 할 트레이트를 명시하는 방법은 클로저와 반복자를 이용하는 상황에 특히 유용하다.

pub fn resturns_summary() -> impl Summary {
    return Tweet {
        username: String::from("Donghyung"),
        content: String::from("러스트 언어 공부를 시작했습니다"),
        reply: false,
        retweet: false,
    };
}

// 10.3.6 트레이트 경계를 이용해 largest 함수 수정하기

// 10.2에서 컴파일에 실패한 예제를 수정해보자!
// 아래의 함수는 PartialOrd와 Copy 트레이트를 구현한 제네릭 타입에 대해서만 동작하게 된다.
pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    return largest;
}

pub fn largest2<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    return &largest;
}

// (1) 트레이트 경계를 이용해 조건에 따라 메서드 구현하기
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// cmp_display 매서드는 Display trait와 PartialOrd trait를
// 구현한 제네릭 타입에서만 호출 가능
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("가장 큰 멤버는 x: {}", self.x);
        } else {
            println!("가장 큰 멤버는 y: {}", self.y);
        }
    }
}

// 또한, 타입이 원하는 트레이트를 구현하는 경우에만
// 다른 트레이트를 조건적으로 구현하게 할 수도 있다.
// >> 덮개 구현(blanket implementation)

// 트레이트와 트레이트 경계는 제네릭 타입 매개변수를 이용해
// 중복된 코드를 줄일 수 있음은 물론, 컴파일러에게 제네릭 타입에
// 특정한 동작을 추가하고 싶다는 것을 명시하는 방법이기도 하다.

// 일반적인 동적 타입 언어는 타입이 구현하지 않는 메서드를 호출하면
// 런타임 에러가 발생한다. 하지만, 러스트는 이 에러를 컴파일타임으로
// 코드를 실행하기 전에 문제를 해결하도록 강제한다.
// 게다가 컴파일타임에 이미 동작의 지원 여부를 검사하기 때문에
// 런타임에 동장의 실행 가능 여부를 검사하는 코드를 작성할 필요도 없다.
