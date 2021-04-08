//https://github.com/TheAlgorithms/Rust/blob/master/src/data_structures/linked_list.rs

use std::ptr::NonNull;

// [[ Key Concepts ]]
//
// 1. NonNull<T> vs *mut T
// NonNull<T> is same as *mut T but non-zero and covariant

// 2. Covariant
// https://medium.com/@lazysoul/%EA%B3%B5%EB%B3%80%EA%B3%BC-%EB%B6%88%EB%B3%80-297cadba191
// 한국어로는 '공변'으로 번역된다. 공변성은 타입생성자에게 리스코프 치환 법칙을 허용하는 것을
// 의미한다.
//
// 3. 리스코프 치환 법칙?
// BaseClass가 자신을 상속한 SubClass로 치환되어도 동일한 동작을 보장해야 한다는 원칙을 의미한다.
// => 즉, 공변성(Covariant)는 자신과 자신의 SubClass를 허용한다는 것을 의미함.
// => Unline *mut T, NonNull<T> was chosen to be covariant over T !!

// 3. Overriding in Rust
// 러스트에는 Struct 상속이 존재하지 않는다.
// 대신, 이를 대체할 수 있는 Triat Inheritance가 존재한다. ex) trait A: B + Clone

// 4. LinkedList vs Array
// LinkedList는 순차적으로 연결된 공간에 데이터를 나열하는 배열(contiguous)과 달리,
// 서로 떨어진 공간에 존재하는 데이터를 연결한 구조를 갖고 있다.
// 시작 지점에서 데이터 사이즈만큼 포인터를 옮겨가며 읽으면 되는 배열과 달리
// LinkedList는 기본 단위인 Node가 자신의 앞뒤에 연결되어 있는 데이터의 포인터를 소유하여 연결을
// 유지한다.

struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            prev: None,
            next: None,
        }
    }

    fn to_ptr(self) -> Option<NonNull<Node<T>>> {
        Some(unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(self))) })
    }
}

pub struct LinkedList<T> {
    length: u32,
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            head: None,
            tail: None,
        }
    }
    pub fn add(&mut self, obj: T) {
        let node = Node::new(obj);

        if self.length == 0 {
            // when LinkedList is empty, no linking occurs yet
            self.head = node.to_ptr(); // head is updated only once
            self.tail = self.head;
        } else {
            // Now, linking occurs!
            let node_ptr = self.link_new_node_to_tail(node);
            self.tail = node_ptr; // update tail
        }

        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.head, index)
    }

    fn link_new_node_to_tail(&mut self, mut node: Node<T>) -> Option<NonNull<Node<T>>> {
        node.prev = self.tail; // prev of new node is linked to the tail of LinkedList

        let node_ptr = node.to_ptr();
        unsafe { (*self.tail.unwrap().as_ptr()).next = node_ptr } // next of LinkedList's tail is linked to new node! (None is unreachable)
        node_ptr
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        // recursively follow linked nodes
        match node {
            None => None, // not enough
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }), // reached end
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1), // to next node
            },
        }
    }
}

impl<T> std::fmt::Display for LinkedList<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.head {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}
impl<T> std::fmt::Display for Node<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn get_by_index_in_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        println!("Linked List is {}", list);
        let retrived_item = list.get(1);
        assert!(retrived_item.is_some());
        assert_eq!(2 as i32, *retrived_item.unwrap());
    }

    #[test]
    fn get_by_index_in_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        println!("Linked List is {}", list_str);
        let retrived_item = list_str.get(1);
        assert!(retrived_item.is_some());
        assert_eq!("B", *retrived_item.unwrap());
    }
}
