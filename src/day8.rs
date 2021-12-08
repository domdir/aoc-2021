use std::collections::HashSet;

use crate::{Solution, SolutionResult};

pub(crate) struct Day8();

impl Solution for Day8 {
    fn solve(self, input_lines: impl Iterator<Item = String>) -> SolutionResult {
        let mut total = 0;
        for line in input_lines {
            let mut parts = line.split(" | ");
            let input = parts.next().ok_or("No input found")?;
            let output = parts.next().ok_or("No output found")?;

            let digits = input
                .split_whitespace()
                .chain(output.split_whitespace())
                .collect::<Vec<_>>();

            let mut possible_segments = vec![gen_possibilities(); 7];
            //   00
            // 1    2
            // 1    2
            //   33
            // 4    5
            // 4    5
            //   66
            for digit in &digits {
                match digit.len() {
                    2 => remain(&mut possible_segments, digit, &[2, 5], true),
                    3 => remain(&mut possible_segments, digit, &[2, 5, 0], true),
                    4 => remain(&mut possible_segments, digit, &[1, 2, 3, 5], true),
                    5 => remain(&mut possible_segments, digit, &[0, 3, 6], false),
                    6 => remain(&mut possible_segments, digit, &[0, 1, 5, 6], false),
                    _ => {}
                }
            }
            while remove_uniq(&mut possible_segments) {}

            let final_assignment = possible_segments
                .iter()
                .map(|segment| Some(segment.iter().next()?.to_owned()))
                .collect::<Option<Vec<_>>>()
                .ok_or("Not enough information")?;
            let mut inter_total = 0;
            for output_digit in output.split_whitespace() {
                inter_total *= 10;
                inter_total += to_digit(&final_assignment, output_digit)
                    .ok_or(format!("Invalid digit: {}", output_digit))?;
            }
            total += inter_total;
        }
        println!("{}", total);
        Ok(())
    }

    fn file_name(&self) -> &'static str {
        "day8"
    }
}

fn gen_possibilities() -> HashSet<char> {
    let mut all = HashSet::new();
    all.insert('a');
    all.insert('b');
    all.insert('c');
    all.insert('d');
    all.insert('e');
    all.insert('f');
    all.insert('g');
    all
}

fn remain(possible_segments: &mut Vec<HashSet<char>>, digit: &str, allowed: &[usize], exact: bool) {
    let mut remaining_segments = gen_possibilities();
    for segment in digit.chars() {
        remaining_segments.remove(&segment);
    }
    for (i, possible_segment) in possible_segments.into_iter().enumerate() {
        if allowed.contains(&i) {
            for segment in &remaining_segments {
                possible_segment.remove(segment);
            }
        } else {
            if exact {
                for segment in digit.chars() {
                    possible_segment.remove(&segment);
                }
            }
        }
    }
}

fn remove_uniq(possible_segments: &mut Vec<HashSet<char>>) -> bool {
    let uniq_segments = possible_segments
        .iter()
        .filter_map(|p| {
            if p.len() == 1 {
                Some(p.iter().next()?.to_owned())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let mut removed = false;
    for uniq_segment in uniq_segments {
        for possible_segment in possible_segments.iter_mut() {
            if possible_segment.len() > 1 {
                removed |= possible_segment.remove(&uniq_segment);
            }
        }
    }
    removed
}

fn to_digit(assignment: &Vec<char>, digit: &str) -> Option<usize> {
    let digit_set = digit.chars().collect::<HashSet<_>>();
    let mut segment_order = assignment
        .iter()
        .enumerate()
        .filter_map(|(i, segment)| digit_set.contains(&segment).then(|| i))
        .collect::<Vec<_>>();
    segment_order.sort();
    Some(match segment_order[..] {
        [2, 5] => 1,
        [0, 2, 3, 4, 6] => 2,
        [0, 2, 3, 5, 6] => 3,
        [1, 2, 3, 5] => 4,
        [0, 1, 3, 5, 6] => 5,
        [0, 1, 3, 4, 5, 6] => 6,
        [0, 2, 5] => 7,
        [0, 1, 2, 3, 4, 5, 6] => 8,
        [0, 1, 2, 3, 5, 6] => 9,
        [0, 1, 2, 4, 5, 6] => 0,
        _ => return None,
    })
}
