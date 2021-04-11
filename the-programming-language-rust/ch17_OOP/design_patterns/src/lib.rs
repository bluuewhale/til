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
