use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
    path::Path,
};

use day1::Day1;
use day2::Day2;
use day3::Day3;

mod day1;
mod day2;
mod day3;

type InputLines = Lines<BufReader<File>>;

trait Solution {
    fn file_name(&self) -> &'static str;
    fn solve(self, input_lines: InputLines);
}

fn main() {
    solve(Day1());
    solve(Day2());
    solve(Day3());
}

fn solve(s: impl Solution) {
    let file_name = s.file_name();
    let input_lines = read_file(Path::new("inputs").join(file_name).to_str().unwrap());
    println!("Solving {}:", file_name);
    s.solve(input_lines);
    println!();
}

fn read_file(file_name: &str) -> InputLines {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    reader.lines()
}
