#[derive(Clone)]
struct Trie {
    children: Vec<Option<Box<Trie>>>,
    is_end: bool,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Trie {
            children: vec![None; 26],
            is_end: false,
        }
    }
    
    /** Inserts a word into the trie. */
    fn insert(&mut self, word: String) {
        let mut curr = self;
        for c in word.chars() {
            let idx = c as usize - 'a' as usize;
            if curr.children[idx].is_none() {
                curr.children[idx] = Some(Box::new(Trie::new()));
            }
            curr = curr.children[idx].as_mut().unwrap();
        }
        curr.is_end = true;
    }
    
    /** Returns if the word is in the trie. */
    fn search(&self, word: String) -> bool {
        let mut curr = self;
        for c in word.chars() {
            let idx = c as usize - 'a' as usize;
            if curr.children[idx].is_none() {
                return false;
            }
            curr = curr.children[idx].as_ref().unwrap();
        }
        curr.is_end
    }
    
    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&self, prefix: String) -> bool {
        let mut curr = self;
        for c in prefix.chars() {
            let idx = c as usize - 'a' as usize;
            if curr.children[idx].is_none() {
                return false;
            }
            curr = curr.children[idx].as_ref().unwrap();
        }
        true
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */