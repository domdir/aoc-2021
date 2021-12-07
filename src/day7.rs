use crate::{Solution, SolutionResult};

pub(crate) struct Day7();

impl Solution for Day7 {
    fn solve(self, mut input_lines: impl Iterator<Item = String>) -> SolutionResult {
        let crabs = input_lines
            .next()
            .unwrap()
            .split(",")
            .map(|p| p.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?;

        let min = crabs.iter().min().ok_or("Input is empty")?.to_owned();
        let max = crabs.iter().max().ok_or("Input is empty")?.to_owned();
        let fuel = (min..=max)
            .map(|goal| {
                crabs
                    .iter()
                    // part 1
                    // .map(|&c| goal.abs_diff(c))
                    // part 2
                    // .map(|&c| (1usize..=goal.abs_diff(c)).sum::<usize>())
                    .map(|&c| {
                        let s = goal.abs_diff(c);
                        s * (s + 1) / 2
                    })
                    .sum::<usize>()
            })
            .min()
            .ok_or("Unable to calculate fuel")?;

        println!("Fuel: {}", fuel);

        Ok(())
    }

    fn file_name(&self) -> &'static str {
        "day7"
    }
}
