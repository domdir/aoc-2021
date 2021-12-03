use std::collections::VecDeque;

use crate::Solution;

const WINDOW_SIZE: usize = 3;

pub(crate) struct Day1();

impl Solution for Day1 {
    fn solve(self, input_lines: impl Iterator<Item = String>) {
        let mut increases = 0;
        let mut cur_depth = usize::MAX;
        for depth in input_lines
            .into_iter()
            .map(|line| line.parse::<usize>().unwrap())
            .scan(VecDeque::with_capacity(WINDOW_SIZE), |values, depth| {
                if values.len() >= WINDOW_SIZE {
                    values.pop_front();
                }
                values.push_back(depth);
                Some(values.iter().sum())
            })
            .skip(WINDOW_SIZE - 1)
        {
            if depth > cur_depth {
                increases += 1;
            }
            cur_depth = depth;
        }

        println!("{}", increases);
    }

    fn file_name(&self) -> &'static str {
        "day1"
    }
}
