impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let n: usize = nums.len();
        let mut dp: Vec<i32> = vec![1; n];
        let mut count: Vec<i32> = vec![1; n];
        let mut max_len = 1;
        let mut ans = 0;
        for i in 0..nums.len() {
            for j in 0..i {
                if nums[j] < nums[i] {
                    if dp[i] == dp[j] + 1 {
                        count[i] += count[j];
                    } else if dp[i] < dp[j] + 1 {
                        dp[i] = dp[j] + 1;
                        count[i] = count[j];
                    }
                }
            }
            if dp[i] > max_len {
                max_len = dp[i];
                ans = count[i];
            } else if dp[i] == max_len {
                ans += count[i];
            }
        }
        ans
    }
}