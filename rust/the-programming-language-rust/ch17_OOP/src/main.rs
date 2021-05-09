// 17.1.1 상세 구현을 은닉화하는 캡슐화
struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new() -> Self {
        Self {
            list: vec![],
            average: 0.0,
        }
    }
    pub fn push(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn pop(&mut self) -> Option<i32> {
        match self.list.pop() {
            Some(v) => {
                self.update_average();
                return Some(v);
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        return self.average;
    }

    pub fn update_average(&mut self) {
        let sum: i32 = self.list.iter().sum();
        self.average = sum as f64 / self.list.len() as f64;
    }
}

#[test]
fn test_averaged_collection() {
    let mut avg_collection = AveragedCollection::new();

    avg_collection.push(1);
    assert_eq!(avg_collection.average(), 1.0);
    avg_collection.push(2);
    assert_eq!(avg_collection.average(), 1.5);

    avg_collection.pop();
    assert_eq!(avg_collection.average(), 1.0);
}

// 17.2 트레이트 객체를 사용하여 다른 타입 간의 값 허용하기

// 17.2.1 공통된 동작을 위한 트레이트(interface) 정의하기
// 공통된 인터페이스를 갖는 트레이트를 정의함으로써
// 트레잇 객체(trait object)를 취하는 벡터를 정의할 수 있다.
// 트레잇 객체는 특정 트레잇을 구현한 타입의 인스턴스를 의미한다.

// 우리는 & 참조자나 Box<T>와 같은 스마트 포인터 같은 포인터 종류로 명시함으로서
// 트레잇 객체를 만들고, 그런 다음 관련된 트레잇을 정의합니다.
// 우리는 제네릭 타입이나 구체 타입 대신 트레잇 객체를 사용할 수 있다.

pub trait Draw {
    fn draw(&self);
}

// Screen 구조체의 componenets 필드는
// Draw 트레잇을 구현한 트레잇 객체에 대한 Box<T> 포인터를 취한 벡터이다.
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn new() -> Self {
        Self { components: vec![] }
    }
    pub fn add(&mut self, component: Box<Draw>) {
        self.components.push(component);
    }
    pub fn run(&self) {
        // run 메서드는 components의 아이템이 구체적으로 어떤 타입인지 알 필요가 없다.
        // 컴파일러는 component의 아이템이 Draw 트레이트를 구현한 트레잇 객체임을 보장한다.
        // 따라서, run 메서드는 단순히 각 아이템의 draw 메서드를 호출할 뿐이다.
        for c in self.components.iter() {
            c.draw(); // Draw 트레잇 객체는 모두 draw 메서드를 갖고 있음을 보장
        }
    }
}

// 이것은 트레잇 바운드와 함께 제네릭 타입 파라미터를 사용하여 구조체를 정의하는 것과는 사뭇 다르다.
// 제네릭 타입 파라미턴느 한 번에 하나의 구체 타입으로만 대입될 수 있는 반면,
// 트레잇 객체를 사용하면 런타임에 여러 구체 타입을 트레잇 객체에 대해 채워넣을 수 있다!

// 아래와 같이 트레잇 바운드 + 제네릭 타입으로 정의하면
// Draw 트레잇을 구현한 동일한 타입만 가질 수 있다.
// ex) Buttom 타입과 TextField 타입이 모두 Draw 트레잇을 구현하였다 하더라도
// 둘을 한번에 components 벡터에 담을 수 없다..

// 만약 동일한 유형의 콜렉션을 담는게 목적이라면 아래와 같은 방법이 더 나은 방법이다.
// 그 이유는, 컴파일 타임에 단형성화(monomorphize) 되기 때문이다.
// 이 두 방법은 런타임 성능에서 차이를 보인다.
pub struct Screen2<T: Draw> {
    pub componenets: Vec<T>,
}

impl<T> Screen2<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for c in self.componenets.iter() {
            c.draw();
        }
    }
}

// 트레잇 구현하기
use std::fmt::Debug;
#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Draw a button {:?}", self);
    }
}
#[derive(Debug)]
pub struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Draw a select box {:?}", self);
    }
}

#[test]
fn test_screen() {
    let mut screen = Screen::new();
    screen.add(Box::new(SelectBox {
        width: 75,
        height: 10,
        options: vec![
            String::from("Yes"),
            String::from("Maybe"),
            String::from("No"),
        ],
    }));

    screen.add(Box::new(Button {
        width: 50,
        height: 10,
        label: String::from("Ok"),
    }));
    screen.run();
}

// 제네릭 타입을 사용하면 컴파일 타임에 단일화(monomorphization)라는 작업을 수행하여,
// 제네릭 코드를 실제로 구체화된 타입으로 변환(정적 디스패치)하기 때문에 런타임 성능 저하가 발생하지 않는다.

// 반면, 트레잇 객체는 동적 디스패치를 수행한다.
// 즉, 컴파일 타임에는 어떤 메서드를 호출하는지 컴파일러는 알지 못한다.
// 대신, 컴파일러는 런타임에 어떤 메서드가 호출되는지 알아내는 코드를 생성한다.
// 따라서, 트레잇 객체를 사용하면 런타임 비용(어떤 메서드를 호출할지 탐색)이 발생한다.
// 그러므로, 트레잇 객체를 사용하면 유연성이 향상되지만 런타임 오버헤드가 증가하는 trade-off가 존재한다.

// 트레잇 객체에 대해서는 안정성이 요구된다.
// 모든 트레잇 객체는 객체-안전(object-safe)가 보장되어야 한다.
// 트레잇 객체를 안전하게 만드는 모든 속성들을 관장하는 몇가지 복잡한 규칙이 있지만
// 실전에서는 두 가지 규칙만 관련되어 있습니다.
// 1. 반환값의 타입이 Self가 아니다
// 2. 제네릭 타입 매개변수가 없다.

// 트레잇 객체가 반드시 객체-안전 해야 하는 이유는
// 러스트가 트레잇에 구현된 구체(concrete) 타입을 알 수 없기 때문이다.
// 메소드가 객채 안전하지 않는 대표적인 사례는 Clone 트레잇이다.

// 17.3 객체 지행 디자인 패턴 구현하기

// 상태(state) 패턴
// 일반적으로 복잡한 조건문을 대체하기 위해 많이 활용됨
// 어떤 값이 상태 객체들의 집합으로 표현되는 일종의 내부 상태를 가지며
// 이러한 내부 상태에 따라서 동작이 결정된다.

//우리는 점진적인 방식으로 블로그에 게시물을 올리는 작업 흐름을 구현하려고 합니다.
// 블로그의 최종적인 기능은 다음과 같을 것입니다:

// 블로그 게시물은 빈 초안으로 시작합니다.
// 초안이 완료되면 게시물의 검토가 요청됩니다.
// 게시물이 승인되면 게시됩니다.
// 오직 게시된 블로그 게시물만이 출력될 내용물을 반환하므로, 승인되지 않은 게시물은 실수로라도 게시될 수 없습니다.

// States
trait State {
    // 동작
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
    // 상태 전환
    // 매개변수로 Box<self>를 받는 이유
    // Box<self>는 메서드가 Box 상에서 호출될 경우에만 유효함을 의미함
    // => 즉, 역참조 연산자로 박스가 가르키는 데이터를 가져올 필요가 없이 바로 메서드 호출 가능함
    // => 또한, 참조가 아니라 데이터를 소유하는 형태로 하여, 이전 상태를 무효로 만들어줌
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn apporve(self: Box<Self>) -> Box<dyn State>;
}
struct DraftState {}
struct PendingReviewState {}
struct ApprovedState {}

impl State for DraftState {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        return Box::new(PendingReviewState {});
    }
    fn apporve(self: Box<Self>) -> Box<dyn State> {
        return self;
    }
}
impl State for PendingReviewState {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        return self;
    }
    fn apporve(self: Box<Self>) -> Box<dyn State> {
        return Box::new(ApprovedState {});
    }
}

impl State for ApprovedState {
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        return &post.content;
    }
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        return self;
    }
    fn apporve(self: Box<Self>) -> Box<dyn State> {
        return self;
    }
}

// Posts
trait PostInterface {
    fn new() -> Post;
    fn content(&self) -> &str;

    fn add_text(&mut self, text: &str);
    fn request_review(&mut self);
    fn approve(&mut self);
}

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl PostInterface for Post {
    fn new() -> Self {
        Self {
            state: Some(Box::new(DraftState {})),
            content: String::from(""),
        }
    }

    fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }
    fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    fn request_review(&mut self) {
        // take 메서드를 호출하여, state 데이터의 소유권을 가져오고 이전 값을 None으로 만들어 버림
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.apporve())
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_text() {
        let post = Post::new();
        assert_eq!(post.content(), "");
    }

    #[test]
    fn test_request_review() {
        let mut post = Post::new();
        post.add_text("Hi");
        post.request_review();
        assert_eq!(post.content(), "");
    }

    #[test]
    fn test_approve() {
        let mut post = Post::new();
        post.add_text("Hi");
        post.request_review();
        post.approve();
        assert_eq!(post.content(), "Hi");
    }
}
