use std::collections::HashSet;

use crate::{Solution, SolutionResult};

pub(crate) struct Day11();

impl Solution for Day11 {
    fn solve(self, input_lines: impl Iterator<Item = String>) -> SolutionResult {
        let mut grid = input_lines
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse::<usize>())
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        for i in 1.. {
            let mut h = HashSet::new();
            grid.iter_mut()
                .for_each(|f| f.iter_mut().for_each(|f| *f += 1));

            while glow(&mut grid, &mut h) {}
            grid.iter_mut()
                .for_each(|f| f.iter_mut().filter(|f| **f > 9).for_each(|f| *f = 0));

            if h.len() == 100 {
                println!("After {} steps", i);
                break;
            }
        }
        Ok(())
    }

    fn file_name(&self) -> &'static str {
        "day11"
    }
}

fn glow(grid: &mut Vec<Vec<usize>>, h: &mut HashSet<(usize, usize)>) -> bool {
    let mut did_glow = false;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if h.contains(&(i, j)) {
                continue;
            }
            if grid[i][j] > 9 {
                did_glow = true;
                h.insert((i, j));
                let i = i as isize;
                let j = j as isize;
                inc(grid, i - 1, j - 1);
                inc(grid, i - 1, j);
                inc(grid, i - 1, j + 1);
                inc(grid, i, j - 1);
                inc(grid, i, j + 1);
                inc(grid, i + 1, j - 1);
                inc(grid, i + 1, j);
                inc(grid, i + 1, j + 1);
            }
        }
    }
    did_glow
}

fn inc(grid: &mut Vec<Vec<usize>>, i: isize, j: isize) {
    if i >= 0 && j >= 0 {
        let i = i as usize;
        let j = j as usize;
        match grid.get_mut(i) {
            Some(r) => match r.get_mut(j) {
                Some(x) => *x += 1,
                None => {}
            },
            None => {}
        }
    }
}
