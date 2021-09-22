impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut ms = s.clone();
        let mut ans: i32 = 0;
        while let Some(c) = ms.pop() {
            if c == ' ' {
                if ans > 0 {
                    break;
                }
            } else {
                ans += 1;
            }
        }
        ans
    }
}