impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut x = x;
        let mut y = y;
        let mut count = 0;
        while x != 0 || y != 0 {
            if (x & 1) != (y & 1) {
                count += 1;
            }
            x >>= 1;
            y >>= 1;
        }
        count
    }
}