use std::str::FromStr;

use crate::{Solution, SolutionResult};

enum Direction {
    Forward,
    Up,
    Down,
}

struct Command {
    direction: Direction,
    value: isize,
}

impl FromStr for Command {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let direction = match parts.next().ok_or("No direction found")? {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            d => panic!("Invalid direction: {}", d),
        };
        let value = parts.next().ok_or("No value found")?.parse()?;

        Ok(Self { direction, value })
    }
}

pub(crate) struct Day2();

impl Solution for Day2 {
    fn solve(self, input_lines: impl Iterator<Item = String>) -> SolutionResult {
        let mut horizontal = 0;
        let mut depth = 0;
        let mut aim = 0;
        for command in input_lines
            .into_iter()
            .map(|line| line.parse::<Command>())
            .collect::<Result<Vec<_>, _>>()?
        {
            match command.direction {
                Direction::Forward => {
                    horizontal += command.value;
                    depth += aim * command.value;
                }
                Direction::Up => aim -= command.value,
                Direction::Down => aim += command.value,
            }
        }

        println!("{}", horizontal * depth);
        Ok(())
    }

    fn file_name(&self) -> &'static str {
        "day2"
    }
}
