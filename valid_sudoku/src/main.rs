use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // Rows
        for row in board.iter() {
            if is_valid_row(row) {
                continue;
            } else {
                return false;
            }
        }

        // Columns
        for x in 0..board[0].len() {
            let mut seen: HashSet<char> = HashSet::new();
            for y in 0..board.len() {
                let item = board[y][x];
                if item != '.' {
                    if seen.contains(&item) {
                        return false;
                    } else {
                        seen.insert(item);
                    }
                }
            }
        }

        // Block
        for block_x in 0..3 {
            for block_y in 0..3 {
                let mut seen: HashSet<char> = HashSet::new();
                for x in 0..3 {
                    for y in 0..3 {
                        let item = board[y + block_y * 3][x + block_x * 3];
                        if item != '.' {
                            if seen.contains(&item) {
                                return false;
                            } else {
                                seen.insert(item);
                            }
                        }
                    }
                }
            }
        }

        return true;
    }
}

fn is_valid_row(row: &Vec<char>) -> bool {
    let mut seen: HashSet<char> = HashSet::new();
    for item in row {
        if *item != '.' {
            if seen.contains(item) {
                return false;
            } else {
                seen.insert(*item);
            }
        }
    }
    return true;
}

fn main() {
    {
        let board = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        println!("{}", Solution::is_valid_sudoku(board));
    }

    {
        let board = vec![
            vec!['.', '8', '7', '6', '5', '4', '3', '2', '1'],
            vec!['2', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['3', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['4', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['5', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['6', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['7', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['8', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['9', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];
        println!("{}", Solution::is_valid_sudoku(board));
    }

    {
        let board = vec![
            vec!['.', '.', '4', '.', '.', '.', '6', '3', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['5', '.', '.', '.', '.', '.', '.', '9', '.'],
            vec!['.', '.', '.', '5', '6', '.', '.', '.', '.'],
            vec!['4', '.', '3', '.', '.', '.', '.', '.', '1'],
            vec!['.', '.', '.', '7', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '5', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];
        println!("{}", Solution::is_valid_sudoku(board));
    }
}
