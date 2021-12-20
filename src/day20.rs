use std::collections::HashMap;

use crate::{Solution, SolutionResult};

pub(crate) struct Day20();

impl Solution for Day20 {
    fn solve(self, mut input_lines: impl Iterator<Item = String>) -> SolutionResult {
        let alg = input_lines
            .next()
            .unwrap()
            .chars()
            .map(|c| c == '#')
            .collect::<Vec<_>>();

        assert_eq!(alg.len(), 512);

        assert_eq!(input_lines.next().unwrap().len(), 0);

        let mut map = HashMap::<isize, HashMap<isize, bool>>::new();
        let mut size_i = 0;
        let mut size_j = 0;
        for (i, l) in input_lines.enumerate() {
            for (j, c) in l.chars().enumerate() {
                *map.entry(i as isize)
                    .or_default()
                    .entry(j as isize)
                    .or_default() = c == '#';
                size_j = size_j.max(j);
            }
            size_i = size_i.max(i);
        }

        let mut r = 0;
        for i in 0..50 {
            println!("{}/50", i + 1);
            r = enhance(
                &mut map,
                -200 + i,
                (size_i + 200 - i as usize) as isize,
                -200 + i,
                (size_j + 200 - i as usize) as isize,
                &alg,
            );
        }

        println!("Result: {}", r);

        Ok(())
    }

    fn file_name(&self) -> &'static str {
        "day20.txt"
    }
}

fn enhance(
    map: &mut HashMap<isize, HashMap<isize, bool>>,
    start_i: isize,
    end_i: isize,
    start_j: isize,
    end_j: isize,
    alg: &Vec<bool>,
) -> isize {
    let mut result: HashMap<isize, HashMap<isize, bool>> = HashMap::new();
    let mut count = 0;
    for i in start_i..=end_i {
        for j in start_j..=end_j {
            let mut n = String::new();
            n.push(b(*map.entry(i - 1).or_default().entry(j - 1).or_default()));
            n.push(b(*map.entry(i - 1).or_default().entry(j).or_default()));
            n.push(b(*map.entry(i - 1).or_default().entry(j + 1).or_default()));
            n.push(b(*map.entry(i).or_default().entry(j - 1).or_default()));
            n.push(b(*map.entry(i).or_default().entry(j).or_default()));
            n.push(b(*map.entry(i).or_default().entry(j + 1).or_default()));
            n.push(b(*map.entry(i + 1).or_default().entry(j - 1).or_default()));
            n.push(b(*map.entry(i + 1).or_default().entry(j).or_default()));
            n.push(b(*map.entry(i + 1).or_default().entry(j + 1).or_default()));

            let index = usize::from_str_radix(&n, 2).unwrap();
            *result.entry(i).or_default().entry(j).or_default() = alg[index];
            if alg[index] {
                count += 1;
            }
        }
    }
    *map = result;
    count
}

fn b(b: bool) -> char {
    if b {
        '1'
    } else {
        '0'
    }
}
