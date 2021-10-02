impl Solution {
    pub fn to_hex(num: i32) -> String {
        if num == 0 {
            return "0".to_string();
        }
        let mut res = String::new();
        let mut num = num;
        let mut map = vec![
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
        ];
        while num != 0 {
            let idx = (num & 0xf) as usize;
            res.push(map[idx]);
            if res.len() >= 8 {
                break;
            }
            num >>= 4;
        }
        res.chars().rev().collect()
    }
}