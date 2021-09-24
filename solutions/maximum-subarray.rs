impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let n: usize = nums.len();
        let mut dp: Vec<i32> = Vec::new();
        dp.push(nums[0]);
        for i in 1..n {
            dp.push(nums[i] + if dp[i - 1] > 0 { dp[i - 1] } else { 0 });
        }
        *dp.iter().max().unwrap()
    }
}