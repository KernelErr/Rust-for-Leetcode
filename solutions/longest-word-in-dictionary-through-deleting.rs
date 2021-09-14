impl Solution {
    pub fn find_longest_word(s: String, dictionary: Vec<String>) -> String {
        let n: usize = dictionary.len();
        let slen = s.len();
        let mut ans = "";
        let mut l = 0;
        let mut r = 0;
        for string in dictionary.iter() {
            l = 0;
            r = 0;
            while l < slen && r < string.len() {
                if s.as_bytes()[l] == string.as_bytes()[r] {
                    l += 1;
                    r += 1;
                } else {
                    l += 1;
                }
            }
            if r == string.len() {
                if ans.len() < string.len() {
                    ans = string;
                } else if ans.len() == string.len() {
                    match ans.cmp(string) {
                        std::cmp::Ordering::Less => {},
                        _ => {
                            ans = string;
                        }
                    }
                }
            }
        }
        ans.to_string()
    }
}