use std::cmp::min;

impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        let n: usize = n as usize;
        let mut dp: Vec<i32> = vec![0; n + 1];
        for i in 2..=n {
            dp[i] = i32::MAX;
            for j in 1..i {
                if i % j == 0 {
                    dp[i] = min(dp[i], dp[j]+ i as i32 / j as i32);
                }
            }
        }
        dp[n]
    }
}