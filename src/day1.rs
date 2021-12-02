use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader, Lines},
    path::Path,
};

struct Solution {
    input_lines: Lines<BufReader<File>>,
}

impl Solution {
    const WINDOW_SIZE: usize = 3;

    fn new(filename: &str) -> Solution {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        Solution {
            input_lines: reader.lines(),
        }
    }

    fn solve(self) {
        let mut increases = 0;
        let mut cur_depth = usize::MAX;
        for depth in self
            .input_lines
            .into_iter()
            .map(|line| line.unwrap())
            .map(|line| line.parse::<usize>().unwrap())
            .scan(
                VecDeque::with_capacity(Self::WINDOW_SIZE),
                |values, depth| {
                    if values.len() >= Self::WINDOW_SIZE {
                        values.pop_front();
                    }
                    values.push_back(depth);
                    Some(values.iter().sum())
                },
            )
            .skip(Self::WINDOW_SIZE - 1)
        {
            if depth > cur_depth {
                increases += 1;
            }
            cur_depth = depth;
        }

        println!("{}", increases);
    }
}

fn main() {
    let day = 1;

    let filename = format!("day{}", day);
    let solution = Solution::new(Path::new("inputs").join(filename).to_str().unwrap());
    solution.solve();
}
