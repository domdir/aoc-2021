#![feature(int_abs_diff)]

use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

type SolutionResult = Result<(), Box<dyn Error>>;

trait Solution {
    fn file_name(&self) -> &'static str;
    fn solve(self, input_lines: impl Iterator<Item = String>) -> SolutionResult;
}

fn main() {
    #[cfg(feature = "all")]
    {
        solve(day1::Day1());
        solve(day2::Day2());
        solve(day3::Day3::<{ day3::MEASUREMENT_LENGTH }>());
        solve(day4::Day4());
        solve(day5::Day5());
        solve(day6::Day6());
        solve(day7::Day7());
        solve(day8::Day8());
        solve(day9::Day9());
        solve(day10::Day10());
        solve(day11::Day11());
        solve(day12::Day12());
        solve(day13::Day13());
        solve(day14::Day14());
        solve(day15::Day15());
        solve(day16::Day16());
    }
    solve(day17::Day17());
}

fn solve(s: impl Solution) {
    let file_name = s.file_name();
    let input_path = Path::new("inputs").join(file_name);
    let path = match input_path.to_str() {
        Some(p) => p,
        None => {
            println!("Unable to create path to input file: {}", file_name);
            return;
        }
    };
    let input_lines = match read_file(path) {
        Ok(it) => it,
        Err(e) => {
            println!("Unable to read file {}: {}", path, e);
            return;
        }
    };
    println!("Solving {}:", file_name);
    if let Err(e) = s.solve(input_lines) {
        println!("Error: {}", e)
    }
    println!();
}

fn read_file(file_name: &str) -> Result<impl Iterator<Item = String>, std::io::Error> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    Ok(reader.lines().filter_map(|line| line.ok()))
}
