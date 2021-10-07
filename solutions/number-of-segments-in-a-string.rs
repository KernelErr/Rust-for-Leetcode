impl Solution {
    pub fn count_segments(s: String) -> i32 {
        let mut count = 0;
        let mut is_segment = false;
        for c in s.chars() {
            if c == ' ' {
                if is_segment {
                    count += 1;
                    is_segment = false;
                }
            } else {
                is_segment = true;
            }
        }
        if is_segment {
            count += 1;
        }
        count
    }
}