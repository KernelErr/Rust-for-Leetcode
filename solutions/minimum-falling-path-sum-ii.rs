impl Solution {
    pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        use std::cmp::min;
        let n: usize = grid.len();
        let mut dp = vec![vec![0; n]; n];
        for i in (0..n).rev() {
            for j in 0..n {
                if i == n - 1 {
                    dp[i][j] = grid[i][j];
                } else {
                    let mut min_n = std::i32::MAX;
                    for k in 0..n {
                        if k == j {
                            continue;
                        }
                        min_n = min(min_n, dp[i + 1][k]);
                    }
                    dp[i][j] = min_n + grid[i][j];
                }
            }
        }
        dp[0].iter().min().unwrap().clone()
    }
}