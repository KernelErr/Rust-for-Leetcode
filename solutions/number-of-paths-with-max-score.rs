use std::collections::HashMap;

impl Solution {
    pub fn paths_with_max_score(board: Vec<String>) -> Vec<i32> {
        let n = board.len();
        let mut ans = 0;
        let mut score = 0;
        const MOD: usize = 100_000_000_7;
        let mut dp: Vec<HashMap<usize, usize>> = vec![HashMap::new(); n * n];
        let directions = [(0, -1), (-1, 0), (-1, -1)];

        let get_idx = |x: usize, y: usize| {
            y * n + x
        };

        dp[get_idx(n - 1, n - 1)].insert(0, 1);
        for i in (0..n).rev() {
            for j in (0..n).rev() {
                if board[i].chars().nth(j).unwrap() == 'X' {
                    continue;
                }
                for dir in directions {
                    let origin_idx = get_idx(i as usize, j as usize);
                    let max_score = *match dp[origin_idx].keys().max() {
                        Some(score) => score,
                        None => continue,
                    };
                    let x = i as i32 + dir.0;
                    let y = j as i32 + dir.1;
                    if x < 0 || x >= n as i32 || y < 0 || y >= n as i32 {
                        continue;
                    }
                    let new_idx = get_idx(x as usize, y as usize);
                    let mut new_score = max_score + board[x as usize].chars().nth(y as usize).unwrap() as usize - '0' as usize;
                    if x == 0 && y == 0 {
                        new_score = max_score;
                    }
                    let ways = *dp[origin_idx].get(&max_score).unwrap_or(&0);
                    let new_ways = *dp[new_idx].get(&new_score).unwrap_or(&0);
                    println!("{} {} to {} {}: {} {}", i, j, x, y, new_score, ways);
                    dp[new_idx].insert(new_score,  (new_ways + ways) % MOD);
                    if x == 0 && y == 0 {
                        if new_score >= score {
                            score = new_score;
                            ans = (new_ways + ways) % MOD;
                        }
                    }
                }
            }
        }
        
        return vec![score as i32, ans as i32];
    }
}