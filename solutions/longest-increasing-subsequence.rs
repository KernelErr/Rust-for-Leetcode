impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let n: usize = nums.len();
        let mut dp: Vec<i32> = vec![1; n];
        let mut ans = 1;
        for i in 0..nums.len() {
            for j in 0..i {
                if nums[j] < nums[i] {
                    dp[i] = std::cmp::max(dp[i], dp[j] + 1);
                }
            }
            if dp[i] > ans {
                ans = dp[i];
            }
        }
        ans
    }
}