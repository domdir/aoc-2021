use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader, Lines},
    path::Path,
    str::FromStr,
};

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

struct Solution {
    input_lines: Lines<BufReader<File>>,
}

impl Solution {
    fn new(filename: &str) -> Solution {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        Solution {
            input_lines: reader.lines(),
        }
    }

    fn solve(self) {
        let mut horizontal = 0;
        let mut depth = 0;
        for command in self
            .input_lines
            .into_iter()
            .map(|line| line.unwrap())
            .map(|line| line.parse::<Command>().unwrap())
        {
            match command.direction {
                Direction::Forward => horizontal += command.value,
                Direction::Up => depth -= command.value,
                Direction::Down => depth += command.value,
            }
        }

        println!("{}", horizontal * depth);
    }
}

fn main() {
    let day = 2;

    let filename = format!("day{}", day);
    let solution = Solution::new(Path::new("inputs").join(filename).to_str().unwrap());
    solution.solve();
}
