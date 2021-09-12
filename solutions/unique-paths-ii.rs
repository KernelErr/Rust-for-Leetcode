impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let n = obstacle_grid[0].len();
        let m = obstacle_grid.len();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n as usize]; m as usize];
        dp[0][0] = match obstacle_grid[0][0] {
            0 => 1,
            _ => 0,
        };
        for i in 0..m as usize {
            for j in 0..n as usize {
                if i == 0 && j == 0 {
                    continue;
                }
                if obstacle_grid[i][j] != 0 {
                    continue;
                }
                if i == 0 {
                    dp[i][j] = dp[i][j - 1];
                } else if j == 0 {
                    dp[i][j] = dp[i - 1][j];
                } else {
                    dp[i][j] += dp[i - 1][j] + dp[i][j - 1];
                }
            }
        }
        dp[m as usize - 1][n as usize - 1]
    }
}