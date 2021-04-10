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
    pub(crate) next: Option<NonNull<Node<T>>>,
}

// public methods
impl<T> Node<T> {
    pub fn new(t: T) -> Node<T> {
        Node { val: t, next: None }
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

    /// Returns `true` if the `LinkedList` is empty.
    /// This operation should compute in 0(1) time.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

// setter methods (front/back)
impl<T> LinkedList<T> {
    /// Appends element to the back.
    #[inline]
    pub fn push_back(&mut self, val: T) {
        self.push_back_node(Box::new(Node::new(val)))
    }

    /// Appends element to the front
    #[inline]
    pub fn push_front(&mut self, val: T) {
        self.push_front_node(Box::new(Node::new(val)))
    }

    /// Appends the given node to the back of LinkedList
    #[inline]
    fn push_back_node(&mut self, node: Box<Node<T>>) {
        unsafe {
            let node = Some(NonNull::new_unchecked(Box::into_raw(node)));
            match self.tail {
                None => self.head = node,
                Some(mut tail) => tail.as_mut().next = node,
            }

            self.tail = node;
            self.length += 1;
        }
    }

    /// Appends the given node to the front of LinkedList
    #[inline]
    fn push_front_node(&mut self, node: Box<Node<T>>) {
        unsafe {
            let mut node = Some(NonNull::new_unchecked(Box::into_raw(node)));
            match self.head {
                None => self.tail = node,
                Some(head) => node.as_mut().unwrap().as_mut().next = Some(head),
            }

            self.head = node;
            self.length += 1;
        }
    }
}

// getter methods (front/back)
impl<T> LinkedList<T> {
    /// Provides a reference to the front element or `None` LinkedList is empty
    #[inline]
    pub fn front(&self) -> Option<&T> {
        unsafe { self.head.as_ref().map(|node| &(node.as_ref().val)) }
    }

    /// Provides a mutable reference to the front element or `None` LinkedList is empty
    #[inline]
    pub fn front_mut(&mut self) -> Option<&T> {
        unsafe { self.head.as_mut().map(|node| &(node.as_mut().val)) }
    }

    /// Provides a reference to the back element or `None` LinkedList is empty
    #[inline]
    pub fn back(&self) -> Option<&T> {
        unsafe { self.tail.as_ref().map(|node| &(node.as_ref().val)) }
    }

    /// Provides a mutable reference to the back element or `None` LinkedList is empty
    #[inline]
    pub fn back_mut(&mut self) -> Option<&T> {
        unsafe { self.tail.as_mut().map(|node| &(node.as_mut().val)) }
    }
}

// setter methods (by index)
impl<T> LinkedList<T> {
    /// Insert node to i-th index of LinkedList.
    /// If given index if bigger than length of LinkedList,
    /// Node will be attached to the tail of LinkedList
    #[inline]
    pub fn insert(&mut self, val: T, index: u32) {
        self.insert_node(Box::new(Node::new(val)), index)
    }

    #[inline]
    fn insert_node(&mut self, mut node: Box<Node<T>>, index: u32) {
        if index > self.length {
            return self.push_back_node(node);
        }

        if index == 0 {
            return self.push_front_node(node);
        }

        unsafe {
            let mut front = self.get_ith_node(self.head, index - 1).unwrap(); // None is unreachable
            let back = front.as_ref().next;

            // [front] -> [middle] -> [back]
            node.next = back;
            let middle = NonNull::new_unchecked(Box::into_raw(node));
            front.as_mut().next = Some(middle);
        }

        self.length += 1;
    }
}

// getter methods (by index)
impl<T> LinkedList<T> {
    /// Provides a reference to i-th element or `None` if i-th node does not exists
    #[inline]
    pub fn get(&mut self, index: u32) -> Option<&T> {
        if index > self.length {
            return None;
        }

        self.get_ith_node(self.head, index)
            .map(|node| unsafe { &((*node.as_ptr()).val) })
    }

    /// Provide i-th node of LinkedList or `None` if i-th node does not exists
    #[inline]
    fn get_ith_node(
        &mut self,
        node: Option<NonNull<Node<T>>>,
        index: u32,
    ) -> Option<NonNull<Node<T>>> {
        // recursively follow linked nodes
        match node {
            None => None, // not enough
            Some(next_ptr) => match index {
                0 => Some(unsafe { next_ptr }), // reached end
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
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.push_back("A".to_string());
        list_str.push_back("B".to_string());
        list_str.push_back("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn get_by_index_in_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.push_back(1);
        assert!(list.get(0).is_some());
        assert_eq!(1, *list.get(0).unwrap());
    }

    #[test]
    fn get_by_index_in_string_list() {
        let mut list = LinkedList::<String>::new();
        list.push_back("A".to_string());
        assert!(list.get(0).is_some());
        assert_eq!("A", list.get(0).unwrap());
    }

    #[test]
    fn push_back() {
        let mut list = LinkedList::<i32>::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(3, list.length);
        assert_eq!(1, *list.get(0).unwrap());
        assert_eq!(2, *list.get(1).unwrap());
        assert_eq!(3, *list.get(2).unwrap());
    }

    #[test]
    fn push_front() {
        let mut list = LinkedList::<i32>::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(3, list.length);
        assert_eq!(3, *list.get(0).unwrap());
        assert_eq!(2, *list.get(1).unwrap());
        assert_eq!(1, *list.get(2).unwrap());
    }

    #[test]
    fn front_back() {
        let mut list = LinkedList::<i32>::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(3, list.length);
        assert_eq!(1, *list.front().unwrap());
        assert_eq!(3, *list.back().unwrap());
    }

    #[test]
    fn get() {
        let mut list = LinkedList::<i32>::new();
        list.push_back(1);
        list.push_front(2);
        list.push_back(3);
        assert_eq!(2, *list.get(0).unwrap());
        assert_eq!(1, *list.get(1).unwrap());
        assert_eq!(3, *list.get(2).unwrap());
    }

    #[test]
    fn insert() {
        let mut list = LinkedList::<i32>::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.insert(10, 1);
        assert_eq!(4, list.length);
        assert_eq!(1, *list.get(0).unwrap());
        assert_eq!(10, *list.get(1).unwrap());
        assert_eq!(2, *list.get(2).unwrap());
        assert_eq!(3, *list.get(3).unwrap());
    }

    #[test]
    fn insert_back() {
        let mut list = LinkedList::<i32>::new();
        list.push_back(1);
        list.push_back(2);
        list.insert(4, 10);
        assert_eq!(3, list.length);
        assert_eq!(1, *list.get(0).unwrap());
        assert_eq!(2, *list.get(1).unwrap());
        assert_eq!(4, *list.get(2).unwrap());
    }
}
