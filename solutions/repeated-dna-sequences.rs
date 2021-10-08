use std::collections::{HashMap, VecDeque};
impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let mut ret: Vec<String> = Vec::new();
        let mut map: HashMap<String, i32> = HashMap::new();
        let chars = s.chars().collect::<Vec<char>>();
        let mut count: VecDeque<char> = VecDeque::with_capacity(10);
        for i in 0..s.len() {
            if i <= 9 {
                count.push_back(chars[i]);
            } else {
                count.pop_front();
                count.push_back(chars[i]);
            }
            let key = count.iter().collect::<String>();
            if map.contains_key(&key) {
                let val = *map.get(&key).unwrap();
                if val == 1 {
                    ret.push(key.clone());
                }
                map.insert(key, val + 1);
            } else {
                map.insert(key, 1);
            }
        }   
        ret
    }
}