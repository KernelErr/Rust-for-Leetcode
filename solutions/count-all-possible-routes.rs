impl Solution {
    pub fn count_routes(locations: Vec<i32>, start: i32, finish: i32, fuel: i32) -> i32 {
        static MOD: i32 = 1_000_000_007;
        let n: usize = locations.len();
        let fuel: usize = fuel as usize;
        let mut dp: Vec<Vec<i32>> = vec![vec![0; fuel + 1]; n];
        for i in 0..=fuel {
            dp[finish as usize][i] = 1;
        }
        for i in 0..=fuel { // i is the current fuel
            for j in 0..n { // j is the target location
                for k in 0..n { // k is the current location
                    if j == k {
                        continue;
                    }
                    let need: usize = (locations[j] - locations[k]).abs() as usize;
                    if need <= i {
                        dp[j][i] += dp[k][i - need];
                        dp[j][i] %= MOD;
                    }
                }
            }
        }
        dp[start as usize][fuel]
    }
}