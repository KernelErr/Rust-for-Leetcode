use std::cmp::max;
impl Solution {
    pub fn find_min_moves(machines: Vec<i32>) -> i32 {
        let tot: i32 = machines.iter().sum();
        if tot as usize % machines.len() != 0 {
            return -1;
        }
        let avg: i32 = (tot as usize / machines.len()) as i32;
        let mut ans: i32 = 0;
        let mut sum: i32 = 0;
        let mut machines: Vec<i32> = machines;
        for i in machines.iter_mut() {
            *i -= avg;
            sum += *i;
            ans = max(ans, max(sum.abs(), *i));
        }
        ans
    }
}