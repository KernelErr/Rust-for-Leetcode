impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut stack = Vec::new();
        let mut star = Vec::new();
        for (i, c) in s.chars().enumerate() {
            if c == '(' {
                stack.push(i);
            } else if c == '*' {
                star.push(i);
            } else {
                if stack.pop().is_none() && star.pop().is_none() {
                    return false;
                }
            }
        }
        while let Some(i) = star.pop() {
            if let Some(j) = stack.pop() {
                if j > i {
                    return false;
                }
            }
        }
        stack.is_empty()
    }
}