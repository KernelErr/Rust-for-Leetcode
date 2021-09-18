impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        let start_row = start_row as usize;
        let start_column = start_column as usize;

        let mut ans: usize = 0;
        const MOD: usize = 100_000_000_7;
        let mut moves: Vec<Vec<usize>> = vec![vec![0; n as usize]; m as usize];
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        moves[start_row][start_column] = 1;

        for _ in 0..max_move {
            let mut new_moves: Vec<Vec<usize>> = vec![vec![0; n as usize]; m as usize];
            for row in 0..m {
                for col in 0..n {
                    let count: usize = moves[row as usize][col as usize];
                    if count == 0 {
                        continue;
                    }
                    for (dx, dy) in directions {
                        let next_row = row + dx;
                        let next_col = col + dy;
                        if (next_row < 0 || next_row >= m || next_col < 0 || next_col >= n) {
                            ans = (ans + count) % MOD;
                        } else {
                            new_moves[next_row as usize][next_col as usize] = (new_moves[next_row as usize][next_col as usize] + count) % MOD;
                        }
                    }
                }
            }
            moves = new_moves;
        }

        ans as i32
    }
}