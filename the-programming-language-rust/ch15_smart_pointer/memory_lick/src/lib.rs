// 15.6 메모리 누수의 원인이 되는 순환 참조

use std::cell::RefCell;
use std::rc::Rc;
use List::{Cons, Nil};
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match *self {
            Cons(_, ref item) => Some(item),
            Nil => None,
        }
    }
}

#[test]
fn recursive_ref() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a의 최초 rc 카운트: {}", Rc::strong_count(&a));
    println!("a의 다음 아이템 = {:?}", a.tail());

    let b = Rc::new(Cons(5, RefCell::new(Rc::clone(&a))));

    println!("b를 생성한 후 a의 카운트: {}", Rc::strong_count(&a));
    println!("b의 최초 카운트: {}", Rc::strong_count(&b));
    println!("b의 다음 아이템 = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("a를 변경한 후의 b의 카운트 = {}", Rc::strong_count(&b));
    println!("a를 변경한 후의 a의 카운트 = {}", Rc::strong_count(&a));
}
