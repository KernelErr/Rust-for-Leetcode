impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let n: usize = board.len();

        let get_col = |col| {
            let mut vec: Vec<char> = Vec::new();
            for i in 0..n {
                vec.push(board[i][col]);
            }
            vec
        };

        let get_block = |num| {
            let mut vec: Vec<char> = Vec::new();
            let block_x = (num - 1) % 3 * 3;
            let block_y = ((num - 1) / 3) * 3;
            for i in block_x..=block_x + 2 {
                for j in block_y..=block_y + 2 {
                    vec.push(board[i as usize][j as usize]);
                }
            }
            // println!("{} {:?}", num, vec);
            vec
        };

        for i in 0..n {
            let chars = board[i].clone();
            let mut map = vec![0; 10];
            // println!("{:?}", chars);
            for char in chars {
                if char != '.' {
                    let idx = char as usize - '0' as usize;
                    if map[idx] != 0 {
                        return false;
                    }
                    map[idx] = 1;
                }
            }
        }

        for i in 0..n {
            let chars = get_col(i);
            let mut map = vec![0; 10];
            // println!("{:?}", chars);
            for char in chars {
                if char != '.' {
                    let idx = char as usize - '0' as usize;
                    if map[idx] != 0 {
                        return false;
                    }
                    map[idx] = 1;
                }
            }
        }

        for i in 1..=9 {
            let chars = get_block(i);
            let mut map = vec![0; 10];
            // println!("{:?}", chars);
            for char in chars {
                if char != '.' {
                    let idx = char as usize - '0' as usize;
                    if map[idx] != 0 {
                        return false;
                    }
                    map[idx] = 1;
                }
            }
        }

        true
    }
}