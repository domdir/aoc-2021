use std::collections::HashMap;

use crate::{Solution, SolutionResult};

pub(crate) struct Day14();

impl Solution for Day14 {
    fn solve(self, mut input_lines: impl Iterator<Item = String>) -> SolutionResult {
        let mut counts: HashMap<u8, usize> = HashMap::new();
        let t = input_lines
            .next()
            .ok_or("No input")?
            .chars()
            .map(|f| f as u8)
            .collect::<Vec<_>>();
        t.iter().for_each(|c| {
            *counts.entry(*c).or_insert(0) += 1;
        });
        input_lines.next().unwrap();

        let ins = input_lines
            .filter_map(|f| {
                let x = f.split_once(" -> ").unwrap();
                let c = x.1.chars().next()? as u8;
                counts.entry(c).or_insert(0);
                let mut it = x.0.chars();
                Some(((it.next()? as u8, it.next()? as u8), c))
            })
            .collect::<HashMap<_, _>>();

        let n = t.clone();
        let windows = n.windows(2);
        let mut pairs = HashMap::new();
        windows.for_each(|win| *pairs.entry((win[0], win[1])).or_insert(0) += 1);

        for _ in 0..40 {
            for (val, count) in pairs.clone() {
                let (first, second) = val;
                let ch = ins[&val];
                *pairs.entry(val).or_insert(0) -= count;
                *pairs.entry((first, ch)).or_insert(0) += count;
                *pairs.entry((ch, second)).or_insert(0) += count;
                *counts.entry(ch).or_insert(0) += count;
            }
        }

        let max = counts.iter().map(|f| f.1).max().ok_or("Empty result")?;
        let min = counts.iter().map(|f| f.1).min().ok_or("Empty result")?;

        println!("Max ({}) - Min ({}) = Result ({})", max, min, max - min);

        Ok(())
    }

    fn file_name(&self) -> &'static str {
        "day14"
    }
}
