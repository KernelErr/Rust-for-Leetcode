impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        let mut m = n;
        while m > 1 {
            if m % 3 != 0 {
                return false;
            }
            m /= 3;
        }
        m == 1
    }
}