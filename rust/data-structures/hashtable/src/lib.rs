#![feature(linked_list_remove)]

use std::collections::LinkedList;

trait Hash {
    fn hash(&self) -> usize;
}
impl Hash for String {
    fn hash(&self) -> usize {
        self.as_bytes().iter().map(|x| *x as usize).sum()
    }
}

struct Node<K: Hash, V> {
    key: K,
    val: V,
}
impl<K: Hash, V> Node<K, V> {
    pub fn new(key: K, val: V) -> Self {
        Self { key, val }
    }
    pub fn key(&self) -> &K {
        &self.key
    }
    pub fn val(&self) -> &V {
        &self.val
    }
    pub fn to_val(self) -> V {
        self.val
    }
    pub fn hash(&self) -> usize {
        self.key.hash()
    }
}

struct HashTable<K: Hash, V> {
    inner: Vec<LinkedList<Node<K, V>>>,
    len: usize,
}
impl<K: Hash + PartialEq, V> HashTable<K, V> {
    fn new(len: usize) -> Self {
        let mut inner = Vec::with_capacity(len);
        for _ in 0..len {
            inner.push(LinkedList::new())
        }
        Self { inner, len }
    }

    fn insert(&mut self, key: K, val: V) {
        let node = Node::new(key, val);
        let index = node.hash() % self.len;
        match self.inner.get_mut(index) {
            Some(ll) => {
                ll.push_back(node);
            }
            None => {
                let mut ll = LinkedList::new();
                ll.push_back(node);
                self.inner.insert(index, ll);
            }
        }
    }

    fn search(&self, key: &K) -> bool {
        let index = key.hash() % self.len;
        match self.inner.get(index) {
            Some(ll) => {
                for node in ll.iter() {
                    if node.key() == key {
                        return true;
                    }
                }
                return false;
            }
            None => false,
        }
    }

    fn get(&self, key: &K) -> Option<&V> {
        let index = key.hash() % self.len;
        match self.inner.get(index) {
            Some(ll) => {
                for node in ll.iter() {
                    if node.key() == key {
                        return Some(node.val());
                    }
                }
                return None;
            }
            None => None,
        }
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        let index = key.hash() % self.len;
        match self.inner.get_mut(index) {
            Some(ll) => {
                for (i, node) in ll.iter().enumerate() {
                    if node.key() == key {
                        return Some(ll.remove(i).to_val());
                    }
                }
                None
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::HashTable;

    #[test]
    fn test_search() {
        let mut hashmap = HashTable::new(10);
        hashmap.insert("elephant".to_string(), 3);
        hashmap.insert("giraffe".to_string(), 8);

        assert!(hashmap.search(&"elephant".to_string()));
        assert!(hashmap.search(&"giraffe".to_string()));
        assert!(!hashmap.search(&"lion".to_string()));
    }

    #[test]
    fn test_get() {
        let mut hashmap = HashTable::new(10);
        hashmap.insert("elephant".to_string(), 3);
        hashmap.insert("giraffe".to_string(), 8);

        assert_eq!(hashmap.get(&"elephant".to_string()), Some(&3));
        assert_eq!(hashmap.get(&"giraffe".to_string()), Some(&8));
        assert_eq!(hashmap.get(&"lion".to_string()), None);
    }

    #[test]
    fn test_remove() {
        let mut hashmap = HashTable::new(10);
        hashmap.insert("elephant".to_string(), 3);
        hashmap.insert("giraffe".to_string(), 8);
        hashmap.remove(&"elephant".to_string());
        hashmap.remove(&"giraffe".to_string());

        assert_eq!(hashmap.get(&"elephant".to_string()), None);
        assert_eq!(hashmap.get(&"giraffe".to_string()), None);
    }
}
