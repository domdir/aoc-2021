use crate::{Solution, SolutionResult};

pub(crate) struct Day10();

impl Solution for Day10 {
    fn solve(self, input_lines: impl Iterator<Item = String>) -> SolutionResult {
        let mut score = 0;
        let mut scores = vec![];
        'outer: for l in input_lines {
            let mut stack = vec![];
            for c in l.chars() {
                match c {
                    '(' => stack.push(')'),
                    '[' => stack.push(']'),
                    '{' => stack.push('}'),
                    '<' => stack.push('>'),
                    x => {
                        if stack.pop().unwrap_or('x') != x {
                            match x {
                                ')' => score += 3,
                                ']' => score += 57,
                                '}' => score += 1197,
                                '>' => score += 25137,
                                _ => {}
                            }
                            continue 'outer;
                        }
                    }
                }
            }
            // part 2
            let mut s = 0usize;
            while let Some(c) = stack.pop() {
                s *= 5;
                match c {
                    ')' => s += 1,
                    ']' => s += 2,
                    '}' => s += 3,
                    '>' => s += 4,
                    _ => {}
                }
            }
            scores.push(s);
        }
        println!("Part1: {}", score);
        scores.sort();
        println!("Part2: {}", scores[scores.len() / 2]);
        Ok(())
    }

    fn file_name(&self) -> &'static str {
        "day10"
    }
}
