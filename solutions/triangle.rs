impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut dp = triangle.last().unwrap().to_vec();
        for i in (0..triangle.len() - 1).rev() {
            for j in 0..i + 1 {
                dp[j] = triangle[i][j] + std::cmp::min(dp[j], dp[j + 1]);
            }
        }
        dp[0]
    }
}