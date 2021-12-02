use std::str::FromStr;

use crate::{InputLines, Solution};

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
        let direction = match parts.next().unwrap() {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            d => panic!("Invalid direction: {}", d),
        };
        let value = parts.next().unwrap().parse().unwrap();

        Ok(Self { direction, value })
    }
}

pub(crate) struct Day2();

impl Solution for Day2 {
    fn solve(self, input_lines: InputLines) {
        let mut horizontal = 0;
        let mut depth = 0;
        let mut aim = 0;
        for command in input_lines
            .into_iter()
            .map(|line| line.unwrap())
            .map(|line| line.parse::<Command>().unwrap())
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
    }

    fn file_name(&self) -> &'static str {
        "day2"
    }
}
