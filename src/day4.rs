use crate::Solution;

struct Board {
    grid: Vec<Vec<usize>>,
    marked: Vec<Vec<bool>>,
    already_won: bool,
}

impl Board {
    fn check(&self) -> bool {
        if self.already_won {
            return false;
        }
        for i in 0..5 {
            if self.marked[i][0]
                && self.marked[i][1]
                && self.marked[i][2]
                && self.marked[i][3]
                && self.marked[i][4]
            {
                return true;
            }
            if self.marked[0][i]
                && self.marked[1][i]
                && self.marked[2][i]
                && self.marked[3][i]
                && self.marked[4][i]
            {
                return true;
            }
        }
        false
    }

    fn sum_unmarked(&self) -> usize {
        let mut sum = 0;
        for i in 0..5 {
            for j in 0..5 {
                if !self.marked[i][j] {
                    sum += self.grid[i][j];
                }
            }
        }
        sum
    }
}

pub(crate) struct Day4();

impl Solution for Day4 {
    fn solve(self, mut input_lines: impl Iterator<Item = String>) {
        let sequence = input_lines.next().unwrap();
        // remove empty line
        input_lines.next();
        let mut boards = vec![];
        while let Some(row) = input_lines.next() {
            let mut board = Board {
                grid: Vec::new(),
                marked: vec![
                    vec![false, false, false, false, false],
                    vec![false, false, false, false, false],
                    vec![false, false, false, false, false],
                    vec![false, false, false, false, false],
                    vec![false, false, false, false, false],
                ],
                already_won: false,
            };
            board.grid.push(get_board_row(&row));
            board.grid.push(get_board_row(&input_lines.next().unwrap()));
            board.grid.push(get_board_row(&input_lines.next().unwrap()));
            board.grid.push(get_board_row(&input_lines.next().unwrap()));
            board.grid.push(get_board_row(&input_lines.next().unwrap()));
            boards.push(board);
            // remove empty line
            input_lines.next();
        }
        let total_boards = boards.len();
        let mut boards_won = 0;
        for v in sequence.split(',').map(|s| s.parse::<usize>().unwrap()) {
            for b in &mut boards {
                for (i, row) in b.grid.iter().enumerate() {
                    for (j, num) in row.iter().enumerate() {
                        if num == &v {
                            b.marked[i][j] = true;
                            if b.check() {
                                b.already_won = true;
                                boards_won += 1;
                                if boards_won == 1 {
                                    println!("Score of winner: {}", b.sum_unmarked() * v);
                                }
                                if boards_won == total_boards {
                                    println!("Score of loser: {}", b.sum_unmarked() * v);
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn file_name(&self) -> &'static str {
        "day4"
    }
}

fn get_board_row(input: &str) -> Vec<usize> {
    let mut parts = input
        .split_whitespace()
        .map(|f| f.parse::<usize>().unwrap());
    vec![
        parts.next().unwrap(),
        parts.next().unwrap(),
        parts.next().unwrap(),
        parts.next().unwrap(),
        parts.next().unwrap(),
    ]
}
