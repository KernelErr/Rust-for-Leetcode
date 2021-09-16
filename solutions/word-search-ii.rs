use std::collections::HashSet;

struct MySolution {
    board: Vec<Vec<char>>,
    words: Vec<String>,
    pub ans: Vec<String>,
    set: HashSet<String>,
    visited: Vec<Vec<bool>>,
    m: usize,
    n: usize,
}

impl MySolution {
    pub fn new(board: Vec<Vec<char>>, words: Vec<String>) -> MySolution {
        let ans: Vec<String> = Vec::new();
        let set: HashSet<String> = HashSet::new();
        let visited = vec![vec![false; 12]; 12];
        let m = board.len();;
        let n = board[0].len();
        MySolution {
            board,
            words,
            ans,
            set,
            visited,
            m,
            n,
        }
    }

    pub fn find_words(&mut self) -> Vec<String> {
        for word in &self.words {
            if self.set.contains(word) {
                continue;
            }
            self.set.insert(word.clone());
        }
        let mut cs: String = String::new();
        for i in 0..self.m {
            for j in 0..self.n {
                self.visited[i][j] = true;
                cs.push(self.board[i][j]);
                self.dfs(i, j, &mut cs);
                cs.pop();
                self.visited[i][j] = false;
            }
        }
        self.ans.clone()
    }

    pub fn dfs(&mut self, i: usize, j: usize, cs: &mut String) {
        if (cs.len() > 10) {
            return;
        }
        if (self.set.contains(cs)) {
            self.ans.push(cs.to_string());
            self.set.remove(cs);
        }
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nx: i32 = i as i32 + dx;
            let ny: i32 = j as i32 + dy;
            if (nx < 0 || nx >= self.m as i32 || ny < 0 || ny >= self.n as i32) {
                continue;
            }
            let nx: usize = nx as usize;
            let ny: usize = ny as usize;
            if (self.visited[nx][ny]) {
                continue;
            }
            self.visited[nx][ny] = true;
            cs.push(self.board[nx][ny]);
            self.dfs(nx, ny, cs);
            self.visited[nx][ny] = false;
            cs.pop();
        }
    }
}

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut my_solution = MySolution::new(board, words);
        my_solution.find_words()
    }
}