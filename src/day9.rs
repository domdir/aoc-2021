use std::collections::HashSet;

use crate::{Solution, SolutionResult};

pub(crate) struct Day9();

impl Solution for Day9 {
    fn solve(self, input_lines: impl Iterator<Item = String>) -> SolutionResult {
        let height_map = input_lines
            .map(|p| {
                p.chars()
                    .map(|c| c.to_string().parse::<usize>())
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        let mut h = HashSet::new();
        let mut basins = Vec::new();
        for i in 0..height_map.len() {
            for j in 0..height_map[i].len() {
                basins.push(count_basin(&height_map, i as isize, j as isize, &mut h));
            }
        }

        basins.sort();

        println!(
            "Basin size: {}",
            basins.iter().rev().take(3).product::<usize>()
        );

        Ok(())
    }

    fn file_name(&self) -> &'static str {
        "day9"
    }
}

fn count_basin(
    height_map: &Vec<Vec<usize>>,
    m: isize,
    n: isize,
    h: &mut HashSet<(usize, usize)>,
) -> usize {
    if m < 0 || n < 0 {
        return 0;
    }
    let i = m as usize;
    let j = n as usize;
    if h.contains(&(i, j)) {
        return 0;
    }
    match height_map.get(i) {
        Some(_) => match height_map[i].get(j) {
            None => 0,
            _ => {
                h.insert((i, j));
                if height_map[i][j] == 9 {
                    return 0;
                }
                let mut s = 0;
                s += count_basin(height_map, m + 1, n, h);
                s += count_basin(height_map, m - 1, n, h);
                s += count_basin(height_map, m, n + 1, h);
                s += count_basin(height_map, m, n - 1, h);
                s + 1
            }
        },
        None => 0,
    }
}
