impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        use std::cmp::max;
        const MOD: i64 = 1_000_000_007;
        let n: usize = s.len();
        let mut dp: Vec<i64> = vec![0; n + 1];
        let s: Vec<char> = s.chars().collect();
        dp[0] = 1;
        for i in 1..=n {
            if s[i - 1] == '*' {
                dp[i] = (dp[i - 1] * 9) % MOD;
                if i > 1 {
                    if s[i - 2] == '1' {
                        dp[i] = (dp[i] + dp[i - 2] * 9) % MOD;
                    } else if s[i - 2] == '2' {
                        dp[i] = (dp[i] + dp[i - 2] * 6) % MOD;
                    } else if s[i - 2] == '*' {
                        dp[i] = (dp[i] + dp[i - 2] * 15) % MOD;
                    }
                }
            } else {
                if s[i - 1] == '0' {
                    dp[i] = 0;
                } else {
                    dp[i] = dp[i - 1];
                }
                if i > 1 {
                    if s[i - 2] == '1' {
                        dp[i] = (dp[i] + dp[i - 2]) % MOD;
                    } else if s[i - 2] == '2' && s[i - 1] <= '6' {
                        dp[i] = (dp[i] + dp[i - 2]) % MOD;
                    } else if s[i - 2] == '*' {
                        if s[i - 1] <= '6' {
                            dp[i] = (dp[i] + dp[i - 2] * 2) % MOD;
                        } else {
                            dp[i] = (dp[i] + dp[i - 2]) % MOD;
                        }
                    }
                }
            }
        }
        dp[n] as i32
    }
}