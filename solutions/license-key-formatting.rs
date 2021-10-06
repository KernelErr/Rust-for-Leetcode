impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let mut n = 0;
        let mut chars: Vec<char> = Vec::new();
        let mut ans = String::new();
        for char in s.chars() {
            if char != '-' {
                chars.push(char.to_ascii_uppercase());
                n += 1;
            }
        }
        let m = n / k;
        let r = n - m * k;
        let mut idx = 0;
        for i in 0..r {
            ans.push(chars[idx]);
            idx += 1;
            if i == r - 1 && idx != chars.len() {
                ans.push('-');
            }
        }
        for i in 0..m {
            for _ in 0..k {
                ans.push(chars[idx]);
                idx += 1;
            }
            if i != m - 1 {
                ans.push('-');
            }
        }
        ans
    }
}