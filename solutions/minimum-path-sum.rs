impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let n: usize = grid.len();
        let m: usize = grid[0].len();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; m as usize]; n as usize];
        dp[0][0] = grid[0][0];
        for i in 0..n {
            for j in 0..m {
                if i == 0 && j == 0 {
                    continue;
                }
                if i == 0 {
                    dp[i][j] = dp[i][j - 1] + grid[i][j];
                } else if j == 0 {
                    dp[i][j] = dp[i - 1][j] + grid[i][j];
                } else {
                    use std::cmp::min;
                    dp[i][j] = min(dp[i - 1][j], dp[i][j - 1]) + grid[i][j];
                }
            }
        }
        return dp[n as usize - 1][m as usize - 1]
    }
}