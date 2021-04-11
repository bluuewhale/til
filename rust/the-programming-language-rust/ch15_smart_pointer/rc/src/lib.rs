// 15.4 Rc<T>, 참조 카운터 스마트 포인터
// 하나의 값을 여러 변수가 소유하는 때도 있다.
// 예를 들어, 그래프 데이터 구조의 경우, 여러 엣지(edge)가 같은 노드를 가르키고 있을 수 있으며,
// 이 노드는 개념적으로는 자신을 가지키는 모든 엣지를 소유한다.
// 이 노드는 자신을 가르키는 엣지가 모두 사라질 때까지 해제할 수 없다.

// Rc<T> 타입은 값에 대한 참조의 개수를 추적해서 그 값이 여전히 사용 중인지 확인한다.
// 값에 대한 참조가 더 존재하지 않으면 그제야 값을 해제한다.

// Rc<T> 타입은 프로그램의 여러 부분에서 데이터를 읽을 수 있도록 데이터를 힙 메모리에 저장할 때 사용하며,
// 컴파일타임에는 어떤 곳의 데이터가 가장 마지막까지 사용되는지 결정할 수 없다.
// Rc<T> 타입은 단일 스레드 환경에서만 사용해야 한다

// 15.4.1 Rc<T> 타입을 이용해 데이터 공유하기

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use std::rc::Rc;
use List::{Cons, Nil};

#[test]
fn test_rc() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a)); // Rc::clone은 참조 카운트를 증가시키는 복사
    let c = Cons(4, Rc::clone(&a));
}

// 15.4.2 Rc<T>의 복제는 참조 카운터를 증가시킨다.
#[test]
fn test_rc_ref_count() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
