impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        use std::cmp::min;
        let n: usize = matrix.len();
        let m: usize = matrix[0].len();
        let mut dp = vec![vec![0; m]; n];
        for i in (0..n).rev() {
            for j in 0..m {
                if i == n - 1 {
                    dp[i][j] = matrix[i][j];
                } else {
                    if j == 0 {
                        dp[i][j] = matrix[i][j] + min(dp[i + 1][j], dp[i + 1][j + 1]);
                    } else if j == m - 1 {
                        dp[i][j] = matrix[i][j] + min(dp[i + 1][j - 1], dp[i + 1][j]);
                    } else {
                        dp[i][j] = matrix[i][j] + min(dp[i + 1][j], min(dp[i + 1][j - 1], dp[i + 1][j + 1]));
                    }
                }
            }
        }
        dp[0].iter().min().unwrap().clone()
    }
}