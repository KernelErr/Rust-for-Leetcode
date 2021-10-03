use std::collections::HashMap;
impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        let numerator: i64 = numerator as i64;
        let denominator: i64 = denominator as i64;
        if numerator == 0 {
            return "0".to_string();
        }
        let mut res = String::new();
        if (numerator < 0) ^ (denominator < 0) {
            res.push('-');
        }
        let mut num = numerator.abs();
        let mut den = denominator.abs();
        res.push_str(&(num / den).to_string());
        num %= den;
        if num != 0 {
            res.push('.');
        }
        let dot_pos = res.len();
        let mut map = HashMap::new();
        let mut index = 0;
        while num != 0 {
            num *= 10;
            let mut tmp = num / den;
            if let Some(i) = map.get(&num) {
                res.insert_str(dot_pos + i, "(");
                res.push(')');
                break;
            } else {
                map.insert(num, index);
                res.push_str(&tmp.to_string());
            }
            num %= den;
            index += 1;
        }
        res
    }
}