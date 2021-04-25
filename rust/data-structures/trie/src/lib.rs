pub struct TrieNode<K, V> {
    key: K,
    val: Option<V>,
    children: Vec<TrieNode<K, V>>,
}

impl<K, V> TrieNode<K, V> {
    pub fn new(key: K, val: Option<V>, children: Option<Vec<TrieNode<K, V>>>) -> Self {
        let children = children.or(Some(Vec::new())).unwrap();
        Self { key, val, children }
    }
    pub fn push(&mut self, child: TrieNode<K, V>) {
        self.children.push(child)
    }

    pub fn val(&self) -> Option<&V> {
        self.val.as_ref()
    }

    pub fn val_mut(&mut self) -> Option<&mut V> {
        self.val.as_mut()
    }
    pub fn set_val(&mut self, val: V) {
        self.val = Some(val);
    }
}

/// check its own children only (1-level search)
pub fn find_child<'a, K: Eq, V>(node: &'a TrieNode<K, V>, key: &K) -> bool {
    for c in node.children.iter() {
        if &c.key == key {
            return true;
        }
    }

    false
}

/// check its own children only (1-level search)
pub fn get_child<'a, 'b, K: Eq, V>(
    node: &'a TrieNode<K, V>,
    key: &'b K,
) -> Option<&'a TrieNode<K, V>> {
    for child in node.children.iter() {
        if &child.key == key {
            return Some(child);
        }
    }

    None
}
/// check its own children only (1-level search)
pub fn get_child_mut<'a, 'b, K: Eq, V>(
    node: &'a mut TrieNode<K, V>,
    key: &'b K,
) -> Option<&'a mut TrieNode<K, V>> {
    for child in node.children.iter_mut() {
        if &child.key == key {
            return Some(child);
        }
    }

    None
}

/// Trie Tree that has String as key
pub struct TrieTree<V> {
    root: TrieNode<String, V>,
}

impl<V> TrieTree<V> {
    pub fn new() -> Self {
        Self {
            root: TrieNode::new("".to_string(), None, None),
        }
    }

    /// Push key and value to Trie
    pub fn push(&mut self, key: String, val: V) {
        let mut node = &mut self.root;
        for c in key.chars() {
            let c = c.to_string();

            let child = if find_child(node, &c) {
                get_child_mut(node, &c).unwrap()
            } else {
                let child = TrieNode::new(c, None, None);
                node.push(child);
                node.children.last_mut().unwrap()
            };

            node = child
        }

        node.set_val(val);
    }

    /// Return `true` if given key exists in Trie, else return `false`
    pub fn find(&self, key: &str) -> bool {
        let mut node = &self.root;
        for c in key.chars() {
            match get_child(node, &c.to_string()) {
                Some(child) => {
                    node = child;
                }
                None => {
                    return false;
                }
            }
        }

        true
    }

    fn get_node(&self, key: &str) -> Option<&TrieNode<String, V>> {
        let mut node = &self.root;
        for c in key.chars() {
            match get_child(node, &c.to_string()) {
                Some(child) => {
                    node = child;
                }
                None => {
                    return None;
                }
            }
        }

        Some(node)
    }
    fn get_node_mut(&mut self, key: &str) -> Option<&mut TrieNode<String, V>> {
        let mut node = &mut self.root;
        for c in key.chars() {
            match get_child_mut(node, &c.to_string()) {
                Some(child) => {
                    node = child;
                }
                None => {
                    return None;
                }
            }
        }

        Some(node)
    }

    /// Return `Some(&V) `if given key exists in Trie, else return `None`
    pub fn get(&self, key: &str) -> Option<&V> {
        match self.get_node(key) {
            Some(node) => node.val(),
            None => None,
        }
    }

    /// Return `Some(mut &V) `if given key exists in Trie, else return `None`
    pub fn get_mut(&mut self, key: &str) -> Option<&mut V> {
        match self.get_node_mut(key) {
            Some(node) => node.val_mut(),
            None => None,
        }
    }
}

#[cfg(test)]
mod test {

    use super::TrieTree;

    #[test]
    fn test_push() {
        let mut trie = TrieTree::new();
        trie.push(
            "elephant".to_string(),
            "an animal with long nose".to_string(),
        );
        trie.push(
            "giraffe".to_string(),
            "an animal with long neck".to_string(),
        );
    }

    #[test]
    fn test_find() {
        let mut trie = TrieTree::new();
        trie.push(
            "elephant".to_string(),
            "an animal with long nose".to_string(),
        );
        trie.push(
            "giraffe".to_string(),
            "an animal with long neck".to_string(),
        );

        assert!(trie.find("elephant"));
        assert!(trie.find("giraffe"));
        assert!(trie.find("ele")); // this is created as parents of elephant
        assert!(trie.find("gir")); // this is created as parents of giraffe
    }

    #[test]
    fn test_get() {
        let mut trie = TrieTree::new();
        trie.push("elephant".to_string(), "an animal with long nose");

        assert_eq!(
            trie.get("elephant").map(|x| *x),
            Some("an animal with long nose")
        );
        assert_eq!(trie.get("ele"), None);
    }

    #[test]
    fn test_get_mut() {
        let mut trie = TrieTree::new();
        trie.push("elephant".to_string(), "an animal with long nose");

        let v = trie.get_mut("elephant").unwrap();
        *v = "an animal with big ears";

        assert_eq!(
            trie.get("elephant").map(|x| *x),
            Some("an animal with big ears")
        );
    }
}
